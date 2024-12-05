// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/WrappedSimpleOrphanedImpl.dfy"

// There are no non-wrapped tests for this TestModel.
// This TestModel requires implementing externs that call Polymorph-generated code.
// Polymorph must be in the mix, even for the non-wrapped client, so it is reasonable to only have wrapped tests.

module WrappedTest {
  import WrappedSimpleOrphanedService
  import opened Types = SimpleOrphanedTypes

  method {:test} TestWrappedClient()
  {
    var client :- expect WrappedSimpleOrphanedService.WrappedSimpleOrphaned();
    TestCreateOrphanedStructure(client);
    TestCreateOrphanedResource(client);
    TestCreateOrphanedError(client);
  }

  method TestCreateOrphanedStructure(client: Types.ISimpleOrphanedClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
  {
    var ret := client.CreateOrphanedStructure(Types.CreateOrphanedStructureInput);
    expect ret.Success?;
  }

    method TestCreateOrphanedResource(client: Types.ISimpleOrphanedClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
  {
    var ret := client.CreateOrphanedResource(Types.CreateOrphanedResourceInput);
    expect ret.Success?;
  }

  method TestCreateOrphanedError(client: Types.ISimpleOrphanedClient)
      requires client.ValidState()
      modifies client.Modifies
      ensures client.ValidState()
  {
    var ret := client.CreateOrphanedError(Types.CreateOrphanedErrorInput);
    expect ret.Success?;
  }
}
