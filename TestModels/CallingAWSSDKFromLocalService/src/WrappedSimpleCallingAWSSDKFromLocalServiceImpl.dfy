// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/SimpleCallingawssdkfromlocalserviceTypesWrapped.dfy"

module WrappedSimpleCallingawssdkfromlocalserviceService refines WrappedAbstractSimpleCallingawssdkfromlocalserviceService {
  import WrappedService = SimpleCallingawssdkfromlocalservice
  function method WrappedDefaultSimpleCallingawssdkfromlocalserviceConfig(): SimpleCallingawssdkfromlocalserviceConfig {
    SimpleCallingawssdkfromlocalserviceConfig
  }
}
