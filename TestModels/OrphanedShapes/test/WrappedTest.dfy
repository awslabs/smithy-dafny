// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/WrappedSimpleOrphanedImpl.dfy"
include "ExternDefinitions.dfy"

// There are no non-wrapped tests for this TestModel.
// This TestModel requires implementing externs that call Polymorph-generated code.
// Polymorph must be in the mix, even for the non-wrapped client, so it is reasonable to only have wrapped tests.

module WrappedTest {
  import WrappedSimpleOrphanedService
  import opened Types = SimpleOrphanedTypes
  import ExternDefinitions

  // method {:test} TestWrappedClient()
  // {
  //   // ExternDefinitions.TestOrphanedStructure();
  //   ExternDefinitions.TestOrphanedResource();
  //   // ExternDefinitions.TestOrphanedError();

  //   // var client :- expect WrappedSimpleOrphanedService.WrappedSimpleOrphaned();
  //   // TestCreateOrphanedStructure(client);
  //   // TestCreateOrphanedResource(client);
  //   // TestCreateOrphanedError(client);
  // }

  method {:test} TestOrphanedStructure() {
    ExternDefinitions.TestOrphanedStructure();
  }

  method {:test} TestOrphanedResource() {
    ExternDefinitions.TestOrphanedResource();
  }

  method {:test} TestOrphanedError() {
    ExternDefinitions.TestOrphanedError();
  }
}
