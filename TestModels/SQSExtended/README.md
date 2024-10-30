# SQSExtended

This project serves as an example of a common pattern:
using `smithy-dafny` to build a thick client library for a remote service,
such as an AWS service.
The single source implementation is in Dafny,
but `smithy-dafny` transforms this into a full library with an idiomatic API
in multiple other common programming languages,
such as Java, Rust, and Python.
This transformation is known as "polymorphing".

The language-agnostic API of the library is
[defined in Smithy here](Model/sqsextended.smithy).
This looks very similar to a remote service definition,
with a few extra traits used to specify that this is a "local service".

In this case we're making it easier to implement a very common pattern
when using SQS.
The core operations in SQS are `SendMessage` and `ReceiveMessage`.
But receiving and processing messages is actually more non-trivial than you might think at first.
When you receive a message, it isn't immediately removed from the queue.
Instead it is just hidden from other consumers for a configurable timeout.
This is done so that if a consumer fails to process the message or flat out crashes,
the message isn't lost, and can be retried by some other consumer.
Therefore, once a consumer has finished processing a message,
it needs to call `DeleteMessage` to explicitly remove the message.
Or if the message couldn't be processed for some reason,
but should be retried,
the consumer should call `ChangeMessageVisibility` instead
to make it available for other consumers.

In addition, for efficiency reasons `ReceiveMessage` is actually a batch operation:
it will return up to 10 messages from a single call.
This means whatever code invokes this operation has to handle up to 10
unrelated messages, and therefore do some kind of dispatch to processing code.

The extended SQS client we're building here helps encode this pattern
and reduce boilerplate: it adds a `HandleMessages` operation
which takes a message handler callback to be called on each received message.
It then automatically deletes each successfully-processed message.
The callback is modeled as an `@extendable` Smithy resource
(the `MessageHandler` resource in [this model](Model/sqsextended.smithy)),
meaning a resource that the caller is allowed to implement.

## Build

To build this library for a particular language,
you can use the top-level `make <lang>` target,
which generates code from both Smithy models and Dafny source code.
This test model is currently only set up for Java,
but more target languages will be added soon.

If you run `make java`,
you will find a fully-functional Java library
under [`runtimes/java`](runtimes/java).
This also includes a manually-written test
showing what it looks like to interact with the generated library in Java.

Note that `make java` will also execute tests,
which require having credentials set up to actually make calls to SQS,
and may incur charges.

## Building your own Polymorph library

This is a good project to copy for similar thick client libraries.
You will also need to copy and modify the [sqs-via-cli](../aws-sdks/sqs-via-cli/)
Dafny client for SQS that it depends on,
to give it the Smithy model for whichever service you're building on.

We hope to provide a `smithy init` template for this soon.
