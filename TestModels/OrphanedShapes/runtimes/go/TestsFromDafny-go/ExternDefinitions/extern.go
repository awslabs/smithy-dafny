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

func (CompanionStruct_Default___) InitializeOrphanedStructure(input SimpleOrphanedTypes.OrphanedStructure) SimpleOrphanedTypes.OrphanedStructure {
	// Missing Structure converter
	return input
}

func (CompanionStruct_Default___) CallNativeOrphanedResource(input *OrphanedResource.OrphanedResource) Wrappers.Result {
	native_resource := simpleorphanedsmithygenerated.OrphanedResource_FromDafny(input)
	fmt.Printf("Type of native_resource: %T\n", native_resource)
	someString := "the extern MUST provide this string to the native resource's operation"
	fmt.Printf("Type of someString: %T\n", someString)
	fmt.Printf("Type of &someString: %T\n", &someString)
	native_output, err := native_resource.OrphanedResourceOperation(simpleorphanedsmithygeneratedtypes.OrphanedResourceOperationInput{SomeString: &someString})
	fmt.Printf("Type of native_output: %T\n", native_output)
	if err != nil {
		return Wrappers.Companion_Result_.Create_Failure_(err)
	}
	dafny_output := simpleorphanedsmithygenerated.OrphanedResourceOperationOutput_ToDafny(*native_output)
	return Wrappers.Companion_Result_.Create_Success_(dafny_output)
}

func (CompanionStruct_Default___) CallNativeOrphanedError(input SimpleOrphanedTypes.Error) SimpleOrphanedTypes.Error {
	native_error := simpleorphanedsmithygenerated.Error_FromDafny(input)
	native_error.Message = "the extern MUST use Smithy-generated conversions to set this value in the native error"
	dafny_error_again := simpleorphanedsmithygenerated.Error_ToDafny(native_error)
	return dafny_error_again
}
