package software.amazon.polymorph.smithygo.utils;

import static software.amazon.polymorph.smithygo.utils.Constants.DAFNY_RUNTIME_GO_LIBRARY_MODULE;

import software.amazon.polymorph.smithygo.codegen.GoWriter;
import software.amazon.polymorph.smithygo.codegen.SymbolUtils;
import software.amazon.polymorph.smithygo.localservice.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithygo.localservice.nameresolver.SmithyNameResolver;
import software.amazon.polymorph.traits.PositionalTrait;
import software.amazon.polymorph.traits.ReferenceTrait;
import software.amazon.smithy.aws.traits.ServiceTrait;
import software.amazon.smithy.codegen.core.Symbol;
import software.amazon.smithy.codegen.core.SymbolProvider;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.knowledge.NeighborProviderIndex;
import software.amazon.smithy.model.neighbor.NeighborProvider;
import software.amazon.smithy.model.neighbor.Relationship;
import software.amazon.smithy.model.neighbor.RelationshipType;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeType;
import software.amazon.smithy.model.traits.EnumTrait;
import software.amazon.smithy.model.traits.UnitTypeTrait;

public class GoCodegenUtils {

  public static String getType(
    final Symbol symbol,
    final ServiceTrait serviceTrait,
    final Boolean includeNamespace
  ) {
    if (
      symbol.getProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class).isEmpty()
    ) {
      return includeNamespace
        ? SmithyNameResolver.getSmithyTypeAws(serviceTrait, symbol, true)
        : symbol.getName();
    }
    final var type = getType(
      symbol.expectProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class),
      serviceTrait,
      includeNamespace
    );
    if (symbol.getProperty(SymbolUtils.GO_MAP).isPresent()) {
      return "map[string]" + type;
    }
    if (symbol.getProperty(SymbolUtils.GO_SLICE).isPresent()) {
      return "[]" + type;
    }
    throw new RuntimeException("Failed to determine shape type");
  }

  public static String getType(
    final Symbol symbol,
    final Shape shape,
    final Boolean includeNamespace
  ) {
    if (
      symbol.getProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class).isEmpty()
    ) {
      return includeNamespace
        ? SmithyNameResolver.getSmithyType(shape, symbol)
        : symbol.getName();
    }
    var type = getType(
      symbol.expectProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class),
      shape,
      includeNamespace
    );
    if (symbol.getProperty(SymbolUtils.GO_MAP).isPresent()) {
      type = "map[string]" + type;
    }
    if (symbol.getProperty(SymbolUtils.GO_SLICE).isPresent()) {
      type = "[]" + type;
    }
    return type;
  }

  public static Symbol getRootSymbol(final Symbol symbol) {
    if (
      symbol.getProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class).isEmpty()
    ) {
      return symbol;
    }
    return getRootSymbol(
      symbol.expectProperty(SymbolUtils.GO_ELEMENT_TYPE, Symbol.class)
    );
  }

  public static boolean isOperationStruct(
    final Model model,
    final Shape shape
  ) {
    final NeighborProvider provider = NeighborProviderIndex
      .of(model)
      .getReverseProvider();
    for (Relationship relationship : provider.getNeighbors(shape)) {
      final RelationshipType relationshipType =
        relationship.getRelationshipType();
      if (
        relationshipType == RelationshipType.INPUT ||
        relationshipType == RelationshipType.OUTPUT
      ) {
        return true;
      }
    }

    return false;
  }

  public static void importNamespace(Shape shape, GoWriter writer) {
    var type = shape.getType();
    if (shape.hasTrait(EnumTrait.class)) {
      type = ShapeType.ENUM;
    }
    switch (type) {
      case DOUBLE, STRING, BLOB, LIST, TIMESTAMP, MAP:
        writer.addImportFromModule(DAFNY_RUNTIME_GO_LIBRARY_MODULE, "dafny");
      case ENUM, STRUCTURE, UNION, RESOURCE:
        writer.addImportFromModule(
          SmithyNameResolver.getGoModuleNameForSmithyNamespace(
            shape.toShapeId().getNamespace()
          ),
          DafnyNameResolver.dafnyTypesNamespace(shape)
        );
    }
  }

  public static String getOperationalShapeInputName(
    final Model model,
    final OperationShape operationShape,
    final SymbolProvider symbolProvider
  ) {
    return (
      getInputOrOutputName(
        model,
        model.expectShape(operationShape.getInputShape()),
        symbolProvider,
        false
      )
    );
  }

  public static String getOperationalShapeOutputName(
    final Model model,
    final OperationShape operationShape,
    final SymbolProvider symbolProvider
  ) {
    return (
      getInputOrOutputName(
        model,
        model.expectShape(operationShape.getOutputShape()),
        symbolProvider,
        true
      )
    );
  }

  private static String getInputOrOutputName(
    final Model model,
    final Shape shape,
    final SymbolProvider symbolProvider,
    final Boolean includeDeference
  ) {
    if (shape.hasTrait(UnitTypeTrait.class)) {
      return "";
    } else if (shape.hasTrait(PositionalTrait.class)) {
      Shape curShape = model.expectShape(
        shape.getAllMembers().values().stream().findFirst().get().getTarget()
      );
      if (curShape.hasTrait(ReferenceTrait.class)) {
        curShape =
          model.expectShape(
            curShape.expectTrait(ReferenceTrait.class).getReferentId()
          );
      }
      return (
        SmithyNameResolver.getSmithyType(
          curShape,
          symbolProvider.toSymbol(curShape)
        )
      );
    } else {
      return (includeDeference ? "*".concat(shape.getId().getName()) : shape.getId().getName());
    }
  }
}
