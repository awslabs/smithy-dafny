# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# src imports
from simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.client import SimpleOrphanedShapes
from simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.shim import SimpleOrphanedShapesShim
from simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.config import dafny_config_to_smithy_config
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers

# test imports, not qualified since this isn't in a package
import WrappedSimpleOrphanedShapesService

class default__(WrappedSimpleOrphanedShapesService.default__):

    @staticmethod
    def WrappedSimpleOrphanedShapes(config):
        wrapped_config = dafny_config_to_smithy_config(config)
        impl = SimpleOrphanedShapes(wrapped_config)
        wrapped_client = SimpleOrphanedShapesShim(impl)
        return Wrappers.Result_Success(wrapped_client)

WrappedSimpleOrphanedShapesService.default__ = default__