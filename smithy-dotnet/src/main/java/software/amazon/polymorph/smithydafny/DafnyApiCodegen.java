// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package software.amazon.polymorph.smithydafny;

import com.google.common.annotations.VisibleForTesting;
import software.amazon.polymorph.traits.DafnyUtf8BytesTrait;
import software.amazon.polymorph.traits.DataTypeUnionTrait;
import software.amazon.polymorph.traits.PositionalTrait;
import software.amazon.polymorph.traits.ReferenceTrait;
import software.amazon.polymorph.utils.ModelUtils;
import software.amazon.polymorph.utils.Token;
import software.amazon.polymorph.utils.TokenTree;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.*;
import software.amazon.smithy.model.traits.*;

import java.nio.file.Path;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.HashSet;
import java.util.stream.Stream;

public class DafnyApiCodegen {
    private final Model model;
    private final ServiceShape serviceShape;
    private final DafnyNameResolver nameResolver;
    private final Path modelPath;

    public DafnyApiCodegen(
      final Model model,
      final ServiceShape serviceShape,
      final Path modelPath,
      final Path[] dependentModelPaths
    ) {
        this.model = model;
        this.serviceShape = serviceShape;
        this.modelPath = modelPath;
        this.nameResolver = new DafnyNameResolver(
          model,
          this.serviceShape.toShapeId().getNamespace(),
          new HashSet(),
          dependentModelPaths.clone()
        );
    }

    public Map<Path, TokenTree> generate() {

        // I generate the types *first*
        // This is because the generated types
        // MAY depend on other models.
        // In this case I need these modules
        // so that I can include them.
        final TokenTree generatedTypes = TokenTree
          .of(
            model
              .shapes()
              .filter(shape -> ModelUtils.isInServiceNamespace(shape.getId(), serviceShape))
              // Sort by shape ID for deterministic generated code
              .collect(Collectors.toCollection(TreeSet::new))
              .stream()
              .map(this::generateCodeForShape)
              .flatMap(Optional::stream)
            )
          .lineSeparated();

        // The generated module MUST be abstract.
        // Because can never have a concrete implementation.
        // TODO: Resolve the above :(
        final String moduleName = DafnyNameResolver
          .dafnyTypesModuleForNamespace(serviceShape.getId().getNamespace());
        final TokenTree moduleHeader = Token.of("module %s".formatted(moduleName));

        // A smithy model may reference a model in a different package.
        // In which case we need to include it.
        final TokenTree includeDirectives = TokenTree
          .of(Stream
            .concat(
              Stream
                .of(
                  // These are Obviously wrong #JustHardCodedThings...
                  "../../StandardLibrary/StandardLibrary.dfy",
                  "../../Util/UTF8.dfy"
                ),
              nameResolver
                .dependentModels()
                .stream()
                .map(d -> modelPath
                  .relativize(d.modelPath().resolve(nameResolver.dafnyTypesModuleForNamespace(d.namespace()) + ".dfy"))
                )
                .map(Path::toString)
            )
            .map(p -> "include \"" + p + "\"")
            .map(Token::of)
          )
          .lineSeparated();

        // A smithy model may reference a model in a different package.
        // In which case we need to import it.
        final TokenTree modulePrelude = TokenTree
          .of(Stream
            .concat(
                Stream
                .of(
                  "import opened Wrappers",
                  "import opened StandardLibrary.UInt",
                  "import opened UTF8"
                ),
              nameResolver
                .dependentModels()
                .stream()
                .map(d ->
                  "import " + nameResolver.dafnyTypesModuleForNamespace(d.namespace())))
            .map(i -> Token.of(i)))
          .lineSeparated();

        final TokenTree moduleBody = TokenTree
            .of(
              modulePrelude,
              generatedTypes,
              // Error types are generates *after*
              // all other types to account
              // for any dependant modules
              generateModeledErrorDataType()
            )
            .lineSeparated()
            .braced();

        final Path path = Path.of("%s.dfy".formatted(moduleName));
        final TokenTree fullCode = TokenTree
          .of(includeDirectives, moduleHeader, moduleBody)
          .lineSeparated();
        return Map.of(path, fullCode);
    }

    private Optional<TokenTree> generateCodeForShape(final Shape shape) {
        final ShapeId shapeId = shape.getId();
        return Optional.ofNullable(switch (shape.getType()) {
            case SERVICE -> {
                if (shape != serviceShape) {
                    throw new IllegalStateException("Unexpected service shape found");
                }
                yield TokenTree
                  .of(generateServiceTraitDefinition())
                  .lineSeparated();
            }
            case BLOB -> generateBlobTypeDefinition(shapeId);
            case BOOLEAN -> generateBoolTypeDefinition(shapeId);
            case STRING -> {
                if (shape.hasTrait(EnumTrait.class)) {
                    yield generateEnumTypeDefinition(shapeId);
                } else if (shape.hasTrait(DafnyUtf8BytesTrait.ID)) {
                    yield generateValidUTF8BytesType(shapeId);
                } else {
                    yield generateStringTypeDefinition(shapeId);
                }
            }
            case INTEGER, LONG -> generateNumericTypeDefinition(shapeId);
            case LIST -> generateListTypeDefinition(shapeId);
            case MAP -> generateMapTypeDefinition(shapeId);
            case STRUCTURE -> {
                if (shape.hasTrait(TraitDefinition.class)) {
                    yield null;
                } else if (shape.hasTrait(PositionalTrait.class)) {
                    yield null;
                } else if (shape.hasTrait(ReferenceTrait.class)) {
                    yield generateReferenceTraitDefinition(shapeId);
                } else if (shape.hasTrait(ErrorTrait.class)) {
                    // All error shapes are a single integrated data type
                    yield null;
                } else if (shape.hasTrait(DataTypeUnionTrait.class)) {
                    // Such structures only exist within the datatype formed by the union
                    yield null;
                } else {
                    yield generateStructureTypeDefinition(shapeId);
                }
            }
            case UNION -> generateUnionTypeDefinition(shapeId);
            default -> null;
        });
    }

    public TokenTree generateValidUTF8BytesType(final ShapeId shapeId) {
        final StringShape stringShape = model.expectShape(shapeId, StringShape.class);
        final Optional<TokenTree> lengthConstraint = stringShape
                .getTrait(LengthTrait.class)
                .map(DafnyApiCodegen::generateLengthConstraint);
        return generateSubsetType(shapeId, "ValidUTF8Bytes", lengthConstraint);
    }

    public TokenTree generateBlobTypeDefinition(final ShapeId blobShapeId) {
        final BlobShape blobShape = model.expectShape(blobShapeId, BlobShape.class);
        final Optional<TokenTree> lengthConstraint = blobShape.getTrait(LengthTrait.class)
                .map(DafnyApiCodegen::generateLengthConstraint);
        return generateSubsetType(blobShapeId, "seq<uint8>", lengthConstraint);
    }

    public TokenTree generateBoolTypeDefinition(final ShapeId boolShapeId) {
        return generateTypeSynonym(boolShapeId, "bool");
    }

    public TokenTree generateStringTypeDefinition(final ShapeId stringShapeId) {
        final StringShape stringShape = model.expectShape(stringShapeId, StringShape.class);
        final Optional<TokenTree> lengthConstraint = stringShape.getTrait(LengthTrait.class)
                .map(DafnyApiCodegen::generateLengthConstraint);
        return generateSubsetType(stringShapeId, "string", lengthConstraint);
    }

    public TokenTree generateEnumTypeDefinition(final ShapeId stringShapeId) {
        final StringShape stringShape = model.expectShape(stringShapeId, StringShape.class);
        final EnumTrait enumTrait = stringShape.getTrait(EnumTrait.class).orElseThrow();

        if (!enumTrait.hasNames()) {
            throw new UnsupportedOperationException("Unnamed enums not supported");
        }
        //noinspection OptionalGetWithoutIsPresent
        final TokenTree constructors = TokenTree.of(enumTrait.getValues().stream()
                .map(enumDefinition -> enumDefinition.getName().get())
                .peek(name -> {
                    if (!ModelUtils.isValidEnumDefinitionName(name)) {
                        throw new UnsupportedOperationException("Invalid enum definition name: %s".formatted(name));
                    }
                })
                .map(name -> TokenTree.of("\n\t|", name)));

        return Token.of("datatype %s =".formatted(nameResolver.baseTypeForShape(stringShapeId))).append(constructors);
    }

    public TokenTree generateNumericTypeDefinition(final ShapeId numberShapeId) {
        final NumberShape numberShape = model.expectShape(numberShapeId, NumberShape.class);
        final Optional<TokenTree> rangeConstraint = numberShape.getTrait(RangeTrait.class)
                .map(DafnyApiCodegen::generateRangeConstraint);
        final String baseType = Objects.requireNonNull(
                DafnyNameResolver.DAFNY_TYPES_BY_SIMPLE_SHAPE_TYPE.get(numberShape.getType()));
        return generateSubsetType(numberShapeId, baseType, rangeConstraint);
    }

    public TokenTree generateListTypeDefinition(final ShapeId listShapeId) {
        final ListShape listShape = model.expectShape(listShapeId, ListShape.class);
        final Optional<TokenTree> lengthConstraint = listShape.getTrait(LengthTrait.class)
                .map(DafnyApiCodegen::generateLengthConstraint);
        final String baseType = "seq<%s>".formatted(nameResolver.baseTypeForShape(listShape.getMember().getId()));
        return generateSubsetType(listShapeId, baseType, lengthConstraint);
    }

    public TokenTree generateMapTypeDefinition(final ShapeId mapShapeId) {
        final MapShape mapShape = model.expectShape(mapShapeId, MapShape.class);
        final Optional<TokenTree> lengthConstraint = mapShape.getTrait(LengthTrait.class)
                .map(DafnyApiCodegen::generateLengthConstraint);
        final String keyType = nameResolver.baseTypeForShape(mapShape.getKey().getId());
        final String valueType = nameResolver.baseTypeForShape(mapShape.getValue().getId());
        final String baseType = "map<%s, %s>".formatted(keyType, valueType);
        return generateSubsetType(mapShapeId, baseType, lengthConstraint);
    }

    public TokenTree generateStructureTypeDefinition(final ShapeId structureShapeId) {
        final StructureShape structureShape = model.expectShape(structureShapeId, StructureShape.class);

        final String typeName = structureShapeId.getName();
        return TokenTree.of(
          Token.of("datatype %1$s =".formatted(typeName)),
          generateDataTypeConstructorFromStructure(structureShapeId)
        );
    }

    public TokenTree generateUnionTypeDefinition(final ShapeId unionShapeId) {
        final UnionShape unionShape = model.expectShape(unionShapeId, UnionShape.class);

        return TokenTree.of(
          Token.of("datatype %s =".formatted(nameResolver.baseTypeForShape(unionShapeId))),
          TokenTree.of(
            unionShape
              .members()
              .stream()
                .map(member -> {
                    if (member.getMemberName().equals(member.getTarget().getName())) {
                        return generateDataTypeConstructorFromStructure(member.getTarget());
                    } else {
                        throw new UnsupportedOperationException("for now, they MUST match");
                    }
                })).lineSeparated()
          ).lineSeparated();
    }

    private TokenTree generateStructureTypeParameter(final MemberShape memberShape) {
        return Token.of("nameonly %s: %s".formatted(
                memberShape.getMemberName(), nameResolver.baseTypeForShape(memberShape.getId())));
    }

    public TokenTree generateServiceTraitDefinition() {

        final TokenTree trait = TokenTree.of("trait {:termination false}", nameResolver.traitForServiceClient(serviceShape));
        final TokenTree predicatesAndMethods = TokenTree.of(
                serviceShape.getAllOperations().stream().map(this::generateOperationPredicatesAndMethod))
                .lineSeparated();
        return TokenTree.of(trait, predicatesAndMethods.braced());
    }
    
    public TokenTree generateReferenceTraitDefinition(final ShapeId shapeWithReference) {
        final ReferenceTrait referenceTrait = model
          .getShape(shapeWithReference)
          .orElseThrow()
          .expectTrait(ReferenceTrait.class);

        // This is a reference structure for returning a service
        // As such, there is no need to build any code here.
        // The actual implementation of the service
        // would be in that services Smithy module.
        if (referenceTrait.isService()) {
            return null;
        }

        final ResourceShape resource = model
          .getShape(referenceTrait.getReferentId())
          .orElseThrow()
          .asResourceShape()
          .orElseThrow();
    
        final TokenTree trait = TokenTree.of("trait {:termination false}", nameResolver.baseTypeForShape(shapeWithReference));
        final TokenTree predicatesAndMethods = TokenTree
          .of(
              resource
                .getAllOperations()
                .stream()
                .map(this::generateOperationPredicatesAndMethod)
          )
          .lineSeparated();
        return TokenTree.of(trait, predicatesAndMethods.braced());
    }

    public TokenTree generateOperationPredicatesAndMethod(final ShapeId operationShapeId) {

        final OperationShape operationShape = model.expectShape(operationShapeId, OperationShape.class);
        final TokenTree operationParams = operationShape.getInput()
                .map(nameResolver::baseTypeForShape)
                .map(inputType -> TokenTree.of("input:", inputType))
                .orElse(TokenTree.empty())
                ;
        final Optional<String> outputType = this.nameResolver.returnTypeForResult(operationShape);
        TokenTree returnType = TokenTree.empty();
        if (outputType.isPresent()) {
            returnType = TokenTree.of("output: %s".formatted(outputType.get()));
        }

        final TokenTree wrappedReply = Token.of("output: %s"
                .formatted(nameResolver.returnTypeForOperation(operationShape)));
        final TokenTree calledWithPredicate = this.generatePredicateCalledWith(operationShape, operationParams);
        final TokenTree succeededWithPredicate = this.generatePredicateSucceededWith(operationShape, operationParams, returnType);
        final TokenTree method = this.generateOperationMethod(operationShape, operationParams, wrappedReply);

        return TokenTree.of(calledWithPredicate, succeededWithPredicate, method).lineSeparated().append(TokenTree.of("\n"));
    }

    private TokenTree generateOperationMethod(
            final OperationShape operationShape,
            final TokenTree operationParams,
            final TokenTree wrappedReply
    ) {
        final TokenTree name = TokenTree.of("method", nameResolver.methodForOperation(operationShape));
        final TokenTree params = operationParams.parenthesized();
        final TokenTree returns = TokenTree.of("returns ").append(wrappedReply.parenthesized());

        // The formal Dafny name for this section of a method is "specification".
        // To avoid confusion with RFC-style "specifications", instead use the term "conditions".
        TokenTree conditions = TokenTree.empty();

        TokenTree ensureCalledWith = TokenTree.of(
                "\n\tensures "
                        + nameResolver.predicateCalledWith(operationShape)
        );
        TokenTree ensureSucceededWith = TokenTree.of(
                "\n\tensures output.Success? ==> "
                        + nameResolver.predicateSucceededWith(operationShape)
        );
        TokenTree ensureCalledWithParams = TokenTree.empty();
        TokenTree ensureSucceededWithParams = TokenTree.empty();
        if (operationShape.getInput().isPresent()) {
            ensureCalledWithParams = ensureCalledWithParams.append(TokenTree.of("input"));
            ensureSucceededWithParams = ensureSucceededWithParams.append(TokenTree.of("input"));
        }
        if (operationShape.getInput().isPresent() && operationShape.getOutput().isPresent()) {
            ensureSucceededWithParams = ensureSucceededWithParams.append(TokenTree.of(","));
        }
        if (operationShape.getOutput().isPresent()) {
            ensureSucceededWithParams = ensureSucceededWithParams.append(TokenTree.of("output.value"));
        }
        ensureCalledWithParams = ensureCalledWithParams.parenthesized();
        ensureSucceededWithParams = ensureSucceededWithParams.parenthesized();

        ensureCalledWith = ensureCalledWith.append(ensureCalledWithParams);
        ensureSucceededWith = ensureSucceededWith.append(ensureSucceededWithParams);
        conditions = conditions.append(ensureCalledWith);
        conditions = conditions.append(ensureSucceededWith);
        return TokenTree.of(name, params, returns, conditions);
    }

    private TokenTree generatePredicateCalledWith(
            final OperationShape operationShape,
            final TokenTree operationParams
    ) {
        final TokenTree name = TokenTree.of("predicate {:opaque}", nameResolver.predicateCalledWith(operationShape));
        TokenTree params = TokenTree.of("(");
        if (operationShape.getInput().isPresent()) {
            params = TokenTree.of(params, operationParams);
        }
        params = params.append(TokenTree.of(")"));
        final TokenTree body = TokenTree.of("{true}");
        return TokenTree.of(name, params, body);
    }

    private TokenTree generatePredicateSucceededWith(
            final OperationShape operationShape,
            final TokenTree operationParams,
            final TokenTree returnType
    ) {
        TokenTree params = TokenTree.empty();
        if (operationShape.getInput().isPresent()) {
            params = TokenTree.of(params, operationParams);
        }
        if (operationShape.getInput().isPresent() && operationShape.getOutput().isPresent()) {
            params = params.append(TokenTree.of(","));
        }
        if (operationShape.getOutput().isPresent()) {
            params = params.append(returnType);
        }
        params = params.parenthesized();
        final TokenTree name = TokenTree.of("predicate {:opaque}", nameResolver.predicateSucceededWith(operationShape));
        final TokenTree body = TokenTree.of("{true}");
        return TokenTree.of(name, params, body);
    }


    /**
     * Generates the service's error types that are not modeled directly. These include:
     * <ul>
     *     <li>the error trait</li>
     *     <li>an "unknown error" class</li>
     * </ul>
     */
    public TokenTree generateModeledErrorDataType() {
    // TODO need to add dependent errors...
        return TokenTree.of(
          Token.of("datatype Error ="),
          Token.of("// Local Error structures are listed here"),
          TokenTree.of(
            ModelUtils
              .streamNamespaceErrors(model, serviceShape.getId().getNamespace())
              .map(Shape::getId)
              .map(this::generateDataTypeConstructorFromStructure)
          ).lineSeparated(),
          Token.of("// Any dependent models are listed here"),
          TokenTree.of(
            nameResolver
              .dependentModels()
              .stream()
              .map(this::generateDependantWrappedDataTypeConstructor)
          ).lineSeparated(),
          Token.of("// The Opaque error, used for native, extern, wrapped or unknown errors"),
          Token.of("| Opaque(obj: object)"),
          // Helper error for use with `extern`
          Token.of("type OpaqueError = e: Error | e.Opaque? witness *")
        ).lineSeparated();
    }

    public TokenTree generateDataTypeConstructorFromStructure(final ShapeId shapeId) {
        final StructureShape structureShape = model.expectShape(shapeId, StructureShape.class);
        final String typeName = shapeId.getName();

        final TokenTree params = TokenTree
          .of(ModelUtils
            .streamStructureMembers(structureShape)
            .map(this::generateStructureTypeParameter)
          )
          // This combines the trees
          .separated(TokenTree.of(Token.of(","), Token.NEWLINE))
          .parenthesized()
          // Because `separated` combined things, this works nicely
          .lineSeparated();

        return TokenTree.of(
          Token.of("| %1$s".formatted(typeName)),
          params);
    }

    public TokenTree generateWrappedDataTypeConstructorFromUnionMember(final MemberShape memberShape) {
        final String name = memberShape.getMemberName();
        final String wrappedType = nameResolver.baseTypeForShape(memberShape.getTarget());


        return TokenTree.empty();
    }



    public TokenTree generateDependantWrappedDataTypeConstructor(final DependentSmithyModel dependentSmithyModel) {
        final String errorType = nameResolver.dafnyTypesModuleForNamespace(dependentSmithyModel.namespace()) + ".Error";
        final String errorConstructorName = errorType
          .replace("Types.Error", "");
        final String errorDestructorsName = Character.toLowerCase(errorConstructorName.charAt(0)) + errorConstructorName.substring(1);

        return TokenTree.of(
          Token.of("| %s(%s: %s)"
            .formatted(errorConstructorName, errorDestructorsName, errorType))
        );
    }


    private static TokenTree generateLengthConstraint(final LengthTrait lengthTrait) {
        final String min = lengthTrait.getMin().map("%s <="::formatted).orElse("");
        final String max = lengthTrait.getMax().map("<= %s"::formatted).orElse("");
        return TokenTree.of(min, "|x|", max);
    }

    private static TokenTree generateRangeConstraint(final RangeTrait rangeTrait) {
        final String min = rangeTrait.getMin().map("%s <="::formatted).orElse("");
        final String max = rangeTrait.getMax().map("<= %s"::formatted).orElse("");
        return TokenTree.of(min, "x", max);
    }

    /**
     * Given a name {@code TypeName}, base type {@code BaseType}, and constraint predicate expressions
     * {@code c1, c2, ..., cN} over a free variable {@code x}, generates a subset type like
     * <pre>
     * type TypeName = x: BaseType | (c1) && (c2) && ... && (cN) witness *
     * </pre>
     *
     * If no constraint expressions are provided, then instead generates a type synonym like
     * <pre>
     * type TypeName = BaseType
     * </pre>
     */
    private TokenTree generateSubsetType(
            final ShapeId shapeId, final String baseType, final Collection<TokenTree> constraints) {
        final String typeName = nameResolver.generatedTypeForShape(shapeId);
        if (constraints.size() == 0) {
            return TokenTree.of("type", typeName, "=", baseType);
        }

        final TokenTree constraintsConjunct = TokenTree.of(constraints.stream().map(TokenTree::parenthesized))
                .separated(Token.of("&&"));
        final String validityPredicateName = nameResolver.validityPredicateForShape(shapeId);
        final TokenTree validityPredicate = Token.of(
                "predicate method %s(x: %s)".formatted(validityPredicateName, baseType))
                .append(constraintsConjunct.braced());
        final TokenTree subsetType =
                Token.of("type %s = x: %s | %s(x) witness *".formatted(typeName, baseType, validityPredicateName));

        return TokenTree.of(subsetType, validityPredicate).lineSeparated();
    }

    /**
     * Like {@link DafnyApiCodegen#generateSubsetType(ShapeId, String, Collection<TokenTree>)}, but accepts
     * {@link Optional}-wrapped constraints and discards the empty ones.
     */
    @SuppressWarnings("JavaDoc")
    @SafeVarargs
    private TokenTree generateSubsetType(
            final ShapeId shapeId, final String baseType, final Optional<TokenTree>... constraintOptionals) {
        final List<TokenTree> constraints = Arrays.stream(constraintOptionals).flatMap(Optional::stream).toList();
        return generateSubsetType(shapeId, baseType, constraints);
    }

    @SuppressWarnings("SameParameterValue")
    private TokenTree generateTypeSynonym(
            final ShapeId shapeId, final String baseType) {
        return generateSubsetType(shapeId, baseType, Optional.empty());
    }

    @VisibleForTesting
    public Model getModel() {
        return model;
    }
}
