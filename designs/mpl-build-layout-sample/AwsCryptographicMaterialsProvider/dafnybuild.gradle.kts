plugins {
    id("software.amazon.smithy.gradle.smithy-jar").version("1.1.0") 
    id("org.dafny.dafny").version("0.1.0")
}

dafny {
    dafnyVersion.set("4.8.1")

    optionsMap.put("resource-limit", 90_000_000)
    optionsMap.put("function-syntax", "3")
    optionsMap.put("unicode-char", false)
    optionsMap.put("cores", <calculate cores as currently in SmithyDafnyMakefile.mk>)
    // ...and other Dafny-specific configuration.
    // The Dafny Gradle plugin will be updated to generate a dfyconfig.toml,
    // so to do local development you'll just want to build once first.
    // This is common for Brazil development especially.
}

dependencies {
    // Dependencies for the Smithy model
    implementation("software.amazon.smithy:smithy-model:1.28.0")
    implementation("software.amazon.smithy:smithy-polymorph:1.28.0")

    // Smithy build plugins for the Dafny server (library) SDK codegen
    // all target language client codegen.
    // These do not initially all have to be published to Maven Central:
    // some can just be locally installed.
    smithyBuild("software.amazon.smithy.dafny:dafny-server-codegen:0.1.0")
    smithyBuild("software.amazon.smithy.java:java-client-codegen:0.1.0")
    smithyBuild("software.amazon.smithy.rust:rust-client-codegen:0.1.0")
    smithyBuild("software.amazon.smithy.python:python-client-codegen:0.1.0")
    smithyBuild("software.amazon.smithy.go:go-client-codegen:0.1.0")
    smithyBuild("software.amazon.smithy.net:net-client-codegen:0.1.0")

    // Dafny API generated from Smithy model.
    // Will be a full project also using the Dafny Gradle plugin,
    // generated by the software.amazon.smithy.gradle.smithy-jar plugin,
    // and therefore located in 
    // build/smithyprojections/source/dafny-server-codegen/AwsCryptographicMaterialProviders-API.
    // This is very similar to the workflow for building Smithy-based servers, 
    // ala https://smithy.io/2.0/tutorials/full-stack-tutorial.html#generating-the-server-sdk
    api("aws.cryptography.materialProviders:Dafny-AwsCryptographicMaterialProviders-API:1.0-SNAPSHOT")

    // Polymorph dependencies.
    //
    // Most of these will already be transitive Dafny-*-API dependencies.
    // Polymorphing will automatically map them to their target language equivalent,
    // based on the metadata that the Smithy build plugins add.
    // In practice these will often be inside the same repo and configured as sister Gradle projects,
    // so it's easy to build recursively when you want to.
    //
    implementation("software.amazon.awssdk:Dafny-DynamoDB:1.0-SNAPSHOT")
    implementation("software.amazon.awssdk:Dafny-KMS:1.0-SNAPSHOT")
    implementation("aws.cryptography.primitives:Dafny-AwsCryptographicPrimitives:1.0-SNAPSHOT")
}