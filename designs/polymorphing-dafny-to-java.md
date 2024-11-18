```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}} }%%
flowchart TB
    classDef Process stroke:#f80
    classDef Library stroke:#0ff
    classDef Authored stroke:#0f0
    classDef Generated stroke:#ff0

    SmithyModel["Smithy model"]:::Authored

    SmithySourceServerCodegen[["smithy
        (dafny-server-codegen)"]]:::Process

    subgraph SourceProject["Dafny Library"]
        SourceImpl["Dafny Implementation"]:::Authored
        SourceAPI["Dafny API"]:::Generated

        SourceAPI <==> SourceImpl
    end
    SourceProject:::Library

    SmithyModel -.-> SmithySourceServerCodegen -.-> SourceAPI

    subgraph SmithyTargetClientCodegen["smithy 
                                    (java-client-codegen)"]
        SmithyTargetAPICodegen[["API/Shim Generation"]]:::Process

        Compiler[["dafny
        (translate rust)"]]:::Process
    end
    SmithyTargetClientCodegen:::Process

    subgraph TargetProject["Rust Library"]
        TargetAPI["Rust API"]:::Generated
        SourceToTargetShims["Dafny to Rust shims"]:::Generated

        subgraph SourceEmbeddedInTarget["Dafny Library in Rust"]
            SourceAPIInTarget["Dafny API"]:::Generated
            SourceImplInTarget["Dafny Implementation"]:::Generated
        end
        SourceEmbeddedInTarget:::Library

        TargetAPI <==> SourceToTargetShims <==> SourceAPIInTarget <==> SourceImplInTarget
    end
    TargetProject:::Library

    SmithyModel -.-> SmithyTargetClientCodegen
    SmithyTargetAPICodegen -.-> TargetAPI & SourceToTargetShims
    SourceProject -.-> Compiler -.-> SourceEmbeddedInTarget

    subgraph SmithyTarget2ClientCodegen["smithy 
                                    (python-client-codegen)"]
        SmithyTarget2APICodegen[["API/Shim Generation"]]:::Process

        Compiler2[["dafny
        (translate python)"]]:::Process
    end
    SmithyTarget2ClientCodegen:::Process

    subgraph Target2Project["Python Library"]
        Target2API["Python API"]:::Generated
        SourceToTarget2Shims["Dafny to Python shims"]:::Generated

        subgraph SourceEmbeddedInTarget2["Dafny Library in Python"]
            SourceAPIInTarget2["Dafny API"]:::Generated
            SourceImplInTarget2["Dafny Implementation"]:::Generated
        end
        SourceEmbeddedInTarget2:::Library

        Target2API <==> SourceToTarget2Shims <==> SourceAPIInTarget2 <==> SourceImplInTarget2
    end
    Target2Project:::Library

    SmithyModel -.-> SmithyTarget2ClientCodegen
    SmithyTarget2APICodegen -.-> Target2API & SourceToTarget2Shims
    SourceProject -.-> Compiler2 -.-> SourceEmbeddedInTarget2

```