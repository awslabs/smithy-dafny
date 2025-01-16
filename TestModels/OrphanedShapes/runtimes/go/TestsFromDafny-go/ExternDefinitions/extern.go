package ExternDefinitions

import (
	"fmt"

	Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	OrphanedResource "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/OrphanedResource"
	SimpleOrphanedTypes "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
	simpleorphanedsmithygenerated "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/simpleorphanedsmithygenerated"
	simpleorphanedsmithygeneratedtypes "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/simpleorphanedsmithygeneratedtypes"
)

var _ Wrappers.Dummy__

// TODO: Finish implementation.
// This is missing structure converter.

func (CompanionStruct_Default___) InitializeOrphanedStructure(input SimpleOrphanedTypes.OrphanedStructure) SimpleOrphanedTypes.OrphanedStructure {
	// Missing Structure converter
	return input
}

func (CompanionStruct_Default___) CallNativeOrphanedResource(input *OrphanedResource.OrphanedResource) Wrappers.Result {
	native_resource := simpleorphanedsmithygenerated.OrphanedResource_FromDafny(input)
	someString := "the extern MUST provide this string to the native resource's operation"
	native_output, err := native_resource.OrphanedResourceOperation(simpleorphanedsmithygeneratedtypes.OrphanedResourceOperationInput{SomeString: &someString})
	if err != nil {
		return Wrappers.Companion_Result_.Create_Failure_(err)
	}
	dafny_output := simpleorphanedsmithygenerated.OrphanedResourceOperationOutput_ToDafny(*native_output)
	return Wrappers.Companion_Result_.Create_Success_(dafny_output)
}

func (CompanionStruct_Default___) CallNativeOrphanedError(input SimpleOrphanedTypes.Error) SimpleOrphanedTypes.Error {
	native_error := simpleorphanedsmithygenerated.Error_FromDafny(input)
	native_error.Message = "the extern MUST set this string using the catch-all error converter, NOT the orphaned error-specific converter"
	dafny_error_again := simpleorphanedsmithygenerated.Error_ToDafny(native_error)
	return dafny_error_again
}
