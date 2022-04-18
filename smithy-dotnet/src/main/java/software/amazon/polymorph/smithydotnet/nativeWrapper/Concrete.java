package software.amazon.polymorph.smithydotnet.nativeWrapper;

import java.util.Optional;

import software.amazon.polymorph.smithydotnet.NativeWrapperCodegen;
import software.amazon.polymorph.utils.Token;
import software.amazon.polymorph.utils.TokenTree;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.EntityShape;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ShapeId;

import static software.amazon.polymorph.smithydotnet.DotNetNameResolver.classForCommonServiceException;
import static software.amazon.polymorph.smithydotnet.DotNetNameResolver.qualifiedTypeConverter;
import static software.amazon.polymorph.smithydotnet.DotNetNameResolver.qualifiedTypeConverterForCommonError;
import static software.amazon.polymorph.smithydotnet.TypeConversionDirection.FROM_DAFNY;
import static software.amazon.polymorph.smithydotnet.TypeConversionDirection.TO_DAFNY;

public class Concrete extends NativeWrapperCodegen {

    public Concrete(Model model, ShapeId serviceShapeId, ShapeId resourceShapeId) {
        super(model, serviceShapeId, resourceShapeId);
    }

    /**
     * Generates concreate NativeWrapper class, complete with copyright
     * and import statements.
     */
    public TokenTree generateConcrete() {
        TokenTree clazz = generateClass();
        return TokenTree
                .of(getPrelude())
                .append(TokenTree.of("\n"))
                .append(clazz)
                .lineSeparated();
    }

    TokenTree generateClass() {
        String className = nameResolver.nativeWrapperClassForResource(resourceShapeId);
        final TokenTree header = Token.of("internal class %s : %s ".formatted(
                className,
                nameResolver.dafnyTypeForShape(resourceShapeId)
        ));
        final TokenTree nativeBaseProperty = TokenTree.of(
                "internal readonly %s %s;".formatted(
                        nameResolver.baseClassForResource(resourceShapeId),
                        NATIVE_BASE_PROPERTY
                ));
        final TokenTree constructor = generateConstructor(className);
        final TokenTree operationWrappers = TokenTree.of(
                model.expectShape(resourceShapeId, EntityShape.class)
                        .getAllOperations().stream()
                        .map(this::generateOperationWrapper))
                .lineSeparated();
        final TokenTree body = TokenTree
                .of(nativeBaseProperty, constructor, operationWrappers)
                .lineSeparated()
                .braced();
        return header
                .append(body)
                .lineSeparated()
                .namespaced(Token.of(nameResolver.namespaceForService()));
    }

    TokenTree generateConstructor(String className) {
        final String methodName = "public %s(%s %s)".formatted(
                className,
                nameResolver.baseClassForResource(resourceShapeId),
                NATIVE_IMPL_INPUT
        );
        final String body = "%s = %s;".formatted(
                NATIVE_BASE_PROPERTY,
                NATIVE_IMPL_INPUT
        );
        return TokenTree.of(methodName)
                .append(TokenTree.of(body).braced())
                .lineSeparated();
    }

     TokenTree generateOperationWrapper(final ShapeId operationShapeId) {
        final OperationShape operationShape = model.expectShape(operationShapeId, OperationShape.class);
        final String abstractDafnyOutput = nameResolver.dafnyTypeForServiceOperationOutput(operationShape);
        final String concreteDafnyOutput = nameResolver.dafnyTypeForServiceOperationOutput(operationShape, true);
        final String methodName = nameResolver.methodForOperation(operationShapeId);
        final Optional<String> input = operationShape.getInput()
                .map(shapeId -> "%s %s".formatted(
                        nameResolver.dafnyTypeForShape(shapeId), INPUT));
        final String signature = "public override %s %s(%s)".formatted(
                abstractDafnyOutput, methodName, input.orElse(""));
        final Optional<String> inputConversion = operationShape.getInput()
                .map(shapeId -> "%s%s %s = %s(%s);".formatted(
                        IGNORE_NAME,
                        nameResolver.baseTypeForShape(shapeId),
                        NATIVE_INPUT,
                        qualifiedTypeConverter(shapeId, FROM_DAFNY),
                        INPUT));
        final TokenTree tryBlock = generateTryNativeCall(
                operationShape,
                methodName,
                input,
                concreteDafnyOutput);
        final TokenTree body = TokenTree.of(
                        TokenTree.of(inputConversion.orElse("")),
                        tryBlock,
                        generateCatchServiceException(concreteDafnyOutput),
                        generateCatchGeneralException(concreteDafnyOutput))
                .lineSeparated().braced();
        return TokenTree.of(TokenTree.of(signature), body).lineSeparated();
    }

    TokenTree generateTryNativeCall(
            OperationShape operationShape,
            String methodName,
            Optional<String> input,
            String concreteDafnyOutput
    ) {
        final Optional<String> nativeCallPrefix = operationShape.getOutput()
                .map(shapeId -> "%s %s = ".formatted(
                        nameResolver.baseTypeForShape(shapeId),
                        NATIVE_OUTPUT));
        final String nativeCall = "%s%s %s.%s(%s);".formatted(
                nativeCallPrefix.isPresent() ? IGNORE_NAME : "",
                nativeCallPrefix.orElse(""),
                NATIVE_BASE_PROPERTY,
                methodName,
                input.isPresent() ? NATIVE_INPUT : "");
        final Optional<String> returnSuccessConversion = operationShape
                .getOutput()
                .map(shapeId -> "%s(%s)".formatted(
                        qualifiedTypeConverter(shapeId, TO_DAFNY),
                        NATIVE_OUTPUT));
        final String returnSuccess = "return %s.create_Success(%s);".formatted(
                concreteDafnyOutput, returnSuccessConversion.orElse(""));
        return TokenTree.of("try").append(
                TokenTree.of(nativeCall, returnSuccess).lineSeparated().braced()
        );
    }


     TokenTree generateCatchServiceException(final String dafnyOutput) {
        final String catchStatement = "catch(%s e)".formatted(
                classForCommonServiceException(serviceShape)
        );
        final String returnError = "return %s.create_Failure(%s(e));".formatted(
                dafnyOutput,
                qualifiedTypeConverterForCommonError(serviceShape, TO_DAFNY)
        );
        return TokenTree
                .of(catchStatement)
                .append(TokenTree.of(returnError).braced())
                .lineSeparated();
    }

    TokenTree generateCatchGeneralException(final String dafnyOutput) {
        final String catchStatement = "catch(Exception e)";
        final String castStatement = "new %s(e.Message)".formatted(
                classForCommonServiceException(serviceShape)
        );
        final String returnError = "return %s.create_Failure(%s(%s));".formatted(
                dafnyOutput,
                qualifiedTypeConverterForCommonError(serviceShape, TO_DAFNY),
                castStatement
        );
        return TokenTree
                .of(catchStatement)
                .append(TokenTree.of(returnError).braced())
                .lineSeparated();
    }
}
