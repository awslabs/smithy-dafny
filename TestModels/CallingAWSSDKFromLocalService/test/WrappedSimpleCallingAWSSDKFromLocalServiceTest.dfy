// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/WrappedSimpleCallingawssdkfromlocalserviceImpl.dfy"
include "SimpleCallingawssdkfromlocalserviceImplTest.dfy"

module WrappedSimpleCallingawssdkfromlocalserviceTest {
  import Com.Amazonaws.Kms
  import Com.Amazonaws.Dynamodb
  import SimpleCallingawssdkfromlocalservice

  import opened WrappedSimpleCallingawssdkfromlocalserviceService
  import SimpleCallingawssdkfromlocalserviceImplTest
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened SimpleCallingawssdkfromlocalserviceTypes

  method{:test} TestCallDDBScan() {
    var client :- expect WrappedSimpleCallingawssdkfromlocalserviceService.WrappedSimpleCallingawssdkfromlocalservice();
    SimpleCallingawssdkfromlocalserviceImplTest.TestCallDDBScan_Success(client);
    SimpleCallingawssdkfromlocalserviceImplTest.TestCallDDBScan_Failure(client);
  }

  method{:test} TestCallKMSEncrypt() {
    var client :- expect WrappedSimpleCallingawssdkfromlocalserviceService.WrappedSimpleCallingawssdkfromlocalservice();
    SimpleCallingawssdkfromlocalserviceImplTest.TestCallKMSEncrypt_Success(client);
    SimpleCallingawssdkfromlocalserviceImplTest.TestCallKMSEncrypt_Failure(client);
  }
}
