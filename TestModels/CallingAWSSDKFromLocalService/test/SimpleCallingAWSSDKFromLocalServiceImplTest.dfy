// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../src/Index.dfy"
include "../src/WrappedSimpleCallingAWSSDKFromLocalServiceImpl.dfy"

module SimpleCallingAWSSDKFromLocalServiceImplTest {
  import DDB = Com.Amazonaws.Dynamodb
  import KMS = Com.Amazonaws.Kms
  import SimpleCallingAWSSDKFromLocalService

  import opened SimpleCallingawssdkfromlocalserviceTypes
  import opened Wrappers
  method{:test} CallDDB(){
    var client :- expect SimpleCallingAWSSDKFromLocalService.SimpleCallingAWSSDKFromLocalService();
    TestCallDDB(client);
  }

  method TestCallDDB(client: ISimpleCallingAWSSDKFromLocalServiceClient)
    requires client.ValidState()
    modifies client.Modifies
    ensures client.ValidState()
  {
    var ddbClient :- expect DDB.DynamoDBClient();
    var ret := client.CallDDB(SimpleCallingAWSSDKFromLocalService.Types.CallDDBInput(ddbClient := ddbClient));
    expect ret.Success?;
  }

  method{:test} CallKMS(){
    var client :- expect SimpleCallingAWSSDKFromLocalService.SimpleCallingAWSSDKFromLocalService();
    TestCallKMS(client);
  }

  method TestCallKMS(client: ISimpleCallingAWSSDKFromLocalServiceClient)
    requires client.ValidState()
    modifies client.Modifies
    ensures client.ValidState()
  {
    var kmsClient :- expect KMS.KMSClient();
    var ret := client.CallKMS(SimpleCallingAWSSDKFromLocalService.Types.CallKMSInput(kmsClient := kmsClient));
    expect ret.Success?;
  }
}