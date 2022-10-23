package software.amazon.polymorph.smithyjava.generator.library;

class Constants {
    static String TEST_ERROR_EXPECTED = """
            package software.amazon.cryptography.test.model;
                        
            public class TestError extends NativeError {
              protected TestError(BuilderImpl builder) {
                super(builder);
              }
                        
              @Override
              public Builder toBuilder() {
                return new BuilderImpl(this);
              }
                        
              public static Builder builder() {
                return new BuilderImpl();
              }
                        
              interface Builder extends NativeError.Builder {
                Builder message(String message);
                        
                Builder cause(Throwable cause);
                        
                TestError build();
              }
                        
              protected static class BuilderImpl extends NativeError.BuilderImpl implements Builder {
                protected BuilderImpl() {
                }
                        
                protected BuilderImpl(TestError model) {
                  super(model);
                }
                        
                @Override
                public Builder message(String message) {
                  super.message(message);
                  return this;
                }
                        
                @Override
                public Builder cause(Throwable cause) {
                  super.cause(cause);
                  return this;
                }
                        
                @Override
                public TestError build() {
                  if (this.message() == null)  {
                    throw new IllegalArgumentException("Missing value for required field `message`");
                  }
                  return new TestError(this);
                }
              }
            }
            """;

    static String RANGE_TRAIT_INTEGER_BUILD_METHOD_START = "public TestRangeMinMaxInteger build()";
    static String RANGE_TRAIT_INTEGER_BUILD_METHOD_RETURN = "return new TestRangeMinMaxInteger(this);";
    static String RANGE_TRAIT_INTEGER_BUILD_EXPECTED = """
            %s {
              if (this.zeroToTen() < 0) {
                throw new IllegalArgumentException("`zeroToTen` must be greater than or equal to 0");
              }
              if (this.zeroToTen() > 10) {
                throw new IllegalArgumentException("`zeroToTen` must be less than or equal to 10.");
              }
              %s
            }""".formatted(RANGE_TRAIT_INTEGER_BUILD_METHOD_START, RANGE_TRAIT_INTEGER_BUILD_METHOD_RETURN);

    // Method's end is down 1 line (1 \n), in class (2 spaces), inside BuilderImpl (2 spaces), and the bracket (1 } )
    static int BUILD_METHOD_END_OFFSET = 6;

    static String LENGTH_TRAIT_BLOB_BUILD_METHOD_START = "public TestLengthMinMaxBlob build()";
    static String LENGTH_TRAIT_BLOB_BUILD_METHOD_RETURN = "return new TestLengthMinMaxBlob(this);";
    static String LENGTH_TRAIT_BLOB_BUILD_METHOD_EXPECTED = """
            %s {
              if (this.key().remaining() < 256) {
                throw new IllegalArgumentException("The size of `key` must be greater than or equal to 256");
              }
              if (this.key().remaining() > 256) {
                throw new IllegalArgumentException("The size of `key` must be less than or equal to 256");
              }
              %s
            }
            """.formatted(LENGTH_TRAIT_BLOB_BUILD_METHOD_START, LENGTH_TRAIT_BLOB_BUILD_METHOD_RETURN);
}
