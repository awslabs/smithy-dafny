// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleTypesSmithyEnumTypesWrapped.dfy"

module {:extern "simpletypessmithyenuminternaldafnywrapped"} WrappedSimpleTypesEnumService refines WrappedAbstractSimpleTypesSmithyEnumService {
    import WrappedService = SimpleEnum
    function method WrappedDefaultSimpleEnumConfig(): SimpleEnumConfig {
        SimpleEnumConfig
    }
}
