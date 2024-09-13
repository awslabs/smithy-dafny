// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/WrappedSimpleCallingAWSSDKFromLocalServiceImpl.dfy"
include "SimpleCallingAWSSDKFromLocalServiceImplTest.dfy"

module WrappedSimpleCallingAWSSDKFromLocalServiceTest {
  import opened WrappedSimpleCallingAWSSDKFromLocalServiceService
  import SimpleCallingAWSSDKFromLocalServiceImplTest
  import opened Wrappers
  import opened StandardLibrary.UInt
  // method{:test} TestCallDDBGetItem() {
  //   var client :- expect WrappedSimpleCallingAWSSDKFromLocalServiceService.WrappedSimpleCallingAWSSDKFromLocalService();
  //   SimpleCallingAWSSDKFromLocalServiceImplTest.TestCallDDBGetItem_Success(client);
  //   SimpleCallingAWSSDKFromLocalServiceImplTest.TestCallDDBGetItem_Failure(client);
  // }

  method{:test} TestCallKMSEncrypt() {
    var client :- expect WrappedSimpleCallingAWSSDKFromLocalServiceService.WrappedSimpleCallingAWSSDKFromLocalService();
    SimpleCallingAWSSDKFromLocalServiceImplTest.TestCallKMSEncrypt_Success(client);
    SimpleCallingAWSSDKFromLocalServiceImplTest.TestCallKMSEncrypt_Failure(client);
  }
}