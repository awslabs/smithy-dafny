// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/Index.dfy"
include "../src/WrappedSimpleStreamingImpl.dfy"

module SimpleStreamingImplTest {
    import SimpleStreaming
    import SimpleStreamingImpl
    import Std.Enumerators
    import Std.Aggregators
    import opened StandardLibrary.UInt
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
        var s: seq<bytes> := [[0x0], [0x1, 0x2], [0x3], [], [0x4, 0x5]];
        var stream := new Enumerators.SeqEnumerator(s);
        var input: CountBitsInput := CountBitsInput(bits:=stream);

        var ret :- expect client.CountBits(input);

        expect ret.sum == 7;
    }

    method TestBinaryOf(client: ISimpleStreamingClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
    {
        var s: seq<bytes> := [[0x0], [0x1, 0x2], [0x3], [], [0x4, 0x5]];
        var stream := new Enumerators.SeqEnumerator(s);
        var input: BinaryOfInput := BinaryOfInput(number:=42);

        var ret :- expect client.BinaryOf(input);

        var collector := new Aggregators.Collector<bytes>();
 
        Enumerators.ForEach(ret.binary, collector);

        expect collector.values == [[12], [34, 56]];
    }
}