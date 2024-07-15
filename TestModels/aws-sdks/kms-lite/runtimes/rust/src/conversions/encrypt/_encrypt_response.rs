// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::encrypt::EncryptOutput
) -> ::std::rc::Rc<
    crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptResponse,
>{
    ::std::rc::Rc::new(crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::EncryptResponse::EncryptResponse { 
      CiphertextBlob: dafny_standard_library::conversion::oblob_to_dafny(&value.ciphertext_blob),
      KeyId: dafny_standard_library::conversion::ostring_to_dafny(&value.key_id),
      EncryptionAlgorithm: dafny_standard_library::conversion::option_to_dafny(value.encryption_algorithm.clone(),
        |x| crate::conversions::encryption_algorithm_spec::to_dafny(x.clone())),
    })
}

// #[allow(dead_code)]
// pub fn from_dafny(
//     dafny_value: ::std::rc::Rc<
//         crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_dkms_dinternaldafny_dtypes::GenerateDataKeyResponse,
//     >
// ) -> aws_sdk_kms::operation::generate_data_key::GenerateDataKeyOutput {
//   aws_sdk_kms::operation::generate_data_key::GenerateDataKeyOutput::builder()
//     .set_ciphertext_blob(dafny_standard_library::conversion::oblob_from_dafny(dafny_value.CiphertextBlob()))
//     .set_plaintext(dafny_standard_library::conversion::oblob_from_dafny(dafny_value.Plaintext()))
//     .set_key_id(dafny_standard_library::conversion::ostring_from_dafny(dafny_value.KeyId()))
//     .set_ciphertext_for_recipient(dafny_standard_library::conversion::oblob_from_dafny(*dafny_value.CiphertextForRecipient()))
//     .build()
// }
