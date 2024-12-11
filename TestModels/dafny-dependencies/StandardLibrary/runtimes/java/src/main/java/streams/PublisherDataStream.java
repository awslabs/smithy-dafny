package streams;

import StandardLibrary_Compile.Streams_Compile.DataStream;
import StandardLibrary_Compile.Streams_Compile.RewindableDataStream;
import Std_Compile.Wrappers_Compile.Option;
import dafny.DafnySequence;
import dafny.Tuple0;
import org.reactivestreams.Publisher;

import java.nio.ByteBuffer;
import java.util.function.Function;

public class PublisherDataStream implements RewindableDataStream {

    private final Publisher<ByteBuffer> publisher;
    private final long contentLength;
    private final String contentType;

    public PublisherDataStream(Publisher<ByteBuffer> publisher, long contentLength, String contentType) {
        this.publisher = publisher;
        this.contentLength = contentLength;
        this.contentType = contentType;
    }

    @Override
    public long ContentLength() {
        return contentLength;
    }

    @Override
    public Option<DafnySequence<? extends Byte>> Next() {
        return Invoke(Tuple0.create());
    }

    @Override
    public void RepeatUntil(Tuple0 tuple0, Function<Option<DafnySequence<? extends Byte>>, Boolean> stop) {

    }

    @Override
    public Option<DafnySequence<? extends Byte>> Invoke(Tuple0 tuple0) {
        return null;
    }

    @Override
    public long Position() {
        return 0;
    }

    @Override
    public void Seek(long newPosition) {
        // TODO
        throw new UnsupportedOperationException();
    }
}