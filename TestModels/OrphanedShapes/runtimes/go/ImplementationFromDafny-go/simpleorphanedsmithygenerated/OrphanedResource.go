// Code generated by smithy-go-codegen DO NOT EDIT.

package simpleorphanedsmithygenerated

import (
	"github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
	"github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/simpleorphanedsmithygeneratedtypes"
)

type OrphanedResource struct {
	Impl SimpleOrphanedTypes.IOrphanedResource
}

func (this *OrphanedResource) OrphanedResourceOperation(params simpleorphanedsmithygeneratedtypes.OrphanedResourceOperationInput) (*simpleorphanedsmithygeneratedtypes.OrphanedResourceOperationOutput, error) {
	var dafny_request SimpleOrphanedTypes.OrphanedResourceOperationInput = OrphanedResourceOperationInput_ToDafny(params)
	var dafny_response = this.Impl.OrphanedResourceOperation(dafny_request)

	if dafny_response.Is_Failure() {
		err := dafny_response.Dtor_error().(SimpleOrphanedTypes.Error)
		return nil, Error_FromDafny(err)
	}
	var native_response = OrphanedResourceOperationOutput_FromDafny(dafny_response.Dtor_value().(SimpleOrphanedTypes.OrphanedResourceOperationOutput))
	return &native_response, nil

}