# SQSExtended

This project serves as an example of a common pattern:
using `smithy-dafny` to build a thick client library for a remote service,
such as an AWS service.
The single source implementation is in Dafny,
but `smithy-dafny` transforms this into a full library with an idiomatic API
in multiple other common programming languages,
such as Java, Rust and Python.
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

The extended SQS client we're building here helps encode this pattern
and reduce boilerplate: it adds a `HandleMessages` operation
which takes a message handler callback to be called on each received message.
It then automatically deletes each successfully-processed message.
The callback is modeled as an `@extendable` Smithy resource,
meaning a resource that the caller is allowed to implement.

# Building your own Polymorph library

This is a good project to copy for similar thick client libraries.
We hope to provide a `smithy init` template for this soon.