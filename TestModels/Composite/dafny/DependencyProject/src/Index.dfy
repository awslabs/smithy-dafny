// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "SimpleCompositeDependencyprojectImpl.dfy"

module {:extern "simple.composite.dependencyproject.internaldafny" } SimpleCompositeDependencyprojectService refines AbstractSimpleCompositeDependencyprojectService {
    import Operations = SimpleCompositeDependencyprojectImpl

    function method DefaultDependencyProjectConfig(): DependencyProjectConfig {
       DependencyProjectConfig
    }

    method DependencyProject(config: DependencyProjectConfig)
    returns (res: Result<IDependencyProjectClient, Error>) {
        var client := new DependencyProjectClient(Operations.Config);
        return Success(client);
    }

    class DependencyProjectClient... {
        predicate ValidState()
        {
            && Operations.ValidInternalConfig?(config)
            && Modifies == Operations.ModifiesInternalConfig(config) + {History}
        }
        constructor(config: Operations.InternalConfig) {
            this.config := config;
            History := new IDependencyProjectClientCallHistory();
            Modifies := Operations.ModifiesInternalConfig(config) + {History};
        }
    }
}
