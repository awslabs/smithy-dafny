// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/LanguageSpecificLogicTypesWrapped.dfy"

// Replaced in test/ directory
replaceable module WrappedLanguageSpecificLogicService refines WrappedAbstractLanguageSpecificLogicService {
    import WrappedService = LanguageSpecificLogic
    function method WrappedDefaultLanguageSpecificLogicConfig(): LanguageSpecificLogicConfig {
        WrappedService.DefaultLanguageSpecificLogicConfig()
    }
}
