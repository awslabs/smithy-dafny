package software.amazon.polymorph.smithypython.customize;

import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;
import software.amazon.polymorph.smithypython.shapevisitor.DafnyToSmithyShapeVisitor;
import software.amazon.polymorph.smithypython.shapevisitor.SmithyToDafnyShapeVisitor;
import software.amazon.polymorph.smithypython.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithypython.nameresolver.SmithyNameResolver;
import software.amazon.polymorph.smithypython.nameresolver.Utils;
import software.amazon.polymorph.traits.LocalServiceTrait;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.traits.ErrorTrait;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonWriter;

/**
 * Writes the shim.py file.
 * The shim wraps the client.py implementation (which itself wraps the underlying Dafny implementation).
 * Other Dafny-generated Python code will use the Shim class to interact with this project's Dafny implementation.
 */
public class ShimFileWriter implements CustomFileWriter {

  @Override
  public void customizeFileForServiceShape(
      ServiceShape serviceShape, GenerationContext codegenContext) {
    String typesModulePrelude = DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(serviceShape.getId());
    String moduleName = codegenContext.settings().getModuleName();
    codegenContext.writerDelegator().useFileWriter(moduleName + "/shim.py", "", writer -> {
      writer.addImport(".errors", "ServiceError");
      writer.addImport(".errors", "CollectionOfErrors");
      writer.addImport(".errors", "OpaqueError");

      writer.write(
          """
          import Wrappers
          import asyncio
          import $L
          import $L.smithygenerated.client as client_impl
                          
          def smithy_error_to_dafny_error(e: ServiceError):
              ${C|}
                          
          class $L($L.$L):
              def __init__(self, _impl: client_impl) :
                  self._impl = _impl
                          
              ${C|}
              
              """, typesModulePrelude, moduleName,
          writer.consumer(w -> generateErrorsBlock(codegenContext, serviceShape, w)),
          SmithyNameResolver.shimForService(serviceShape),
          typesModulePrelude, DafnyNameResolver.getDafnyClientInterfaceTypeForServiceShape(serviceShape),
          writer.consumer(w -> generateOperationsBlock(codegenContext, serviceShape, w))
      );
    });
  }

  private void generateErrorsBlock(
      GenerationContext codegenContext, ServiceShape serviceShape, PythonWriter writer) {

    // Write modelled error converters for this service
    TreeSet<ShapeId> errorShapeSet = new TreeSet<ShapeId>(
        codegenContext.model().getStructureShapesWithTrait(ErrorTrait.class)
            .stream()
            .filter(structureShape -> structureShape.getId().getNamespace()
                .equals(codegenContext.settings().getService().getNamespace()))
            .map(Shape::getId)
            .collect(Collectors.toSet()));
    for (ShapeId errorShapeId : errorShapeSet) {
      SmithyNameResolver.importSmithyGeneratedTypeForShape(writer, errorShapeId, codegenContext);
      writer.write("""
              if isinstance(e, $L):
                  return $L.$L(message=e.message)
              """,
          errorShapeId.getName(),
          DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(errorShapeId),
          DafnyNameResolver.getDafnyTypeForError(errorShapeId)
      );
    }

    // Write out wrapping errors for dependencies.
    // This service will generate a dependency-specific error for each dependency.
    // This dependency-specific error is only known to this service, and not to the dependency service.
    // The Dafny type for this error contains one member: the dependency's name.
    // ex. Dafny type for MyDependency: `Error_MyDependency(Error...) = { MyDependency: ... }`
    // This member's value can take on any of the error types modelled in the dependency.
    // Polymorph will generate a similar error structure in the primary service's errors.py file.
    // ex. Smithy type for MyDependency: `MyDependency(ApiError...) = { MyDependency: ... }`
    LocalServiceTrait localServiceTrait = serviceShape.getTrait(LocalServiceTrait.class).get();
    Set<ShapeId> serviceDependencyShapeIds = localServiceTrait.getDependencies();
    for (ShapeId serviceDependencyShapeId : serviceDependencyShapeIds) {
      writer.addImport(
          SmithyNameResolver.getPythonModuleNamespaceForSmithyNamespace(serviceDependencyShapeId.getNamespace())
              + ".smithygenerated.shim",
          "smithy_error_to_dafny_error",
          SmithyNameResolver.getPythonModuleNamespaceForSmithyNamespace(serviceDependencyShapeId.getNamespace())
              + "_smithy_error_to_dafny_error"
      );
      writer.addImport(".errors", serviceDependencyShapeId.getName());
      // Generate conversion method that says:
      // "If this is a dependency-specific error, defer to the dependency's `smithy_error_to_dafny_error`"
      writer.write("""
              if isinstance(e, $L):
                  return $L.Error_$L($L(e.message))
              """,
          serviceDependencyShapeId.getName(),
          DafnyNameResolver.getDafnyTypesModuleNamespaceForShape(serviceShape),
          serviceDependencyShapeId.getName(),
          SmithyNameResolver.getPythonModuleNamespaceForSmithyNamespace(serviceDependencyShapeId.getNamespace())
              + "_smithy_error_to_dafny_error"
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
      GenerationContext codegenContext, ServiceShape serviceShape, PythonWriter writer) {

    for (ShapeId operationShapeId : serviceShape.getOperations()) {
      OperationShape operationShape = codegenContext.model().expectShape(operationShapeId, OperationShape.class);

      // Add imports for operation errors
      for (ShapeId errorShapeId : operationShape.getErrors(serviceShape)) {
        SmithyNameResolver.importSmithyGeneratedTypeForShape(writer, errorShapeId, codegenContext);
      }

      ShapeId inputShape = operationShape.getInputShape();
      ShapeId outputShape = operationShape.getOutputShape();
      // Import Dafny types for inputs and outputs
      DafnyNameResolver.importDafnyTypeForShape(writer, inputShape);
      DafnyNameResolver.importDafnyTypeForShape(writer, outputShape);
      // Import Smithy types for inputs and outputs
      SmithyNameResolver.importSmithyGeneratedTypeForShape(writer, inputShape, codegenContext);
      SmithyNameResolver.importSmithyGeneratedTypeForShape(writer, outputShape, codegenContext);
      
      // Write the Shim operation block.
      // This takes in a Dafny input and returns a Dafny output.
      // This operation will:
      //  1) Convert the Dafny input to a Smithy-modelled input,
      //  2) Call the Smithy-generated client with the transformed input, and
      //  3) Convert the Smithy output to the Dafny type.
      writer.openBlock("def $L(self, $L) -> $L:", "",
          operationShape.getId().getName(),
          // Do not generate an `input` parameter if the operation does not take in an input
          Utils.isUnitShape(inputShape) ? "" : "input: " + DafnyNameResolver.getDafnyTypeForShape(inputShape),
          // Return `None` type if the operation does not return an output
          Utils.isUnitShape(outputShape) ? "None" : DafnyNameResolver.getDafnyTypeForShape(outputShape),
          () -> {

            Shape targetShapeInput = codegenContext.model().expectShape(operationShape.getInputShape());
            // Generate code that converts the input from the Dafny type to the corresponding Smithy type
            // `input` will hold a string that converts the Dafny `input` to the Smithy-modelled output.
            // This has a side effect of possibly writing transformation code at the writer's current position.
            // For example, a service shape may require some calls to `ctor__()` after it is created,
            //   and cannot be constructed inline.
            // Polymorph will create an object representing the service's client, instantiate it,
            //   then reference that object in its `input` string.
            String input = targetShapeInput.accept(new DafnyToSmithyShapeVisitor(
                codegenContext,
                "input",
                writer,
                false
            ));

            // Generate code that:
            // 1) "unwraps" the request (converts from the Dafny type to the Smithy type),
            // 2) calls Smithy client,
            // 3) wraps Smithy failures as Dafny failures
            writer.write(
              """
              unwrapped_request: $L = $L
              try:
                  wrapped_response = asyncio.run(self._impl.$L(unwrapped_request))
              except ServiceError as e:
                  return Wrappers.Result_Failure(smithy_error_to_dafny_error(e))
                      
              """,
              inputShape.getName(),
              input,
              codegenContext.symbolProvider().toSymbol(operationShape).getName()
            );

            Shape targetShape = codegenContext.model().expectShape(operationShape.getOutputShape());
            // Generate code that converts the output from Smithy type to the corresponding Dafny type
            // This has a side effect of possibly writing transformation code at the writer's current position.
            String output = targetShape.accept(new SmithyToDafnyShapeVisitor(
                codegenContext,
                "wrapped_response",
                writer,
                false
            ));

            // Generate code that wraps Smithy success shapes as Dafny success shapes
            writer.write(
                """
                return Wrappers.Result_Success($L)
                """,
                Utils.isUnitShape(outputShape) ? "None" : output
            );
          }
      );
    }
  }
}
