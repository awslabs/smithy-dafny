// Package SimpleOrphanedTypes
// Dafny module SimpleOrphanedTypes compiled into Go

package SimpleOrphanedTypes

import (
	os "os"

	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_StandardLibraryInterop.Dummy__

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
	return "SimpleOrphanedTypes.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) IsDummySubsetType(x _dafny.Int) bool {
	return (x).Sign() == 1
}

// End of class Default__

// Definition of datatype DafnyCallEvent
type DafnyCallEvent struct {
	Data_DafnyCallEvent_
}

func (_this DafnyCallEvent) Get_() Data_DafnyCallEvent_ {
	return _this.Data_DafnyCallEvent_
}

type Data_DafnyCallEvent_ interface {
	isDafnyCallEvent()
}

type CompanionStruct_DafnyCallEvent_ struct {
}

var Companion_DafnyCallEvent_ = CompanionStruct_DafnyCallEvent_{}

type DafnyCallEvent_DafnyCallEvent struct {
	Input  interface{}
	Output interface{}
}

func (DafnyCallEvent_DafnyCallEvent) isDafnyCallEvent() {}

func (CompanionStruct_DafnyCallEvent_) Create_DafnyCallEvent_(Input interface{}, Output interface{}) DafnyCallEvent {
	return DafnyCallEvent{DafnyCallEvent_DafnyCallEvent{Input, Output}}
}

func (_this DafnyCallEvent) Is_DafnyCallEvent() bool {
	_, ok := _this.Get_().(DafnyCallEvent_DafnyCallEvent)
	return ok
}

func (CompanionStruct_DafnyCallEvent_) Default(_default_I interface{}, _default_O interface{}) DafnyCallEvent {
	return Companion_DafnyCallEvent_.Create_DafnyCallEvent_(_default_I, _default_O)
}

func (_this DafnyCallEvent) Dtor_input() interface{} {
	return _this.Get_().(DafnyCallEvent_DafnyCallEvent).Input
}

func (_this DafnyCallEvent) Dtor_output() interface{} {
	return _this.Get_().(DafnyCallEvent_DafnyCallEvent).Output
}

func (_this DafnyCallEvent) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case DafnyCallEvent_DafnyCallEvent:
		{
			return "SimpleOrphanedTypes.DafnyCallEvent.DafnyCallEvent" + "(" + _dafny.String(data.Input) + ", " + _dafny.String(data.Output) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DafnyCallEvent) Equals(other DafnyCallEvent) bool {
	switch data1 := _this.Get_().(type) {
	case DafnyCallEvent_DafnyCallEvent:
		{
			data2, ok := other.Get_().(DafnyCallEvent_DafnyCallEvent)
			return ok && _dafny.AreEqual(data1.Input, data2.Input) && _dafny.AreEqual(data1.Output, data2.Output)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DafnyCallEvent) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DafnyCallEvent)
	return ok && _this.Equals(typed)
}

func Type_DafnyCallEvent_(Type_I_ _dafny.TypeDescriptor, Type_O_ _dafny.TypeDescriptor) _dafny.TypeDescriptor {
	return type_DafnyCallEvent_{Type_I_, Type_O_}
}

type type_DafnyCallEvent_ struct {
	Type_I_ _dafny.TypeDescriptor
	Type_O_ _dafny.TypeDescriptor
}

func (_this type_DafnyCallEvent_) Default() interface{} {
	Type_I_ := _this.Type_I_
	_ = Type_I_
	Type_O_ := _this.Type_O_
	_ = Type_O_
	return Companion_DafnyCallEvent_.Default(Type_I_.Default(), Type_O_.Default())
}

func (_this type_DafnyCallEvent_) String() string {
	return "SimpleOrphanedTypes.DafnyCallEvent"
}
func (_this DafnyCallEvent) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DafnyCallEvent{}

// End of datatype DafnyCallEvent

// Definition of datatype CreateOrphanedErrorInput
type CreateOrphanedErrorInput struct {
	Data_CreateOrphanedErrorInput_
}

func (_this CreateOrphanedErrorInput) Get_() Data_CreateOrphanedErrorInput_ {
	return _this.Data_CreateOrphanedErrorInput_
}

type Data_CreateOrphanedErrorInput_ interface {
	isCreateOrphanedErrorInput()
}

type CompanionStruct_CreateOrphanedErrorInput_ struct {
}

var Companion_CreateOrphanedErrorInput_ = CompanionStruct_CreateOrphanedErrorInput_{}

type CreateOrphanedErrorInput_CreateOrphanedErrorInput struct {
}

func (CreateOrphanedErrorInput_CreateOrphanedErrorInput) isCreateOrphanedErrorInput() {}

func (CompanionStruct_CreateOrphanedErrorInput_) Create_CreateOrphanedErrorInput_() CreateOrphanedErrorInput {
	return CreateOrphanedErrorInput{CreateOrphanedErrorInput_CreateOrphanedErrorInput{}}
}

func (_this CreateOrphanedErrorInput) Is_CreateOrphanedErrorInput() bool {
	_, ok := _this.Get_().(CreateOrphanedErrorInput_CreateOrphanedErrorInput)
	return ok
}

func (CompanionStruct_CreateOrphanedErrorInput_) Default() CreateOrphanedErrorInput {
	return Companion_CreateOrphanedErrorInput_.Create_CreateOrphanedErrorInput_()
}

func (_ CompanionStruct_CreateOrphanedErrorInput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedErrorInput_.Create_CreateOrphanedErrorInput_(), true
		default:
			return CreateOrphanedErrorInput{}, false
		}
	}
}

func (_this CreateOrphanedErrorInput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedErrorInput_CreateOrphanedErrorInput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedErrorInput.CreateOrphanedErrorInput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedErrorInput) Equals(other CreateOrphanedErrorInput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedErrorInput_CreateOrphanedErrorInput:
		{
			_, ok := other.Get_().(CreateOrphanedErrorInput_CreateOrphanedErrorInput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedErrorInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedErrorInput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedErrorInput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedErrorInput_{}
}

type type_CreateOrphanedErrorInput_ struct {
}

func (_this type_CreateOrphanedErrorInput_) Default() interface{} {
	return Companion_CreateOrphanedErrorInput_.Default()
}

func (_this type_CreateOrphanedErrorInput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedErrorInput"
}
func (_this CreateOrphanedErrorInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedErrorInput{}

// End of datatype CreateOrphanedErrorInput

// Definition of datatype CreateOrphanedErrorOutput
type CreateOrphanedErrorOutput struct {
	Data_CreateOrphanedErrorOutput_
}

func (_this CreateOrphanedErrorOutput) Get_() Data_CreateOrphanedErrorOutput_ {
	return _this.Data_CreateOrphanedErrorOutput_
}

type Data_CreateOrphanedErrorOutput_ interface {
	isCreateOrphanedErrorOutput()
}

type CompanionStruct_CreateOrphanedErrorOutput_ struct {
}

var Companion_CreateOrphanedErrorOutput_ = CompanionStruct_CreateOrphanedErrorOutput_{}

type CreateOrphanedErrorOutput_CreateOrphanedErrorOutput struct {
}

func (CreateOrphanedErrorOutput_CreateOrphanedErrorOutput) isCreateOrphanedErrorOutput() {}

func (CompanionStruct_CreateOrphanedErrorOutput_) Create_CreateOrphanedErrorOutput_() CreateOrphanedErrorOutput {
	return CreateOrphanedErrorOutput{CreateOrphanedErrorOutput_CreateOrphanedErrorOutput{}}
}

func (_this CreateOrphanedErrorOutput) Is_CreateOrphanedErrorOutput() bool {
	_, ok := _this.Get_().(CreateOrphanedErrorOutput_CreateOrphanedErrorOutput)
	return ok
}

func (CompanionStruct_CreateOrphanedErrorOutput_) Default() CreateOrphanedErrorOutput {
	return Companion_CreateOrphanedErrorOutput_.Create_CreateOrphanedErrorOutput_()
}

func (_ CompanionStruct_CreateOrphanedErrorOutput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedErrorOutput_.Create_CreateOrphanedErrorOutput_(), true
		default:
			return CreateOrphanedErrorOutput{}, false
		}
	}
}

func (_this CreateOrphanedErrorOutput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedErrorOutput_CreateOrphanedErrorOutput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedErrorOutput.CreateOrphanedErrorOutput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedErrorOutput) Equals(other CreateOrphanedErrorOutput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedErrorOutput_CreateOrphanedErrorOutput:
		{
			_, ok := other.Get_().(CreateOrphanedErrorOutput_CreateOrphanedErrorOutput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedErrorOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedErrorOutput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedErrorOutput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedErrorOutput_{}
}

type type_CreateOrphanedErrorOutput_ struct {
}

func (_this type_CreateOrphanedErrorOutput_) Default() interface{} {
	return Companion_CreateOrphanedErrorOutput_.Default()
}

func (_this type_CreateOrphanedErrorOutput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedErrorOutput"
}
func (_this CreateOrphanedErrorOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedErrorOutput{}

// End of datatype CreateOrphanedErrorOutput

// Definition of datatype CreateOrphanedResourceInput
type CreateOrphanedResourceInput struct {
	Data_CreateOrphanedResourceInput_
}

func (_this CreateOrphanedResourceInput) Get_() Data_CreateOrphanedResourceInput_ {
	return _this.Data_CreateOrphanedResourceInput_
}

type Data_CreateOrphanedResourceInput_ interface {
	isCreateOrphanedResourceInput()
}

type CompanionStruct_CreateOrphanedResourceInput_ struct {
}

var Companion_CreateOrphanedResourceInput_ = CompanionStruct_CreateOrphanedResourceInput_{}

type CreateOrphanedResourceInput_CreateOrphanedResourceInput struct {
}

func (CreateOrphanedResourceInput_CreateOrphanedResourceInput) isCreateOrphanedResourceInput() {}

func (CompanionStruct_CreateOrphanedResourceInput_) Create_CreateOrphanedResourceInput_() CreateOrphanedResourceInput {
	return CreateOrphanedResourceInput{CreateOrphanedResourceInput_CreateOrphanedResourceInput{}}
}

func (_this CreateOrphanedResourceInput) Is_CreateOrphanedResourceInput() bool {
	_, ok := _this.Get_().(CreateOrphanedResourceInput_CreateOrphanedResourceInput)
	return ok
}

func (CompanionStruct_CreateOrphanedResourceInput_) Default() CreateOrphanedResourceInput {
	return Companion_CreateOrphanedResourceInput_.Create_CreateOrphanedResourceInput_()
}

func (_ CompanionStruct_CreateOrphanedResourceInput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedResourceInput_.Create_CreateOrphanedResourceInput_(), true
		default:
			return CreateOrphanedResourceInput{}, false
		}
	}
}

func (_this CreateOrphanedResourceInput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedResourceInput_CreateOrphanedResourceInput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedResourceInput.CreateOrphanedResourceInput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedResourceInput) Equals(other CreateOrphanedResourceInput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedResourceInput_CreateOrphanedResourceInput:
		{
			_, ok := other.Get_().(CreateOrphanedResourceInput_CreateOrphanedResourceInput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedResourceInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedResourceInput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedResourceInput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedResourceInput_{}
}

type type_CreateOrphanedResourceInput_ struct {
}

func (_this type_CreateOrphanedResourceInput_) Default() interface{} {
	return Companion_CreateOrphanedResourceInput_.Default()
}

func (_this type_CreateOrphanedResourceInput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedResourceInput"
}
func (_this CreateOrphanedResourceInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedResourceInput{}

// End of datatype CreateOrphanedResourceInput

// Definition of datatype CreateOrphanedResourceOutput
type CreateOrphanedResourceOutput struct {
	Data_CreateOrphanedResourceOutput_
}

func (_this CreateOrphanedResourceOutput) Get_() Data_CreateOrphanedResourceOutput_ {
	return _this.Data_CreateOrphanedResourceOutput_
}

type Data_CreateOrphanedResourceOutput_ interface {
	isCreateOrphanedResourceOutput()
}

type CompanionStruct_CreateOrphanedResourceOutput_ struct {
}

var Companion_CreateOrphanedResourceOutput_ = CompanionStruct_CreateOrphanedResourceOutput_{}

type CreateOrphanedResourceOutput_CreateOrphanedResourceOutput struct {
}

func (CreateOrphanedResourceOutput_CreateOrphanedResourceOutput) isCreateOrphanedResourceOutput() {}

func (CompanionStruct_CreateOrphanedResourceOutput_) Create_CreateOrphanedResourceOutput_() CreateOrphanedResourceOutput {
	return CreateOrphanedResourceOutput{CreateOrphanedResourceOutput_CreateOrphanedResourceOutput{}}
}

func (_this CreateOrphanedResourceOutput) Is_CreateOrphanedResourceOutput() bool {
	_, ok := _this.Get_().(CreateOrphanedResourceOutput_CreateOrphanedResourceOutput)
	return ok
}

func (CompanionStruct_CreateOrphanedResourceOutput_) Default() CreateOrphanedResourceOutput {
	return Companion_CreateOrphanedResourceOutput_.Create_CreateOrphanedResourceOutput_()
}

func (_ CompanionStruct_CreateOrphanedResourceOutput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedResourceOutput_.Create_CreateOrphanedResourceOutput_(), true
		default:
			return CreateOrphanedResourceOutput{}, false
		}
	}
}

func (_this CreateOrphanedResourceOutput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedResourceOutput_CreateOrphanedResourceOutput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedResourceOutput.CreateOrphanedResourceOutput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedResourceOutput) Equals(other CreateOrphanedResourceOutput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedResourceOutput_CreateOrphanedResourceOutput:
		{
			_, ok := other.Get_().(CreateOrphanedResourceOutput_CreateOrphanedResourceOutput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedResourceOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedResourceOutput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedResourceOutput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedResourceOutput_{}
}

type type_CreateOrphanedResourceOutput_ struct {
}

func (_this type_CreateOrphanedResourceOutput_) Default() interface{} {
	return Companion_CreateOrphanedResourceOutput_.Default()
}

func (_this type_CreateOrphanedResourceOutput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedResourceOutput"
}
func (_this CreateOrphanedResourceOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedResourceOutput{}

// End of datatype CreateOrphanedResourceOutput

// Definition of datatype CreateOrphanedStructureInput
type CreateOrphanedStructureInput struct {
	Data_CreateOrphanedStructureInput_
}

func (_this CreateOrphanedStructureInput) Get_() Data_CreateOrphanedStructureInput_ {
	return _this.Data_CreateOrphanedStructureInput_
}

type Data_CreateOrphanedStructureInput_ interface {
	isCreateOrphanedStructureInput()
}

type CompanionStruct_CreateOrphanedStructureInput_ struct {
}

var Companion_CreateOrphanedStructureInput_ = CompanionStruct_CreateOrphanedStructureInput_{}

type CreateOrphanedStructureInput_CreateOrphanedStructureInput struct {
}

func (CreateOrphanedStructureInput_CreateOrphanedStructureInput) isCreateOrphanedStructureInput() {}

func (CompanionStruct_CreateOrphanedStructureInput_) Create_CreateOrphanedStructureInput_() CreateOrphanedStructureInput {
	return CreateOrphanedStructureInput{CreateOrphanedStructureInput_CreateOrphanedStructureInput{}}
}

func (_this CreateOrphanedStructureInput) Is_CreateOrphanedStructureInput() bool {
	_, ok := _this.Get_().(CreateOrphanedStructureInput_CreateOrphanedStructureInput)
	return ok
}

func (CompanionStruct_CreateOrphanedStructureInput_) Default() CreateOrphanedStructureInput {
	return Companion_CreateOrphanedStructureInput_.Create_CreateOrphanedStructureInput_()
}

func (_ CompanionStruct_CreateOrphanedStructureInput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedStructureInput_.Create_CreateOrphanedStructureInput_(), true
		default:
			return CreateOrphanedStructureInput{}, false
		}
	}
}

func (_this CreateOrphanedStructureInput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedStructureInput_CreateOrphanedStructureInput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedStructureInput.CreateOrphanedStructureInput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedStructureInput) Equals(other CreateOrphanedStructureInput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedStructureInput_CreateOrphanedStructureInput:
		{
			_, ok := other.Get_().(CreateOrphanedStructureInput_CreateOrphanedStructureInput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedStructureInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedStructureInput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedStructureInput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedStructureInput_{}
}

type type_CreateOrphanedStructureInput_ struct {
}

func (_this type_CreateOrphanedStructureInput_) Default() interface{} {
	return Companion_CreateOrphanedStructureInput_.Default()
}

func (_this type_CreateOrphanedStructureInput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedStructureInput"
}
func (_this CreateOrphanedStructureInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedStructureInput{}

// End of datatype CreateOrphanedStructureInput

// Definition of datatype CreateOrphanedStructureOutput
type CreateOrphanedStructureOutput struct {
	Data_CreateOrphanedStructureOutput_
}

func (_this CreateOrphanedStructureOutput) Get_() Data_CreateOrphanedStructureOutput_ {
	return _this.Data_CreateOrphanedStructureOutput_
}

type Data_CreateOrphanedStructureOutput_ interface {
	isCreateOrphanedStructureOutput()
}

type CompanionStruct_CreateOrphanedStructureOutput_ struct {
}

var Companion_CreateOrphanedStructureOutput_ = CompanionStruct_CreateOrphanedStructureOutput_{}

type CreateOrphanedStructureOutput_CreateOrphanedStructureOutput struct {
}

func (CreateOrphanedStructureOutput_CreateOrphanedStructureOutput) isCreateOrphanedStructureOutput() {
}

func (CompanionStruct_CreateOrphanedStructureOutput_) Create_CreateOrphanedStructureOutput_() CreateOrphanedStructureOutput {
	return CreateOrphanedStructureOutput{CreateOrphanedStructureOutput_CreateOrphanedStructureOutput{}}
}

func (_this CreateOrphanedStructureOutput) Is_CreateOrphanedStructureOutput() bool {
	_, ok := _this.Get_().(CreateOrphanedStructureOutput_CreateOrphanedStructureOutput)
	return ok
}

func (CompanionStruct_CreateOrphanedStructureOutput_) Default() CreateOrphanedStructureOutput {
	return Companion_CreateOrphanedStructureOutput_.Create_CreateOrphanedStructureOutput_()
}

func (_ CompanionStruct_CreateOrphanedStructureOutput_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_CreateOrphanedStructureOutput_.Create_CreateOrphanedStructureOutput_(), true
		default:
			return CreateOrphanedStructureOutput{}, false
		}
	}
}

func (_this CreateOrphanedStructureOutput) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case CreateOrphanedStructureOutput_CreateOrphanedStructureOutput:
		{
			return "SimpleOrphanedTypes.CreateOrphanedStructureOutput.CreateOrphanedStructureOutput"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this CreateOrphanedStructureOutput) Equals(other CreateOrphanedStructureOutput) bool {
	switch _this.Get_().(type) {
	case CreateOrphanedStructureOutput_CreateOrphanedStructureOutput:
		{
			_, ok := other.Get_().(CreateOrphanedStructureOutput_CreateOrphanedStructureOutput)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this CreateOrphanedStructureOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(CreateOrphanedStructureOutput)
	return ok && _this.Equals(typed)
}

func Type_CreateOrphanedStructureOutput_() _dafny.TypeDescriptor {
	return type_CreateOrphanedStructureOutput_{}
}

type type_CreateOrphanedStructureOutput_ struct {
}

func (_this type_CreateOrphanedStructureOutput_) Default() interface{} {
	return Companion_CreateOrphanedStructureOutput_.Default()
}

func (_this type_CreateOrphanedStructureOutput_) String() string {
	return "SimpleOrphanedTypes.CreateOrphanedStructureOutput"
}
func (_this CreateOrphanedStructureOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = CreateOrphanedStructureOutput{}

// End of datatype CreateOrphanedStructureOutput

// Definition of datatype OrphanedConfigShape
type OrphanedConfigShape struct {
	Data_OrphanedConfigShape_
}

func (_this OrphanedConfigShape) Get_() Data_OrphanedConfigShape_ {
	return _this.Data_OrphanedConfigShape_
}

type Data_OrphanedConfigShape_ interface {
	isOrphanedConfigShape()
}

type CompanionStruct_OrphanedConfigShape_ struct {
}

var Companion_OrphanedConfigShape_ = CompanionStruct_OrphanedConfigShape_{}

type OrphanedConfigShape_OrphanedConfigShape struct {
	StringMember m_Wrappers.Option
}

func (OrphanedConfigShape_OrphanedConfigShape) isOrphanedConfigShape() {}

func (CompanionStruct_OrphanedConfigShape_) Create_OrphanedConfigShape_(StringMember m_Wrappers.Option) OrphanedConfigShape {
	return OrphanedConfigShape{OrphanedConfigShape_OrphanedConfigShape{StringMember}}
}

func (_this OrphanedConfigShape) Is_OrphanedConfigShape() bool {
	_, ok := _this.Get_().(OrphanedConfigShape_OrphanedConfigShape)
	return ok
}

func (CompanionStruct_OrphanedConfigShape_) Default() OrphanedConfigShape {
	return Companion_OrphanedConfigShape_.Create_OrphanedConfigShape_(m_Wrappers.Companion_Option_.Default())
}

func (_this OrphanedConfigShape) Dtor_stringMember() m_Wrappers.Option {
	return _this.Get_().(OrphanedConfigShape_OrphanedConfigShape).StringMember
}

func (_this OrphanedConfigShape) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedConfigShape_OrphanedConfigShape:
		{
			return "SimpleOrphanedTypes.OrphanedConfigShape.OrphanedConfigShape" + "(" + _dafny.String(data.StringMember) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedConfigShape) Equals(other OrphanedConfigShape) bool {
	switch data1 := _this.Get_().(type) {
	case OrphanedConfigShape_OrphanedConfigShape:
		{
			data2, ok := other.Get_().(OrphanedConfigShape_OrphanedConfigShape)
			return ok && data1.StringMember.Equals(data2.StringMember)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedConfigShape) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedConfigShape)
	return ok && _this.Equals(typed)
}

func Type_OrphanedConfigShape_() _dafny.TypeDescriptor {
	return type_OrphanedConfigShape_{}
}

type type_OrphanedConfigShape_ struct {
}

func (_this type_OrphanedConfigShape_) Default() interface{} {
	return Companion_OrphanedConfigShape_.Default()
}

func (_this type_OrphanedConfigShape_) String() string {
	return "SimpleOrphanedTypes.OrphanedConfigShape"
}
func (_this OrphanedConfigShape) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedConfigShape{}

// End of datatype OrphanedConfigShape

// Definition of datatype OrphanedResourceOperationInput
type OrphanedResourceOperationInput struct {
	Data_OrphanedResourceOperationInput_
}

func (_this OrphanedResourceOperationInput) Get_() Data_OrphanedResourceOperationInput_ {
	return _this.Data_OrphanedResourceOperationInput_
}

type Data_OrphanedResourceOperationInput_ interface {
	isOrphanedResourceOperationInput()
}

type CompanionStruct_OrphanedResourceOperationInput_ struct {
}

var Companion_OrphanedResourceOperationInput_ = CompanionStruct_OrphanedResourceOperationInput_{}

type OrphanedResourceOperationInput_OrphanedResourceOperationInput struct {
	SomeString m_Wrappers.Option
}

func (OrphanedResourceOperationInput_OrphanedResourceOperationInput) isOrphanedResourceOperationInput() {
}

func (CompanionStruct_OrphanedResourceOperationInput_) Create_OrphanedResourceOperationInput_(SomeString m_Wrappers.Option) OrphanedResourceOperationInput {
	return OrphanedResourceOperationInput{OrphanedResourceOperationInput_OrphanedResourceOperationInput{SomeString}}
}

func (_this OrphanedResourceOperationInput) Is_OrphanedResourceOperationInput() bool {
	_, ok := _this.Get_().(OrphanedResourceOperationInput_OrphanedResourceOperationInput)
	return ok
}

func (CompanionStruct_OrphanedResourceOperationInput_) Default() OrphanedResourceOperationInput {
	return Companion_OrphanedResourceOperationInput_.Create_OrphanedResourceOperationInput_(m_Wrappers.Companion_Option_.Default())
}

func (_this OrphanedResourceOperationInput) Dtor_someString() m_Wrappers.Option {
	return _this.Get_().(OrphanedResourceOperationInput_OrphanedResourceOperationInput).SomeString
}

func (_this OrphanedResourceOperationInput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedResourceOperationInput_OrphanedResourceOperationInput:
		{
			return "SimpleOrphanedTypes.OrphanedResourceOperationInput.OrphanedResourceOperationInput" + "(" + _dafny.String(data.SomeString) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedResourceOperationInput) Equals(other OrphanedResourceOperationInput) bool {
	switch data1 := _this.Get_().(type) {
	case OrphanedResourceOperationInput_OrphanedResourceOperationInput:
		{
			data2, ok := other.Get_().(OrphanedResourceOperationInput_OrphanedResourceOperationInput)
			return ok && data1.SomeString.Equals(data2.SomeString)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedResourceOperationInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedResourceOperationInput)
	return ok && _this.Equals(typed)
}

func Type_OrphanedResourceOperationInput_() _dafny.TypeDescriptor {
	return type_OrphanedResourceOperationInput_{}
}

type type_OrphanedResourceOperationInput_ struct {
}

func (_this type_OrphanedResourceOperationInput_) Default() interface{} {
	return Companion_OrphanedResourceOperationInput_.Default()
}

func (_this type_OrphanedResourceOperationInput_) String() string {
	return "SimpleOrphanedTypes.OrphanedResourceOperationInput"
}
func (_this OrphanedResourceOperationInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedResourceOperationInput{}

// End of datatype OrphanedResourceOperationInput

// Definition of datatype OrphanedResourceOperationOutput
type OrphanedResourceOperationOutput struct {
	Data_OrphanedResourceOperationOutput_
}

func (_this OrphanedResourceOperationOutput) Get_() Data_OrphanedResourceOperationOutput_ {
	return _this.Data_OrphanedResourceOperationOutput_
}

type Data_OrphanedResourceOperationOutput_ interface {
	isOrphanedResourceOperationOutput()
}

type CompanionStruct_OrphanedResourceOperationOutput_ struct {
}

var Companion_OrphanedResourceOperationOutput_ = CompanionStruct_OrphanedResourceOperationOutput_{}

type OrphanedResourceOperationOutput_OrphanedResourceOperationOutput struct {
	SomeString m_Wrappers.Option
}

func (OrphanedResourceOperationOutput_OrphanedResourceOperationOutput) isOrphanedResourceOperationOutput() {
}

func (CompanionStruct_OrphanedResourceOperationOutput_) Create_OrphanedResourceOperationOutput_(SomeString m_Wrappers.Option) OrphanedResourceOperationOutput {
	return OrphanedResourceOperationOutput{OrphanedResourceOperationOutput_OrphanedResourceOperationOutput{SomeString}}
}

func (_this OrphanedResourceOperationOutput) Is_OrphanedResourceOperationOutput() bool {
	_, ok := _this.Get_().(OrphanedResourceOperationOutput_OrphanedResourceOperationOutput)
	return ok
}

func (CompanionStruct_OrphanedResourceOperationOutput_) Default() OrphanedResourceOperationOutput {
	return Companion_OrphanedResourceOperationOutput_.Create_OrphanedResourceOperationOutput_(m_Wrappers.Companion_Option_.Default())
}

func (_this OrphanedResourceOperationOutput) Dtor_someString() m_Wrappers.Option {
	return _this.Get_().(OrphanedResourceOperationOutput_OrphanedResourceOperationOutput).SomeString
}

func (_this OrphanedResourceOperationOutput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedResourceOperationOutput_OrphanedResourceOperationOutput:
		{
			return "SimpleOrphanedTypes.OrphanedResourceOperationOutput.OrphanedResourceOperationOutput" + "(" + _dafny.String(data.SomeString) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedResourceOperationOutput) Equals(other OrphanedResourceOperationOutput) bool {
	switch data1 := _this.Get_().(type) {
	case OrphanedResourceOperationOutput_OrphanedResourceOperationOutput:
		{
			data2, ok := other.Get_().(OrphanedResourceOperationOutput_OrphanedResourceOperationOutput)
			return ok && data1.SomeString.Equals(data2.SomeString)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedResourceOperationOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedResourceOperationOutput)
	return ok && _this.Equals(typed)
}

func Type_OrphanedResourceOperationOutput_() _dafny.TypeDescriptor {
	return type_OrphanedResourceOperationOutput_{}
}

type type_OrphanedResourceOperationOutput_ struct {
}

func (_this type_OrphanedResourceOperationOutput_) Default() interface{} {
	return Companion_OrphanedResourceOperationOutput_.Default()
}

func (_this type_OrphanedResourceOperationOutput_) String() string {
	return "SimpleOrphanedTypes.OrphanedResourceOperationOutput"
}
func (_this OrphanedResourceOperationOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedResourceOperationOutput{}

// End of datatype OrphanedResourceOperationOutput

// Definition of class IOrphanedResourceCallHistory
type IOrphanedResourceCallHistory struct {
	dummy byte
}

func New_IOrphanedResourceCallHistory_() *IOrphanedResourceCallHistory {
	_this := IOrphanedResourceCallHistory{}

	return &_this
}

type CompanionStruct_IOrphanedResourceCallHistory_ struct {
}

var Companion_IOrphanedResourceCallHistory_ = CompanionStruct_IOrphanedResourceCallHistory_{}

func (_this *IOrphanedResourceCallHistory) Equals(other *IOrphanedResourceCallHistory) bool {
	return _this == other
}

func (_this *IOrphanedResourceCallHistory) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*IOrphanedResourceCallHistory)
	return ok && _this.Equals(other)
}

func (*IOrphanedResourceCallHistory) String() string {
	return "SimpleOrphanedTypes.IOrphanedResourceCallHistory"
}

func Type_IOrphanedResourceCallHistory_() _dafny.TypeDescriptor {
	return type_IOrphanedResourceCallHistory_{}
}

type type_IOrphanedResourceCallHistory_ struct {
}

func (_this type_IOrphanedResourceCallHistory_) Default() interface{} {
	return (*IOrphanedResourceCallHistory)(nil)
}

func (_this type_IOrphanedResourceCallHistory_) String() string {
	return "SimpleOrphanedTypes.IOrphanedResourceCallHistory"
}
func (_this *IOrphanedResourceCallHistory) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &IOrphanedResourceCallHistory{}

// End of class IOrphanedResourceCallHistory

// Definition of trait IOrphanedResource
type IOrphanedResource interface {
	String() string
	OrphanedResourceOperation(input OrphanedResourceOperationInput) m_Wrappers.Result
	OrphanedResourceOperation_k(input OrphanedResourceOperationInput) m_Wrappers.Result
}

func (_static *CompanionStruct_IOrphanedResource_) OrphanedResourceOperation(_this IOrphanedResource, input OrphanedResourceOperationInput) m_Wrappers.Result {
	{
		var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_OrphanedResourceOperationOutput_.Default())
		_ = output
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = (_this).OrphanedResourceOperation_k(input)
		output = _out0
		return output
	}
}

type CompanionStruct_IOrphanedResource_ struct {
	TraitID_ *_dafny.TraitID
}

var Companion_IOrphanedResource_ = CompanionStruct_IOrphanedResource_{
	TraitID_: &_dafny.TraitID{},
}

func (CompanionStruct_IOrphanedResource_) CastTo_(x interface{}) IOrphanedResource {
	var t IOrphanedResource
	t, _ = x.(IOrphanedResource)
	return t
}

// End of trait IOrphanedResource

// Definition of datatype OrphanedStructure
type OrphanedStructure struct {
	Data_OrphanedStructure_
}

func (_this OrphanedStructure) Get_() Data_OrphanedStructure_ {
	return _this.Data_OrphanedStructure_
}

type Data_OrphanedStructure_ interface {
	isOrphanedStructure()
}

type CompanionStruct_OrphanedStructure_ struct {
}

var Companion_OrphanedStructure_ = CompanionStruct_OrphanedStructure_{}

type OrphanedStructure_OrphanedStructure struct {
	BlobValue    m_Wrappers.Option
	BooleanValue m_Wrappers.Option
	StringValue  m_Wrappers.Option
	IntegerValue m_Wrappers.Option
	LongValue    m_Wrappers.Option
	UnionValue   m_Wrappers.Option
	EnumValue    m_Wrappers.Option
	MapValue     m_Wrappers.Option
	ListValue    m_Wrappers.Option
}

func (OrphanedStructure_OrphanedStructure) isOrphanedStructure() {}

func (CompanionStruct_OrphanedStructure_) Create_OrphanedStructure_(BlobValue m_Wrappers.Option, BooleanValue m_Wrappers.Option, StringValue m_Wrappers.Option, IntegerValue m_Wrappers.Option, LongValue m_Wrappers.Option, UnionValue m_Wrappers.Option, EnumValue m_Wrappers.Option, MapValue m_Wrappers.Option, ListValue m_Wrappers.Option) OrphanedStructure {
	return OrphanedStructure{OrphanedStructure_OrphanedStructure{BlobValue, BooleanValue, StringValue, IntegerValue, LongValue, UnionValue, EnumValue, MapValue, ListValue}}
}

func (_this OrphanedStructure) Is_OrphanedStructure() bool {
	_, ok := _this.Get_().(OrphanedStructure_OrphanedStructure)
	return ok
}

func (CompanionStruct_OrphanedStructure_) Default() OrphanedStructure {
	return Companion_OrphanedStructure_.Create_OrphanedStructure_(m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default())
}

func (_this OrphanedStructure) Dtor_blobValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).BlobValue
}

func (_this OrphanedStructure) Dtor_booleanValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).BooleanValue
}

func (_this OrphanedStructure) Dtor_stringValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).StringValue
}

func (_this OrphanedStructure) Dtor_integerValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).IntegerValue
}

func (_this OrphanedStructure) Dtor_longValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).LongValue
}

func (_this OrphanedStructure) Dtor_unionValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).UnionValue
}

func (_this OrphanedStructure) Dtor_enumValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).EnumValue
}

func (_this OrphanedStructure) Dtor_mapValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).MapValue
}

func (_this OrphanedStructure) Dtor_listValue() m_Wrappers.Option {
	return _this.Get_().(OrphanedStructure_OrphanedStructure).ListValue
}

func (_this OrphanedStructure) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedStructure_OrphanedStructure:
		{
			return "SimpleOrphanedTypes.OrphanedStructure.OrphanedStructure" + "(" + _dafny.String(data.BlobValue) + ", " + _dafny.String(data.BooleanValue) + ", " + _dafny.String(data.StringValue) + ", " + _dafny.String(data.IntegerValue) + ", " + _dafny.String(data.LongValue) + ", " + _dafny.String(data.UnionValue) + ", " + _dafny.String(data.EnumValue) + ", " + _dafny.String(data.MapValue) + ", " + _dafny.String(data.ListValue) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedStructure) Equals(other OrphanedStructure) bool {
	switch data1 := _this.Get_().(type) {
	case OrphanedStructure_OrphanedStructure:
		{
			data2, ok := other.Get_().(OrphanedStructure_OrphanedStructure)
			return ok && data1.BlobValue.Equals(data2.BlobValue) && data1.BooleanValue.Equals(data2.BooleanValue) && data1.StringValue.Equals(data2.StringValue) && data1.IntegerValue.Equals(data2.IntegerValue) && data1.LongValue.Equals(data2.LongValue) && data1.UnionValue.Equals(data2.UnionValue) && data1.EnumValue.Equals(data2.EnumValue) && data1.MapValue.Equals(data2.MapValue) && data1.ListValue.Equals(data2.ListValue)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedStructure) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedStructure)
	return ok && _this.Equals(typed)
}

func Type_OrphanedStructure_() _dafny.TypeDescriptor {
	return type_OrphanedStructure_{}
}

type type_OrphanedStructure_ struct {
}

func (_this type_OrphanedStructure_) Default() interface{} {
	return Companion_OrphanedStructure_.Default()
}

func (_this type_OrphanedStructure_) String() string {
	return "SimpleOrphanedTypes.OrphanedStructure"
}
func (_this OrphanedStructure) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedStructure{}

// End of datatype OrphanedStructure

// Definition of datatype OrphanedUnion
type OrphanedUnion struct {
	Data_OrphanedUnion_
}

func (_this OrphanedUnion) Get_() Data_OrphanedUnion_ {
	return _this.Data_OrphanedUnion_
}

type Data_OrphanedUnion_ interface {
	isOrphanedUnion()
}

type CompanionStruct_OrphanedUnion_ struct {
}

var Companion_OrphanedUnion_ = CompanionStruct_OrphanedUnion_{}

type OrphanedUnion_integerValue struct {
	IntegerValue int32
}

func (OrphanedUnion_integerValue) isOrphanedUnion() {}

func (CompanionStruct_OrphanedUnion_) Create_integerValue_(IntegerValue int32) OrphanedUnion {
	return OrphanedUnion{OrphanedUnion_integerValue{IntegerValue}}
}

func (_this OrphanedUnion) Is_integerValue() bool {
	_, ok := _this.Get_().(OrphanedUnion_integerValue)
	return ok
}

type OrphanedUnion_stringValue struct {
	StringValue _dafny.Sequence
}

func (OrphanedUnion_stringValue) isOrphanedUnion() {}

func (CompanionStruct_OrphanedUnion_) Create_stringValue_(StringValue _dafny.Sequence) OrphanedUnion {
	return OrphanedUnion{OrphanedUnion_stringValue{StringValue}}
}

func (_this OrphanedUnion) Is_stringValue() bool {
	_, ok := _this.Get_().(OrphanedUnion_stringValue)
	return ok
}

func (CompanionStruct_OrphanedUnion_) Default() OrphanedUnion {
	return Companion_OrphanedUnion_.Create_integerValue_(int32(0))
}

func (_this OrphanedUnion) Dtor_integerValue() int32 {
	return _this.Get_().(OrphanedUnion_integerValue).IntegerValue
}

func (_this OrphanedUnion) Dtor_stringValue() _dafny.Sequence {
	return _this.Get_().(OrphanedUnion_stringValue).StringValue
}

func (_this OrphanedUnion) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedUnion_integerValue:
		{
			return "SimpleOrphanedTypes.OrphanedUnion.integerValue" + "(" + _dafny.String(data.IntegerValue) + ")"
		}
	case OrphanedUnion_stringValue:
		{
			return "SimpleOrphanedTypes.OrphanedUnion.stringValue" + "(" + _dafny.String(data.StringValue) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedUnion) Equals(other OrphanedUnion) bool {
	switch data1 := _this.Get_().(type) {
	case OrphanedUnion_integerValue:
		{
			data2, ok := other.Get_().(OrphanedUnion_integerValue)
			return ok && data1.IntegerValue == data2.IntegerValue
		}
	case OrphanedUnion_stringValue:
		{
			data2, ok := other.Get_().(OrphanedUnion_stringValue)
			return ok && data1.StringValue.Equals(data2.StringValue)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedUnion) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedUnion)
	return ok && _this.Equals(typed)
}

func Type_OrphanedUnion_() _dafny.TypeDescriptor {
	return type_OrphanedUnion_{}
}

type type_OrphanedUnion_ struct {
}

func (_this type_OrphanedUnion_) Default() interface{} {
	return Companion_OrphanedUnion_.Default()
}

func (_this type_OrphanedUnion_) String() string {
	return "SimpleOrphanedTypes.OrphanedUnion"
}
func (_this OrphanedUnion) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedUnion{}

// End of datatype OrphanedUnion

// Definition of datatype OrphanedV1Enum
type OrphanedV1Enum struct {
	Data_OrphanedV1Enum_
}

func (_this OrphanedV1Enum) Get_() Data_OrphanedV1Enum_ {
	return _this.Data_OrphanedV1Enum_
}

type Data_OrphanedV1Enum_ interface {
	isOrphanedV1Enum()
}

type CompanionStruct_OrphanedV1Enum_ struct {
}

var Companion_OrphanedV1Enum_ = CompanionStruct_OrphanedV1Enum_{}

type OrphanedV1Enum_FIRST struct {
}

func (OrphanedV1Enum_FIRST) isOrphanedV1Enum() {}

func (CompanionStruct_OrphanedV1Enum_) Create_FIRST_() OrphanedV1Enum {
	return OrphanedV1Enum{OrphanedV1Enum_FIRST{}}
}

func (_this OrphanedV1Enum) Is_FIRST() bool {
	_, ok := _this.Get_().(OrphanedV1Enum_FIRST)
	return ok
}

type OrphanedV1Enum_SECOND struct {
}

func (OrphanedV1Enum_SECOND) isOrphanedV1Enum() {}

func (CompanionStruct_OrphanedV1Enum_) Create_SECOND_() OrphanedV1Enum {
	return OrphanedV1Enum{OrphanedV1Enum_SECOND{}}
}

func (_this OrphanedV1Enum) Is_SECOND() bool {
	_, ok := _this.Get_().(OrphanedV1Enum_SECOND)
	return ok
}

type OrphanedV1Enum_THIRD struct {
}

func (OrphanedV1Enum_THIRD) isOrphanedV1Enum() {}

func (CompanionStruct_OrphanedV1Enum_) Create_THIRD_() OrphanedV1Enum {
	return OrphanedV1Enum{OrphanedV1Enum_THIRD{}}
}

func (_this OrphanedV1Enum) Is_THIRD() bool {
	_, ok := _this.Get_().(OrphanedV1Enum_THIRD)
	return ok
}

func (CompanionStruct_OrphanedV1Enum_) Default() OrphanedV1Enum {
	return Companion_OrphanedV1Enum_.Create_FIRST_()
}

func (_ CompanionStruct_OrphanedV1Enum_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_OrphanedV1Enum_.Create_FIRST_(), true
		case 1:
			return Companion_OrphanedV1Enum_.Create_SECOND_(), true
		case 2:
			return Companion_OrphanedV1Enum_.Create_THIRD_(), true
		default:
			return OrphanedV1Enum{}, false
		}
	}
}

func (_this OrphanedV1Enum) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case OrphanedV1Enum_FIRST:
		{
			return "SimpleOrphanedTypes.OrphanedV1Enum.FIRST"
		}
	case OrphanedV1Enum_SECOND:
		{
			return "SimpleOrphanedTypes.OrphanedV1Enum.SECOND"
		}
	case OrphanedV1Enum_THIRD:
		{
			return "SimpleOrphanedTypes.OrphanedV1Enum.THIRD"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this OrphanedV1Enum) Equals(other OrphanedV1Enum) bool {
	switch _this.Get_().(type) {
	case OrphanedV1Enum_FIRST:
		{
			_, ok := other.Get_().(OrphanedV1Enum_FIRST)
			return ok
		}
	case OrphanedV1Enum_SECOND:
		{
			_, ok := other.Get_().(OrphanedV1Enum_SECOND)
			return ok
		}
	case OrphanedV1Enum_THIRD:
		{
			_, ok := other.Get_().(OrphanedV1Enum_THIRD)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this OrphanedV1Enum) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(OrphanedV1Enum)
	return ok && _this.Equals(typed)
}

func Type_OrphanedV1Enum_() _dafny.TypeDescriptor {
	return type_OrphanedV1Enum_{}
}

type type_OrphanedV1Enum_ struct {
}

func (_this type_OrphanedV1Enum_) Default() interface{} {
	return Companion_OrphanedV1Enum_.Default()
}

func (_this type_OrphanedV1Enum_) String() string {
	return "SimpleOrphanedTypes.OrphanedV1Enum"
}
func (_this OrphanedV1Enum) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = OrphanedV1Enum{}

// End of datatype OrphanedV1Enum

// Definition of class ISimpleOrphanedClientCallHistory
type ISimpleOrphanedClientCallHistory struct {
	dummy byte
}

func New_ISimpleOrphanedClientCallHistory_() *ISimpleOrphanedClientCallHistory {
	_this := ISimpleOrphanedClientCallHistory{}

	return &_this
}

type CompanionStruct_ISimpleOrphanedClientCallHistory_ struct {
}

var Companion_ISimpleOrphanedClientCallHistory_ = CompanionStruct_ISimpleOrphanedClientCallHistory_{}

func (_this *ISimpleOrphanedClientCallHistory) Equals(other *ISimpleOrphanedClientCallHistory) bool {
	return _this == other
}

func (_this *ISimpleOrphanedClientCallHistory) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*ISimpleOrphanedClientCallHistory)
	return ok && _this.Equals(other)
}

func (*ISimpleOrphanedClientCallHistory) String() string {
	return "SimpleOrphanedTypes.ISimpleOrphanedClientCallHistory"
}

func Type_ISimpleOrphanedClientCallHistory_() _dafny.TypeDescriptor {
	return type_ISimpleOrphanedClientCallHistory_{}
}

type type_ISimpleOrphanedClientCallHistory_ struct {
}

func (_this type_ISimpleOrphanedClientCallHistory_) Default() interface{} {
	return (*ISimpleOrphanedClientCallHistory)(nil)
}

func (_this type_ISimpleOrphanedClientCallHistory_) String() string {
	return "SimpleOrphanedTypes.ISimpleOrphanedClientCallHistory"
}
func (_this *ISimpleOrphanedClientCallHistory) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &ISimpleOrphanedClientCallHistory{}

// End of class ISimpleOrphanedClientCallHistory

// Definition of trait ISimpleOrphanedClient
type ISimpleOrphanedClient interface {
	String() string
}
type CompanionStruct_ISimpleOrphanedClient_ struct {
	TraitID_ *_dafny.TraitID
}

var Companion_ISimpleOrphanedClient_ = CompanionStruct_ISimpleOrphanedClient_{
	TraitID_: &_dafny.TraitID{},
}

func (CompanionStruct_ISimpleOrphanedClient_) CastTo_(x interface{}) ISimpleOrphanedClient {
	var t ISimpleOrphanedClient
	t, _ = x.(ISimpleOrphanedClient)
	return t
}

// End of trait ISimpleOrphanedClient

// Definition of datatype SimpleOrphanedConfig
type SimpleOrphanedConfig struct {
	Data_SimpleOrphanedConfig_
}

func (_this SimpleOrphanedConfig) Get_() Data_SimpleOrphanedConfig_ {
	return _this.Data_SimpleOrphanedConfig_
}

type Data_SimpleOrphanedConfig_ interface {
	isSimpleOrphanedConfig()
}

type CompanionStruct_SimpleOrphanedConfig_ struct {
}

var Companion_SimpleOrphanedConfig_ = CompanionStruct_SimpleOrphanedConfig_{}

type SimpleOrphanedConfig_SimpleOrphanedConfig struct {
	StructureMember m_Wrappers.Option
}

func (SimpleOrphanedConfig_SimpleOrphanedConfig) isSimpleOrphanedConfig() {}

func (CompanionStruct_SimpleOrphanedConfig_) Create_SimpleOrphanedConfig_(StructureMember m_Wrappers.Option) SimpleOrphanedConfig {
	return SimpleOrphanedConfig{SimpleOrphanedConfig_SimpleOrphanedConfig{StructureMember}}
}

func (_this SimpleOrphanedConfig) Is_SimpleOrphanedConfig() bool {
	_, ok := _this.Get_().(SimpleOrphanedConfig_SimpleOrphanedConfig)
	return ok
}

func (CompanionStruct_SimpleOrphanedConfig_) Default() SimpleOrphanedConfig {
	return Companion_SimpleOrphanedConfig_.Create_SimpleOrphanedConfig_(m_Wrappers.Companion_Option_.Default())
}

func (_this SimpleOrphanedConfig) Dtor_structureMember() m_Wrappers.Option {
	return _this.Get_().(SimpleOrphanedConfig_SimpleOrphanedConfig).StructureMember
}

func (_this SimpleOrphanedConfig) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case SimpleOrphanedConfig_SimpleOrphanedConfig:
		{
			return "SimpleOrphanedTypes.SimpleOrphanedConfig.SimpleOrphanedConfig" + "(" + _dafny.String(data.StructureMember) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this SimpleOrphanedConfig) Equals(other SimpleOrphanedConfig) bool {
	switch data1 := _this.Get_().(type) {
	case SimpleOrphanedConfig_SimpleOrphanedConfig:
		{
			data2, ok := other.Get_().(SimpleOrphanedConfig_SimpleOrphanedConfig)
			return ok && data1.StructureMember.Equals(data2.StructureMember)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this SimpleOrphanedConfig) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(SimpleOrphanedConfig)
	return ok && _this.Equals(typed)
}

func Type_SimpleOrphanedConfig_() _dafny.TypeDescriptor {
	return type_SimpleOrphanedConfig_{}
}

type type_SimpleOrphanedConfig_ struct {
}

func (_this type_SimpleOrphanedConfig_) Default() interface{} {
	return Companion_SimpleOrphanedConfig_.Default()
}

func (_this type_SimpleOrphanedConfig_) String() string {
	return "SimpleOrphanedTypes.SimpleOrphanedConfig"
}
func (_this SimpleOrphanedConfig) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = SimpleOrphanedConfig{}

// End of datatype SimpleOrphanedConfig

// Definition of datatype Error
type Error struct {
	Data_Error_
}

func (_this Error) Get_() Data_Error_ {
	return _this.Data_Error_
}

type Data_Error_ interface {
	isError()
}

type CompanionStruct_Error_ struct {
}

var Companion_Error_ = CompanionStruct_Error_{}

type Error_OrphanedError struct {
	Message _dafny.Sequence
}

func (Error_OrphanedError) isError() {}

func (CompanionStruct_Error_) Create_OrphanedError_(Message _dafny.Sequence) Error {
	return Error{Error_OrphanedError{Message}}
}

func (_this Error) Is_OrphanedError() bool {
	_, ok := _this.Get_().(Error_OrphanedError)
	return ok
}

type Error_CollectionOfErrors struct {
	List    _dafny.Sequence
	Message _dafny.Sequence
}

func (Error_CollectionOfErrors) isError() {}

func (CompanionStruct_Error_) Create_CollectionOfErrors_(List _dafny.Sequence, Message _dafny.Sequence) Error {
	return Error{Error_CollectionOfErrors{List, Message}}
}

func (_this Error) Is_CollectionOfErrors() bool {
	_, ok := _this.Get_().(Error_CollectionOfErrors)
	return ok
}

type Error_Opaque struct {
	Obj interface{}
}

func (Error_Opaque) isError() {}

func (CompanionStruct_Error_) Create_Opaque_(Obj interface{}) Error {
	return Error{Error_Opaque{Obj}}
}

func (_this Error) Is_Opaque() bool {
	_, ok := _this.Get_().(Error_Opaque)
	return ok
}

type Error_OpaqueWithText struct {
	Obj        interface{}
	ObjMessage _dafny.Sequence
}

func (Error_OpaqueWithText) isError() {}

func (CompanionStruct_Error_) Create_OpaqueWithText_(Obj interface{}, ObjMessage _dafny.Sequence) Error {
	return Error{Error_OpaqueWithText{Obj, ObjMessage}}
}

func (_this Error) Is_OpaqueWithText() bool {
	_, ok := _this.Get_().(Error_OpaqueWithText)
	return ok
}

func (CompanionStruct_Error_) Default() Error {
	return Companion_Error_.Create_OrphanedError_(_dafny.EmptySeq.SetString())
}

func (_this Error) Dtor_message() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case Error_OrphanedError:
		return data.Message
	default:
		return data.(Error_CollectionOfErrors).Message
	}
}

func (_this Error) Dtor_list() _dafny.Sequence {
	return _this.Get_().(Error_CollectionOfErrors).List
}

func (_this Error) Dtor_obj() interface{} {
	switch data := _this.Get_().(type) {
	case Error_Opaque:
		return data.Obj
	default:
		return data.(Error_OpaqueWithText).Obj
	}
}

func (_this Error) Dtor_objMessage() _dafny.Sequence {
	return _this.Get_().(Error_OpaqueWithText).ObjMessage
}

func (_this Error) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case Error_OrphanedError:
		{
			return "SimpleOrphanedTypes.Error.OrphanedError" + "(" + _dafny.String(data.Message) + ")"
		}
	case Error_CollectionOfErrors:
		{
			return "SimpleOrphanedTypes.Error.CollectionOfErrors" + "(" + _dafny.String(data.List) + ", " + _dafny.String(data.Message) + ")"
		}
	case Error_Opaque:
		{
			return "SimpleOrphanedTypes.Error.Opaque" + "(" + _dafny.String(data.Obj) + ")"
		}
	case Error_OpaqueWithText:
		{
			return "SimpleOrphanedTypes.Error.OpaqueWithText" + "(" + _dafny.String(data.Obj) + ", " + _dafny.String(data.ObjMessage) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this Error) Equals(other Error) bool {
	switch data1 := _this.Get_().(type) {
	case Error_OrphanedError:
		{
			data2, ok := other.Get_().(Error_OrphanedError)
			return ok && data1.Message.Equals(data2.Message)
		}
	case Error_CollectionOfErrors:
		{
			data2, ok := other.Get_().(Error_CollectionOfErrors)
			return ok && data1.List.Equals(data2.List) && data1.Message.Equals(data2.Message)
		}
	case Error_Opaque:
		{
			data2, ok := other.Get_().(Error_Opaque)
			return ok && _dafny.AreEqual(data1.Obj, data2.Obj)
		}
	case Error_OpaqueWithText:
		{
			data2, ok := other.Get_().(Error_OpaqueWithText)
			return ok && _dafny.AreEqual(data1.Obj, data2.Obj) && data1.ObjMessage.Equals(data2.ObjMessage)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this Error) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(Error)
	return ok && _this.Equals(typed)
}

func Type_Error_() _dafny.TypeDescriptor {
	return type_Error_{}
}

type type_Error_ struct {
}

func (_this type_Error_) Default() interface{} {
	return Companion_Error_.Default()
}

func (_this type_Error_) String() string {
	return "SimpleOrphanedTypes.Error"
}
func (_this Error) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = Error{}

// End of datatype Error

// Definition of class OpaqueError
type OpaqueError struct {
}

func New_OpaqueError_() *OpaqueError {
	_this := OpaqueError{}

	return &_this
}

type CompanionStruct_OpaqueError_ struct {
}

var Companion_OpaqueError_ = CompanionStruct_OpaqueError_{}

func (*OpaqueError) String() string {
	return "SimpleOrphanedTypes.OpaqueError"
}

// End of class OpaqueError

func Type_OpaqueError_() _dafny.TypeDescriptor {
	return type_OpaqueError_{}
}

type type_OpaqueError_ struct {
}

func (_this type_OpaqueError_) Default() interface{} {
	return Companion_Error_.Default()
}

func (_this type_OpaqueError_) String() string {
	return "SimpleOrphanedTypes.OpaqueError"
}
func (_this *CompanionStruct_OpaqueError_) Is_(__source Error) bool {
	var _0_e Error = (__source)
	_ = _0_e
	return ((_0_e).Is_Opaque()) || ((_0_e).Is_OpaqueWithText())
}

// Definition of class DummySubsetType
type DummySubsetType struct {
}

func New_DummySubsetType_() *DummySubsetType {
	_this := DummySubsetType{}

	return &_this
}

type CompanionStruct_DummySubsetType_ struct {
}

var Companion_DummySubsetType_ = CompanionStruct_DummySubsetType_{}

func (*DummySubsetType) String() string {
	return "SimpleOrphanedTypes.DummySubsetType"
}
func (_this *CompanionStruct_DummySubsetType_) Witness() _dafny.Int {
	return _dafny.One
}

// End of class DummySubsetType

func Type_DummySubsetType_() _dafny.TypeDescriptor {
	return type_DummySubsetType_{}
}

type type_DummySubsetType_ struct {
}

func (_this type_DummySubsetType_) Default() interface{} {
	return Companion_DummySubsetType_.Witness()
}

func (_this type_DummySubsetType_) String() string {
	return "SimpleOrphanedTypes.DummySubsetType"
}
func (_this *CompanionStruct_DummySubsetType_) Is_(__source _dafny.Int) bool {
	var _1_x _dafny.Int = (__source)
	_ = _1_x
	return Companion_Default___.IsDummySubsetType(_1_x)
}
