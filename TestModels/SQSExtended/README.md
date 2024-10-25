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


# Building your own Polymorph library

We hope to provide a `smithy init` template for this soon.