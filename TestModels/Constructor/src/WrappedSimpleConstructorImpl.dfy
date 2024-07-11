// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleConstructorTypesWrapped.dfy"

module WrappedSimpleConstructorService refines WrappedAbstractSimpleConstructorService {
    import WrappedService = SimpleConstructor
    function method WrappedDefaultSimpleConstructorConfig(): SimpleConstructorConfig {
        WrappedService.DefaultSimpleConstructorConfig()
    }
}
