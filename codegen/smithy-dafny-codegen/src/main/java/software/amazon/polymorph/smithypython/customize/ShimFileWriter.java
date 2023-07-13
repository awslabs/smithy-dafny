package software.amazon.polymorph.smithypython.customize;

import java.util.HashSet;
import java.util.Set;
import java.util.TreeSet;
import software.amazon.polymorph.smithypython.DafnyProtocolGenerator.DafnyMemberDeserVisitor;
import software.amazon.polymorph.smithypython.DafnyProtocolGenerator.DafnyMemberSerVisitor;
import software.amazon.polymorph.smithypython.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithypython.nameresolver.PythonNameResolver;
import software.amazon.polymorph.smithypython.nameresolver.Utils;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonWriter;

public class ShimFileWriter implements CustomFileWriter {

  @Override
  public void generateFileForServiceShape(
      ServiceShape serviceShape, GenerationContext codegenContext) {

    Set<ShapeId> allInputShapesSet = new HashSet<>();
    Set<ShapeId> allOutputShapesSet = new HashSet<>();
    String typesModulePrelude = DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(serviceShape.getId());

    String moduleName =  codegenContext.settings().getModuleName();
    codegenContext.writerDelegator().useFileWriter(moduleName + "/shim.py", "", writer -> {
      writer.addImport(".errors", "ServiceError");
      writer.addImport(".errors", "CollectionOfErrors");
      writer.addImport(".errors", "OpaqueError");

      writer.write(
          """
          import Wrappers_Compile
          import asyncio
          import $L
          import $L.smithy_generated.$L.client as client_impl
         
                          
          def smithy_error_to_dafny_error(e: ServiceError):
              ${C|}
                          
          class $L($L.$L):
              def __init__(self, _impl: client_impl) :
                  self._impl = _impl
                          
              ${C|}
              
              """, typesModulePrelude, moduleName, moduleName,
          writer.consumer(w -> generateErrorsBlock(codegenContext, serviceShape, w)),
          PythonNameResolver.shimForService(serviceShape),
          typesModulePrelude, DafnyNameResolver.getDafnyClientInterfaceTypeForServiceShape(serviceShape),
          writer.consumer(w -> generateOperationsBlock(codegenContext, serviceShape, w))
      );
    });
  }

  private void generateErrorsBlock(
      GenerationContext codegenContext,
      ServiceShape serviceShape, PythonWriter writer) {
    // TODO: StringBuilder? Writer?

    // Write modelled error converters
    Set<ShapeId> errorShapeSet = new HashSet<>();
    for (ShapeId operationShapeId : serviceShape.getAllOperations()) {
      OperationShape operationShape = codegenContext.model()
          .expectShape(operationShapeId, OperationShape.class);

      for (ShapeId errorShapeId : operationShape.getErrors(serviceShape)) {
        errorShapeSet.add(errorShapeId);
      }
    }

    for (ShapeId errorShapeId : errorShapeSet) {
      writer.write("""
              if isinstance(e, $L):
                  return $L.$L(message=e.message)
              """,
          errorShapeId.getName(),
          DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(errorShapeId),
          DafnyNameResolver.getDafnyTypeForError(errorShapeId)
      );
    }

    // Add service-specific CollectionOfErrors
    writer.write("""
            if isinstance(e, CollectionOfErrors):
                return $L.Error_CollectionOfErrors(message=e.message, list=e.list)
            """,
        DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(serviceShape.getId())
    );
    // Add service-specific OpaqueError
    writer.write("""
            if isinstance(e, OpaqueError):
                return $L.Error_Opaque(obj=e.obj)
            """,
        DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(serviceShape.getId())
    );
  }

  private void generateOperationsBlock(
      GenerationContext codegenContext,
      ServiceShape serviceShape, PythonWriter writer) {

    // TODO: .getAllOperations? Maybe .getOperations? or .getIntroducedOperations?
    // Might learn which one when working on Resources
    for (ShapeId operationShapeId : serviceShape.getAllOperations()) {
      OperationShape operationShape = codegenContext.model().expectShape(operationShapeId, OperationShape.class);

      // Add imports for operation errors
      for (ShapeId errorShapeId : operationShape.getErrors(serviceShape)) {
        writer.addImport(".errors", errorShapeId.getName());
      }

      ShapeId inputShape = operationShape.getInputShape();
      ShapeId outputShape = operationShape.getOutputShape();

      String dafnyInputType = DafnyNameResolver.getDafnyTypeForShape(inputShape);
      String dafnyOutputType = DafnyNameResolver.getDafnyTypeForShape(outputShape);
      String operationSymbol = codegenContext.symbolProvider().toSymbol(operationShape).getName();

      DafnyNameResolver.importDafnyTypeForShape(writer, inputShape);
      DafnyNameResolver.importDafnyTypeForShape(writer, outputShape);

      writer.addImport(".models", inputShape.getName());
      writer.addImport(".models", outputShape.getName());

      Shape targetShape = codegenContext.model().expectShape(operationShape.getOutputShape());
      var output = targetShape.accept(new DafnyMemberDeserVisitor(
          codegenContext,
          "wrapped_response",
          true
      ));

      Shape targetShapeInput = codegenContext.model().expectShape(operationShape.getInputShape());
      var input = targetShapeInput.accept(new DafnyMemberSerVisitor(
          codegenContext,
          "input",
          false
      ));

      writer.write("""
          def $L(self, $L) -> $L:
              unwrapped_request: $L = $L($L)
              try:
                  wrapped_response = asyncio.run(self._impl.$L(unwrapped_request))
              except ServiceError as e:
                  return Wrappers_Compile.Result_Failure(smithy_error_to_dafny_error(e))
              return Wrappers_Compile.Result_Success($L$L)

          """,
          operationShape.getId().getName(),
          Utils.isUnitShape(inputShape) ? "" : "input: " + dafnyInputType,
          Utils.isUnitShape(outputShape) ? "None" : dafnyOutputType,
          inputShape.getName(),
          inputShape.getName(),
          input,
          operationSymbol,
          Utils.isUnitShape(outputShape) ? "None" : dafnyOutputType,
          Utils.isUnitShape(outputShape) ? "" : "(" + output + ")"
      );
    }
  }
}
