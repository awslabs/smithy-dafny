module github.com/smithy-lang/smithy-dafny/TestModels/Positional/test

go 1.23.0

require github.com/dafny-lang/DafnyStandardLibGo v0.0.0

require (
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.9.1
	github.com/smithy-lang/smithy-dafny/TestModels/Positional v0.0.0
)

replace github.com/smithy-lang/smithy-dafny/TestModels/Positional v0.0.0 => ../ImplementationFromDafny-go

replace github.com/dafny-lang/DafnyStandardLibGo => ../../../../dafny-dependencies/StandardLibrary/runtimes/go/ImplementationFromDafny-go/
