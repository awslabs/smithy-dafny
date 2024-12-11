package ExternDefinitions

import (
	Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	OrphanedResource "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/OrphanedResource"
	SimpleOrphanedTypes "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
)

var _ Wrappers.Dummy__

func (CompanionStruct_Default___) InitializeOrphanedStructure(input SimpleOrphanedTypes.OrphanedStructure) SimpleOrphanedTypes.OrphanedStructure {
	return input
}

func (CompanionStruct_Default___) CallNativeOrphanedResource(input *OrphanedResource.OrphanedResource) Wrappers.Result {
	OrphanedResource_FromDafny(input)
	return Wrappers.Result_Success(input)
}

func (CompanionStruct_Default___) CallNativeOrphanedError(input SimpleOrphanedTypes.Error) SimpleOrphanedTypes.Error {
	return input
}
