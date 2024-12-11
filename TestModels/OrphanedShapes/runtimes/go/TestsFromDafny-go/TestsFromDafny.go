// Dafny program the_program compiled into Go
package main

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
	m_ExternDefinitions "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/test/ExternDefinitions"
	m_WrappedSimpleOrphanedService "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/test/WrappedSimpleOrphanedService"
	m_WrappedTest "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/test/WrappedTest"
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
var _ m_ExternDefinitions.Dummy__
var _ m_WrappedTest.Dummy__

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
	return "_module.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) Test____Main____(__noArgsParameter _dafny.Sequence) {
	var _0_success bool
	_ = _0_success
	_0_success = true
	_dafny.Print((_dafny.SeqOfString("WrappedTest.TestOrphanedStructure: ")).SetString())
	func() {
		defer func() {
			if r := recover(); r != nil {
				var _1_haltMessage = _dafny.SeqOfString(r.(string))
				{
					_dafny.Print((_dafny.SeqOfString("FAILED\n	")).SetString())
					_dafny.Print((_1_haltMessage).SetString())
					_dafny.Print((_dafny.SeqOfString("\n")).SetString())
					_0_success = false
				}
			}
		}()
		{
			m_WrappedTest.Companion_Default___.TestOrphanedStructure()
			{
				_dafny.Print((_dafny.SeqOfString("PASSED\n")).SetString())
			}
		}
	}()
	_dafny.Print((_dafny.SeqOfString("WrappedTest.TestOrphanedResource: ")).SetString())
	func() {
		defer func() {
			if r := recover(); r != nil {
				var _2_haltMessage = _dafny.SeqOfString(r.(string))
				{
					_dafny.Print((_dafny.SeqOfString("FAILED\n	")).SetString())
					_dafny.Print((_2_haltMessage).SetString())
					_dafny.Print((_dafny.SeqOfString("\n")).SetString())
					_0_success = false
				}
			}
		}()
		{
			m_WrappedTest.Companion_Default___.TestOrphanedResource()
			{
				_dafny.Print((_dafny.SeqOfString("PASSED\n")).SetString())
			}
		}
	}()
	_dafny.Print((_dafny.SeqOfString("WrappedTest.TestOrphanedError: ")).SetString())
	func() {
		defer func() {
			if r := recover(); r != nil {
				var _3_haltMessage = _dafny.SeqOfString(r.(string))
				{
					_dafny.Print((_dafny.SeqOfString("FAILED\n	")).SetString())
					_dafny.Print((_3_haltMessage).SetString())
					_dafny.Print((_dafny.SeqOfString("\n")).SetString())
					_0_success = false
				}
			}
		}()
		{
			m_WrappedTest.Companion_Default___.TestOrphanedError()
			{
				_dafny.Print((_dafny.SeqOfString("PASSED\n")).SetString())
			}
		}
	}()
	if !(_0_success) {
		panic("<stdin>(1,0): " + (_dafny.SeqOfString("Test failures occurred: see above.\n")).String())
	}
}

// End of class Default__
func main() {
	defer _dafny.CatchHalt()
	Companion_Default___.Test____Main____(_dafny.FromMainArguments(os.Args))
}
