
import io
from _dafny import Seq
from smithy_python.interfaces.blobs import ByteStream
from standard_library.internaldafny.generated.Std_Enumerators import Enumerator
from standard_library.internaldafny.generated.Wrappers import Option, Option_Some, Option_None


class EnumeratorByteStream(ByteStream):
  
  def __init__(self, dafny_enumerator):
    self.dafny_enumerator = dafny_enumerator
    self.counter = 0
    # this is obviously horrible, just testing
    self.buffer = []

  def read(self, size: int = -1) -> bytes:
    # TODO: assert size is -1, buffer, 
    # or define a more specialized Action<int, bytes> type for streams.
    next = Enumerator.Next(self.dafny_enumerator)
    while next.is_Some and len(next.value) == 0:
      next = Enumerator.Next(self.dafny_enumerator)
    # Do NOT return None, because that indicates "no data right now, might be more later"
    nv = next.value if next.is_Some else None
    b = bytes(nv) if nv else bytes()
    # TODO: ideally this would be done inside dafny, hack for now
    self.counter += len(b)
    if len(b) >= 1:
      self.buffer.append(b)
    return b

  def seek(self, offset, whence=0):
    # TODO: horrible hack that doesnt even work
    # Result_Failure(error=Error_Opaque(obj=ClientError('An error occurred (BadDigest) when calling the PutObject
    # operation (reached max retries: 4): The Content-MD5 you specified did not match what we received.')))
    ba = b''.join(self.buffer)
    shadow = io.BytesIO(ba)
    return shadow.seek(offset, whence)


  def tell(self):
    return self.counter


class StreamingBlobEnumerator(Enumerator):
  
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
    
