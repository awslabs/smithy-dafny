// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleOrphanedTypes.dfy"
include "OrphanedResource.dfy"
include "ExternDefinitions.dfy"

module SimpleOrphanedImpl refines AbstractSimpleOrphanedOperations  {
  // import OrphanedResource
  // import ExternDefinitions

  datatype Config = Config
  type InternalConfig = Config
  predicate ValidInternalConfig?(config: InternalConfig)
  {true}
  function ModifiesInternalConfig(config: InternalConfig) : set<object>
  {{}}
  // predicate CreateOrphanedStructureEnsuresPublicly(input: CreateOrphanedStructureInput, output: Result<CreateOrphanedStructureOutput, Error>) {
  //   true
  // }
  // predicate CreateOrphanedResourceEnsuresPublicly(input: CreateOrphanedResourceInput, output: Result<CreateOrphanedResourceOutput, Error>) {
  //   true
  // }
  // predicate CreateOrphanedErrorEnsuresPublicly(input: CreateOrphanedErrorInput, output: Result<CreateOrphanedErrorOutput, Error>) {
  //   true
  // }
  // method CreateOrphanedStructure ( config: InternalConfig,  input: CreateOrphanedStructureInput )
  //   returns (output: Result<CreateOrphanedStructureOutput, Error>)
  // {
  //   // Create Dafny OrphanedStructure. (This implicitly tests Dafny codegen's orphaned shapes generation.)
  //   var uninitializedStructure := Types.OrphanedStructure();
  //   var initializedStructure := ExternDefinitions.InitializeOrphanedStructure(uninitializedStructure);

  //   expect initializedStructure.stringValue.Some?;
  //   expect initializedStructure.stringValue.value == "the extern MUST use Smithy-generated conversions to set this value in the native structure";

  //   return Success(CreateOrphanedStructureOutput());
  // }

  // method CreateOrphanedResource ( config: InternalConfig,  input: CreateOrphanedResourceInput )
  //   returns (output: Result<CreateOrphanedResourceOutput, Error>)
  // {
  //   var resource := new OrphanedResource.OrphanedResource();
  //   var ret :- ExternDefinitions.CallNativeOrphanedResource(resource);

  //   expect ret.someString.Some?;
  //   expect ret.someString.value == "correct string";

  //   return Success(CreateOrphanedResourceOutput());
  // }

  // method CreateOrphanedError ( config: InternalConfig,  input: CreateOrphanedErrorInput )
  //   returns (output: Result<CreateOrphanedErrorOutput, Error>)
  // {
  //   var error := Types.Error.OrphanedError(message := "TBD");
  //   var out_error := ExternDefinitions.CallNativeOrphanedError(error);

  //   expect out_error.OrphanedError?;
  //   expect out_error.message == "the extern MUST use Smithy-generated conversions to set this value in the native error";

  //   return Success(CreateOrphanedErrorOutput());
  // }
}