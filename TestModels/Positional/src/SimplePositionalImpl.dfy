// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
include "../Model/SimplePositionalTypes.dfy"
include "./SimpleResource.dfy"
module SimplePositionalImpl refines AbstractSimplePositionalOperations {
  import SimpleResource

  datatype Config = Config
  type InternalConfig = Config

  predicate ValidInternalConfig?(config: InternalConfig)
  {true}
  function ModifiesInternalConfig(config: InternalConfig): set<object>
  {{}}

  predicate GetResourceEnsuresPublicly(input: GetResourceInput , output: Result<GetResourceOutput, Error>)
  {true}

  method GetResource ( config: InternalConfig , input: GetResourceInput )
    returns (output: Result<GetResourceOutput, Error>)
  {
    var resource := new SimpleResource.SimpleResource(
      input.name
    );
    var result: GetResourceOutput := GetResourceOutput(
      output := resource
    );
    return Success(result);
  }

  predicate GetResourcePositionalEnsuresPublicly(input: string , output: Result<ISimpleResource, Error>)
  {true}

  // @positional allows use to accept the input parameters directly without the input structure
  method GetResourcePositional ( config: InternalConfig , input: string )
    returns (output: Result<ISimpleResource, Error>)
  {
    var resource := new SimpleResource.SimpleResource(input);

    // @positional allows use to return the result without the output structure
    return Success(resource);
  }

  predicate GetIntegerInputPositionEnsuresPublicly(input: int32 , output: Result<GetIntegerInputPositionOutput, Error>)
  {true}

  method GetIntegerInputPosition ( config: InternalConfig , input: int32 )
    returns (output: Result<GetIntegerInputPositionOutput, Error>)
  {
    var res := GetIntegerInputPositionOutput(value := Some(input));
    return Success(res);
  }

  predicate GetIntegerOutputPositionEnsuresPublicly(input: GetIntegerOutputPositionInput , output: Result<int32, Error>)
  {true}

  method GetIntegerOutputPosition ( config: InternalConfig , input: GetIntegerOutputPositionInput )
    returns (output: Result<int32, Error>)
  {
    expect input.value.Some?;
    return Success(input.value.UnwrapOr(-1));
  }
}