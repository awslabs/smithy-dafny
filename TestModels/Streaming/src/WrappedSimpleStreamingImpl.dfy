// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleStreamingTypesWrapped.dfy"

module WrappedSimpleStreamingService refines WrappedAbstractSimpleStreamingService {
  import WrappedService = SimpleStreaming
  function method WrappedDefaultSimpleStreamingConfig(): SimpleStreamingConfig {
    SimpleStreamingConfig
  }
}