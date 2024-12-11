// Package ExternDefinitions
// Dafny module ExternDefinitions compiled into Go

package ExternDefinitions

import (
	os "os"

	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	m_OrphanedResource "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/OrphanedResource"
	m_SimpleOrphaned "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphaned"
	m_SimpleOrphanedImpl "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedImpl"
	m_SimpleOrphanedTypes "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
	m_WrappedSimpleOrphanedService "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/test/WrappedSimpleOrphanedService"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_SimpleOrphanedTypes.Dummy__
var _ m_SimpleOrphanedImpl.Dummy__
var _ m_SimpleOrphaned.Dummy__
var _ m_OrphanedResource.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_WrappedSimpleOrphanedService.Dummy__

type Dummy__ struct{}

// Definition of class Default__
type Default__ struct {
	dummy byte
}

func New_Default___() *Default__ {
	_this := Default__{}

	return &_this
}

type CompanionStruct_Default___ struct {
}

var Companion_Default___ = CompanionStruct_Default___{}

func (_this *Default__) Equals(other *Default__) bool {
	return _this == other
}

func (_this *Default__) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*Default__)
	return ok && _this.Equals(other)
}

func (*Default__) String() string {
	return "ExternDefinitions.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) TestOrphanedStructure() {
	var _0_uninitializedStructure m_SimpleOrphanedTypes.OrphanedStructure
	_ = _0_uninitializedStructure
	_0_uninitializedStructure = m_SimpleOrphanedTypes.Companion_OrphanedStructure_.Create_OrphanedStructure_(m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_())
	var _1_initializedStructure m_SimpleOrphanedTypes.OrphanedStructure
	_ = _1_initializedStructure
	var _out0 m_SimpleOrphanedTypes.OrphanedStructure
	_ = _out0
	_out0 = Companion_Default___.InitializeOrphanedStructure(_0_uninitializedStructure)
	_1_initializedStructure = _out0
	if !(((_1_initializedStructure).Dtor_stringValue()).Is_Some()) {
		panic("test/ExternDefinitions.dfy(16,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !(_dafny.Companion_Sequence_.Equal(((_1_initializedStructure).Dtor_stringValue()).Dtor_value().(_dafny.Sequence), _dafny.SeqOfString("the extern MUST use Smithy-generated conversions to set this value in the native structure"))) {
		panic("test/ExternDefinitions.dfy(17,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestOrphanedResource() {
	var _0_resource *m_OrphanedResource.OrphanedResource
	_ = _0_resource
	var _nw0 *m_OrphanedResource.OrphanedResource = m_OrphanedResource.New_OrphanedResource_()
	_ = _nw0
	_nw0.Ctor__()
	_0_resource = _nw0
	var _1_ret m_Wrappers.Result
	_ = _1_ret
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = Companion_Default___.CallNativeOrphanedResource(_0_resource)
	_1_ret = _out0
	if !((_1_ret).Is_Success()) {
		panic("test/ExternDefinitions.dfy(25,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !((((_1_ret).Dtor_value().(m_SimpleOrphanedTypes.OrphanedResourceOperationOutput)).Dtor_someString()).Is_Some()) {
		panic("test/ExternDefinitions.dfy(26,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !(_dafny.Companion_Sequence_.Equal((((_1_ret).Dtor_value().(m_SimpleOrphanedTypes.OrphanedResourceOperationOutput)).Dtor_someString()).Dtor_value().(_dafny.Sequence), _dafny.SeqOfString("correct string"))) {
		panic("test/ExternDefinitions.dfy(27,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestOrphanedError() {
	var _0_error m_SimpleOrphanedTypes.Error
	_ = _0_error
	_0_error = m_SimpleOrphanedTypes.Companion_Error_.Create_OrphanedError_(_dafny.SeqOfString("TBD"))
	var _1_out__error m_SimpleOrphanedTypes.Error
	_ = _1_out__error
	var _out0 m_SimpleOrphanedTypes.Error
	_ = _out0
	_out0 = Companion_Default___.CallNativeOrphanedError(_0_error)
	_1_out__error = _out0
	if !((_1_out__error).Is_OrphanedError()) {
		panic("test/ExternDefinitions.dfy(35,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !(_dafny.Companion_Sequence_.Equal((_1_out__error).Dtor_message(), _dafny.SeqOfString("the extern MUST use Smithy-generated conversions to set this value in the native error"))) {
		panic("test/ExternDefinitions.dfy(36,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}

// End of class Default__
