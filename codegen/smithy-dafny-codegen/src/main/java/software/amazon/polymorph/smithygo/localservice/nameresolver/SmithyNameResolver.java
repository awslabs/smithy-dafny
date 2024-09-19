package software.amazon.polymorph.smithygo.localservice.nameresolver;

import software.amazon.smithy.aws.traits.ServiceTrait;
import software.amazon.smithy.codegen.core.Symbol;
import software.amazon.smithy.model.shapes.ListShape;
import software.amazon.smithy.model.shapes.MapShape;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.codegen.core.SymbolProvider;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Map;

import static software.amazon.polymorph.smithygo.localservice.nameresolver.Constants.BLANK;
import static software.amazon.polymorph.smithygo.localservice.nameresolver.Constants.DOT;

public class SmithyNameResolver {

    private static Map<String, String> smithyNamespaceToGoModuleNameMap;
    // Known native type of simple shapes
    private static HashSet<String> knownSmithyType = new HashSet<>(Arrays.asList(
            "int32", //integer shape
            "string", // string shape
            "[]byte", // blob shape
            "int64", // long shape
            "float64", // double shape
            "bool" //boolean shape
        ));

    public static void setSmithyNamespaceToGoModuleNameMap(
            Map<String, String> smithyNamespaceToGoModuleNameMap) {
        SmithyNameResolver.smithyNamespaceToGoModuleNameMap = smithyNamespaceToGoModuleNameMap;
    }

    public static String getGoModuleNameForSmithyNamespace(final String smithyNamespace) {
        if (smithyNamespace.contains("smithy.")) return "";
        if (!smithyNamespaceToGoModuleNameMap.containsKey(smithyNamespace)) {
            throw new IllegalArgumentException("Go module name not found for Smithy namespace: " + smithyNamespace);
        }
        return smithyNamespaceToGoModuleNameMap.get(smithyNamespace);
    }

    public static String shapeNamespace(final Shape shape) {
        return shape.toShapeId().getNamespace().replace(DOT, BLANK).toLowerCase();
    }

    public static String smithyTypesNamespace(final Shape shape) {
        return shape.toShapeId().getNamespace().replace(DOT, BLANK).toLowerCase().concat("types");
    }

    public static String getGoModuleNameForSdkNamespace(final String smithyNamespace) {
        return getGoModuleNameForSmithyNamespace("sdk.".concat(smithyNamespace));
    }

    public static String smithyTypesNamespaceAws(final ServiceTrait serviceTrait, boolean isAwsSubType) {
        if (isAwsSubType) {
            return "types";
        }
        return serviceTrait.getSdkId().toLowerCase();
    }

    public static String getSmithyType(final Shape shape, final Symbol symbol, Model model, SymbolProvider symbolProvider) {
        if(symbol.getNamespace().contains("smithy.") || symbol.getNamespace().equals("smithyapitypes") || knownSmithyType.contains(symbol.getName())) {
            return symbol.getName();
        }
        if (shape.isMapShape()) {
            MemberShape valueMemberShape = ((MapShape) shape).getValue();
            Shape valueShape = model.expectShape(valueMemberShape.getTarget());
            return "map[string]".concat(SmithyNameResolver.getSmithyType(valueShape, symbolProvider.toSymbol(valueShape), model, symbolProvider));
        }
        if (shape.isListShape()) {
            MemberShape memberShape = ((ListShape) shape).getMember();
            Shape memberShapeTarget = model.expectShape(memberShape.getTarget());
            return "[]".concat(SmithyNameResolver.getSmithyType(memberShapeTarget, symbolProvider.toSymbol(memberShapeTarget), model, symbolProvider));
        }
        return SmithyNameResolver.smithyTypesNamespace(shape).concat(DOT).concat(symbol.getName());
    }

    public static String getSmithyType(final Shape shape, final Symbol symbol) {
        if(symbol.getNamespace().contains("smithy.") || symbol.getNamespace().equals("smithyapitypes") || symbol.getName().contains("string")) {
            return symbol.getName();
        }
        return SmithyNameResolver.smithyTypesNamespace(shape).concat(DOT).concat(symbol.getName());
    }

    public static String getSmithyTypeAws(final ServiceTrait serviceTrait, final Symbol symbol, boolean subtype) {
        if(symbol.getNamespace().contains("smithy.") || symbol.getName().contains("string")) {
            return symbol.getName();
        }
        return SmithyNameResolver.smithyTypesNamespaceAws(serviceTrait, subtype).concat(DOT).concat(symbol.getName());
    }

    public static String getSmithyType(final Shape shape) {
        return SmithyNameResolver.smithyTypesNamespace(shape).concat(DOT).concat(shape.toShapeId().getName());
    }

    public static String getToDafnyMethodName(final ServiceShape serviceShape, final Shape shape, final String disambiguator) {
        final var methodName = serviceShape.getContextualName(shape).concat("_ToDafny");
        if (serviceShape.toShapeId().getNamespace().equals(shape.toShapeId().getNamespace())) {
            return methodName;
        } else {
            return SmithyNameResolver.shapeNamespace(shape).concat(DOT).concat(methodName);
        }
    }

    public static String getToDafnyMethodName(final Shape shape, final String disambiguator) {
        final var methodName = shape.getId().getName().concat("_ToDafny");
            return SmithyNameResolver.shapeNamespace(shape).concat(DOT).concat(methodName);
    }

    public static String getFromDafnyMethodName(final ServiceShape serviceShape, final Shape shape, final String disambiguator) {
        final var methodName = serviceShape.getContextualName(shape).concat("_FromDafny");
        if (serviceShape.toShapeId().getNamespace().equals(shape.toShapeId().getNamespace())) {
            return methodName;
        } else {
            return SmithyNameResolver.shapeNamespace(shape).concat(DOT).concat(methodName);
        }
    }

    public static String getFromDafnyMethodName(final Shape shape, final String disambiguator) {
        final var methodName = shape.getId().getName().concat("_FromDafny");
        return SmithyNameResolver.shapeNamespace(shape).concat(DOT).concat(methodName);
    }

    public static String getAwsServiceClient(final ServiceTrait serviceTrait) {
        return SmithyNameResolver.smithyTypesNamespaceAws(serviceTrait, false)
                                .concat(DOT)
                                .concat("Client");
    }
}
