module github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes/test

go 1.23.0

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.0.0
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.9.2
	github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes v0.0.0
)

replace github.com/smithy-lang/smithy-dafny/TestModels/OrphanedShapes v0.0.0 => ../ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library => ../../../../dafny-dependencies/StandardLibrary/runtimes/go/ImplementationFromDafny-go/