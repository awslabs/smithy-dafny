include "SimpleUnionImpl.dfy"

module {:extern "Dafny.Simple.Union" } SimpleUnion refines AbstractSimpleUnionService {
    import Operations = SimpleUnionImpl

    function method DefaultSimpleUnionConfig(): SimpleUnionConfig {
        SimpleUnionConfig
    }

    method SimpleUnion(config: SimpleUnionConfig)
      returns (res: Result<SimpleUnionClient, Error>)
    {
        var client := new SimpleUnionClient(Operations.Config);
        return Success(client);
    }

    class SimpleUnionClient... {
        predicate ValidState() {
            && Operations.ValidInternalConfig?(config)
            && Modifies == Operations.ModifiesInternalConfig(config) + {History}
        }

        constructor(config: Operations.InternalConfig) {
            this.config := config;
            History := new ISimpleUnionClientCallHistory();
            Modifies := Operations.ModifiesInternalConfig(config) + {History};
        }
    }
}