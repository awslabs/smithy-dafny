# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# src imports
from simple_positional.smithygenerated.simple_positional.client import SimplePositional
from simple_positional.smithygenerated.simple_positional.shim import SimplePositionalShim
from simple_positional.smithygenerated.simple_positional.config import dafny_config_to_smithy_config
import standard_library.internaldafny.generated.Wrappers as Wrappers

# test imports, not qualified since this isn't in a package
import WrappedSimplePositionalService

class default__(WrappedSimplePositionalService.default__):

    @staticmethod
    def WrappedSimplePositional(config):
        wrapped_config = dafny_config_to_smithy_config(config)
        impl = SimplePositional(wrapped_config)
        wrapped_client = SimplePositionalShim(impl)
        return Wrappers.Result_Success(wrapped_client)

WrappedSimplePositionalService.default__ = default__