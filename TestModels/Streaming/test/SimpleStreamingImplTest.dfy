// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/Index.dfy"
include "../src/WrappedSimpleStreamingImpl.dfy"

module SimpleStreamingImplTest {
    import SimpleStreaming
    import SimpleStreamingImpl
    import Std.Enumerators
    import Std.Aggregators
    import Std.BoundedInts
    import opened StandardLibrary.UInt
    import opened Std.Streams
    import opened SimpleStreamingTypes
    import opened Wrappers
    method{:test} TestClient(){
        var client :- expect SimpleStreaming.SimpleStreaming();
        TestCountBits(client);
    }

    method TestCountBits(client: ISimpleStreamingClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
    {
        var s: seq<BoundedInts.bytes> := [[0x0], [0x1, 0x2], [0x3], [], [0x4, 0x5]];
        var e := new Enumerators.SeqEnumerator(s);
        var stream := new EnumeratorDataStream(e, 5 as BoundedInts.uint64);
        var input: CountBitsInput := CountBitsInput(bits:=stream);

        var ret :- expect client.CountBits(input);

        expect ret.sum == 7;
    }

    method TestBinaryOf(client: ISimpleStreamingClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
    {
        var s: seq<BoundedInts.bytes> := [[0x0], [0x1, 0x2], [0x3], [], [0x4, 0x5]];
        var stream := new Enumerators.SeqEnumerator(s);
        var input: BinaryOfInput := BinaryOfInput(number:=42);

        var ret :- expect client.BinaryOf(input);

        var collector := new Aggregators.Collector<BoundedInts.bytes>();
 
        Enumerators.ForEach(ret.binary, collector);

        expect collector.values == [[12], [34, 56]];
    }
}