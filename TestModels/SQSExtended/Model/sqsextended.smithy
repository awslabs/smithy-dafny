$version: "2"

namespace polymorph.tutorial.sqsextended

use com.amazonaws.sqs#Message

///
/// An extended client for SQS that adds common client-side operations.
///
@aws.polymorph#localService(
  sdkId: "SQSExtended",
  config: SQSExtendedClientConfig,
  configRequired: true,
  dependencies: [
    com.amazonaws.sqs#AmazonSQS
  ]
)
service AmazonSQSExtended {
  version: "2021-11-01",
  resources: [ MessageHandler ],
  operations: [
    HandleMessages,
  ],
  errors: [SQSExtendedError],
}

///
/// Configuration for creating a new extended SQS client.
///
structure SQSExtendedClientConfig {
  /// The underlying SQS client.
  @required
  sqsClient: SQSClientReference
}

@error("server")
structure SQSExtendedError {
  @required
  message: String
}

@aws.polymorph#reference(service: com.amazonaws.sqs#AmazonSQS)
structure SQSClientReference {}

///
/// A callback for handling SQS messages.
///
@aws.polymorph#extendable
resource MessageHandler {
  operations: [
    HandleMessage,
  ],
}

@aws.polymorph#reference(resource: MessageHandler)
structure MessageHandlerReference {}

///
/// Processes the given message.
/// 
/// If this does not produce an error, the message will be automatically deleted.
///
operation HandleMessage {
  input := {
    @required
    message: Message
  }
}

///
/// Calls ReceiveMessage and applies the given handler to each message.
/// Automatically deletes each successfully handled message.
///
operation HandleMessages {
  input := {
    @required
    receiveRequest: com.amazonaws.sqs#ReceiveMessageRequest,

    @required
    handler: MessageHandlerReference
  }
}