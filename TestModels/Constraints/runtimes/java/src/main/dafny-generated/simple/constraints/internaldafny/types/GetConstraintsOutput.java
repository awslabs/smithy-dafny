// Class GetConstraintsOutput
// Dafny class GetConstraintsOutput compiled into Java
package simple.constraints.internaldafny.types;


@SuppressWarnings({"unchecked", "deprecation"})
public class GetConstraintsOutput {
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> _MyString;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> _NonEmptyString;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> _StringLessThanOrEqualToTen;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> _MyBlob;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> _NonEmptyBlob;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> _BlobLessThanOrEqualToTen;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> _MyList;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> _NonEmptyList;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> _ListLessThanOrEqualToTen;
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> _MyMap;
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> _NonEmptyMap;
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> _MapLessThanOrEqualToTen;
  public Wrappers_Compile.Option<java.lang.Integer> _OneToTen;
  public Wrappers_Compile.Option<java.lang.Long> _thatTenToTen;
  public Wrappers_Compile.Option<java.lang.Integer> _GreaterThanOne;
  public Wrappers_Compile.Option<java.lang.Integer> _LessThanTen;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> _MyUtf8Bytes;
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>> _MyListOfUtf8Bytes;
  public GetConstraintsOutput (Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> MyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> NonEmptyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> StringLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> NonEmptyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> BlobLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> MyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> NonEmptyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> ListLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> NonEmptyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MapLessThanOrEqualToTen, Wrappers_Compile.Option<java.lang.Integer> OneToTen, Wrappers_Compile.Option<java.lang.Long> thatTenToTen, Wrappers_Compile.Option<java.lang.Integer> GreaterThanOne, Wrappers_Compile.Option<java.lang.Integer> LessThanTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyUtf8Bytes, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>> MyListOfUtf8Bytes) {
    this._MyString = MyString;
    this._NonEmptyString = NonEmptyString;
    this._StringLessThanOrEqualToTen = StringLessThanOrEqualToTen;
    this._MyBlob = MyBlob;
    this._NonEmptyBlob = NonEmptyBlob;
    this._BlobLessThanOrEqualToTen = BlobLessThanOrEqualToTen;
    this._MyList = MyList;
    this._NonEmptyList = NonEmptyList;
    this._ListLessThanOrEqualToTen = ListLessThanOrEqualToTen;
    this._MyMap = MyMap;
    this._NonEmptyMap = NonEmptyMap;
    this._MapLessThanOrEqualToTen = MapLessThanOrEqualToTen;
    this._OneToTen = OneToTen;
    this._thatTenToTen = thatTenToTen;
    this._GreaterThanOne = GreaterThanOne;
    this._LessThanTen = LessThanTen;
    this._MyUtf8Bytes = MyUtf8Bytes;
    this._MyListOfUtf8Bytes = MyListOfUtf8Bytes;
  }

  @Override
  public boolean equals(Object other) {
    if (this == other) return true;
    if (other == null) return false;
    if (getClass() != other.getClass()) return false;
    GetConstraintsOutput o = (GetConstraintsOutput)other;
    return true && java.util.Objects.equals(this._MyString, o._MyString) && java.util.Objects.equals(this._NonEmptyString, o._NonEmptyString) && java.util.Objects.equals(this._StringLessThanOrEqualToTen, o._StringLessThanOrEqualToTen) && java.util.Objects.equals(this._MyBlob, o._MyBlob) && java.util.Objects.equals(this._NonEmptyBlob, o._NonEmptyBlob) && java.util.Objects.equals(this._BlobLessThanOrEqualToTen, o._BlobLessThanOrEqualToTen) && java.util.Objects.equals(this._MyList, o._MyList) && java.util.Objects.equals(this._NonEmptyList, o._NonEmptyList) && java.util.Objects.equals(this._ListLessThanOrEqualToTen, o._ListLessThanOrEqualToTen) && java.util.Objects.equals(this._MyMap, o._MyMap) && java.util.Objects.equals(this._NonEmptyMap, o._NonEmptyMap) && java.util.Objects.equals(this._MapLessThanOrEqualToTen, o._MapLessThanOrEqualToTen) && java.util.Objects.equals(this._OneToTen, o._OneToTen) && java.util.Objects.equals(this._thatTenToTen, o._thatTenToTen) && java.util.Objects.equals(this._GreaterThanOne, o._GreaterThanOne) && java.util.Objects.equals(this._LessThanTen, o._LessThanTen) && java.util.Objects.equals(this._MyUtf8Bytes, o._MyUtf8Bytes) && java.util.Objects.equals(this._MyListOfUtf8Bytes, o._MyListOfUtf8Bytes);
  }
  @Override
  public int hashCode() {
    long hash = 5381;
    hash = ((hash << 5) + hash) + 0;
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyString);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._NonEmptyString);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._StringLessThanOrEqualToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyBlob);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._NonEmptyBlob);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._BlobLessThanOrEqualToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyList);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._NonEmptyList);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._ListLessThanOrEqualToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyMap);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._NonEmptyMap);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MapLessThanOrEqualToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._OneToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._thatTenToTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._GreaterThanOne);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._LessThanTen);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyUtf8Bytes);
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._MyListOfUtf8Bytes);
    return (int)hash;
  }

  @Override
  public String toString() {
    StringBuilder s = new StringBuilder();
    s.append("SimpleConstraintsTypes.GetConstraintsOutput.GetConstraintsOutput");
    s.append("(");
    s.append(dafny.Helpers.toString(this._MyString));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._NonEmptyString));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._StringLessThanOrEqualToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MyBlob));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._NonEmptyBlob));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._BlobLessThanOrEqualToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MyList));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._NonEmptyList));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._ListLessThanOrEqualToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MyMap));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._NonEmptyMap));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MapLessThanOrEqualToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._OneToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._thatTenToTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._GreaterThanOne));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._LessThanTen));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MyUtf8Bytes));
    s.append(", ");
    s.append(dafny.Helpers.toString(this._MyListOfUtf8Bytes));
    s.append(")");
    return s.toString();
  }
  private static final dafny.TypeDescriptor<GetConstraintsOutput> _TYPE = dafny.TypeDescriptor.<GetConstraintsOutput>referenceWithInitializer(GetConstraintsOutput.class, () -> GetConstraintsOutput.Default());
  public static dafny.TypeDescriptor<GetConstraintsOutput> _typeDescriptor() {
    return (dafny.TypeDescriptor<GetConstraintsOutput>) (dafny.TypeDescriptor<?>) _TYPE;
  }

  private static final GetConstraintsOutput theDefault = simple.constraints.internaldafny.types.GetConstraintsOutput.create(Wrappers_Compile.Option.<dafny.DafnySequence<? extends Character>>Default(MyString._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends Character>>Default(NonEmptyString._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends Character>>Default(StringLessThanOrEqualToTen._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends java.lang.Byte>>Default(MyBlob._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends java.lang.Byte>>Default(NonEmptyBlob._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends java.lang.Byte>>Default(BlobLessThanOrEqualToTen._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>>Default(MyList._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>>Default(NonEmptyList._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>>Default(ListLessThanOrEqualToTen._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>>Default(MyMap._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>>Default(NonEmptyMap._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>>Default(MapLessThanOrEqualToTen._typeDescriptor()), Wrappers_Compile.Option.<java.lang.Integer>Default(OneToTen._typeDescriptor()), Wrappers_Compile.Option.<java.lang.Long>Default(TenToTen._typeDescriptor()), Wrappers_Compile.Option.<java.lang.Integer>Default(GreaterThanOne._typeDescriptor()), Wrappers_Compile.Option.<java.lang.Integer>Default(LessThanTen._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends java.lang.Byte>>Default(Utf8Bytes._typeDescriptor()), Wrappers_Compile.Option.<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>>Default(ListOfUtf8Bytes._typeDescriptor()));
  public static GetConstraintsOutput Default() {
    return theDefault;
  }
  public static GetConstraintsOutput create(Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> MyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> NonEmptyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> StringLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> NonEmptyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> BlobLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> MyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> NonEmptyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> ListLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> NonEmptyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MapLessThanOrEqualToTen, Wrappers_Compile.Option<java.lang.Integer> OneToTen, Wrappers_Compile.Option<java.lang.Long> thatTenToTen, Wrappers_Compile.Option<java.lang.Integer> GreaterThanOne, Wrappers_Compile.Option<java.lang.Integer> LessThanTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyUtf8Bytes, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>> MyListOfUtf8Bytes) {
    return new GetConstraintsOutput(MyString, NonEmptyString, StringLessThanOrEqualToTen, MyBlob, NonEmptyBlob, BlobLessThanOrEqualToTen, MyList, NonEmptyList, ListLessThanOrEqualToTen, MyMap, NonEmptyMap, MapLessThanOrEqualToTen, OneToTen, thatTenToTen, GreaterThanOne, LessThanTen, MyUtf8Bytes, MyListOfUtf8Bytes);
  }
  public static GetConstraintsOutput create_GetConstraintsOutput(Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> MyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> NonEmptyString, Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> StringLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> NonEmptyBlob, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> BlobLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> MyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> NonEmptyList, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> ListLessThanOrEqualToTen, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> NonEmptyMap, Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> MapLessThanOrEqualToTen, Wrappers_Compile.Option<java.lang.Integer> OneToTen, Wrappers_Compile.Option<java.lang.Long> thatTenToTen, Wrappers_Compile.Option<java.lang.Integer> GreaterThanOne, Wrappers_Compile.Option<java.lang.Integer> LessThanTen, Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> MyUtf8Bytes, Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>> MyListOfUtf8Bytes) {
    return create(MyString, NonEmptyString, StringLessThanOrEqualToTen, MyBlob, NonEmptyBlob, BlobLessThanOrEqualToTen, MyList, NonEmptyList, ListLessThanOrEqualToTen, MyMap, NonEmptyMap, MapLessThanOrEqualToTen, OneToTen, thatTenToTen, GreaterThanOne, LessThanTen, MyUtf8Bytes, MyListOfUtf8Bytes);
  }
  public boolean is_GetConstraintsOutput() { return true; }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> dtor_MyString() {
    return this._MyString;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> dtor_NonEmptyString() {
    return this._NonEmptyString;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends Character>> dtor_StringLessThanOrEqualToTen() {
    return this._StringLessThanOrEqualToTen;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> dtor_MyBlob() {
    return this._MyBlob;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> dtor_NonEmptyBlob() {
    return this._NonEmptyBlob;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> dtor_BlobLessThanOrEqualToTen() {
    return this._BlobLessThanOrEqualToTen;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> dtor_MyList() {
    return this._MyList;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> dtor_NonEmptyList() {
    return this._NonEmptyList;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends Character>>> dtor_ListLessThanOrEqualToTen() {
    return this._ListLessThanOrEqualToTen;
  }
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> dtor_MyMap() {
    return this._MyMap;
  }
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> dtor_NonEmptyMap() {
    return this._NonEmptyMap;
  }
  public Wrappers_Compile.Option<dafny.DafnyMap<? extends dafny.DafnySequence<? extends Character>, ? extends dafny.DafnySequence<? extends Character>>> dtor_MapLessThanOrEqualToTen() {
    return this._MapLessThanOrEqualToTen;
  }
  public Wrappers_Compile.Option<java.lang.Integer> dtor_OneToTen() {
    return this._OneToTen;
  }
  public Wrappers_Compile.Option<java.lang.Long> dtor_thatTenToTen() {
    return this._thatTenToTen;
  }
  public Wrappers_Compile.Option<java.lang.Integer> dtor_GreaterThanOne() {
    return this._GreaterThanOne;
  }
  public Wrappers_Compile.Option<java.lang.Integer> dtor_LessThanTen() {
    return this._LessThanTen;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends java.lang.Byte>> dtor_MyUtf8Bytes() {
    return this._MyUtf8Bytes;
  }
  public Wrappers_Compile.Option<dafny.DafnySequence<? extends dafny.DafnySequence<? extends java.lang.Byte>>> dtor_MyListOfUtf8Bytes() {
    return this._MyListOfUtf8Bytes;
  }
}
