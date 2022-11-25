package software.amazon.polymorph.smithyjava.generator;

import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.CodeBlock;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.TypeName;

import java.util.Map;

import software.amazon.polymorph.smithyjava.BuilderSpecs;
import software.amazon.polymorph.smithyjava.MethodReference;
import software.amazon.polymorph.smithyjava.nameresolver.Dafny;
import software.amazon.polymorph.utils.DafnyNameResolverHelpers;
import software.amazon.polymorph.utils.ModelUtils;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.ShapeType;
import software.amazon.smithy.model.shapes.StringShape;
import software.amazon.smithy.model.traits.EnumDefinition;
import software.amazon.smithy.model.traits.EnumTrait;

import static software.amazon.polymorph.smithyjava.generator.Generator.Constants.IDENTITY_FUNCTION;
import static software.amazon.smithy.utils.StringUtils.capitalize;

public abstract class ToNative extends Generator {
    protected final static String VAR_INPUT = "dafnyValue";
    protected final static String TO_NATIVE = "ToNative";
    protected final static String NATIVE_BUILDER = "nativeBuilder";
    /**
     * The keys are the Dafny generated java input type,
     * the values are the method that converts from that input to the
     * idiomatic java type.
     */
    protected static final Map<ShapeType, MethodReference> AGGREGATE_CONVERSION_METHOD_FROM_SHAPE_TYPE;
    protected static final Map<ShapeType, MethodReference> SIMPLE_CONVERSION_METHOD_FROM_SHAPE_TYPE;
    protected static final ClassName COMMON_TO_NATIVE_SIMPLE = ClassName.get(software.amazon.dafny.conversion.ToNative.Simple.class);
    protected static final ClassName COMMON_TO_NATIVE_AGGREGATE = ClassName.get(software.amazon.dafny.conversion.ToNative.Aggregate.class);

    static {
        AGGREGATE_CONVERSION_METHOD_FROM_SHAPE_TYPE = Map.ofEntries(
                Map.entry(ShapeType.LIST, new MethodReference(COMMON_TO_NATIVE_AGGREGATE, "GenericToList")),
                Map.entry(ShapeType.SET, new MethodReference(COMMON_TO_NATIVE_AGGREGATE, "GenericToSet")),
                Map.entry(ShapeType.MAP, new MethodReference(COMMON_TO_NATIVE_AGGREGATE, "GenericToMap"))
        );
        SIMPLE_CONVERSION_METHOD_FROM_SHAPE_TYPE = Map.ofEntries(
                Map.entry(ShapeType.BLOB, new MethodReference(COMMON_TO_NATIVE_SIMPLE, "ByteBuffer")),
                Map.entry(ShapeType.BOOLEAN, IDENTITY_FUNCTION),
                Map.entry(ShapeType.STRING, new MethodReference(COMMON_TO_NATIVE_SIMPLE, "String")),
                // TODO: Timestamp should be service specific
                Map.entry(ShapeType.TIMESTAMP, new MethodReference(COMMON_TO_NATIVE_SIMPLE, "Date")),
                Map.entry(ShapeType.BYTE, IDENTITY_FUNCTION),
                Map.entry(ShapeType.SHORT, IDENTITY_FUNCTION),
                Map.entry(ShapeType.INTEGER, IDENTITY_FUNCTION),
                Map.entry(ShapeType.LONG, IDENTITY_FUNCTION),
                Map.entry(ShapeType.BIG_DECIMAL, IDENTITY_FUNCTION),
                Map.entry(ShapeType.BIG_INTEGER, IDENTITY_FUNCTION)
        );
    }
    /**
     * The class name of the Subject's Shim's ToNative class.
     */
    protected final ClassName thisClassName;

    public ToNative(CodegenSubject subject, ClassName className) {
        super(subject);
        thisClassName = className;
    }

    /** Signature of an Error conversion method. */
    protected MethodSpec.Builder initializeErrorMethodSpec(
            ClassName inputType,
            ClassName returnType
    ) {
        return initializeMethodSpec("Error", inputType, returnType);
    }

    /** Signature of a ToNative conversion method. */
    protected MethodSpec.Builder initializeMethodSpec(
            String methodName,
            TypeName inputType,
            TypeName returnType
    ) {
        return MethodSpec.methodBuilder(methodName)
                .returns(returnType)
                .addModifiers(PUBLIC_STATIC)
                .addParameter(inputType, VAR_INPUT);
    }

    /** Declare and assign the native value's builder. */
    protected MethodSpec.Builder createNativeBuilder(
            MethodSpec.Builder method,
            ClassName returnType
    ) {
        return method.addStatement("$T $L = $T.$L()",
                BuilderSpecs.builderInterfaceName(returnType),
                NATIVE_BUILDER,
                returnType,
                BuilderSpecs.BUILDER_VAR
        );
    }

    /** Return invocation of nativeBuilder's build method. */
    protected MethodSpec buildAndReturn(MethodSpec.Builder method) {
        method.addStatement("return $L.build()", NATIVE_BUILDER);
        return method.build();
    }

    protected MethodSpec modeledEnum(StringShape shape) {
        final ClassName returnType = subject.nativeNameResolver.classForEnum(shape);
        MethodSpec.Builder method = modeledEnumCommon(shape, returnType);
        // No Enum value matched, throw an Exception
        method.addStatement("throw new $T($S + $L)",
                IllegalArgumentException.class,
                "No entry of %s matches the input : ".formatted(returnType),
                VAR_INPUT);
        return method.build();
    }

    protected MethodSpec.Builder modeledEnumCommon(
            StringShape shape, ClassName returnType
    ) {
        final ShapeId shapeId = shape.getId();
        final String methodName = capitalize(shapeId.getName());
        final TypeName inputType = subject.dafnyNameResolver.typeForShape(shapeId);
        MethodSpec.Builder method = initializeMethodSpec(methodName, inputType, returnType);
        final EnumTrait enumTrait = shape.getTrait(EnumTrait.class).orElseThrow();
        if (!enumTrait.hasNames()) {
            throw new UnsupportedOperationException(
                    "Unnamed enums not supported. ShapeId: %s".formatted(shapeId));
        }

        enumTrait.getValues().stream()
                .map(EnumDefinition::getName)
                .map(maybeName -> maybeName.orElseThrow(
                        () -> new IllegalArgumentException(
                                "Unnamed enums not supported. ShapeId: %s".formatted(shapeId))
                ))
                .peek(name -> {
                    if (!ModelUtils.isValidEnumDefinitionName(name)) {
                        throw new UnsupportedOperationException(
                                "Invalid enum definition name: %s".formatted(name));
                    }
                })
                .forEach(name -> method
                        .beginControlFlow("if ($L.$L())", VAR_INPUT, Dafny.datatypeConstructorIs(name))
                        .addStatement("return $T.$L", returnType, name)
                        .endControlFlow()
                );
        return method;
    }

    protected CodeBlock setWithConversionCall(MemberShape member) {
        return CodeBlock.of("$L.$L($L($L.$L))",
                NATIVE_BUILDER,
                setMemberField(member),
                memberConversionMethodReference(member).asNormalReference(),
                VAR_INPUT,
                Dafny.getMemberFieldValue(member));
    }

    /**
     * This logic assumes we are setting a parameter on a builder
     * generated by BuilderSpecs or something equivalent.
     * @return CodeBlock of Method to set a member field. */
    protected CodeBlock setMemberField(MemberShape shape) {
        return CodeBlock.of("$L", shape.getMemberName());
    }

    /**
     * Returns MethodReference that converts from
     * the Java Dafny memberShape to
     * the Java Native memberShape.
     */
    @SuppressWarnings({"DuplicatedCode", "OptionalGetWithoutIsPresent"})
    protected MethodReference memberConversionMethodReference(MemberShape memberShape) {
        Shape targetShape = subject.model.getShape(memberShape.getTarget()).get();
        // If the target is simple, use SIMPLE_CONVERSION_METHOD_FROM_SHAPE_TYPE
        if (ModelUtils.isSmithyApiOrSimpleShape(targetShape)) {
            return SIMPLE_CONVERSION_METHOD_FROM_SHAPE_TYPE.get(targetShape.getType());
        }
        final String methodName = capitalize(targetShape.getId().getName());
        // if in namespace, reference converter from this ToNative class
        if (subject.nativeNameResolver.isInServiceNameSpace(targetShape.getId())) {
            return new MethodReference(thisClassName, methodName);
        }
        // Otherwise, this target must be in another namespace,
        // reference converter from that namespace's ToNative class
        ClassName otherNamespaceToDafny = ClassName.get(
                DafnyNameResolverHelpers.packageNameForNamespace(targetShape.getId().getNamespace()),
                TO_NATIVE
        );
        return new MethodReference(otherNamespaceToDafny, methodName);
    }
}
