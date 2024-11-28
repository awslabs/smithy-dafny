
module {:options "--function-syntax:4"} StandardLibrary.Streams {

  import opened Std.Wrappers
  import opened Std.Actions
  import opened Std.Enumerators
  import opened Std.BoundedInts
  import opened Std.Collections.Seq

  // Alias just for clarity
  type EventStream<T> = Enumerator<T>

  trait DataStream extends Enumerator<BoundedInts.bytes> {

    ghost const value: bytes

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0

    function ConcatenatedOutputs(history: seq<((), Option<bytes>)>): bytes {
      Flatten(Enumerated(Outputs(history)))
    }

    function ContentLength(): (res: Option<uint64>)
      requires Valid()
      reads this, Repr
      ensures res.Some? ==> res.value as int == |value|

    ghost predicate CanProduce(history: seq<((), Option<bytes>)>)
      decreases height
    {
      && (forall o <- Enumerated(Outputs(history)) :: 0 < |o|)
      && ConcatenatedOutputs(history) <= value
    }

    lemma {:axiom} ProducesTerminated(history: seq<((), Option<bytes>)>)
      requires Action().CanProduce(history) 
      requires (forall i <- Inputs(history) :: i == FixedInput())
      ensures exists n: nat | n <= Limit() :: Terminated(Outputs(history), StopFn(), n)
    // {
    //   assert Terminated(Outputs(history), StopFn(), |Enumerated(Outputs(history))|);
    // }

    method RepeatUntil(t: (), stop: Option<bytes> -> bool, ghost eventuallyStopsProof: ProducesTerminatedProof<(), Option<bytes>>)
      requires Valid()
      requires eventuallyStopsProof.Action() == this
      requires eventuallyStopsProof.FixedInput() == t
      requires eventuallyStopsProof.StopFn() == stop
      requires forall i <- Consumed() :: i == t
      modifies Repr
      decreases Repr
      ensures Valid()
    {
      DefaultRepeatUntil(this, t, stop, eventuallyStopsProof);
    }

    /*
     * Whether this stream can be read multiple times.
     * If this is true, it is at least possible to seek to earlier positions.
     * This does not necessarily mean seeking is constant time,
     * because it may mean re-reading from the start of the stream.
     */
    predicate Rewindable()
      decreases height, 1

    function Position(): (res: uint64)
      requires Valid()
      requires Rewindable()
      reads this, Repr
      decreases height, 2
      ensures res as int <= |value|

    method Seek(newPosition: uint64)
      requires Valid()
      requires Rewindable()
      requires newPosition as int <= |value|
      ensures Valid()
      ensures Position() == newPosition
  }

  /*
   * Wraps an Enumerator up as a non-rewindable DataStream.
   */
  class EnumeratorDataStream<bytes> extends DataStream {

    const wrapped: Enumerator<BoundedInts.bytes>
    const length: Option<uint64>

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0
    {
      && this in Repr
      && ValidComponent(wrapped)
      && CanProduce(history)
      && (length.Some? ==> length.value as int == |value|)
    }

    lemma {:axiom} ProducesTerminated(history: seq<((), Option<BoundedInts.bytes>)>)
      requires Action().CanProduce(history) 
      requires (forall i <- Inputs(history) :: i == FixedInput())
      ensures exists n: nat | n <= Limit() :: Terminated(Outputs(history), StopFn(), n)

    ghost function Limit(): nat {
      wrapped.Limit()
    }

    constructor(wrapped: Enumerator<BoundedInts.bytes>, length: Option<uint64>, ghost value: BoundedInts.bytes) 
      requires wrapped.Valid()
      requires wrapped.history == []
      requires length.Some? ==> length.value as int == |value|
      ensures Valid()
    {
      this.wrapped := wrapped;
      this.length := length;
      this.value := value;

      this.history := [];
      this.Repr := {this} + wrapped.Repr;
      this.height := wrapped.height + 1;
    }

    function ContentLength(): (res: Option<uint64>)
      requires Valid()
      reads this, Repr
      ensures res.Some? ==> res.value as int == |value|
    {
      length
    }

    predicate Rewindable()
      decreases height, 1
    {
      false
    }

    function Position(): (res: uint64)
      requires Rewindable()
      reads this, Repr
      ensures res as int <= |value|
    {
      // Unreachable
      assert false;
      0
    }

    method Seek(newPosition: uint64)
      requires Valid()
      requires Rewindable()
      requires newPosition as int <= |value|
      ensures Valid()
      ensures Position() == newPosition
    {
      // Unreachable
      assert false;
    }

    method Invoke(t: ()) returns (r: Option<BoundedInts.bytes>) 
      requires Requires(t)
      modifies Modifies(t)
      decreases Decreases(t).Ordinal()
      ensures Ensures(t, r)
    {
      // TODO: Not sure why this isn't provable from GenericAction?
      assume {:axiom} Valid();
      r := wrapped.Next();
      Update(t, r);
      
      // TODO: Work to do
      assume {:axiom} Ensures(t, r);
    }
  }


}