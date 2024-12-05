import simple_orphaned_shapes.internaldafny.generated.SimpleOrphanedShapesImpl
from simple_orphaned_shapes.internaldafny.generated.SimpleOrphanedShapesImpl import *

class default__(simple_orphaned_shapes.internaldafny.generated.SimpleOrphanedShapesImpl.default__):

    @staticmethod
    def InitializeOrphanedStructure(dafny_uninitialized_structure):
        native_structure = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.dafny_to_smithy.simple_orphanedshapes_OrphanedStructure(dafny_uninitialized_structure)
        native_structure.string_value = "the extern MUST use Smithy-generated conversions to set this value in the native structure"
        dafny_initialized_structure = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.smithy_to_dafny.simple_orphanedshapes_OrphanedStructure(native_structure)
        return dafny_initialized_structure

    @staticmethod
    def CallNativeOrphanedResource(dafny_resource):
        native_resource = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.dafny_to_smithy.simple_orphanedshapes_OrphanedResourceReference(dafny_resource)
        out = native_resource.orphaned_resource_operation(
            simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.models.OrphanedResourceOperationInput(
                some_string = "the extern MUST provide this string to the native resource's operation"
            )
        )
        dafny_resource_again = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.smithy_to_dafny.simple_orphanedshapes_OrphanedResourceReference(native_resource)
        return dafny_resource_again

    @staticmethod
    def CallNativeOrphanedError(dafny_error):
        native_error = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.deserialize._deserialize_error(dafny_error)
        native_error.message = "the extern MUST use Smithy-generated conversions to set this value in the native error"
        dafny_error_again = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.errors._smithy_error_to_dafny_error(native_error)
        return dafny_error_again

simple_orphaned_shapes.internaldafny.generated.SimpleOrphanedShapesImpl.default__ = default__