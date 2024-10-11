// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

module TestComAmazonawsS3 {
    import Com.Amazonaws.S3
    import Std.Enumerators
    import opened StandardLibrary.UInt
    import opened Wrappers

    const testBucket := "smithy-dafny-s3-test-bucket"
    const testObjectKey := "test-model-object-key-enumerator"

    method {:test} BasicRoundTripTests() {
        var testInputSeq: seq<bytes> := [ [0x62], [0x73], [0x64], [0x66] ];
        var testInput := new Enumerators.SeqEnumerator(testInputSeq);
        PutObjectTest(
            input := S3.Types.PutObjectRequest(
                Bucket := testBucket,
                Key := testObjectKey,
                Body := Wrappers.Some(testInput)
            )
        );
        var testOutput := new Enumerators.SeqEnumerator(testInputSeq);
        GetObjectTest(
            input := S3.Types.GetObjectRequest(
                Bucket := testBucket,
                Key := testObjectKey
            ),
            expectedBody := testOutput
        );
    }

    method GetObjectTest(
        nameonly input: S3.Types.GetObjectRequest,
        nameonly expectedBody: S3.Types.StreamingBlob
    )
    {
        var client :- expect S3.S3Client();

        var ret := client.GetObject(input);

        expect(ret.Success?);

        // we only care about the Body
        var MyBody := ret.value.Body;
        expect MyBody.Some?;
        var byteString := MyBody.value.Next();
        expect MyBody.value == expectedBody;
    }

    method PutObjectTest(
        nameonly input: S3.Types.PutObjectRequest
    )
    {
        var client :- expect S3.S3Client();

        var ret := client.PutObject(input);

        expect(ret.Success?);

        // just check that an ETag was returned 
        var MyETag := ret.value.ETag;

        expect MyETag.Some?;
    }
}
