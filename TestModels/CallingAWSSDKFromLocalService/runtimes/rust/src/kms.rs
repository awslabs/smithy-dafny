// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use aws_config::Region;
use std::sync::LazyLock;

static DAFNY_TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

impl crate::r#software::amazon::cryptography::services::kms::internaldafny::_default {
    #[allow(non_snake_case)]
    pub fn KMSClient() -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        let shared_config = match tokio::runtime::Handle::try_current() {
            Ok(curr) => tokio::task::block_in_place(|| {
                curr.block_on(async {
                    aws_config::load_defaults(aws_config::BehaviorVersion::v2024_03_28()).await
                })
            }),
            Err(_) => DAFNY_TOKIO_RUNTIME.block_on(aws_config::load_defaults(
                aws_config::BehaviorVersion::v2024_03_28(),
            )),
        };

        let inner = aws_sdk_kms::Client::new(&shared_config);
        let client = crate::deps::com_amazonaws_kms::client::Client { inner };
        let dafny_client = ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(client));
        std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::Success {
            value: dafny_client,
        })
    }
}