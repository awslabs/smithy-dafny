package software.amazon.polymorph.smithygo.awssdk;

import static software.amazon.polymorph.smithygo.localservice.DafnyLocalServiceTypeConversionProtocol.TO_DAFNY;
import static software.amazon.polymorph.smithygo.localservice.DafnyLocalServiceTypeConversionProtocol.TO_NATIVE;

import java.util.HashSet;
import java.util.Set;
import software.amazon.polymorph.smithygo.awssdk.shapevisitor.AwsSdkToDafnyShapeVisitor;
import software.amazon.polymorph.smithygo.awssdk.shapevisitor.DafnyToAwsSdkShapeVisitor;
import software.amazon.polymorph.smithygo.codegen.AddOperationShapes;
import software.amazon.polymorph.smithygo.codegen.ApplicationProtocol;
import software.amazon.polymorph.smithygo.codegen.GenerationContext;
import software.amazon.polymorph.smithygo.codegen.GoDelegator;
import software.amazon.polymorph.smithygo.codegen.SmithyGoDependency;
import software.amazon.polymorph.smithygo.codegen.integration.ProtocolGenerator;
import software.amazon.polymorph.smithygo.localservice.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithygo.localservice.nameresolver.SmithyNameResolver;
import software.amazon.polymorph.smithygo.awssdk.shapevisitor.ShapeVisitorHelper;
import software.amazon.polymorph.smithygo.utils.GoCodegenUtils;
import software.amazon.smithy.aws.traits.ServiceTrait;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.traits.EnumTrait;
import software.amazon.smithy.model.traits.ErrorTrait;
import software.amazon.smithy.model.traits.UnitTypeTrait;

import static software.amazon.polymorph.smithygo.codegen.SymbolUtils.POINTABLE;

public class DafnyAwsSdkClientTypeConversionProtocol
  implements ProtocolGenerator {

  final Model dafnyNonNormalizedModel;
  final Model awsNormalizedModel;
  final ServiceShape serviceShape;

  public DafnyAwsSdkClientTypeConversionProtocol(
    Model model,
    ServiceShape serviceShape
  ) {
    dafnyNonNormalizedModel = model;
    awsNormalizedModel =
      AddOperationShapes.execute(model, serviceShape.toShapeId());

    this.serviceShape = serviceShape;
  }

  @Override
  public ShapeId getProtocol() {
    return null;
  }

  @Override
  public ApplicationProtocol getApplicationProtocol() {
    return null;
  }

  @Override
  public void generateSerializers(GenerationContext context) {
    final Set<ShapeId> alreadyVisited = new HashSet<>();
    final var symbolProvider = context.symbolProvider();
    final var writerDelegator = context.writerDelegator();
    serviceShape
      .getOperations()
      .forEach(eachOperation -> {
        final var awsNormalizedOperation = awsNormalizedModel.expectShape(
          eachOperation,
          OperationShape.class
        );
        final var awsNormalizedInputShape = awsNormalizedModel.expectShape(
          awsNormalizedOperation.getInputShape()
        );
        if (!alreadyVisited.contains(awsNormalizedInputShape.toShapeId())) {
          alreadyVisited.add(awsNormalizedInputShape.toShapeId());
          if (
            !awsNormalizedInputShape.hasTrait(UnitTypeTrait.class) &&
            awsNormalizedInputShape
              .toShapeId()
              .getNamespace()
              .equals(serviceShape.toShapeId().getNamespace())
          ) {
            final var awsNormalizedInputToDafnyMethodName =
              SmithyNameResolver.getToDafnyMethodName(
                serviceShape,
                awsNormalizedInputShape,
                ""
              );
            final var awsNormalizedInputSymbol = symbolProvider.toSymbol(
              awsNormalizedInputShape
            );
            final var dafnyInput = dafnyNonNormalizedModel
              .expectShape(eachOperation, OperationShape.class)
              .getInputShape();
            final var dafnyInputSymbol = symbolProvider.toSymbol(
              dafnyNonNormalizedModel.expectShape(dafnyInput)
            );
            writerDelegator.useFileWriter(
              "%s/%s".formatted(
                  SmithyNameResolver.shapeNamespace(serviceShape),
                  TO_DAFNY
                ),
              SmithyNameResolver.shapeNamespace(serviceShape),
              writer -> {
                writer.addImport(
                  SmithyNameResolver.getGoModuleNameForSdkNamespace(
                    awsNormalizedInputShape.toShapeId().getNamespace()
                  )
                );
                writer.write(
                  """
                  func $L(nativeInput $L)($L) {
                      ${C|}
                  }""",
                  awsNormalizedInputToDafnyMethodName,
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    awsNormalizedInputSymbol,
                    false
                  ),
                  DafnyNameResolver.getDafnyType(
                    dafnyNonNormalizedModel.expectShape(dafnyInput),
                    dafnyInputSymbol
                  ),
                  writer.consumer(w ->
                    generateRequestSerializer(
                      context,
                      awsNormalizedOperation,
                      context.writerDelegator()
                    )
                  )
                );
              }
            );
          }
        }

        final var awsNormalizedOutputShape = awsNormalizedModel.expectShape(
          awsNormalizedOperation.getOutputShape()
        );
        if (!alreadyVisited.contains(awsNormalizedOutputShape.toShapeId())) {
          alreadyVisited.add(awsNormalizedOutputShape.toShapeId());
          if (
            !awsNormalizedOutputShape.hasTrait(UnitTypeTrait.class) &&
            awsNormalizedOutputShape
              .toShapeId()
              .getNamespace()
              .equals(serviceShape.toShapeId().getNamespace())
          ) {
            final var awsNormalizedOutputToDafnyMethodName =
              SmithyNameResolver.getToDafnyMethodName(
                serviceShape,
                awsNormalizedOutputShape,
                ""
              );
            final var awsNormalizedOutputSymbol = symbolProvider.toSymbol(
              awsNormalizedOutputShape
            );
            final var dafnyOutput = dafnyNonNormalizedModel
              .expectShape(eachOperation, OperationShape.class)
              .getOutputShape();
            final var dafnyOutputSymbol = symbolProvider.toSymbol(
              dafnyNonNormalizedModel.expectShape(dafnyOutput)
            );
            writerDelegator.useFileWriter(
              "%s/%s".formatted(
                  SmithyNameResolver.shapeNamespace(serviceShape),
                  TO_DAFNY
                ),
              SmithyNameResolver.shapeNamespace(serviceShape),
              writer -> {
                writer.addImport(
                  SmithyNameResolver.getGoModuleNameForSdkNamespace(
                    awsNormalizedInputShape.toShapeId().getNamespace()
                  )
                );
                writer.write(
                  """
                  func $L(nativeOutput $L)($L) {
                      ${C|}
                  }""",
                  awsNormalizedOutputToDafnyMethodName,
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    awsNormalizedOutputSymbol,
                    false
                  ),
                  DafnyNameResolver.getDafnyType(
                    dafnyNonNormalizedModel.expectShape(dafnyOutput),
                    dafnyOutputSymbol
                  ),
                  writer.consumer(w ->
                    generateResponseSerializer(
                      context,
                      awsNormalizedOperation,
                      context.writerDelegator()
                    )
                  )
                );
              }
            );
          }
        }
      });
    generateErrorSerializer(context);
    writerDelegator.useFileWriter(
      "%s/%s".formatted(
          SmithyNameResolver.shapeNamespace(serviceShape),
          TO_DAFNY
        ),
      SmithyNameResolver.shapeNamespace(serviceShape),
      writer -> {
        for (final MemberShape visitingMemberShape : AwsSdkToDafnyShapeVisitor.visitorFuncMap.keySet()) {
          final Shape visitingShape = context
            .model()
            .expectShape(visitingMemberShape.getTarget());
          if (alreadyVisited.contains(visitingMemberShape.getId())) {
            continue;
          }
          alreadyVisited.add(visitingMemberShape.toShapeId());
          String inputType;
          String outputType = ShapeVisitorHelper.toDafnyOptionalityMap.get(
              visitingMemberShape
            )
            ? "Wrappers.Option"
            : DafnyNameResolver.getDafnyType(
              visitingShape,
              context.symbolProvider().toSymbol(visitingShape)
            );
          inputType =
          GoCodegenUtils.getType(
            context.symbolProvider().toSymbol(visitingShape),
            serviceShape.expectTrait(ServiceTrait.class)
            );
          if (
            AwsSdkGoPointableIndex
                .of(context.model())
                .isPointable(visitingMemberShape)
          ) {
            inputType = "*".concat(inputType);
          }
          writer.write(
            """
            func $L(input $L)($L) {
                return $L
            }
            """,
            ShapeVisitorHelper.funcNameGenerator(
              visitingMemberShape,
              "ToDafny"
            ),
            inputType,
            outputType,
            AwsSdkToDafnyShapeVisitor.visitorFuncMap.get(visitingMemberShape)
          );
        }
      }
    );
  }

  @Override
  public void generateDeserializers(GenerationContext context) {
    final Set<ShapeId> alreadyVisited = new HashSet<>();
    final var symbolProvider = context.symbolProvider();
    final var delegator = context.writerDelegator();

    serviceShape
      .getOperations()
      .forEach(eachOperation -> {
        final var awsNormalizedOperationShape = awsNormalizedModel.expectShape(
          eachOperation,
          OperationShape.class
        );
        final var awsNormalizedInputShape = awsNormalizedModel.expectShape(
          awsNormalizedOperationShape.getInputShape()
        );
        if (!alreadyVisited.contains(awsNormalizedInputShape.toShapeId())) {
          alreadyVisited.add(awsNormalizedInputShape.toShapeId());
          if (
            !awsNormalizedInputShape.hasTrait(UnitTypeTrait.class) &&
            awsNormalizedInputShape
              .toShapeId()
              .getNamespace()
              .equals(serviceShape.toShapeId().getNamespace())
          ) {
            final var awsNormalizedInputFromDafnyMethodName =
              SmithyNameResolver.getFromDafnyMethodName(
                serviceShape,
                awsNormalizedInputShape,
                ""
              );
            final var awsNormalizedInputSymbol = symbolProvider.toSymbol(
              awsNormalizedInputShape
            );
            final var dafnyInput = dafnyNonNormalizedModel
              .expectShape(eachOperation, OperationShape.class)
              .getInputShape();
            final var dafnyInputSymbol = symbolProvider.toSymbol(
              dafnyNonNormalizedModel.expectShape(dafnyInput)
            );
            delegator.useFileWriter(
              "%s/%s".formatted(
                  SmithyNameResolver.shapeNamespace(serviceShape),
                  TO_NATIVE
                ),
              SmithyNameResolver.shapeNamespace(serviceShape),
              writer -> {
                writer.addImport(
                  SmithyNameResolver.getGoModuleNameForSdkNamespace(
                    awsNormalizedInputShape.toShapeId().getNamespace()
                  )
                );
                writer.write(
                  """
                  func $L(dafnyInput $L)($L) {
                      ${C|}
                  }""",
                  awsNormalizedInputFromDafnyMethodName,
                  DafnyNameResolver.getDafnyType(
                    dafnyNonNormalizedModel.expectShape(dafnyInput),
                    dafnyInputSymbol
                  ),
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    awsNormalizedInputSymbol,
                    false
                  ),
                  writer.consumer(w ->
                    generateRequestDeserializer(
                      context,
                      awsNormalizedOperationShape,
                      context.writerDelegator()
                    )
                  )
                );
              }
            );
          }
        }

        final var awsNormalizedOutputShape = awsNormalizedModel.expectShape(
          awsNormalizedOperationShape.getOutputShape()
        );
        if (!alreadyVisited.contains(awsNormalizedOutputShape.toShapeId())) {
          alreadyVisited.add(awsNormalizedOutputShape.toShapeId());
          if (
            !awsNormalizedOutputShape.hasTrait(UnitTypeTrait.class) &&
            awsNormalizedOutputShape
              .toShapeId()
              .getNamespace()
              .equals(serviceShape.toShapeId().getNamespace())
          ) {
            final var awsNormalizedOutputFromDafnyMethodName =
              SmithyNameResolver.getFromDafnyMethodName(
                serviceShape,
                awsNormalizedOutputShape,
                ""
              );
            final var awsNormalizedOutputSymbol = context
              .symbolProvider()
              .toSymbol(awsNormalizedOutputShape);
            final var dafnyOutput = dafnyNonNormalizedModel
              .expectShape(eachOperation, OperationShape.class)
              .getOutputShape();
            final var dafnyOutputSymbol = symbolProvider.toSymbol(
              dafnyNonNormalizedModel.expectShape(dafnyOutput)
            );
            delegator.useFileWriter(
              "%s/%s".formatted(
                  SmithyNameResolver.shapeNamespace(serviceShape),
                  TO_NATIVE
                ),
              SmithyNameResolver.shapeNamespace(serviceShape),
              writer -> {
                writer.addImport(
                  SmithyNameResolver.getGoModuleNameForSdkNamespace(
                    awsNormalizedInputShape.toShapeId().getNamespace()
                  )
                );
                writer.write(
                  """
                  func $L(dafnyOutput $L)($L) {
                      ${C|}
                  }""",
                  awsNormalizedOutputFromDafnyMethodName,
                  DafnyNameResolver.getDafnyType(
                    dafnyNonNormalizedModel.expectShape(dafnyOutput),
                    dafnyOutputSymbol
                  ),
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    awsNormalizedOutputSymbol,
                    false
                  ),
                  writer.consumer(w ->
                    generateResponseDeserializer(
                      context,
                      awsNormalizedOperationShape,
                      context.writerDelegator()
                    )
                  )
                );
              }
            );
          }
        }
      });

    generateErrorDeserializer(context);

    delegator.useFileWriter(
      "%s/%s".formatted(
          SmithyNameResolver.shapeNamespace(serviceShape),
          TO_NATIVE
        ),
      SmithyNameResolver.shapeNamespace(serviceShape),
      writer -> {
        ServiceTrait serviceTrait = context
            .model()
            .expectShape(context.settings().getService(context.model()).toShapeId())
            .getTrait(ServiceTrait.class)
            .get();
        for (MemberShape visitingMemberShape : DafnyToAwsSdkShapeVisitor.visitorFuncMap.keySet()) {
          final Shape visitingShape = context
            .model()
            .expectShape(visitingMemberShape.getTarget());
          if (alreadyVisited.contains(visitingMemberShape.getId())) {
            continue;
          }
          alreadyVisited.add(visitingMemberShape.toShapeId());
          String outputType = SmithyNameResolver.getSmithyTypeAws(serviceTrait, context.symbolProvider().toSymbol(visitingShape), true);;
          switch (visitingShape.getType()) {
            case STRUCTURE:
            case UNION:
              if (visitingShape.hasTrait(EnumTrait.class)) {
                outputType = SmithyNameResolver.getSmithyTypeAws(serviceTrait, context.symbolProvider().toSymbol(visitingShape), true);
              }
              break;
            case LIST:
            case MAP:
              outputType = GoCodegenUtils.getType(
                context.symbolProvider().toSymbol(visitingShape),
                serviceTrait
                );
              break;
          }
          if (
            ShapeVisitorHelper.toNativeOutputPointerMap.get(visitingMemberShape)
          ) {
            outputType = "*".concat(outputType);
          }
          writer.write(
            """
            func $L(input $L)($L) {
                return $L
            }""",
            ShapeVisitorHelper.funcNameGenerator(
              visitingMemberShape,
              "FromDafny"
            ),
            "interface {}",
            outputType,
            DafnyToAwsSdkShapeVisitor.visitorFuncMap.get(visitingMemberShape)
          );
        }
      }
    );
  }

  private void generateRequestSerializer(
    final GenerationContext context,
    final OperationShape operation,
    final GoDelegator delegator
  ) {
    final var nonNormalizedOperation = dafnyNonNormalizedModel.expectShape(
      operation.toShapeId(),
      OperationShape.class
    );
    final var targetShape = dafnyNonNormalizedModel.expectShape(
      nonNormalizedOperation.getInputShape()
    );
    delegator.useFileWriter(
      "%s/%s".formatted(SmithyNameResolver.shapeNamespace(operation), TO_DAFNY),
      SmithyNameResolver.shapeNamespace(operation),
      writer -> {
        final var input = targetShape.accept(
          new AwsSdkToDafnyShapeVisitor(
            context,
            "nativeInput",
            writer,
            false,
            false,
            false
          )
        );
        writer.write(
          """
          return $L
          """,
          input
        );
      }
    );
  }

  private void generateResponseSerializer(
    final GenerationContext context,
    final OperationShape operation,
    final GoDelegator delegator
  ) {
    final var nonNormalizedOperation = dafnyNonNormalizedModel.expectShape(
      operation.toShapeId(),
      OperationShape.class
    );
    final var targetShape = dafnyNonNormalizedModel.expectShape(
      nonNormalizedOperation.getOutputShape()
    );
    delegator.useFileWriter(
      "%s/%s".formatted(SmithyNameResolver.shapeNamespace(operation), TO_DAFNY),
      SmithyNameResolver.shapeNamespace(operation),
      writer -> {
        final var input = targetShape.accept(
          new AwsSdkToDafnyShapeVisitor(
            context,
            "nativeOutput",
            writer,
            false,
            false,
            false
          )
        );
        writer.write(
          """
          return $L
          """,
          input
        );
      }
    );
  }

  private void generateRequestDeserializer(
    final GenerationContext context,
    final OperationShape operation,
    final GoDelegator delegator
  ) {
    delegator.useFileWriter(
      "%s/%s".formatted(
          SmithyNameResolver.shapeNamespace(operation),
          TO_NATIVE
        ),
      SmithyNameResolver.shapeNamespace(operation),
      writer -> {
        final var inputShape = operation.getInputShape();

        final var targetShape = awsNormalizedModel.expectShape(inputShape);
        final var input = targetShape.accept(
          new DafnyToAwsSdkShapeVisitor(context, "dafnyInput", writer)
        );

        writer.write(
          """
          return $L
          """,
          input
        );
      }
    );
  }

  private void generateResponseDeserializer(
    final GenerationContext context,
    final OperationShape operation,
    final GoDelegator delegator
  ) {
    delegator.useFileWriter(
      "%s/%s".formatted(
          SmithyNameResolver.shapeNamespace(operation),
          TO_NATIVE
        ),
      SmithyNameResolver.shapeNamespace(operation),
      writer -> {
        final var outputShape = operation.getOutputShape();

        final var targetShape = awsNormalizedModel.expectShape(outputShape);
        final var output = targetShape.accept(
          new DafnyToAwsSdkShapeVisitor(context, "dafnyOutput", writer)
        );

        writer.write(
          """
          return $L
          """,
          output
        );
      }
    );
  }

  private void generateErrorSerializer(final GenerationContext context) {
    final Set<ShapeId> alreadyVisited = new HashSet<>();
    final var errorShapes = awsNormalizedModel.getShapesWithTrait(
      ErrorTrait.class
    );

    for (final var errorShape : errorShapes) {
      if (
        !errorShape
          .toShapeId()
          .getNamespace()
          .equals(serviceShape.toShapeId().getNamespace())
      ) {
        continue;
      }
      if (!alreadyVisited.contains(errorShape.toShapeId())) {
        alreadyVisited.add(errorShape.toShapeId());
        final var getInputToDafnyMethodName =
          SmithyNameResolver.getToDafnyMethodName(serviceShape, errorShape, "");

        context
          .writerDelegator()
          .useFileWriter(
            "%s/%s".formatted(
                SmithyNameResolver.shapeNamespace(errorShape),
                TO_DAFNY
              ),
            SmithyNameResolver.shapeNamespace(errorShape),
            writer -> {
              writer.addImportFromModule(
                SmithyNameResolver.getGoModuleNameForSdkNamespace(
                  errorShape.toShapeId().getNamespace()
                ),
                SmithyNameResolver.smithyTypesNamespaceAws(
                  serviceShape.expectTrait(ServiceTrait.class),
                  true
                )
              );
              writer.write(
                """
                func $L(nativeInput types.$L)($L) {
                    ${C|}
                }""",
                getInputToDafnyMethodName,
                context.symbolProvider().toSymbol(errorShape).getName(),
                DafnyNameResolver.getDafnyBaseErrorType(errorShape),
                writer.consumer(w -> {
                  String output = errorShape.accept(
                    new AwsSdkToDafnyShapeVisitor(
                      context,
                      "nativeInput",
                      writer,
                      false,
                      false,
                      false
                    )
                  );
                  writer.write(
                    """
                    return $L
                    """,
                    output
                  );
                })
              );
            }
          );
      }
    }

    context
      .writerDelegator()
      .useFileWriter(
        "%s/%s".formatted(
            SmithyNameResolver.shapeNamespace(serviceShape),
            TO_DAFNY
          ),
        SmithyNameResolver.shapeNamespace(serviceShape),
        writer -> {
          writer.write(
            """
            func OpaqueError_Input_ToDafny(nativeInput error)($L.Error) {
            	return $L.Companion_Error_.Create_Opaque_(nativeInput)
            }""",
            DafnyNameResolver.dafnyTypesNamespace(serviceShape),
            DafnyNameResolver.dafnyTypesNamespace(serviceShape)
          );
        }
      );

    context
      .writerDelegator()
      .useFileWriter(
        "%s/%s".formatted(
            SmithyNameResolver.shapeNamespace(serviceShape),
            TO_DAFNY
          ),
        SmithyNameResolver.shapeNamespace(serviceShape),
        writer -> {
          writer.write(
            """
            func Error_ToDafny(err error)($L.Error) {
                switch err.(type) {
                // Service Errors
                ${C|}

                default:
                    return OpaqueError_Input_ToDafny(err)
                }
            }
            """,
            DafnyNameResolver.dafnyTypesNamespace(serviceShape),
            writer.consumer(w -> {
              for (var error : errorShapes) {
                w.write(
                  """
                    case *$L:
                        return $L(*err.(*$L))
                  """,
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    context
                      .symbolProvider()
                      .toSymbol(
                        awsNormalizedModel.expectShape(error.toShapeId())
                      ),
                    true
                  ),
                  SmithyNameResolver.getToDafnyMethodName(
                    serviceShape,
                    awsNormalizedModel.expectShape(error.toShapeId()),
                    ""
                  ),
                  SmithyNameResolver.getSmithyTypeAws(
                    serviceShape.expectTrait(ServiceTrait.class),
                    context
                      .symbolProvider()
                      .toSymbol(
                        awsNormalizedModel.expectShape(error.toShapeId())
                      ),
                    true
                  )
                );
              }
            })
          );
        }
      );
  }

  private void generateErrorDeserializer(final GenerationContext context) {
    final Set<ShapeId> alreadyVisited = new HashSet<>();
    final var errorShapes = awsNormalizedModel.getShapesWithTrait(
      ErrorTrait.class
    );
    for (final var errorShape : errorShapes) {
      if (
        !errorShape
          .toShapeId()
          .getNamespace()
          .equals(serviceShape.toShapeId().getNamespace())
      ) {
        continue;
      }
      if (!alreadyVisited.contains(errorShape.toShapeId())) {
        alreadyVisited.add(errorShape.toShapeId());
        final var getOutputFromDafnyMethodName =
          SmithyNameResolver.getFromDafnyMethodName(
            serviceShape,
            errorShape,
            ""
          );
        context
          .writerDelegator()
          .useFileWriter(
            "%s/%s".formatted(
                SmithyNameResolver.shapeNamespace(errorShape),
                TO_NATIVE
              ),
            SmithyNameResolver.shapeNamespace(errorShape),
            writer -> {
              writer.addImportFromModule(
                SmithyNameResolver.getGoModuleNameForSdkNamespace(
                  errorShape.toShapeId().getNamespace()
                ),
                SmithyNameResolver.smithyTypesNamespaceAws(
                  serviceShape.expectTrait(ServiceTrait.class),
                  true
                )
              );
              writer.write(
                """
                func $L(dafnyOutput $L)(types.$L) {
                    ${C|}
                }""",
                getOutputFromDafnyMethodName,
                DafnyNameResolver.getDafnyBaseErrorType(errorShape),
                context.symbolProvider().toSymbol(errorShape).getName(),
                writer.consumer(w -> {
                  String output = errorShape.accept(
                    new DafnyToAwsSdkShapeVisitor(
                      context,
                      "dafnyOutput",
                      writer
                    )
                  );
                  writer.write(
                    """
                    return $L
                    """,
                    output
                  );
                })
              );
            }
          );
      }
    }

    context
      .writerDelegator()
      .useFileWriter(
        "%s/%s".formatted(
            SmithyNameResolver.shapeNamespace(serviceShape),
            TO_NATIVE
          ),
        SmithyNameResolver.shapeNamespace(serviceShape),
        writer -> {
          writer.addUseImports(SmithyGoDependency.FMT);
          writer.write(
            """
            func OpaqueError_Output_FromDafny(dafnyOutput $L.Error)(error) {
                return fmt.Errorf(fmt.Sprintf("%v", dafnyOutput.Dtor_obj()))
            }""",
            DafnyNameResolver.dafnyTypesNamespace(serviceShape)
          );
        }
      );

    context
      .writerDelegator()
      .useFileWriter(
        "%s/%s".formatted(
            SmithyNameResolver.shapeNamespace(serviceShape),
            TO_NATIVE
          ),
        SmithyNameResolver.shapeNamespace(serviceShape),
        writer -> {
          writer.write(
            """
            func Error_FromDafny(err $L.Error)(error) {
                // Service Errors
                ${C|}

                return OpaqueError_Output_FromDafny(err)
            }
            """,
            DafnyNameResolver.dafnyTypesNamespace(serviceShape),
            writer.consumer(w -> {
              for (var error : awsNormalizedModel.getShapesWithTrait(
                ErrorTrait.class
              )) {
                w.write(
                  """
                  if err.Is_$L() {
                      e := $L(err)
                      return &e
                  }
                  """,
                  error.toShapeId().getName(),
                  SmithyNameResolver.getFromDafnyMethodName(
                    serviceShape,
                    awsNormalizedModel.expectShape(error.toShapeId()),
                    ""
                  )
                );
              }
            })
          );
        }
      );
  }
}