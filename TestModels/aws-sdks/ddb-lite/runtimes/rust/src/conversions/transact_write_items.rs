// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub mod _transact_write_items_request;

pub mod _transact_write_items_response;
#[allow(dead_code)]
pub fn to_dafny_error(
    value: &::aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    match value {
      aws_sdk_dynamodb::error::SdkError::ServiceError(service_error) => match service_error.err() {
                aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::InternalServerError(e) =>
            crate::conversions::error::internal_server_error::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::InvalidEndpointException(e) =>
            crate::conversions::error::invalid_endpoint_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::ResourceNotFoundException(e) =>
            crate::conversions::error::resource_not_found_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::IdempotentParameterMismatchException(e) =>
            crate::conversions::error::idempotent_parameter_mismatch_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::TransactionCanceledException(e) =>
            crate::conversions::error::transaction_canceled_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::RequestLimitExceeded(e) =>
            crate::conversions::error::request_limit_exceeded::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::ProvisionedThroughputExceededException(e) =>
            crate::conversions::error::provisioned_throughput_exceeded_exception::to_dafny(e.clone()),
         aws_sdk_dynamodb::operation::transact_write_items::TransactWriteItemsError::TransactionInProgressException(e) =>
            crate::conversions::error::transaction_in_progress_exception::to_dafny(e.clone()),
        e => crate::conversions::error::to_opaque_error(e.to_string()),
      },
      _ => {
        crate::conversions::error::to_opaque_error(value.to_string())
      }
   }
}
