// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
namespace simple.recursiveShape

@aws.polymorph#localService(
  sdkId: "SimpleRecursiveShape",
  config: SimpleRecursiveShapeConfig,
)
service SimpleRecursiveShape {
  version: "2021-11-01",
  resources: [],
  operations: [ GetRecursiveShape, GetRecursiveShapeKnownValueTest ],
  errors: [],
}

structure SimpleRecursiveShapeConfig {}

operation GetRecursiveShape {
  input: GetRecursiveShapeInput,
  output: GetRecursiveShapeOutput,
}

operation GetRecursiveShapeKnownValueTest {
  input: GetRecursiveShapeInput,
  output: GetRecursiveShapeOutput,
}

structure GetRecursiveShapeInput {
  recursiveUnion: RecursiveUnion
}

structure GetRecursiveShapeOutput {
  recursiveUnion: RecursiveUnion
}

union RecursiveUnion {
  IntegerValue: Integer,
  StringValue: String,
  DataMap: StructuredDataMap
}

map StructuredDataMap {
    key: String,
    value: StructuredData
}

structure StructuredData {
    content: RecursiveUnion
}