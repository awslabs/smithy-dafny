from simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.references import OrphanedResource

class default__:
    @staticmethod
    def WrappedResource():
        native_resource = OrphanedResource()
        dafny_resource = simple_orphaned_shapes.smithygenerated.simple_orphanedshapes.smithy_to_dafny.