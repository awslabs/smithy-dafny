// Package OrphanedResource
// Dafny module OrphanedResource compiled into Go

package OrphanedResource

import (
	os "os"

	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
	m_SimpleOrphaned "github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/SimpleOrphaned"
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
var _ m_SimpleOrphaned.Dummy__

type Dummy__ struct{}

// Definition of class OrphanedResource
type OrphanedResource struct {
	dummy byte
}

func New_OrphanedResource_() *OrphanedResource {
	_this := OrphanedResource{}

	return &_this
}

type CompanionStruct_OrphanedResource_ struct {
}

var Companion_OrphanedResource_ = CompanionStruct_OrphanedResource_{}

func (_this *OrphanedResource) Equals(other *OrphanedResource) bool {
	return _this == other
}

func (_this *OrphanedResource) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*OrphanedResource)
	return ok && _this.Equals(other)
}

func (*OrphanedResource) String() string {
	return "OrphanedResource.OrphanedResource"
}

func Type_OrphanedResource_() _dafny.TypeDescriptor {
	return type_OrphanedResource_{}
}

type type_OrphanedResource_ struct {
}

func (_this type_OrphanedResource_) Default() interface{} {
	return (*OrphanedResource)(nil)
}

func (_this type_OrphanedResource_) String() string {
	return "OrphanedResource.OrphanedResource"
}
func (_this *OrphanedResource) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){m_SimpleOrphanedTypes.Companion_IOrphanedResource_.TraitID_}
}

var _ m_SimpleOrphanedTypes.IOrphanedResource = &OrphanedResource{}
var _ _dafny.TraitOffspring = &OrphanedResource{}

func (_this *OrphanedResource) OrphanedResourceOperation(input m_SimpleOrphanedTypes.OrphanedResourceOperationInput) m_Wrappers.Result {
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_SimpleOrphanedTypes.Companion_IOrphanedResource_.OrphanedResourceOperation(_this, input)
	return _out1
}
func (_this *OrphanedResource) Ctor__() {
	{
	}
}
func (_this *OrphanedResource) OrphanedResourceOperation_k(input m_SimpleOrphanedTypes.OrphanedResourceOperationInput) m_Wrappers.Result {
	{
		var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_SimpleOrphanedTypes.Companion_OrphanedResourceOperationOutput_.Default())
		_ = output
		if (((input).Dtor_someString()).Is_Some()) && (_dafny.Companion_Sequence_.Equal(((input).Dtor_someString()).Dtor_value().(_dafny.Sequence), _dafny.SeqOfString("the extern MUST provide this string to the native resource's operation"))) {
			output = m_Wrappers.Companion_Result_.Create_Success_(m_SimpleOrphanedTypes.Companion_OrphanedResourceOperationOutput_.Create_OrphanedResourceOperationOutput_(m_Wrappers.Companion_Option_.Create_Some_(_dafny.SeqOfString("correct string"))))
			return output
		} else {
			output = m_Wrappers.Companion_Result_.Create_Failure_(m_SimpleOrphanedTypes.Companion_Error_.Create_OrphanedError_(_dafny.SeqOfString("incorrect string")))
			return output
		}
		return output
	}
}

// End of class OrphanedResource
