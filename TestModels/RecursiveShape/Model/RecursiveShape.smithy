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
  operations: [ GetRecursiveShape, GetRecursiveStructure ],
  errors: [],
}

structure SimpleRecursiveShapeConfig {}

operation GetRecursiveShape {
  input: GetRecursiveShapeInput,
  output: GetRecursiveShapeOutput,
}

operation GetRecursiveStructure {
  input: StructureWithRecursionOne,
  output: StructureWithRecursionOne,
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
  ListValue: ListWithRecursion
}

list ListWithRecursion {
  member: MapWithRecursion
}

map MapWithRecursion {
    key: String,
    value: StructureWithRecursion
}

structure StructureWithRecursion {
    content: RecursiveUnion
}

structure StructureWithRecursionOne {
    content: StructureWithRecursionTwo
}

structure StructureWithRecursionTwo {
    content: StructureWithRecursionOne
}