from standard_library.internaldafny.generated.SortedSets import *
import standard_library.internaldafny.generated.SortedSets
import _dafny

class default__:

  @staticmethod
  def SetToSequence(input_set):
    return _dafny.Seq(input_set.Elements)

standard_library.internaldafny.generated.SortedSets.default__ = default__