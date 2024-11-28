
module StandardLibrary.Streams {

  import opened Std.Wrappers
  import opened Std.Enumerators
  import opened Std.BoundedInts

  trait DataStream extends Enumerator<bytes> {

    ghost const value: bytes
    ghost var contentLength: uint64
    ghost var position: uint64
    ghost var mark: Option<uint64>

    function ContentLength(): Option<uint64>

    predicate SupportsMarkAndReset()

    method Mark() 
      requires SupportsMarkAndReset()
      requires mark.None?
      ensures mark == Some(position)

    method Reset()
      requires SupportsMarkAndReset()
      requires mark.Some?
      ensures position == mark.value

    // TODO: the rest of the fudging owl
  }

  type EventStream<T> = Enumerator<T>
}