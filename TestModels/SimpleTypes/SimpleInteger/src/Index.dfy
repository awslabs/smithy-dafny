include "SimpleIntegerImpl.dfy"

module {:extern "simple.types.integer.internaldafny" } SimpleInteger refines AbstractSimpleTypesIntegerService {
    import Operations = SimpleIntegerImpl

    function method DefaultSimpleIntegerConfig(): SimpleIntegerConfig {
       SimpleIntegerConfig
    }

    method SimpleInteger(config: SimpleIntegerConfig)
    returns (res: Result<SimpleIntegerClient, Error>) {
        var client := new SimpleIntegerClient(Operations.Config);
        return Success(client);
    }

    class SimpleIntegerClient... {
        predicate ValidState()
        {
            && Operations.ValidInternalConfig?(config)
            && Modifies == Operations.ModifiesInternalConfig(config) + {History}
        }
        constructor(config: Operations.InternalConfig) {
            this.config := config;
            History := new ISimpleTypesIntegerClientCallHistory();
            Modifies := Operations.ModifiesInternalConfig(config) + {History};
        }
    }
}
