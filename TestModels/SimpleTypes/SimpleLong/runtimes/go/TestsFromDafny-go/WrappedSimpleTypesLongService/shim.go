// Code generated by smithy-go-codegen DO NOT EDIT.

package WrappedSimpleTypesLongService

import (
	"context"

	"github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	"github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	"github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleLong/SimpleTypesSmithyLongTypes"
	"github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleLong/simpletypessmithylongsmithygenerated"
)

type Shim struct {
	SimpleTypesSmithyLongTypes.ISimpleTypesLongClient
	client *simpletypessmithylongsmithygenerated.Client
}

func (_static *CompanionStruct_Default___) WrappedSimpleLong(inputConfig SimpleTypesSmithyLongTypes.SimpleLongConfig) Wrappers.Result {
	var nativeConfig = simpletypessmithylongsmithygenerated.SimpleLongConfig_FromDafny(inputConfig)
	var nativeClient, nativeError = simpletypessmithylongsmithygenerated.NewClient(nativeConfig)
	if nativeError != nil {
		return Wrappers.Companion_Result_.Create_Failure_(SimpleTypesSmithyLongTypes.Companion_Error_.Create_Opaque_(nativeError))
	}
	return Wrappers.Companion_Result_.Create_Success_(&Shim{client: nativeClient})
}

func (shim *Shim) GetLong(input SimpleTypesSmithyLongTypes.GetLongInput) Wrappers.Result {
	var native_request = simpletypessmithylongsmithygenerated.GetLongInput_FromDafny(input)
	var native_response, native_error = shim.client.GetLong(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpletypessmithylongsmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpletypessmithylongsmithygenerated.GetLongOutput_ToDafny(*native_response))
}

func (shim *Shim) GetLongKnownValueTest(input SimpleTypesSmithyLongTypes.GetLongInput) Wrappers.Result {
	var native_request = simpletypessmithylongsmithygenerated.GetLongInput_FromDafny(input)
	var native_response, native_error = shim.client.GetLongKnownValueTest(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpletypessmithylongsmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpletypessmithylongsmithygenerated.GetLongOutput_ToDafny(*native_response))
}
