// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

module TestComAmazonawsKms {
  import Com.Amazonaws.Kms
  import opened StandardLibrary.UInt
  import opened Wrappers

  const keyId :=  "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f";
  const keyIdFailureCase := "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7d";
  // One test depends on knowing the region it is being run it.
  // For now, hardcode this value to the region we are currently using to test,
  // which is the same region that our test KMS Key lives in.
  // If we want to run tests in other regions we will need a way to
  // grab this value from some config.
  // For now, we prefer to have brittleness in these tests vs. missing a test case
  // that cannot be formally verified.
  const TEST_REGION := "us-west-2";

  // This is required because
  // https://github.com/dafny-lang/dafny/issues/2311
  function method workAround(literal: seq<uint8>)
    :(ret: Kms.Types.CiphertextType)
    requires Kms.Types.IsValid_CiphertextType(literal)
  {literal}

  // These are the operations needed for the Encryption SDK
  method {:test} BasicDecryptTests() {
    var CiphertextBlob : seq<uint8> := [
        1,   1,   1,   0, 120,  64, 243, 140,  39,  94,  49,   9,
      116,  22, 193,   7,  41,  81,  80,  87,  25, 100, 173, 163,
      239,  28,  33, 233,  76, 139, 160, 189, 188, 157,  15, 180,
      20,   0,   0,   0,  98,  48,  96,   6,   9,  42, 134,  72,
      134, 247,  13,   1,   7,   6, 160,  83,  48,  81,   2,   1,
        0,  48,  76,   6,   9,  42, 134,  72, 134, 247,  13,   1,
        7,   1,  48,  30,   6,   9,  96, 134,  72,   1, 101,   3,
        4,   1,  46,  48,  17,   4,  12, 196, 249,  60,   7,  21,
      231,  87,  70, 216,  12,  31,  13,   2,   1,  16, 128,  31,
      222, 119, 162, 112,  88, 153,  39, 197,  21, 182, 116, 176,
      120, 174, 107,  82, 182, 223, 160, 201,  15,  29,   3, 254,
        3, 208,  72, 171,  64, 207, 175
    ];

    BasicDecryptTestSuccessCase(
      input := Kms.Types.DecryptRequest(
        CiphertextBlob := workAround(CiphertextBlob),
        EncryptionContext := Wrappers.None,
        GrantTokens := Wrappers.None,
        KeyId := Wrappers.Some(keyId),
        EncryptionAlgorithm := Wrappers.None
      ),
      expectedPlaintext := [ 165, 191, 67, 62 ],
      expectedKeyId := keyId
    );

    BasicDecryptTestFailureCase(
      input := Kms.Types.DecryptRequest(
        CiphertextBlob := workAround(CiphertextBlob),
        EncryptionContext := Wrappers.None,
        GrantTokens := Wrappers.None,
        KeyId := Wrappers.Some(keyIdFailureCase),
        EncryptionAlgorithm := Wrappers.None
      ),
      expectedPlaintext := [ 165, 191, 67, 62 ],
      expectedKeyId := keyIdFailureCase
    );
  }

  method {:test} BasicGenerateTests() {
    BasicGenerateTest(
      input := Kms.Types.GenerateDataKeyRequest(
        KeyId := keyId,
        EncryptionContext := Wrappers.None,
        NumberOfBytes := Wrappers.Some(32 as Kms.Types.NumberOfBytesType),
        KeySpec := Wrappers.None,
        GrantTokens := Wrappers.None
      )
    );
  }

  method {:test} BasicEncryptTests() {
    BasicEncryptTest(
      input := Kms.Types.EncryptRequest(
      KeyId := keyId,
      // The string "asdf" as bytes
      Plaintext := [ 97, 115, 100, 102 ],
      EncryptionContext := Wrappers.None,
      GrantTokens := Wrappers.None,
      EncryptionAlgorithm := Wrappers.None
      )
    );
  }

  method BasicDecryptTestSuccessCase(
    nameonly input: Kms.Types.DecryptRequest,
    nameonly expectedPlaintext: Kms.Types.PlaintextType,
    nameonly expectedKeyId: Kms.Types.KeyIdType
  )
  {
    var client :- expect Kms.KMSClient();

    var ret := client.Decrypt(input);

    expect(ret.Success?);

    var DecryptResponse(KeyId, Plaintext, EncryptionAlgorithm, CiphertextBlob) := ret.value;

    expect Plaintext.Some?;
    expect KeyId.Some?;
    expect Plaintext.value == expectedPlaintext;
    expect KeyId.value == expectedKeyId;
  }

  method BasicDecryptTestFailureCase(
    nameonly input: Kms.Types.DecryptRequest,
    nameonly expectedPlaintext: Kms.Types.PlaintextType,
    nameonly expectedKeyId: Kms.Types.KeyIdType
  )
  {
    var client :- expect Kms.KMSClient();

    var ret := client.Decrypt(input);

    expect(ret.Failure?);
    expect(ret.error.IncorrectKeyException?);
  }

  method BasicGenerateTest(
    nameonly input: Kms.Types.GenerateDataKeyRequest
  )
    requires input.NumberOfBytes.Some?
  {
    var client :- expect Kms.KMSClient();

    var ret := client.GenerateDataKey(input);

    expect(ret.Success?);

    var GenerateDataKeyResponse(CiphertextBlob, Plaintext, KeyId, CiphertextForRecipient) := ret.value;

    expect CiphertextBlob.Some?;
    expect Plaintext.Some?;
    expect KeyId.Some?;
    expect |Plaintext.value| == input.NumberOfBytes.value as nat;

    var decryptInput := Kms.Types.DecryptRequest(
      CiphertextBlob := CiphertextBlob.value,
      EncryptionContext := input.EncryptionContext,
      GrantTokens := input.GrantTokens,
      KeyId := Wrappers.Some(KeyId.value),
      EncryptionAlgorithm := Wrappers.None
    );

    BasicDecryptTestSuccessCase(
      input := decryptInput,
      expectedPlaintext := Plaintext.value,
      expectedKeyId := input.KeyId
    );
  }

  method BasicEncryptTest(
    nameonly input: Kms.Types.EncryptRequest
  )
  {
    var client :- expect Kms.KMSClient();

    var ret := client.Encrypt(input);

    expect(ret.Success?);

    var EncryptResponse(CiphertextBlob, KeyId, EncryptionAlgorithm) := ret.value;

    expect CiphertextBlob.Some?;
    expect KeyId.Some?;

    var decryptInput := Kms.Types.DecryptRequest(
      CiphertextBlob := CiphertextBlob.value,
      EncryptionContext := input.EncryptionContext,
      GrantTokens := input.GrantTokens,
      KeyId := Wrappers.Some(KeyId.value),
      EncryptionAlgorithm := input.EncryptionAlgorithm
    );

    BasicDecryptTestSuccessCase(
      input := decryptInput,
      expectedPlaintext := input.Plaintext,
      expectedKeyId := input.KeyId
    );
  }

  // While we cannot easily test that the expected implementations
  // return Some(), we can at least ensure that the ones that do are correct.
  method {:test} RegionMatchTest() {
    var client :- expect Kms.KMSClient();
    var region := Kms.RegionMatch(client, TEST_REGION);
    expect region.None? || region.value;
  }

}
