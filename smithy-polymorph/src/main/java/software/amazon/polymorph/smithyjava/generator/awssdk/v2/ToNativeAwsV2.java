package software.amazon.polymorph.smithyjava.generator.awssdk.v2;

import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.CodeBlock;
import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.TypeSpec;

import java.util.Collections;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;

import javax.lang.model.element.Modifier;

import software.amazon.polymorph.smithyjava.generator.ToNative;
import software.amazon.polymorph.smithyjava.nameresolver.Dafny;
import software.amazon.polymorph.utils.ModelUtils;

import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.StringShape;
import software.amazon.smithy.model.shapes.StructureShape;
import software.amazon.smithy.model.traits.EnumTrait;

import static software.amazon.smithy.utils.StringUtils.capitalize;
import static software.amazon.smithy.utils.StringUtils.uncapitalize;

/**
 * ToNativeAwsV2 generates ToNative.
 * ToNative is a helper class for the AwsSdk's {@link ShimV2}.<p>
 * ToNative contains methods to convert
 * a subset of an AWS SDK Service's types
 * from Dafny generated Java to native Java.<p>
 * The subset is composed of:
 * <ul>
 *   <li>All the Service's Operations' inputs
 *   <li>All the fields contained by the above
 * </ul>
 * As such,
 * ToNativeAwsV2 holds the logic to generate these methods based on:
 * <ul>
 *     <li>a smithy model</li>
 *     <li>knowledge of how smithy-dafny generates Dafny for AWS SDK</li>
 *     <li>knowledge of how Dafny compiles Java</li>
 *     <li>knowledge of the patterns of the AWS SDK V2 for Java</li>
 * </ul>
 */
public class ToNativeAwsV2 extends ToNative {
    protected final static String VAR_OUTPUT = "converted";
    protected final static String VAR_TEMP = "temp";

    // TODO: for V2 support, use abstract AwsSdk name resolvers and sub class for V1 or V2.

    // Hack to override CodegenSubject
    // See code comment on ../library/ModelCodegen for details.
    private final JavaAwsSdkV2 subject;

    public ToNativeAwsV2(JavaAwsSdkV2 awsSdk) {
        super(awsSdk, ClassName.get(awsSdk.packageName, TO_NATIVE));
        this.subject = awsSdk;
    }

    @Override
    public Set<JavaFile> javaFiles() {
        JavaFile.Builder builder = JavaFile.builder(subject.packageName, toNative());
        return Collections.singleton(builder.build());
    }

    TypeSpec toNative() {
        LinkedHashSet<ShapeId> operationInputs = subject.serviceShape.getOperations().stream()
                .map(shapeId -> subject.model.expectShape(shapeId, OperationShape.class))
                .map(OperationShape::getInputShape)
                .collect(Collectors.toCollection(LinkedHashSet::new));
        Set<ShapeId> allRelevantShapeIds = ModelUtils.findAllDependentShapes(operationInputs, subject.model);
        List<MethodSpec> convertRelevant = allRelevantShapeIds.stream()
                .map(this::generateConvert).filter(Objects::nonNull).toList();
        return TypeSpec
                .classBuilder(
                        ClassName.get(subject.packageName, TO_NATIVE))
                .addModifiers(Modifier.PUBLIC)
                .addMethods(convertRelevant)
                .build();
    }

    @SuppressWarnings({"OptionalGetWithoutIsPresent"})
    MethodSpec generateConvert(ShapeId shapeId) {
        final Shape shape = subject.model.getShape(shapeId)
                .orElseThrow(() -> new IllegalStateException("Cannot find shape " + shapeId));
        return switch (shape.getType()) {
            // For the AWS SDK for Java V2, we do not generate converters for simple shapes
            case BLOB, BOOLEAN, TIMESTAMP, BYTE, SHORT,
                    INTEGER, LONG, BIG_DECIMAL, BIG_INTEGER, MEMBER -> null;
            case STRING -> generateConvertString(shapeId); // STRING handles enums
            case LIST -> modeledList(shape.asListShape().get());
            case SET -> modeledSet(shape.asSetShape().get());
            case MAP -> modeledMap(shape.asMapShape().get());
            case STRUCTURE -> modeledStructure(shape.asStructureShape().get());
            default -> throw new UnsupportedOperationException(
                    "ShapeId %s is of Type %s, which is not yet supported for ToDafny"
                            .formatted(shapeId, shape.getType()));
        };
    }

    @Override
    protected MethodSpec modeledStructure(StructureShape structureShape) {
        String methodName = capitalize(structureShape.getId().getName());
        ClassName nativeClassName = subject.nativeNameResolver.classNameForStructure(structureShape);
        MethodSpec.Builder builder = MethodSpec
                .methodBuilder(methodName)
                .addModifiers(Modifier.STATIC, Modifier.PUBLIC)
                .returns(nativeClassName)
                .addParameter(subject.dafnyNameResolver.typeForShape(structureShape.getId()), VAR_INPUT);

        if (structureShape.members().size() == 0) {
            builder.addStatement("return new $T()", nativeClassName);
            return builder.build();
        }
        builder.addStatement("$T $L = new $T()", nativeClassName, VAR_OUTPUT, nativeClassName);

        // For each member
        structureShape.members()
                .forEach(member -> {
                    // if optional, check if present
                    if (member.isOptional()) {
                        builder.beginControlFlow("if ($L.$L.is_Some())", VAR_INPUT, Dafny.getMemberField(member));
                    }
                    // if converting a LIST or SET of enums
                    if (ModelUtils.isListOrSetOfEnums(member.getTarget(), subject.model)) {
                        // create temp array
                        builder.addStatement(initTempArray(member));
                        // set with conversion call and toArray
                        builder.addStatement(setWithConversionCallAndToArray(member));
                    } else {
                        // set with conversion call
                        builder.addStatement(setWithConversionCall(member, Dafny.getMemberFieldValue(member)));
                    }
                    if (member.isOptional()) builder.endControlFlow();
                });
        return builder.addStatement("return $L", VAR_OUTPUT).build();
    }

    /**
     * Generates an Array of member's type with size of input's field.
     * i.e:<p> {@code MemberType[] member_temp = new MemberType[dafnyValue.Member.length()];}
     */
    CodeBlock initTempArray(MemberShape member) {
        return CodeBlock.of("$T[] $L_$L = new $T[$L.$L.$L]",
                subject.nativeNameResolver.typeForListOrSetMember(member.getTarget()),
                uncapitalize(member.getMemberName()), VAR_TEMP,
                subject.nativeNameResolver.typeForListOrSetMember(member.getTarget()),
                VAR_INPUT, Dafny.getMemberFieldValue(member),
                Dafny.aggregateSizeMethod(subject.model.expectShape(member.getTarget()).getType()));
    }

    @Override
    protected CodeBlock setWithConversionCall(MemberShape member, CodeBlock getMember) {
        return CodeBlock.of("$L.$L($L($L.$L))",
                VAR_OUTPUT,
                setMemberField(member),
                memberConversionMethodReference(member).asNormalReference(),
                VAR_INPUT,
                Dafny.getMemberFieldValue(member));
    }

    CodeBlock setWithConversionCallAndToArray(MemberShape member) {
        return CodeBlock.of("$L.$L($L($L.$L).toArray($L_$L))",
                VAR_OUTPUT,
                setMemberField(member),
                memberConversionMethodReference(member).asNormalReference(),
                VAR_INPUT,
                Dafny.getMemberFieldValue(member),
                uncapitalize(member.getMemberName()), VAR_TEMP);
    }

    /** @return CodeBlock of Method to set a member field. */
    @Override
    protected CodeBlock setMemberField(MemberShape shape) {
        // In AWS SDK Java V2, using `with` allows for enums or strings
        // while `set` only allows for strings.
        return CodeBlock.of("with$L", capitalize(shape.getMemberName()));
    }

    MethodSpec generateConvertString(ShapeId shapeId) {
        final StringShape shape = subject.model.expectShape(shapeId, StringShape.class);
        if (shape.hasTrait(EnumTrait.class)) {
            return generateConvertEnum(shapeId);
        }
        return null;
    }

    MethodSpec generateConvertEnum(ShapeId shapeId) {
        final StringShape shape = subject.model.expectShape(shapeId, StringShape.class);
        final ClassName returnType = subject.nativeNameResolver.classForEnum(shape);
        MethodSpec.Builder method = modeledEnumCommon(shape, returnType);
        // fromValue is an AWS SDK specific feature
        method.addStatement("return $T.fromValue($L.toString())", returnType, VAR_INPUT);
        return method.build();
    }
}
