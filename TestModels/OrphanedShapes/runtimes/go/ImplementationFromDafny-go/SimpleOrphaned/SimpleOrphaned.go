// Package SimpleOrphaned
// Dafny module SimpleOrphaned compiled into Go

package SimpleOrphaned

import (
	os "os"

	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	m_SimpleOrphanedImpl "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedImpl"
	m_SimpleOrphanedTypes "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphanedTypes"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_SimpleOrphanedTypes.Dummy__
var _ m_SimpleOrphanedImpl.Dummy__

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
	return "SimpleOrphaned.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) DefaultSimpleOrphanedConfig() m_SimpleOrphanedTypes.SimpleOrphanedConfig {
	return m_SimpleOrphanedTypes.Companion_SimpleOrphanedConfig_.Create_SimpleOrphanedConfig_(m_Wrappers.Companion_Option_.Create_None_())
}
func (_static *CompanionStruct_Default___) SimpleOrphaned(config m_SimpleOrphanedTypes.SimpleOrphanedConfig) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_client *SimpleOrphanedClient
	_ = _0_client
	var _nw0 *SimpleOrphanedClient = New_SimpleOrphanedClient_()
	_ = _nw0
	_nw0.Ctor__(m_SimpleOrphanedImpl.Companion_Config_.Create_Config_())
	_0_client = _nw0
	res = m_Wrappers.Companion_Result_.Create_Success_(_0_client)
	return res
	return res
}
func (_static *CompanionStruct_Default___) CreateSuccessOfClient(client m_SimpleOrphanedTypes.ISimpleOrphanedClient) m_Wrappers.Result {
	return m_Wrappers.Companion_Result_.Create_Success_(client)
}
func (_static *CompanionStruct_Default___) CreateFailureOfError(error_ m_SimpleOrphanedTypes.Error) m_Wrappers.Result {
	return m_Wrappers.Companion_Result_.Create_Failure_(error_)
}

// End of class Default__

// Definition of class SimpleOrphanedClient
type SimpleOrphanedClient struct {
	_config m_SimpleOrphanedImpl.Config
}

func New_SimpleOrphanedClient_() *SimpleOrphanedClient {
	_this := SimpleOrphanedClient{}

	_this._config = m_SimpleOrphanedImpl.Companion_Config_.Default()
	return &_this
}

type CompanionStruct_SimpleOrphanedClient_ struct {
}

var Companion_SimpleOrphanedClient_ = CompanionStruct_SimpleOrphanedClient_{}

func (_this *SimpleOrphanedClient) Equals(other *SimpleOrphanedClient) bool {
	return _this == other
}

func (_this *SimpleOrphanedClient) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*SimpleOrphanedClient)
	return ok && _this.Equals(other)
}

func (*SimpleOrphanedClient) String() string {
	return "SimpleOrphaned.SimpleOrphanedClient"
}

func Type_SimpleOrphanedClient_() _dafny.TypeDescriptor {
	return type_SimpleOrphanedClient_{}
}

type type_SimpleOrphanedClient_ struct {
}

func (_this type_SimpleOrphanedClient_) Default() interface{} {
	return (*SimpleOrphanedClient)(nil)
}

func (_this type_SimpleOrphanedClient_) String() string {
	return "SimpleOrphaned.SimpleOrphanedClient"
}
func (_this *SimpleOrphanedClient) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){m_SimpleOrphanedTypes.Companion_ISimpleOrphanedClient_.TraitID_}
}

var _ m_SimpleOrphanedTypes.ISimpleOrphanedClient = &SimpleOrphanedClient{}
var _ _dafny.TraitOffspring = &SimpleOrphanedClient{}

func (_this *SimpleOrphanedClient) Ctor__(config m_SimpleOrphanedImpl.Config) {
	{
		(_this)._config = config
	}
}
func (_this *SimpleOrphanedClient) Config() m_SimpleOrphanedImpl.Config {
	{
		return _this._config
	}
}

// End of class SimpleOrphanedClient
