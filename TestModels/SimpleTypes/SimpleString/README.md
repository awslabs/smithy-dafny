# SimpleString

This project implements the smithy type [String](https://smithy.io/2.0/spec/simple-types.html#string) and the associated operations in `dafny`. This is then transpiled to a target runtime, and each tests are executed - either as CI actions or manually.

## Build
### .NET
1. Generate the Wrappers using `polymorph`
```
make generate_polymorph
```

2. Transpile the tests (and implementation) to the target runtime.
```
make transpile_net
```

3. Generate the executable in the .NET
```
cd runtimes/net/tests/ && dotnet test
```

3. Execute the tests
```
./bin/Debug/net6.0/SimpleStringTest
```

## Development
1. To add another target runtime support, edit the `Makefile` and add the appropriate recipe to generate the `polymorph` wrappers, and dafny transpilation.
2. Provide any glue code between dafny and target runtime if required.
3. Build, execute, and test in the target runtime.

*Example*

`--output-dotnet <PATH>` in the `gradlew run` is used to generate the polymorph wrappers. Similarly `--compileTarget:<RUNTIME>` flags is used in dafny recipe to transpile to C#.