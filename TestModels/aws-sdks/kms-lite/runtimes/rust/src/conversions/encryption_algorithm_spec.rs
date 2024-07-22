// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::EncryptionAlgorithmSpec,
) -> ::std::rc::Rc<crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec>{
    ::std::rc::Rc::new(match value {
        aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha1 => 
          crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::RSAES_OAEP_SHA_1 {  },
        aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha256 => 
          crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::RSAES_OAEP_SHA_256 {  },
        aws_sdk_kms::types::EncryptionAlgorithmSpec::SymmetricDefault => 
          crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::SYMMETRIC_DEFAULT {  },
        // TODO: This should not be a panic, but the Dafny image of the enum shape doesn't have an Unknown variant of any kind,
        // so there's no way to succeed.
        // See https://github.com/smithy-lang/smithy-dafny/issues/476.
        // This could be handled more cleanly if conversion functions returned Results,
        // but that would be a large and disruptie change to the overall code flow.
        _ => panic!("Unknown enum variant: {}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec,
) -> aws_sdk_kms::types::EncryptionAlgorithmSpec {
    match dafny_value {
        crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::SYMMETRIC_DEFAULT {  } => aws_sdk_kms::types::EncryptionAlgorithmSpec::SymmetricDefault,
        crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::RSAES_OAEP_SHA_1 {  } => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha1,
        crate::implementation_from_dafny::_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptionAlgorithmSpec::RSAES_OAEP_SHA_256 {  } => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha256,
    }
}
