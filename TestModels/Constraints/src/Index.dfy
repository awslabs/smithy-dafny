include "SimpleConstraintsImpl.dfy"

module {:extern "simple.constraints.internaldafny" } SimpleConstraints refines AbstractSimpleConstraintsService {
  import Operations = SimpleConstraintsImpl

  function method DefaultSimpleConstraintsConfig(): SimpleConstraintsConfig {
    SimpleConstraintsConfig
  }

  method SimpleConstraints(config: SimpleConstraintsConfig)
    returns (res: Result<SimpleConstraintsClient, Error>)
  {
    var client := new SimpleConstraintsClient(Operations.Config);
    return Success(client);
  }

  class SimpleConstraintsClient... {
    predicate ValidState() {
       && Operations.ValidInternalConfig?(config)
       && Modifies == Operations.ModifiesInternalConfig(config) + {History}
    }

    constructor(config: Operations.InternalConfig) {
       this.config := config;
       History := new ISimpleConstraintsClientCallHistory();
       Modifies := Operations.ModifiesInternalConfig(config) + {History};
    }
  }
}
