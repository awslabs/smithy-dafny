// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleCallingawssdkfromlocalserviceTypes.dfy"

module SimpleCallingawssdkfromlocalserviceImpl refines AbstractSimpleCallingawssdkfromlocalserviceOperations  {
  import ComAmazonawsKmsTypes
  import Com.Amazonaws.Kms
  import Com.Amazonaws.Dynamodb

  datatype Config = Config
  type InternalConfig = Config

  predicate ValidInternalConfig?(config: InternalConfig)
  {true}

  function ModifiesInternalConfig(config: InternalConfig) : set<object>
  {{}}

  predicate CallDDBScanEnsuresPublicly(input: CallDDBScanInput, output: Result<CallDDBScanOutput, Error>) {
    true
  }

  predicate CallDDBGetItemEnsuresPublicly(input: CallDDBGetItemInput, output: Result<CallDDBGetItemOutput, Error>) {
    true
  }

  predicate CallDDBPutItemEnsuresPublicly(input: CallDDBPutItemInput, output: Result<CallDDBPutItemOutput, Error>) {
    true
  }

  predicate CallKMSEncryptEnsuresPublicly(input: CallKMSEncryptInput, output: Result<CallKMSEncryptOutput, Error>) {
    true
  }

  predicate CallKMSDecryptEnsuresPublicly(input: CallKMSDecryptInput, output: Result<CallKMSDecryptOutput, Error>) {
    true
  }

  method CallDDBScan ( config: InternalConfig,  input: CallDDBScanInput )
    returns (output: Result<CallDDBScanOutput, Error>) {
    var ScanInput := Dynamodb.Types.ScanInput(
      TableName := input.tableArn
    );
    var retScan := input.ddbClient.Scan(ScanInput);
    if retScan.Success? {
      return Success(CallDDBScanOutput(itemOutput := retScan.value.Count.UnwrapOr(-1)));
    } else {
      return Failure(ComAmazonawsDynamodb(retScan.error));
    }
  }

  method CallDDBGetItem ( config: InternalConfig,  input: CallDDBGetItemInput )
    returns (output: Result<CallDDBGetItemOutput, Error>) {
    var GetItemInput := Dynamodb.Types.GetItemInput(
      TableName := input.tableArn,
      Key := input.key
    );
    var retGetItem := input.ddbClient.GetItem(GetItemInput);
    var emptyMap : Dynamodb.Types.AttributeMap := map[];
    if retGetItem.Success? {
      return Success(CallDDBGetItemOutput(itemOutput := retGetItem.value.Item.UnwrapOr(emptyMap)));
    } else {
      return Failure(ComAmazonawsDynamodb(retGetItem.error));
    }
  }

  method CallDDBPutItem ( config: InternalConfig,  input: CallDDBPutItemInput )
    returns (output: Result<CallDDBPutItemOutput, Error>) {
    var PutItemInput := Dynamodb.Types.PutItemInput(
      TableName := input.tableArn,
      Item := input.attributeMap,
      ConditionExpression := Wrappers.Some(input.conditionExpression)
    );
    var retPutItem := input.ddbClient.PutItem(PutItemInput);
    var emptyMap : Dynamodb.Types.AttributeMap := map[];
    if retPutItem.Success? {
      return Success(CallDDBPutItemOutput());
    } else {
      return Failure(ComAmazonawsDynamodb(retPutItem.error));
    }
  }

  method CallKMSEncrypt ( config: InternalConfig,  input: CallKMSEncryptInput )
    returns (output: Result<CallKMSEncryptOutput, Error>) {
    var encryptInput := Kms.Types.EncryptRequest(
      KeyId := input.keyId,
      Plaintext := input.plaintext,
      EncryptionContext := Wrappers.None,
      GrantTokens := Wrappers.None,
      EncryptionAlgorithm := Wrappers.None
    );
    var retEncryptResponse := input.kmsClient.Encrypt(encryptInput);
    if retEncryptResponse.Success? {
      return Success(CallKMSEncryptOutput(encryptOutput := retEncryptResponse.value.KeyId.UnwrapOr("DummyString")));
    } else {
      return Failure(Types.ComAmazonawsKms(retEncryptResponse.error));
    }
  }

  method CallKMSDecrypt ( config: InternalConfig,  input: CallKMSDecryptInput )
    returns (output: Result<CallKMSDecryptOutput, Error>) {
    var decryptInput := Kms.Types.DecryptRequest(
      CiphertextBlob := input.ciphertextBlob,
      KeyId := Wrappers.Some(input.keyId)
    );
    var retEncryptResponse := input.kmsClient.Decrypt(decryptInput);
    var emptyPlainText : Kms.Types.PlaintextType := [ 0 ];
    if retEncryptResponse.Success? {
      return Success(CallKMSDecryptOutput(KeyIdType := retEncryptResponse.value.KeyId.UnwrapOr("DummyString"), Plaintext := retEncryptResponse.value.Plaintext.UnwrapOr(emptyPlainText)));
    } else {
      return Failure(Types.ComAmazonawsKms(retEncryptResponse.error));
    }
  }
}
