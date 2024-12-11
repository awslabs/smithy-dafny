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
var _ m_OrphanedResource.Dummy__

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

func (_static *CompanionStruct_Default___) EmptyMethod() {
}

// End of class Default__
