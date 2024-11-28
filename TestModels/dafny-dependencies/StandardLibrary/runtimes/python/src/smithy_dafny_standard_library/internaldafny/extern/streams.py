
from _dafny import Seq
from smithy_python.interfaces.blobs import ByteStream
from smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_Streams import DataStream
from smithy_dafny_standard_library.internaldafny.generated.Std_Enumerators import Enumerator
from smithy_dafny_standard_library.internaldafny.generated.Std_Wrappers import Option, Option_Some, Option_None


class DataStreamByteStream(ByteStream):
  
  def __init__(self, dafny_data_stream):
    if not dafny_data_stream.Rewindable():
      raise ValueError("Python requires rewindable streams")
    if not dafny_data_stream.ContentLength().is_Some:
      raise ValueError("Python requires streams with known lengths")
    self.dafny_data_stream = dafny_data_stream

  def read(self, size: int = -1) -> bytes:
    # TODO: assert size is -1, buffer, 
    # or define a more specialized Action<int, bytes> type for streams.
    next = self.dafny_data_stream.Next()
    while next.is_Some and len(next.value) == 0:
      next = self.dafny_data_stream.Next()
    # Do NOT return None, because that indicates "no data right now, might be more later"
    return bytes(next.value) if next.is_Some else bytes()

  def tell(self) -> int:
    return self.dafny_data_stream.Position()

  def seek(self, offset, whence=0):
    match whence:
      case 0:
        position = offset
      case 1:
        position = self.dafny_data_stream.Position() + offset
      case 2:
        position = self.dafny_data_stream.ContentLength().value + offset
    self.dafny_data_stream.Seek(position)


class StreamingBlobDataStream(DataStream):
  
  def __init__(self, streaming_blob):
    self.streaming_blob = streaming_blob

  def Next(self):
    return Enumerator.Next(self)

  def Invoke(self, _) -> Option:
    next = self.streaming_blob.read()
    if next:
      return Option_Some(Seq(next))
    else:
      return Option_None()
    
