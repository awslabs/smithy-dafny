module github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleTimestamp/test

go 1.23.0

replace github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleTimestamp v0.0.0 => ../ImplementationFromDafny-go

require github.com/dafny-lang/DafnyStandardLibGo v0.0.0

require (
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.8.0
	github.com/smithy-lang/smithy-dafny/TestModels/SimpleTypes/SimpleTimestamp v0.0.0
)

replace github.com/dafny-lang/DafnyStandardLibGo => ../../../../../dafny-dependencies/StandardLibrary/runtimes/go/ImplementationFromDafny-go/
