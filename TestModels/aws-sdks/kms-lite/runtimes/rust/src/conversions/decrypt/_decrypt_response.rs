// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::decrypt::DecryptOutput
) -> ::std::rc::Rc<
    crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::DecryptResponse,
> {
    ::std::rc::Rc::new(crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::DecryptResponse::DecryptResponse { 
      Plaintext: dafny_standard_library::conversion::oblob_to_dafny(&value.plaintext),
      KeyId: dafny_standard_library::conversion::ostring_to_dafny(&value.key_id),
      CiphertextForRecipient: dafny_standard_library::conversion::oblob_to_dafny(&value.ciphertext_for_recipient),
      EncryptionAlgorithm: dafny_standard_library::conversion::option_to_dafny(&value.encryption_algorithm,
        |x| crate::conversions::encryption_algorithm_spec::to_dafny(x.clone())),
    })
}

// from_dafny ommitted to save time until we get to actual code generation