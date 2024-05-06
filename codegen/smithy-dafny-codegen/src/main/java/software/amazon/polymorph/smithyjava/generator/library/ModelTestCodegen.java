package software.amazon.polymorph.smithyjava.generator.library;

import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.TypeName;
import com.squareup.javapoet.TypeSpec;
import software.amazon.polymorph.smithyjava.BuilderSpecs;
import software.amazon.polymorph.smithyjava.generator.Generator;
import software.amazon.polymorph.smithyjava.modeled.ModeledNode;
import software.amazon.polymorph.smithyjava.modeled.ModeledUnion;
import software.amazon.polymorph.smithyjava.unmodeled.CollectionOfErrors;
import software.amazon.polymorph.smithyjava.unmodeled.OpaqueError;
import software.amazon.polymorph.traits.LocalServiceTrait;
import software.amazon.smithy.model.node.Node;
import software.amazon.smithy.model.node.NumberNode;
import software.amazon.smithy.model.node.ObjectNode;
import software.amazon.smithy.model.node.StringNode;
import software.amazon.smithy.model.shapes.IntegerShape;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.NumberShape;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.StringShape;
import software.amazon.smithy.model.shapes.StructureShape;
import software.amazon.smithy.model.shapes.UnionShape;
import software.amazon.smithy.smoketests.traits.SmokeTestCase;
import software.amazon.smithy.smoketests.traits.SmokeTestsTrait;

import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

import static javax.lang.model.element.Modifier.FINAL;
import static javax.lang.model.element.Modifier.PUBLIC;

public class ModelTestCodegen extends Generator {

    final JavaLibrary subject;

    public ModelTestCodegen(JavaLibrary subject) {
        super(subject);
        this.subject = subject;
    }

    @Override
    public Set<JavaFile> javaFiles() {
        LinkedHashSet<JavaFile> rtn = new LinkedHashSet<>();
        subject.model.getOperationShapesWithTrait(SmokeTestsTrait.class).stream()
                .map(this::smokeTestsClass)
                .forEachOrdered(rtn::add);
        return rtn;
    }

    private JavaFile smokeTestsClass(OperationShape shape) {
        TypeSpec.Builder spec = TypeSpec
                .classBuilder(shape.getId().getName() + "SmokeTests")
                .addModifiers(PUBLIC, FINAL);
        SmokeTestsTrait smokeTests = shape.expectTrait(SmokeTestsTrait.class);
        smokeTests.getTestCases().stream()
                .map(testCase -> smokeTest(shape, testCase))
                .forEachOrdered(spec::addMethod);
        TypeSpec classType = spec.build();
        return JavaFile.builder(subject.modelPackageName, classType).build();
    }

    private MethodSpec smokeTest(final OperationShape operationShape, SmokeTestCase testCase) {

        // TODO: escape all names properly
        final String methodName = testCase.getId();
        MethodSpec.Builder method = MethodSpec
                .methodBuilder(methodName)
                .addAnnotation(Constants.JUPITER_TEST)
                .addModifiers(PUBLIC)
                .returns(TypeName.VOID);

        final TypeName clientType = subject.nativeNameResolver.typeForShape(subject.serviceShape.toShapeId());
        final String operationName = operationShape.toShapeId().getName();
        final ShapeId configShapeId = subject.serviceShape.expectTrait(LocalServiceTrait.class).getConfigId();
        final TypeName configType = subject.nativeNameResolver.typeForShape(configShapeId);

        // SimpleConstraintsConfig config = SimpleConstraintsConfig.builder().build();
        // SimpleConstraints client = SimpleConstraints.builder()
        //         .SimpleConstraintsConfig(config)
        //         .build();
        method.addStatement("$T config = $T.builder().build()", configType, configType);
        method.addStatement("$T client = $T.builder().$L(config).build()", clientType, clientType, configShapeId.getName());

        // GetConstraintsInput inputBuilder = GetConstraintsInput.builder();
        // ...
        // (multiple calls to populate builder)
        // ...
        // GetConstraintsInput input = inputBuilder.build();
        // TODO: error handling
        Shape inputShape = subject.model.expectShape(operationShape.getInput().get());
        ModeledNode.declareModeledValue(subject, method, "input", inputShape, testCase.getParams().get());

        // client.GetConstraints(input);
        // TODO: or assertThrows(...)
        method.addStatement("client.$L(input)", operationName);

        return method.build();
    }


}
