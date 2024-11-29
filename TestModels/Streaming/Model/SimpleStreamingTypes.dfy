// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
include "../../dafny-dependencies/StandardLibrary/src/Index.dfy"
module SimpleStreamingTypes
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened StandardLibrary.Streams
  import opened UTF8
    // Generic helpers for verification of mock/unit tests.
  datatype DafnyCallEvent<I, O> = DafnyCallEvent(input: I, output: O)

  // Begin Generated Types

  datatype BinaryOfInput = | BinaryOfInput (
    nameonly number: int32
  )
  datatype BinaryOfOutput = | BinaryOfOutput (
    nameonly binary: DataStream
  )
  datatype ChunksInput = | ChunksInput (
    nameonly bytesIn: DataStream ,
    nameonly chunkSize: CountingInteger
  )
  datatype ChunksOutput = | ChunksOutput (
    nameonly bytesOut: DataStream
  )
  datatype CountBitsInput = | CountBitsInput (
    nameonly bits: DataStream
  )
  datatype CountBitsOutput = | CountBitsOutput (
    nameonly sum: int32
  )
  type CountingInteger = x: int32 | IsValid_CountingInteger(x) witness *
  predicate method IsValid_CountingInteger(x: int32) {
    ( 1 <= x  )
  }
  class ISimpleStreamingClientCallHistory {
    ghost constructor() {
      CountBits := [];
      BinaryOf := [];
      Chunks := [];
    }
    ghost var CountBits: seq<DafnyCallEvent<CountBitsInput, Result<CountBitsOutput, Error>>>
    ghost var BinaryOf: seq<DafnyCallEvent<BinaryOfInput, Result<BinaryOfOutput, Error>>>
    ghost var Chunks: seq<DafnyCallEvent<ChunksInput, Result<ChunksOutput, Error>>>
  }
  trait {:termination false} ISimpleStreamingClient extends object
  {
    // Helper to define any additional modifies/reads clauses.
    // If your operations need to mutate state,
    // add it in your constructor function:
    // Modifies := {your, fields, here, History};
    // If you do not need to mutate anything:
    // Modifies := {History};

    ghost const Modifies: set<object>
    // For an unassigned field defined in a trait,
    // Dafny can only assign a value in the constructor.
    // This means that for Dafny to reason about this value,
    // it needs some way to know (an invariant),
    // about the state of the object.
    // This builds on the Valid/Repr paradigm
    // To make this kind requires safe to add
    // to methods called from unverified code,
    // the predicate MUST NOT take any arguments.
    // This means that the correctness of this requires
    // MUST only be evaluated by the class itself.
    // If you require any additional mutation,
    // then you MUST ensure everything you need in ValidState.
    // You MUST also ensure ValidState in your constructor.
    predicate ValidState()
      ensures ValidState() ==> History in Modifies
    ghost const History: ISimpleStreamingClientCallHistory
    predicate CountBitsEnsuresPublicly(input: CountBitsInput , output: Result<CountBitsOutput, Error>)
    // The public method to be called by library consumers
    method CountBits ( input: CountBitsInput )
      returns (output: Result<CountBitsOutput, Error>)
      requires
        && ValidState()
      // TODO: smithy-dafny isn't yet generating the `input.bits.Valid()` part.
      requires
        && input.bits.Valid() && History !in input.bits.Repr
      modifies Modifies - {History} ,
               History`CountBits ,
               input.bits.Repr
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CountBitsEnsuresPublicly(input, output)
      ensures History.CountBits == old(History.CountBits) + [DafnyCallEvent(input, output)]

    predicate BinaryOfEnsuresPublicly(input: BinaryOfInput , output: Result<BinaryOfOutput, Error>)
    // The public method to be called by library consumers
    method BinaryOf ( input: BinaryOfInput )
      returns (output: Result<BinaryOfOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`BinaryOf
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures BinaryOfEnsuresPublicly(input, output)
      ensures History.BinaryOf == old(History.BinaryOf) + [DafnyCallEvent(input, output)]
  // TODO: smithy-dafny isn't yet generating this
      ensures output.Success? ==> output.value.binary.Valid() && fresh(output.value.binary.Repr)

    predicate ChunksEnsuresPublicly(input: ChunksInput , output: Result<ChunksOutput, Error>)
    // The public method to be called by library consumers
    method Chunks ( input: ChunksInput )
      returns (output: Result<ChunksOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`Chunks
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures ChunksEnsuresPublicly(input, output)
      ensures History.Chunks == old(History.Chunks) + [DafnyCallEvent(input, output)]

  }
  datatype SimpleStreamingConfig = | SimpleStreamingConfig (

                                   )
  type StreamingBlob = DataStream
  datatype Error =
      // Local Error structures are listed here
    | OverflowError (
        nameonly message: string
      )
      // Any dependent models are listed here

      // The Collection error is used to collect several errors together
      // This is useful when composing OR logic.
      // Consider the following method:
      // 
      // method FN<I, O>(n:I)
      //   returns (res: Result<O, Types.Error>)
      //   ensures A(I).Success? ==> res.Success?
      //   ensures B(I).Success? ==> res.Success?
      //   ensures A(I).Failure? && B(I).Failure? ==> res.Failure?
      // 
      // If either A || B is successful then FN is successful.
      // And if A && B fail then FN will fail.
      // But what information should FN transmit back to the caller?
      // While it may be correct to hide these details from the caller,
      // this can not be the globally correct option.
      // Suppose that A and B can be blocked by different ACLs,
      // and that their representation of I is only eventually consistent.
      // How can the caller distinguish, at a minimum for logging,
      // the difference between the four failure modes?
    // || (!access(A(I)) && !access(B(I)))
    // || (!exit(A(I)) && !exit(B(I)))
    // || (!access(A(I)) && !exit(B(I)))
    // || (!exit(A(I)) && !access(B(I)))
    | CollectionOfErrors(list: seq<Error>, nameonly message: string)
      // The Opaque error, used for native, extern, wrapped or unknown errors
    | Opaque(obj: object)
      // A better Opaque, with a visible string representation.
    | OpaqueWithText(obj: object, objMessage : string)
  type OpaqueError = e: Error | e.Opaque? || e.OpaqueWithText? witness *
  // This dummy subset type is included to make sure Dafny
  // always generates a _ExternBase___default.java class.
  type DummySubsetType = x: int | IsDummySubsetType(x) witness 1
  predicate method IsDummySubsetType(x: int) {
    0 < x
  }

}
abstract module AbstractSimpleStreamingService
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened StandardLibrary.Streams
  import opened UTF8
  import opened Types = SimpleStreamingTypes
  import Operations : AbstractSimpleStreamingOperations
  function method DefaultSimpleStreamingConfig(): SimpleStreamingConfig
  method SimpleStreaming(config: SimpleStreamingConfig := DefaultSimpleStreamingConfig())
    returns (res: Result<SimpleStreamingClient, Error>)
    ensures res.Success? ==>
              && fresh(res.value)
              && fresh(res.value.Modifies)
              && fresh(res.value.History)
              && res.value.ValidState()

  // Helper functions for the benefit of native code to create a Success(client) without referring to Dafny internals
  function method CreateSuccessOfClient(client: ISimpleStreamingClient): Result<ISimpleStreamingClient, Error> {
    Success(client)
  }
  function method CreateFailureOfError(error: Error): Result<ISimpleStreamingClient, Error> {
    Failure(error)
  }
  class SimpleStreamingClient extends ISimpleStreamingClient
  {
    constructor(config: Operations.InternalConfig)
      requires Operations.ValidInternalConfig?(config)
      ensures
        && ValidState()
        && fresh(History)
        && this.config == config
    const config: Operations.InternalConfig
    predicate ValidState()
      ensures ValidState() ==>
                && Operations.ValidInternalConfig?(config)
                && History !in Operations.ModifiesInternalConfig(config)
                && Modifies == Operations.ModifiesInternalConfig(config) + {History}
    predicate CountBitsEnsuresPublicly(input: CountBitsInput , output: Result<CountBitsOutput, Error>)
    {Operations.CountBitsEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method CountBits ( input: CountBitsInput )
      returns (output: Result<CountBitsOutput, Error>)
      requires
        && ValidState()
      // TODO: smithy-dafny isn't yet generating the `input.bits.Valid()` parts.
      requires 
        && input.bits.Valid() && History !in input.bits.Repr
      modifies Modifies - {History} ,
               History`CountBits ,
               input.bits.Repr
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CountBitsEnsuresPublicly(input, output)
      ensures History.CountBits == old(History.CountBits) + [DafnyCallEvent(input, output)]
    {
      // TODO: It's not clear how to actually ensure this,
      // since the internal config is not visible to the trait
      // so it can't really be a precondition there.
      assume {:axiom} Operations.ModifiesInternalConfig(config) !! input.bits.Repr;
      output := Operations.CountBits(config, input);
      History.CountBits := History.CountBits + [DafnyCallEvent(input, output)];
    }

    predicate BinaryOfEnsuresPublicly(input: BinaryOfInput , output: Result<BinaryOfOutput, Error>)
    {Operations.BinaryOfEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method BinaryOf ( input: BinaryOfInput )
      returns (output: Result<BinaryOfOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`BinaryOf
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures BinaryOfEnsuresPublicly(input, output)
      ensures History.BinaryOf == old(History.BinaryOf) + [DafnyCallEvent(input, output)]
      // TODO: smithy-dafny isn't yet generating this
      ensures output.Success? ==> output.value.binary.Valid() && fresh(output.value.binary.Repr)
    {
      output := Operations.BinaryOf(config, input);
      History.BinaryOf := History.BinaryOf + [DafnyCallEvent(input, output)];
    }

    predicate ChunksEnsuresPublicly(input: ChunksInput , output: Result<ChunksOutput, Error>)
    {Operations.ChunksEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method Chunks ( input: ChunksInput )
      returns (output: Result<ChunksOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`Chunks
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures ChunksEnsuresPublicly(input, output)
      ensures History.Chunks == old(History.Chunks) + [DafnyCallEvent(input, output)]
    {
      output := Operations.Chunks(config, input);
      History.Chunks := History.Chunks + [DafnyCallEvent(input, output)];
    }

  }
}
abstract module AbstractSimpleStreamingOperations {
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened StandardLibrary.Streams
  import opened UTF8
  import opened Types = SimpleStreamingTypes
  type InternalConfig
  predicate ValidInternalConfig?(config: InternalConfig)
  function ModifiesInternalConfig(config: InternalConfig): set<object>
  predicate CountBitsEnsuresPublicly(input: CountBitsInput , output: Result<CountBitsOutput, Error>)
  // The private method to be refined by the library developer


  method CountBits ( config: InternalConfig , input: CountBitsInput )
    returns (output: Result<CountBitsOutput, Error>)
    requires
      // TODO: smithy-dafny isn't yet generating the `input.bits.Valid()` part.
      && ValidInternalConfig?(config) 
      && input.bits.Valid() 
      && ModifiesInternalConfig(config) !! input.bits.Repr
    // TODO: smithy-dafny isn't yet generating the `input.bits.Repr` part.
    modifies ModifiesInternalConfig(config), input.bits.Repr
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config)
    ensures
      && ValidInternalConfig?(config)
    ensures CountBitsEnsuresPublicly(input, output)


  predicate BinaryOfEnsuresPublicly(input: BinaryOfInput , output: Result<BinaryOfOutput, Error>)
  // The private method to be refined by the library developer


  method BinaryOf ( config: InternalConfig , input: BinaryOfInput )
    returns (output: Result<BinaryOfOutput, Error>)
    requires
      && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config)
    ensures
      && ValidInternalConfig?(config)
    ensures BinaryOfEnsuresPublicly(input, output)
    // TODO: smithy-dafny isn't yet generating this
    ensures output.Success? ==> output.value.binary.Valid() && fresh(output.value.binary.Repr)


  predicate ChunksEnsuresPublicly(input: ChunksInput , output: Result<ChunksOutput, Error>)
  // The private method to be refined by the library developer


  method Chunks ( config: InternalConfig , input: ChunksInput )
    returns (output: Result<ChunksOutput, Error>)
    requires
      && ValidInternalConfig?(config)
      // TODO: smithy-dafny isn't yet generating this
      && input.bytesIn.Valid() 
    modifies ModifiesInternalConfig(config)
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config)
    ensures
      && ValidInternalConfig?(config)
    ensures ChunksEnsuresPublicly(input, output)
}
