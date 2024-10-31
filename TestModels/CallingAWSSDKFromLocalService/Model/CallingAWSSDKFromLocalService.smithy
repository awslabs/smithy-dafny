// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
namespace simple.callingawssdkfromlocalservice

use aws.polymorph#reference
use com.amazonaws.dynamodb#DynamoDB_20120810
use com.amazonaws.kms#TrentService

@reference(service: DynamoDB_20120810)
structure DdbClientReference {}

@reference(service: TrentService)
structure KmsClientReference {}

@aws.polymorph#localService(
  sdkId: "SimpleCallingawssdkfromlocalservice",
  config: SimpleCallingawssdkfromlocalserviceConfig,
  dependencies: [
    DynamoDB_20120810,
    TrentService
  ]
)

service SimpleCallingAWSSDKFromLocalService {
  version: "2021-11-01",
  resources: [],
  operations: [ 
                CallDDBScan,
                CallDDBGetItem,
                CallDDBPutItem,
                CallKMSEncrypt,
                CallKMSDecrypt],
  errors: [ SimpleCallingAWSSDKFromLocalServiceException ],
}

structure SimpleCallingawssdkfromlocalserviceConfig {}

operation CallDDBScan {
  input: CallDDBScanInput,
  output: CallDDBScanOutput,
}

structure CallDDBScanInput {
  @required
  ddbClient: DdbClientReference,
  @required
  tableArn: com.amazonaws.dynamodb#TableArn
}

structure CallDDBScanOutput {
  @required
  itemOutput: com.amazonaws.dynamodb#Integer,
}

operation CallDDBGetItem {
  input: CallDDBGetItemInput,
  output: CallDDBGetItemOutput,
}

structure CallDDBGetItemInput {
  @required
  ddbClient: DdbClientReference,
  @required
  tableArn: com.amazonaws.dynamodb#TableArn,
  @required
  key: com.amazonaws.dynamodb#Key
}

structure CallDDBGetItemOutput {
  @required
  itemOutput: com.amazonaws.dynamodb#AttributeMap,
}

operation CallDDBPutItem {
  input: CallDDBPutItemInput,
  output: CallDDBPutItemOutput
}

structure CallDDBPutItemInput {
  @required
  ddbClient: DdbClientReference,
  @required
  tableArn: com.amazonaws.dynamodb#TableArn,
  @required
  attributeMap: com.amazonaws.dynamodb#PutItemInputAttributeMap
  @required
  conditionExpression: com.amazonaws.dynamodb#ConditionExpression
}

structure CallDDBPutItemOutput {

}

operation CallKMSEncrypt {
  input: CallKMSEncryptInput,
  output: CallKMSEncryptOutput,
}

structure CallKMSEncryptInput {
  @required
  kmsClient: KmsClientReference,
  @required
  keyId: com.amazonaws.kms#KeyIdType,
  @required
  plaintext: com.amazonaws.kms#PlaintextType
}

structure CallKMSEncryptOutput {
  @required
  encryptOutput: com.amazonaws.kms#KeyIdType,
}

operation CallKMSDecrypt {
  input: CallKMSDecryptInput,
  output: CallKMSDecryptOutput,
}

structure CallKMSDecryptInput {
  @required
  kmsClient: KmsClientReference,
  @required
  keyId: com.amazonaws.kms#KeyIdType,
  @required
  ciphertextBlob: com.amazonaws.kms#CiphertextType
}

structure CallKMSDecryptOutput {
  @required
  KeyIdType: com.amazonaws.kms#KeyIdType,
  @required
  Plaintext: com.amazonaws.kms#PlaintextType
}

@error("client")
structure SimpleCallingAWSSDKFromLocalServiceException {
  @required
  message: String,
}
