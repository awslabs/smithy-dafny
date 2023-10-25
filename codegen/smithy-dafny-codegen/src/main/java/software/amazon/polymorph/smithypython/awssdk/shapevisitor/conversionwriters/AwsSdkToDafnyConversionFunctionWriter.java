package software.amazon.polymorph.smithypython.awssdk.shapevisitor.conversionwriters;

import java.util.Map.Entry;
import software.amazon.polymorph.smithypython.awssdk.nameresolver.AwsSdkNameResolver;
import software.amazon.polymorph.smithypython.common.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithypython.common.nameresolver.SmithyNameResolver;
import software.amazon.polymorph.smithypython.awssdk.shapevisitor.AwsSdkToDafnyShapeVisitor;
import software.amazon.polymorph.smithypython.common.shapevisitor.conversionwriter.BaseConversionWriter;
import software.amazon.smithy.codegen.core.WriterDelegator;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.StructureShape;
import software.amazon.smithy.model.shapes.UnionShape;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonWriter;

/**
 * Writes the aws_sdk_to_dafny.py file via the BaseConversionWriter implementation.
 */
public class AwsSdkToDafnyConversionFunctionWriter extends BaseConversionWriter {

  // Use a singleton to preserve generatedShapes through multiple generations
  static AwsSdkToDafnyConversionFunctionWriter singleton;

  private AwsSdkToDafnyConversionFunctionWriter() { }

  // Instantiate singleton at class-load time
  static {
    singleton = new AwsSdkToDafnyConversionFunctionWriter();
  }

  /**
   * Delegate writing conversion methods for the provided shape and its member shapes
   * @param shape
   * @param context
   * @param writer
   */
  public static void writeConverterForShapeAndMembers(Shape shape, GenerationContext context,
      PythonWriter writer) {
    singleton.baseWriteConverterForShapeAndMembers(shape, context, writer);
  }

  protected void writeStructureShapeConverter(StructureShape structureShape) {
    WriterDelegator<PythonWriter> delegator = context.writerDelegator();
    String moduleName = context.settings().getModuleName();

    delegator.useFileWriter(moduleName + "/aws_sdk_to_dafny.py", "", conversionWriter -> {
      // Within the conversion function, the dataSource becomes the function's input
      // This hardcodes the input parameter name for a conversion function to always be "input"
      String dataSourceInsideConversionFunction = "input";

      DafnyNameResolver.importDafnyTypeForShape(conversionWriter, structureShape.getId(), context);

      conversionWriter.openBlock(
          "def $L($L):",
          "",
          AwsSdkNameResolver.getAwsSdkToDafnyFunctionNameForShape(structureShape),
          dataSourceInsideConversionFunction,
          () -> {
            // Open Dafny structure shape
            // e.g.
            // DafnyStructureName(...
            conversionWriter.openBlock(
                "return $L(",
                ")",
                DafnyNameResolver.getDafnyTypeForShape(structureShape),
                () -> {
                  for (Entry<String, MemberShape> memberShapeEntry : structureShape.getAllMembers().entrySet()) {
                    String memberName = memberShapeEntry.getKey();
                    MemberShape memberShape = memberShapeEntry.getValue();
                    final Shape targetShape = context.model().expectShape(memberShape.getTarget());

                    // Adds `DafnyStructureMember=smithy_structure_member(...)`
                    // e.g.
                    // DafnyStructureName(DafnyStructureMember=smithy_structure_member(...), ...)
                    // The nature of the `smithy_structure_member` conversion depends on the properties of the shape,
                    //   as described below
                    conversionWriter.writeInline("$L=", memberName);

                    // If this shape is optional, write conversion logic to detect and possibly pass
                    //   an empty optional at runtime
                    if (memberShape.isOptional()) {
                      conversionWriter.addStdlibImport("Wrappers", "Option_Some");
                      conversionWriter.addStdlibImport("Wrappers", "Option_None");
                      conversionWriter.write("Option_Some($L) if \"$L\" in $L.keys() else Option_None(),",
                          targetShape.accept(
                              new AwsSdkToDafnyShapeVisitor(
                                  context,
                                  dataSourceInsideConversionFunction + "[\"" + memberName + "\"]",
                                  writer
                              )
                          ),
                          memberName,
                          dataSourceInsideConversionFunction
                      );
                    }

                    // If this shape is required, pass in the shape for conversion without any optional-checking
                    else {
                      conversionWriter.write("$L,",
                          targetShape.accept(
                              new AwsSdkToDafnyShapeVisitor(
                                  context,
                                  dataSourceInsideConversionFunction + "[\"" + memberName + "\"]",
                                  writer
                              )
                          )
                      );
                    }
                  }
                });
          }
      );
    });
  }

  /**
   * Writes a function definition to convert a Smithy-modelled union shape
   *   into the corresponding Dafny-modelled union shape.
   * The function definition is written into `aws_sdk_to_dafny.py`.
   * This SHOULD only be called once so only one function definition is written.
   * @param unionShape
   */
  protected void writeUnionShapeConverter(UnionShape unionShape) {
    WriterDelegator<PythonWriter> delegator = context.writerDelegator();
    String moduleName = context.settings().getModuleName();

    delegator.useFileWriter(moduleName + "/aws_sdk_to_dafny.py", "", conversionWriter -> {

      // Within the conversion function, the dataSource becomes the function's input
      // This hardcodes the input parameter name for a conversion function to always be "input"
      String dataSourceInsideConversionFunction = "input";

      // ex. shape: simple.union.ExampleUnion
      // Writes `def SmithyToDafny_simple_union_ExampleUnion(input):`
      //   and wraps inner code inside function definition
      conversionWriter.openBlock(
          "def $L($L):",
          "",
          AwsSdkNameResolver.getAwsSdkToDafnyFunctionNameForShape(unionShape),
          dataSourceInsideConversionFunction,
          () -> {

            // First union value opens a new `if` block; others do not need to and write `elif`
            boolean shouldOpenNewIfBlock = true;
            for (MemberShape memberShape : unionShape.getAllMembers().values()) {
              final Shape targetShape = context.model().expectShape(memberShape.getTarget());
              // Write out conversion:
              // ex. if ExampleUnion can take on either of (IntegerValue, StringValue), write:
              // if isinstance(input, ExampleUnion.IntegerValue):
              //   example_union_union_value = DafnyExampleUnionIntegerValue(input.member.value)
              // elif isinstance(input, ExampleUnion.StringValue):
              //   example_union_union_value = DafnyExampleUnionIntegerValue(input.member.value)
              conversionWriter.write("""
                        $L "$L" in $L.keys():
                            $L_union_value = $L($L)""",
                  // If we need a new `if` block, open one; otherwise, expand on existing one with `elif`
                  shouldOpenNewIfBlock ? "if" : "elif",
                  memberShape.getMemberName(),
                  dataSourceInsideConversionFunction,

                  unionShape.getId().getName(),
                  DafnyNameResolver.getDafnyTypeForUnion(unionShape, memberShape),
                  targetShape.accept(
                      new AwsSdkToDafnyShapeVisitor(
                          context,
                          dataSourceInsideConversionFunction + "[\"" + memberShape.getMemberName() + "\"]",
                          writer
                      )
                  )
              );
              shouldOpenNewIfBlock = false;

              DafnyNameResolver.importDafnyTypeForUnion(conversionWriter, unionShape, memberShape);
            }

            // Write case to handle if union member does not match any of the above cases
            conversionWriter.write("""
                      else:
                          raise ValueError("No recognized union value in union type: " + $L)
                      """,
                dataSourceInsideConversionFunction
            );

            // Return the result of the union conversion
            // `return example_union_union_value`
            conversionWriter.write("return %1$s_union_value".formatted(unionShape.getId().getName()));
          });
    });
  }

}
