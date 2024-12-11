// Code generated by smithy-go-codegen DO NOT EDIT.

package simpleorphanedsmithygenerated

import (
	"github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	"github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
	"github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/simpleorphanedsmithygeneratedtypes"
)

type OrphanedResourceNativeWrapper struct {
	SimpleOrphanedTypes.IOrphanedResource
	Impl simpleorphanedsmithygeneratedtypes.IOrphanedResource
}

func (this *OrphanedResourceNativeWrapper) OrphanedResourceOperation(input SimpleOrphanedTypes.OrphanedResourceOperationInput) Wrappers.Result {
	var native_request = OrphanedResourceOperationInput_FromDafny(input)
	var native_response, native_error = this.Impl.OrphanedResourceOperation(native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(OrphanedResourceOperationOutput_ToDafny(*native_response))
}
