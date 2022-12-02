package software.amazon.polymorph.smithyjava.modeled;

import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.FieldSpec;
import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.TypeSpec;

import java.util.Collections;
import java.util.List;

import javax.lang.model.element.Modifier;

import software.amazon.polymorph.smithyjava.BuildMethod;
import software.amazon.polymorph.smithyjava.BuilderSpecs;
import software.amazon.polymorph.smithyjava.generator.CodegenSubject;
import software.amazon.smithy.model.shapes.Shape;

import static javax.lang.model.element.Modifier.FINAL;
import static javax.lang.model.element.Modifier.PRIVATE;
import static javax.lang.model.element.Modifier.PROTECTED;

public class ModeledStructure {

    public static JavaFile javaFile(String packageName, Shape shape, CodegenSubject subject) {
        if (!(shape.isUnionShape() || shape.isStructureShape())) {
            throw new IllegalArgumentException(
                    "ModeledStructure should only be called for Structures or Unions. ShapeId: %s"
                            .formatted(shape.getId()));
        }
        ClassName className = ClassName.get(packageName, shape.getId().getName());
        List<FieldSpec> modelFields = BuilderSpecs.shapeToArgs(shape, subject.nativeNameResolver);
        List<FieldSpec> superFields = Collections.emptyList();
        boolean override = false;
        BuilderSpecs builderSpecs = new BuilderSpecs(className, null, modelFields, superFields);
        MethodSpec builderImplBuildMethod = BuildMethod.implBuildMethod(override, shape, subject, packageName);
        TypeSpec builderInterface = builderSpecs.builderInterface();
        TypeSpec builderImpl = builderSpecs.builderImpl(
                override, builderSpecs.implModelConstructor(), builderImplBuildMethod
        );
        // Unions are nearly identical to Structures,
        // except they have an additional constraint.
        if (shape.isUnionShape()) {
            // The shape is a union, "asUnionShape" will return a value
            //noinspection OptionalGetWithoutIsPresent
            builderImpl = builderImpl.toBuilder().addMethod(
                    ModeledUnion.unionValidate(shape.asUnionShape().get()))
                    .build();
        }
        TypeSpec.Builder spec = TypeSpec
                .classBuilder(className)
                .addModifiers(Modifier.PUBLIC)
                .addType(builderInterface)
                .addType(builderImpl);

        modelFields.forEach(field -> {
            // Add fields
            spec.addField(field.type, field.name, PRIVATE, FINAL);
            // Add getter methods
            spec.addMethod(MethodSpec
                    .methodBuilder(field.name)
                    .addModifiers(Modifier.PUBLIC)
                    .returns(field.type)
                    .addStatement("return this.$L", field.name)
                    .build());
        });
        spec.addMethod(constructor(builderSpecs, modelFields))
                .addMethod(builderSpecs.toBuilderMethod(override))
                .addMethod(builderSpecs.builderMethod());
        return JavaFile.builder(packageName, spec.build())
                .skipJavaLangImports(true)
                .build();
    }


    private static MethodSpec constructor(BuilderSpecs builderSpecs, List<FieldSpec> fields) {
        MethodSpec.Builder method =  MethodSpec
                .constructorBuilder()
                .addModifiers(PROTECTED)
                .addParameter(builderSpecs.builderImplName(), BuilderSpecs.BUILDER_VAR);
        fields.forEach(field -> method.addStatement(
                "this.$L = $L.$L()",
                field.name, BuilderSpecs.BUILDER_VAR, field.name
        ));
        return method.build();
    }
}
