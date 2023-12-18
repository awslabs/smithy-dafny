// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../../LanguageSpecificLogicImpl.dfy"

module NetLanguageSpecificLogicImpl replaces LanguageSpecificLogicImpl  {
    // This method is listed as an operation on the service in the Smithy model.
    // Smithy-Dafny will write code to call this operation.
    // Internally, the generated Dafny will call the extern.
    // This provides a consistent interface for users.
    method GetRuntimeInformation(config: InternalConfig)
        returns (output: Result<GetRuntimeInformationOutput, Error>)
        ensures output.Success? ==> output.value.language == "NET"
    {
        var runtimeInfo := GetRuntimeInformationNetExternMethod(config);
        var packedOutput := GetRuntimeInformationOutput(
            language := "NET",
            runtime := runtimeInfo.value
        );
        return Success(packedOutput);
    }

    // This method is NOT listed as an operation on the service in the Smithy model.
    // Since this is an extern method with a different name per language, we can't define
    //   the interface for this method on the Smithy model.
    // Instead, we define the `AllRuntimesMethod` which IS a Smithy operation
    //   and call this method from there.
    method {:extern "GetNetRuntimeVersion" } GetRuntimeInformationNetExternMethod(config: InternalConfig)
        returns (output: Result<string, Error>)
}
