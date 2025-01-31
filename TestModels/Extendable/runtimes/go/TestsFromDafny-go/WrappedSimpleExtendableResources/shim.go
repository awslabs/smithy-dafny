// Code generated by smithy-go-codegen DO NOT EDIT.

package WrappedSimpleExtendableResources

import (
	"context"

	"github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	"github.com/smithy-lang/smithy-dafny/TestModels/Extendable/SimpleExtendableResourcesTypes"
	"github.com/smithy-lang/smithy-dafny/TestModels/Extendable/simpleextendableresourcessmithygenerated"
)

type Shim struct {
	SimpleExtendableResourcesTypes.ISimpleExtendableResourcesClient
	client *simpleextendableresourcessmithygenerated.Client
}

func (_static *CompanionStruct_Default___) WrappedSimpleExtendableResources(inputConfig SimpleExtendableResourcesTypes.SimpleExtendableResourcesConfig) Wrappers.Result {
	var nativeConfig = simpleextendableresourcessmithygenerated.SimpleExtendableResourcesConfig_FromDafny(inputConfig)
	var nativeClient, nativeError = simpleextendableresourcessmithygenerated.NewClient(nativeConfig)
	if nativeError != nil {
		return Wrappers.Companion_Result_.Create_Failure_(SimpleExtendableResourcesTypes.Companion_Error_.Create_Opaque_(nativeError))
	}
	return Wrappers.Companion_Result_.Create_Success_(&Shim{client: nativeClient})
}

func (shim *Shim) CreateExtendableResource(input SimpleExtendableResourcesTypes.CreateExtendableResourceInput) Wrappers.Result {
	var native_request = simpleextendableresourcessmithygenerated.CreateExtendableResourceInput_FromDafny(input)
	var native_response, native_error = shim.client.CreateExtendableResource(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpleextendableresourcessmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpleextendableresourcessmithygenerated.CreateExtendableResourceOutput_ToDafny(*native_response))
}

func (shim *Shim) UseExtendableResource(input SimpleExtendableResourcesTypes.UseExtendableResourceInput) Wrappers.Result {
	var native_request = simpleextendableresourcessmithygenerated.UseExtendableResourceInput_FromDafny(input)
	var native_response, native_error = shim.client.UseExtendableResource(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpleextendableresourcessmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpleextendableresourcessmithygenerated.UseExtendableResourceOutput_ToDafny(*native_response))
}

func (shim *Shim) UseExtendableResourceAlwaysModeledError(input SimpleExtendableResourcesTypes.UseExtendableResourceErrorsInput) Wrappers.Result {
	var native_request = simpleextendableresourcessmithygenerated.UseExtendableResourceErrorsInput_FromDafny(input)
	var native_response, native_error = shim.client.UseExtendableResourceAlwaysModeledError(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpleextendableresourcessmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpleextendableresourcessmithygenerated.GetExtendableResourceErrorsOutput_ToDafny(*native_response))
}

func (shim *Shim) UseExtendableResourceAlwaysMultipleErrors(input SimpleExtendableResourcesTypes.UseExtendableResourceErrorsInput) Wrappers.Result {
	var native_request = simpleextendableresourcessmithygenerated.UseExtendableResourceErrorsInput_FromDafny(input)
	var native_response, native_error = shim.client.UseExtendableResourceAlwaysMultipleErrors(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpleextendableresourcessmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpleextendableresourcessmithygenerated.GetExtendableResourceErrorsOutput_ToDafny(*native_response))
}

func (shim *Shim) UseExtendableResourceAlwaysOpaqueError(input SimpleExtendableResourcesTypes.UseExtendableResourceErrorsInput) Wrappers.Result {
	var native_request = simpleextendableresourcessmithygenerated.UseExtendableResourceErrorsInput_FromDafny(input)
	var native_response, native_error = shim.client.UseExtendableResourceAlwaysOpaqueError(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpleextendableresourcessmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpleextendableresourcessmithygenerated.GetExtendableResourceErrorsOutput_ToDafny(*native_response))
}
