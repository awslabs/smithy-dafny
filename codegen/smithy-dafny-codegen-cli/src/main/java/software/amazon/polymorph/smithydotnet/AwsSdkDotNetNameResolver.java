// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package software.amazon.polymorph.smithydotnet;

import software.amazon.polymorph.utils.ModelUtils;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.ListShape;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.StringShape;
import software.amazon.smithy.model.shapes.StructureShape;
import software.amazon.smithy.model.traits.EnumTrait;
import software.amazon.smithy.model.traits.TraitDefinition;
import software.amazon.smithy.utils.StringUtils;

public class AwsSdkDotNetNameResolver extends DotNetNameResolver {
    // The following are used to resolve namespace errors when generating
    // code that uses the DynamoDBv2 service model
    public static final String DDB_NAMESPACE = "com.amazonaws.dynamodb";
    public static final String DDB_SERVICE_NAME = "DynamoDB";
    public static final String DDB_SERVICE_NAME_V2 = "DynamoDBv2";
    public static final String DDB_SMITHY_SERVICE_NAME = "DynamoDB_20120810";
    public static final String DDB_TYPES_SERVICE_NAME = "DynamoDB__20120810";
    public static final String DDB_V2_ATTRIBUTE_VALUE = "Amazon.DynamoDBv2.Model.AttributeValue";
    public static final String DDB_NET_INTERFACE_NAME = "Amazon.DynamoDBv2.IAmazonDynamoDB";
    public static final String DDB_ATTRIBUTE_VALUE_MODEL_NAMESPACE = "Com.Amazonaws.Dynamodb.AttributeValue";
    public static final String DDB_INPUT = "Input";
    public static final String DDB_OUTPUT = "Output";
    public static final String DDB_REQUEST = "Request";
    public static final String DDB_RESPONSE = "Response";

    public AwsSdkDotNetNameResolver(final Model model, final ServiceShape serviceShape) {
        super(model, serviceShape);
    }

    private boolean isGeneratedInSdk(final ShapeId shapeId) {
        return ModelUtils.isInServiceNamespace(shapeId, getServiceShape());
    }

    @Override
    protected String baseTypeForString(final StringShape stringShape) {
        if (isGeneratedInSdk(stringShape.getId()) && stringShape.hasTrait(EnumTrait.class)) {
            return "%s.%s".formatted(namespaceForService(), classForEnum(stringShape.getId()));
        }

        return super.baseTypeForString(stringShape);
    }

    @Override
    protected String baseTypeForList(final ListShape listShape) {
        final MemberShape memberShape = listShape.getMember();
        final Shape targetShape = getModel().expectShape(memberShape.getTarget());

        // The .NET AWS SDK represents a list-of-enums as a list-of-strings, even though it represents enums as the
        // corresponding enum class every where else AFAICT.
        final String memberType = targetShape.hasTrait(EnumTrait.class) ? "string" : baseTypeForMember(memberShape);

        // we need to return the name AttributeValue in the sdk not the name in the model
        if (StringUtils.equals(memberType, DDB_ATTRIBUTE_VALUE_MODEL_NAMESPACE)) {
            return "System.Collections.Generic.List<%s>".formatted(DDB_V2_ATTRIBUTE_VALUE);
        }
        return "System.Collections.Generic.List<%s>".formatted(memberType);
    }

    @Override
    protected String baseTypeForStructure(final StructureShape structureShape) {
        if (isGeneratedInSdk(structureShape.getId())) {
            if (structureShape.hasTrait(TraitDefinition.class)) {
                throw new IllegalArgumentException("Trait definition structures have no corresponding generated type");
            }
            // Structures for the DynamoDB NET SDK MUST be resolved from input to request
            // The actual NET SDK uses Request/Response
            if (StringUtils.equals(structureShape.getId().getNamespace(), DDB_NAMESPACE) &&
                    structureShape.getId().getName().endsWith(DDB_INPUT)) {
                String newRequestString = structureShape.getId().getName().replace(DDB_INPUT, DDB_REQUEST);
                return "%s.Model.%s".formatted(namespaceForService(), newRequestString);
            }
            // Structures for the DynamoDB NET SDK MUST be resolved from output to response
            // The actual NET SDK uses Request/Response
            if (StringUtils.equals(structureShape.getId().getNamespace(), DDB_NAMESPACE) &&
                    structureShape.getId().getName().endsWith(DDB_OUTPUT)) {
                String newResponseString = structureShape.getId().getName().replace(DDB_OUTPUT, DDB_RESPONSE);
                return "%s.Model.%s".formatted(namespaceForService(), newResponseString);
            }
            return "%s.Model.%s".formatted(namespaceForService(), structureShape.getId().getName());
        }

        return super.baseTypeForStructure(structureShape);
    }

    @Override
    protected String baseTypeForService(final ServiceShape serviceShape) {
        if (isGeneratedInSdk(serviceShape.getId())) {
            return "%s.IAmazon%s".formatted(namespaceForService(), getServiceName());
        }

        return super.baseTypeForService(serviceShape);
    }

    public String implForServiceClient() {
        // The Client Implementation MUST be DynamoDB - although the NET SDK is using DynamoDBv2
        // It does not append v2 in the client for backwards compatability purposes.
        if (StringUtils.equals(getServiceName(), DDB_SERVICE_NAME_V2)) {
            return "%s.Amazon%sClient".formatted(namespaceForService(), DDB_SERVICE_NAME);
        }
        return "%s.Amazon%sClient".formatted(namespaceForService(), getServiceName());
    }

    private String getServiceName() {
        // The smithy model appends a version number in the name of the service
        // This version number does not appear in the NET SDK and resolves it to DynamoDBv2
        if (StringUtils.equals(getServiceShape().getId().getName(), DDB_SMITHY_SERVICE_NAME)) {
            return StringUtils.capitalize(DDB_SERVICE_NAME_V2);
        }
        return StringUtils.capitalize(getServiceShape().getId().getName());
    }

    @Override
    public String namespaceForService() {
        return "Amazon.%s".formatted(getServiceName());
    }

    public String syntheticNamespaceForService() {
        return super.namespaceForService();
    }

    public String shimClassForService() {
        return "%sShim".formatted(getServiceName());
    }

    @Override
    public String classForBaseServiceException() {
        // Although using V2 of the DynamoDB Client Exceptions MUST use DynamoDB as opposed to DynamoDBv2
        return StringUtils.equals(getServiceName(), DDB_SERVICE_NAME_V2)
                ? "Amazon%sException".formatted(DDB_SERVICE_NAME)
                : "Amazon%sException".formatted(getServiceName());
    }

    public String qualifiedClassForBaseServiceException() {
        return "%s.%s".formatted(namespaceForService(), classForBaseServiceException());
    }
}
