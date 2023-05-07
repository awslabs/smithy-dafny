using Amazon;
using Amazon.DynamoDBv2;
using Wrappers_Compile;
using Amazon.Runtime;
using Com.Amazonaws.Dynamodb;

// This extern is identified in Dafny code
// that refines the AWS SDK DDB Model
namespace software.amazon.awssdk.services.dynamodb.internaldafny
{
  public partial class __default
  {
    public static
        _IResult<
            types.IDynamoDBClient,
            types._IError
        >
        DynamoDBClient()
    {
      var client = new AmazonDynamoDBClient();

      return Result<
              types.IDynamoDBClient,
              types._IError
          >
          .create_Success(new DynamoDBv2Shim(client));
    }
  }
}
