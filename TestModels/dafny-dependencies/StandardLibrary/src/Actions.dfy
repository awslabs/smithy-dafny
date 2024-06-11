
include "../../libraries/src/Wrappers.dfy"

include "Frames.dfy"
include "GenericAction.dfy"
include "DecreasesClauses.dfy"
include "DynamicArray.dfy"

module {:options "--function-syntax:4"} Std.Actions {

  import opened Wrappers
  import opened Frames
  import opened GenericActions
  import opened Termination
  import opened DynamicArray

  // TODO: Documentation, especially overall design
  trait {:termination false} Action<T, R> extends GenericAction<T, R>, Validatable {

    ghost var history: seq<(T, R)>

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0

    // KEY DESIGN POINT: these predicates specifically avoid reading the current
    // state of the action.
    // That's so extrisnic properties of an action do NOT depend on their current state.
    // This is key to ensure that you can prove properties of a given action that
    // will continue to hold as the Dafny heap changes.
    // This approach works because Dafny understands that for a given object,
    // the implementation of CanConsume/CanProduce cannot change over time.
    //
    // The downside is that these are then forced to use quantifiers
    // to talk about all possible states of an action.

    // TODO: Necessary but not sufficient that:
    // CanConsume(history, nextIn) ==> exists nextOut :: CanProduce(history + [(nextIn, nextOut)])
    // Does that need to be explicitly part of the spec?
    ghost predicate CanConsume(history: seq<(T, R)>, next: T)
      requires CanProduce(history)
      decreases height

    ghost predicate CanProduce(history: seq<(T, R)>)
      decreases height

    ghost predicate Requires(t: T)
      reads Reads(t) 
    {
      && Valid()
      && CanConsume(history, t)
    }
    ghost function Reads(t: T): set<object> 
      reads this
      ensures this in Reads(t)
    {
      {this} + Repr
    }
    ghost function Modifies(t: T): set<object> 
      reads Reads(t)
    {
      Repr
    }
    ghost function Decreases(t: T): TerminationMetric 
      reads Reads(t)
    {
      NatTerminationMetric(height)
    }
    twostate predicate Ensures(t: T, new r: R) 
      reads Reads(t)
    {
      && Valid()
      && history == old(history) + [(t, r)]
      && fresh(Repr - old(Repr))
    }

    // Helpers

    ghost method Update(t: T, r: R)
      modifies `history
      ensures history == old(history) + [(t, r)]
    {
      history := history + [(t, r)];
    }

    ghost function Consumed(): seq<T> 
      reads this
    {
      Inputs(history)
    }

    ghost function Produced(): seq<R> 
      reads this
    {
      Outputs(history)
    }
  }


  // Dependencies stolen from DafnyStandardLibraries
  
  function {:opaque} SeqMap<T, R>(f: (T ~> R), xs: seq<T>): (result: seq<R>)
    requires forall i :: 0 <= i < |xs| ==> f.requires(xs[i])
    ensures |result| == |xs|
    ensures forall i {:trigger result[i]} :: 0 <= i < |xs| ==> result[i] == f(xs[i])
    reads set i, o | 0 <= i < |xs| && o in f.reads(xs[i]) :: o
  {
    if |xs| == 0 then []
    else [f(xs[0])] + SeqMap(f, xs[1..])
  }

  function Max(a: int, b: int): int
  {
    if a < b
    then b
    else a
  }

  // Common action invariants

  function Inputs<T, R>(history: seq<(T, R)>): seq<T> {
    SeqMap((e: (T, R)) => e.0, history)
  }

  function Outputs<T, R>(history: seq<(T, R)>): seq<R> {
    SeqMap((e: (T, R)) => e.1, history)
  }

  ghost predicate OnlyProduces<T, R>(i: Action<T, R>, history: seq<(T, R)>, c: R) 
  {
    i.CanProduce(history) <==> forall e <- history :: e.1 == c
  }

  ghost predicate CanConsumeAll<T(!new), R(!new)>(a: Action<T, R>, input: seq<T>) 
  {
    forall i | 0 < i < |input| ::
      var consumed := input[..(i - 1)];
      var next := input[i];
      forall history | a.CanProduce(history) && Inputs(history) == consumed :: a.CanConsume(history, next)
  }

  ghost predicate Terminated<T>(s: seq<T>, c: T, n: nat) {
    forall i | 0 <= i < |s| :: n <= i <==> s[i] == c
  }

  lemma TerminatedUndistributes<T>(left: seq<T>, right: seq<T>, c: T, n: nat)
    requires Terminated(left + right, c, n)
    ensures Terminated(left, c, n)
    ensures Terminated(right, c, Max(0, n - |left|))
  {
    assert forall i | 0 <= i < |left| :: left[i] == (left + right)[i];
    assert forall i | 0 <= i < |right| :: right[i] == (left + right)[i + |left|];
  }

  // TODO: generalize to "EventuallyProducesSequence"?
  ghost predicate ProducesTerminatedBy<T(!new), R(!new)>(i: Action<T, R>, c: R, limit: nat) {
    forall history: seq<(T, R)> | i.CanProduce(history) 
      :: exists n: nat | n <= limit :: Terminated(Outputs(history), c, n)
  }

  // Class of actions whose precondition doesn't depend on history (probably needs a better name)
  ghost predicate ContextFree<T(!new), R(!new)>(a: Action<T, R>, p: T -> bool) {
    forall history, next | a.CanProduce(history)
      :: a.CanConsume(history, next) <==> p(next)
  }

  // Enumerators

  type IEnumerator<T> = Action<(), T>
  type Enumerator<T(!new)> = a: Action<(), Option<T>> | exists limit :: ProducesTerminatedBy(a, None, limit) witness *
  

  // Aggregators

  // TODO: Names need improvement
  type IAggregator<T> = Action<T, ()>
  type Aggregator<T(!new)> = a: Action<T, bool> | exists limit :: ProducesTerminatedBy(a, false, limit) witness *
  type Accumulator<T(!new)> = Action<Option<T>, ()> // | exists limit :: ConsumesTerminatedBy(a, None, limit) witness *

  // Streams

  trait Stream<T(!new)> extends Action<(), Option<T>> {

    method ForEach(a: Accumulator<T>)

    lemma IsEnumerator()
      ensures (this as object) is Enumerator<T>
  }

  method {:verify false} DefaultForEach<T(!new)>(s: Enumerator<T>, a: Aggregator<T>) {
    // TODO: Actual specs to prove this terminates
    while (true) {
      var next := s.Invoke(());
      if next == None {
        break;
      }

      var success := a.Invoke(next.value);
      assert success;
    }
  }

  class ArrayAggregator<T> extends Action<T, ()> {

    var storage: DynamicArray<T>

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> 
        && CanProduce(history)
      decreases height, 0
    {
      && this in Repr
      && storage in Repr
      && this !in storage.Repr
      && storage.Repr <= Repr
      && storage.Valid?()
      && Consumed() == storage.items
    }

    constructor() 
      ensures Valid()
      ensures fresh(Repr - {this})
      ensures history == []
    {
      var a := new DynamicArray();

      history := [];
      height := 1;
      Repr := {this} + {a} + a.Repr;
      this.storage := a;
    }

    ghost predicate CanConsume(history: seq<(T, ())>, next: T)
      decreases height
    {
      true
    }
    ghost predicate CanProduce(history: seq<(T, ())>)
      decreases height
    {
      true
    }

    method Invoke(t: T) returns (r: ()) 
      requires Requires(t)
      modifies Modifies(t)
      decreases Decreases(t).Ordinal()
      ensures Ensures(t, r)
    {
      storage.Push(t);

      r := ();
      Update(t, r);
      Repr := {this} + {storage} + storage.Repr;
      assert Valid();
    }
  }

  method {:rlimit 0} AggregatorExample() {
    var a := new ArrayAggregator();
    var _ := a.Invoke(1);
    var _ := a.Invoke(2);
    var _ := a.Invoke(3);
    var _ := a.Invoke(4);
    var _ := a.Invoke(5);
    assert a.storage.items == [1, 2, 3, 4, 5];
  }

  // Other primitives/examples todo:
  //  * Promise-like single-use Action<T, ()> to capture a value for reading later
  //  * Eliminate all the (!new) restrictions - look into "older" parameters?
  //    * How to state the invariant that a constructor as an action creates a new object every time?
  //      * Lemma that takes produced as input, instead of forall produced?
  //  * Enumerable ==> defines e.Enumerator()
  //    * BUT can have infinite containers, probably need IEnumerable as well? (different T for the Action)
  //  * Expressing that an Action "Eventually produces something" (look at how VMC models this for randomness)
  //    * IsEnumerator(a) == "a eventually produces None" && "a then only produces None"
  //    * Build on that to make CrossProduct(enumerable1, enumerable2)
  //  * Example of adapting an iterator
  //  * Example of enumerating all possible values of a type (for test generation)
  //    * Needs to handle infinite types too, hence needs the "eventually produces something" concept
  //  * Document: useful pattern to add an Action<Command, Result> facade
  //        to a set of methods that participate in a protocol.
  //        Then you have a history that ties the behavior
  //        of the methods together,
  //        supporting constraints on the order of calls etc.


  class SeqEnumerator<T> extends Action<(), Option<T>> {

    var values: seq<T>

    constructor(values: seq<T>) {
      this.values := values;
    }

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0
    {
      this in Repr
    }
    
    ghost predicate CanConsume(history: seq<((), Option<T>)>, next: ())
      requires CanProduce(history)
      decreases height
    {
      true
    }

    ghost predicate CanProduce(history: seq<((), Option<T>)>)
      decreases height
    {
      true
    }

    method {:verify false} Invoke(input: ()) returns (value: Option<T>) 
      modifies Repr
    {
      if |values| == 0 {
        value := None;
      } else {
        value := Some(values[0]);
        values := values[1..];
      }
    }

  }

  // TODO
  // class Folder<T, R> extends Action<T, bool> {

  //   const f: (R, T) -> R
  //   var value: R

  //   constructor(init: R, f: (R, T) -> R) {
  //     this.f := f;
  //     this.value := init;
  //   }

  //   method {:verify false} Call(t: T) returns (success: bool) {
  //     value := f(value, t);
  //     return success;
  //   }

  // }

  // TODO: This is also a Folder([], (x, y) => x + [y])
  class Collector<T> extends Action<Option<T>, ()> {

    var values: seq<T>

    constructor() {
      values := [];
    }

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0
    {
      this in Repr 
    }

    ghost predicate CanConsume(history: seq<(Option<T>, ())>, next: Option<T>)
      requires CanProduce(history)
      decreases height
    {
      true
    }

    ghost predicate CanProduce(history: seq<(Option<T>, ())>)
      decreases height
    {
      true
    }

    method {:verify false} Invoke(t: Option<T>) returns (nothing: ()) {
      if t.Some? {
        values := values + [t.value];
      }
    }

    method {:verify false} Pop() returns (t: T) 
      requires 0 < |values|
    {
      t := values[0];
      values := values[1..];
    }

  }

  trait {:termination false} Pipeline<U(!new), T(!new)> extends Stream<T> {

    const upstream: Stream<U>
    const buffer: Collector<T>

    method {:verify false} Invoke(nothing: ()) returns (t: Option<T>) {
      while (|buffer.values| == 0) {
        var u := upstream.Invoke(());
        Process(u, buffer);

        if u.None? {
          break;
        }
      }

      if 0 < |buffer.values| {
        var next := buffer.Pop();
        return Some(next);
      } else {
        return None;
      }
    }

    method {:verify false} ForEach(a: Accumulator<T>)
    {
      var a' := new PipelineProcessor(this, a);
      upstream.ForEach(a');
    }

    method Process(u: Option<U>, a: Accumulator<T>)

  }

  class PipelineProcessor<U(!new), T(!new)> extends Action<Option<U>, ()> {

    const pipeline: Pipeline<U, T>
    const accumulator: Accumulator<T>

    constructor(pipeline: Pipeline<U, T>, accumulator: Accumulator<T>) {
      this.pipeline := pipeline;
      this.accumulator := accumulator;
    }

    ghost predicate Valid() 
      reads this, Repr 
      ensures Valid() ==> this in Repr 
      ensures Valid() ==> CanProduce(history)
      decreases height, 0
    {
      this in Repr 
    }

    ghost predicate CanConsume(history: seq<(Option<U>, ())>, next: Option<U>)
      requires CanProduce(history)
      decreases height
    {
      true
    }

    ghost predicate CanProduce(history: seq<(Option<U>, ())>)
      decreases height
    {
      true
    }

    method {:verify false} Invoke(u: Option<U>) returns (nothing: ()) {
      pipeline.Process(u, accumulator);
    }
  }
}