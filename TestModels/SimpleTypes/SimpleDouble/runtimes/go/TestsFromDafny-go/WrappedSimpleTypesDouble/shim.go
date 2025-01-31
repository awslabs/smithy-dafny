// Code generated by smithy-go-codegen DO NOT EDIT.

package WrappedSimpleTypesDouble

import (
	"context"

	"github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	"github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	"github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleDouble/SimpleTypesSmithyDoubleTypes"
	"github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleDouble/simpletypessmithydoublesmithygenerated"
)

type Shim struct {
	SimpleTypesSmithyDoubleTypes.ISimpleTypesDoubleClient
	client *simpletypessmithydoublesmithygenerated.Client
}

func (_static *CompanionStruct_Default___) WrappedSimpleDouble(inputConfig SimpleTypesSmithyDoubleTypes.SimpleDoubleConfig) Wrappers.Result {
	var nativeConfig = simpletypessmithydoublesmithygenerated.SimpleDoubleConfig_FromDafny(inputConfig)
	var nativeClient, nativeError = simpletypessmithydoublesmithygenerated.NewClient(nativeConfig)
	if nativeError != nil {
		return Wrappers.Companion_Result_.Create_Failure_(SimpleTypesSmithyDoubleTypes.Companion_Error_.Create_Opaque_(nativeError))
	}
	return Wrappers.Companion_Result_.Create_Success_(&Shim{client: nativeClient})
}

func (shim *Shim) GetDouble(input SimpleTypesSmithyDoubleTypes.GetDoubleInput) Wrappers.Result {
	var native_request = simpletypessmithydoublesmithygenerated.GetDoubleInput_FromDafny(input)
	var native_response, native_error = shim.client.GetDouble(context.Background(), native_request)
	if native_error != nil {
		return Wrappers.Companion_Result_.Create_Failure_(simpletypessmithydoublesmithygenerated.Error_ToDafny(native_error))
	}
	return Wrappers.Companion_Result_.Create_Success_(simpletypessmithydoublesmithygenerated.GetDoubleOutput_ToDafny(*native_response))
}
