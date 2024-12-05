// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleOrphanedTypes.dfy"
include "OrphanedResource.dfy"

module {:extern} SimpleOrphanedImpl refines AbstractSimpleOrphanedOperations  {
  import OrphanedResource

  datatype Config = Config
  type InternalConfig = Config
  predicate ValidInternalConfig?(config: InternalConfig)
  {true}
  function ModifiesInternalConfig(config: InternalConfig) : set<object>
  {{}}
  predicate CreateOrphanedStructureEnsuresPublicly(input: CreateOrphanedStructureInput, output: Result<CreateOrphanedStructureOutput, Error>) {
    true
  }
  predicate CreateOrphanedResourceEnsuresPublicly(input: CreateOrphanedResourceInput, output: Result<CreateOrphanedResourceOutput, Error>) {
    true
  }
  predicate CreateOrphanedErrorEnsuresPublicly(input: CreateOrphanedErrorInput, output: Result<CreateOrphanedErrorOutput, Error>) {
    true
  }
  method CreateOrphanedStructure ( config: InternalConfig,  input: CreateOrphanedStructureInput )
    returns (output: Result<CreateOrphanedStructureOutput, Error>)
  {
    // Create Dafny OrphanedStructure. (This implicitly tests Dafny codegen's orphaned shapes generation.)
    var uninitializedStructure := Types.OrphanedStructure();
    var initializedStructure := InitializeOrphanedStructure(uninitializedStructure);

    expect initializedStructure.stringValue.Some?;
    expect initializedStructure.stringValue.value == "the extern MUST use Smithy-generated conversions to set this value in the native structure";

    return Success(CreateOrphanedStructureOutput());
  }

  // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
  // This will ensure that OrphanedStructure and its conversions are generated,
  // even though OrphanedStructure is "orphaned".

  // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
  // The Smithy model does not know about this operation,
  //   so it doesn't register OrphanedStructure as an operation shape.
  // If this operation were on the Smithy model,
  //   both the operation and OrphanedStructure would no longer be orphaned,
  //   and wouldn't be useful in an "orphaned shapes" TestModel.
  // Putting all usage of the orphaned shape outside the model's knowledge
  //   lets us test orphaned shape model/conversion generation.
  method {:extern} InitializeOrphanedStructure( input: Types.OrphanedStructure )
    returns (output: Types.OrphanedStructure)

  method CreateOrphanedResource ( config: InternalConfig,  input: CreateOrphanedResourceInput )
    returns (output: Result<CreateOrphanedResourceOutput, Error>)
  {
    var resource := new OrphanedResource.OrphanedResource();
    var ret :- CallNativeOrphanedResource(resource);

    expect ret.someString.Some?;
    expect ret.someString.value == "correct string";

    return Success(CreateOrphanedResourceOutput());
  }

  // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
  // This will ensure that OrphanedStructure and its conversions are generated,
  // even though OrphanedStructure is "orphaned".

  // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
  // The Smithy model does not know about this operation,
  //   so it doesn't register OrphanedStructure as an operation shape.
  // If this operation were on the Smithy model,
  //   both the operation and OrphanedStructure would no longer be orphaned,
  //   and wouldn't be useful in an "orphaned shapes" TestModel.
  // Putting all usage of the orphaned shape outside the model's knowledge
  //   lets us test orphaned shape model/conversion generation.
  method {:extern} CallNativeOrphanedResource( input: OrphanedResource.OrphanedResource )
    returns (output: Result<Types.OrphanedResourceOperationOutput, Types.Error>)

  method CreateOrphanedError ( config: InternalConfig,  input: CreateOrphanedErrorInput )
    returns (output: Result<CreateOrphanedErrorOutput, Error>)
  {
    var error := Types.Error.OrphanedError(message := "TBD");
    var out_error := CallNativeOrphanedError(error);

    expect out_error.OrphanedError?;
    expect out_error.message == "the extern MUST use Smithy-generated conversions to set this value in the native error";

    return Success(CreateOrphanedErrorOutput());
  }

  // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
  // This will ensure that OrphanedStructure and its conversions are generated,
  // even though OrphanedStructure is "orphaned".

  // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
  // The Smithy model does not know about this operation,
  //   so it doesn't register OrphanedStructure as an operation shape.
  // If this operation were on the Smithy model,
  //   both the operation and OrphanedStructure would no longer be orphaned,
  //   and wouldn't be useful in an "orphaned shapes" TestModel.
  // Putting all usage of the orphaned shape outside the model's knowledge
  //   lets us test orphaned shape model/conversion generation.
  method {:extern} CallNativeOrphanedError( input: Types.Error )
    returns (output: Types.Error )
}