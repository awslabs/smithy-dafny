package software.amazon.polymorph.smithyjava.generator.awssdk.v2;

import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.TypeSpec;

import org.junit.Before;
import org.junit.Test;

import java.nio.file.Path;
import java.util.Map;

import javax.lang.model.element.Modifier;

import software.amazon.polymorph.smithyjava.ModelConstants;
import software.amazon.polymorph.util.Tokenizer;
import software.amazon.polymorph.utils.TokenTree;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.ShapeId;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import static software.amazon.polymorph.smithyjava.generator.awssdk.Constants.DoSomethingOperation;
import static software.amazon.polymorph.smithyjava.generator.awssdk.Constants.DoVoidOperation;
import static software.amazon.polymorph.smithyjava.generator.awssdk.Constants.MockKmsShim;

public class ShimTest {
    protected ShimV1 underTest;
    protected Model model;
    protected JavaAwsSdkV1 subject;

    @Before
    public void setup() {
        model = TestSetupUtils.setupLocalModel(ModelConstants.MOCK_KMS);
        subject = TestSetupUtils.setupAwsSdk(model, "kms");
        underTest = new ShimV1(subject);
    }

    @Test
    public void operation() {
        final MethodSpec actual = underTest.operation(
                ShapeId.fromParts("software.amazon.awssdk.kms", "DoSomething")
        ).orElseThrow(AssertionError::new);
        // By wrapping the actual method spec with a
        // TypeSpec and JavaFile,
        // Javapoet does not fully qualify every type name.
        // Which is nice.
        TypeSpec shim = TypeSpec.classBuilder("Shim")
                .addModifiers(Modifier.PUBLIC, Modifier.FINAL)
                .addMethod(actual)
                .build();

        JavaFile javaFile = JavaFile
                .builder(
                        subject.packageName,
                        shim)
                .build();
        final String actualString = javaFile.toString();
        assertTrue(
                ("""
                        Expected actual to contain excepted.
                        Actual:
                        %s

                        Expected:
                        %s""").formatted(
                        actualString, DoSomethingOperation
                ),
                actualString.contains(DoSomethingOperation)
        );
    }

    @Test
    public void operationVoid() {
        final MethodSpec actual = underTest.operation(
                ShapeId.fromParts("com.amazonaws.kms", "DoVoid")
        ).orElseThrow(AssertionError::new);
        // By wrapping the actual method spec with a
        // TypeSpec and JavaFile,
        // Javapoet does not fully qualify every type name.
        // Which is nice.
        TypeSpec shim = TypeSpec.classBuilder("Shim")
                .addModifiers(Modifier.PUBLIC, Modifier.FINAL)
                .addMethod(actual)
                .build();

        JavaFile javaFile = JavaFile
                .builder(
                        subject.packageName,
                        shim)
                .build();
        final String actualString = javaFile.toString();
        assertTrue(
                ("""
                        Expected actual to contain excepted.
                        Actual:
                        %s

                        Expected:
                        %s""").formatted(
                        actualString, DoVoidOperation
                ),
                actualString.contains(DoVoidOperation)
        );
    }

    @Test
    public void generate() {
        final Map<Path, TokenTree> actual = underTest.generate();
        // TODO: refactor so that Shim is written as
        // com.amazonaws.encryptionsdk.kms.Shim.java
        final Path expectedPath = Path.of("Dafny/Com/Amazonaws/Kms/Shim.java");
        Path[] temp = new Path[1];
        final Path actualPath = actual.keySet().toArray(temp)[0];
        assertEquals(expectedPath, actualPath);
        final String actualSource = actual.get(actualPath).toString();
        Tokenizer.tokenizeAndAssertEqual(MockKmsShim, actualSource);
    }
}
