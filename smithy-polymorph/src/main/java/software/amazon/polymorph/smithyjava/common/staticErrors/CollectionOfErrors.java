package software.amazon.polymorph.smithyjava.common.staticErrors;

import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.FieldSpec;
import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.ParameterizedTypeName;
import com.squareup.javapoet.TypeSpec;

import java.util.List;

import software.amazon.polymorph.smithyjava.common.BuilderSpecs;

import static javax.lang.model.element.Modifier.FINAL;
import static javax.lang.model.element.Modifier.PRIVATE;
import static javax.lang.model.element.Modifier.PROTECTED;
import static javax.lang.model.element.Modifier.PUBLIC;
import static software.amazon.polymorph.smithyjava.common.staticErrors.NativeError.NATIVE_ERROR;
import static software.amazon.polymorph.smithyjava.common.staticErrors.NativeError.THROWABLE_ARGS;

public class CollectionOfErrors {
    public final static String COLLECTION_ERROR = "CollectionOfErrors";

    public static JavaFile javaFile(String packageName) {
        ClassName className = ClassName.get(packageName, COLLECTION_ERROR);
        ClassName superName = ClassName.get(packageName, NATIVE_ERROR);
        List<FieldSpec> collectionArgs = getArgs(packageName);
        BuilderSpecs builderSpecs = new BuilderSpecs(
                className, superName, collectionArgs, THROWABLE_ARGS);
        TypeSpec.Builder spec = TypeSpec
                .classBuilder(className)
                .addModifiers(PUBLIC)
                .superclass(superName)
                .addType(builderSpecs.builderInterface())
                .addType(builderSpecs.builderImpl(
                        true,
                        builderSpecs.implModelConstructor(),
                        builderSpecs.implBuildMethod(true))
                );
        collectionArgs.forEach(field -> {
            spec.addField(field.type, field.name, PRIVATE, FINAL);
            spec.addMethod(MethodSpec
                    .methodBuilder(field.name)
                    .returns(field.type)
                    .addModifiers(PUBLIC)
                    .addStatement("return this.$L", field.name)
                    .build());
        });
        spec.addMethod(superBuilder(builderSpecs, packageName))
                .addMethod(builderSpecs.toBuilderMethod(true))
                .addMethod(builderSpecs.builderMethod());

        return JavaFile.builder(packageName, spec.build())
                .skipJavaLangImports(true)
                .build();
    }

    private static List<FieldSpec> getArgs(String packageName) {
        return List.of(
                FieldSpec.builder(
                        ParameterizedTypeName.get(
                                ClassName.get(List.class),
                                ClassName.get(packageName, NATIVE_ERROR)),
                        "list"
                ).build()
        );
    }

    private static MethodSpec superBuilder(BuilderSpecs builderSpecs, String packageName) {
        MethodSpec.Builder method =  MethodSpec
                .constructorBuilder()
                .addModifiers(PROTECTED)
                .addParameter(builderSpecs.builderImplName(), BuilderSpecs.BUILDER_VAR)
                .addStatement("super($L)", BuilderSpecs.BUILDER_VAR);
        getArgs(packageName).forEach(field ->
                method.addStatement(
                        "this.$L = $L.$L()",
                        field.name, BuilderSpecs.BUILDER_VAR, field.name
                ));
        return method.build();
    }
}
