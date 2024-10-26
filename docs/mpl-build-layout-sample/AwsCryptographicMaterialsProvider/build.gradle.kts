// This is an example of an all-pieces-in-one-build configuration.
// In practice they will often be split apart much more.
// Especially in Brazil, so that each piece has a single simple build-system.
// There may need to be some ordering/dependencies tweaks
// to make sure the plugins apply in the right order.

// Following a major tenet here of putting all generated code/assets
// under build/..., and in complete projects.
// Besides following suit with how existing Smithy tools work,
// this should clarify the architecture of how things fit together
// (especially being more explicit about how wrapped clients work),
// avoid complicated .gitignore files,
// and allow Gradle to not repeat work when inputs to individual tasks
// have not changed.

// The project directory after building everything might look like:

plugins {
    // Existing Smithy Gradle plugin.
    // Will be updated to attach metadata about the input model
    // and code generation plugins used.
    id("software.amazon.smithy.gradle.smithy-jar").version("1.1.0")
    
    // Existing Dafny Gradle plugin.
    // Already passes Dafny code between projects as doo files.
    // Will be updated to also include dtr files of dependencies automatically.
    id("org.dafny.dafny").version("0.1.0")

    // A new Gradle plugin that builds target language projects
    // from source language projects that have a Smithy-generated API.
    // Likely a Gradle wrapper around the `polymorph` CLI,
    // just as the two above are wrappers around `smithy` and `dafny` respectively.
    // Will support more source langauges besides Dafny in the future.
    // Useful to be a Gradle plugin even though Dafny should get its own non-JVM-based build system,
    // since it IS the obvious choice if using Java as a source langauge in the future.
    id("software.amazon.smithy.gradle.polymorph").version("0.1.0")
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

    // The Dafny project can make use of replaceable modules
    // and may include per-language externs as input.
    // The polymorph plugin will take those into account as input,
    // copying as necessary into the right places in build/polymorph/<lang>/...
}

// Note this is necessary because it doesn't make sense to cram all configuration into smithy-build.json,
// because `smithy build` fundamentally only wants Smithy models as input,
// but polymorphing needs actual implementation code/projects as well.
// TODO: legacy tasks to copy from build back into src, 
polymorph {
    // TODO: Somewhat redundant with smithy-build.json and the metadata in the Dafny API dependencies.
    // But on the other hand, being explicit is probably better.
    services.add("aws.cryptography.materialProviders#AwsCryptographicMaterialProviders")
    services.add("aws.cryptography.keyStore#KeyStore")

    // Configures what target languages this build will polymorph for you,
    // into paths like build/polymorph/java/...
    //
    // Note that the project doesn't have to be responsible for or even aware of
    // all the languages it is polymorphed to.
    // This section is just for the targets you want to build and test directly.
    // Probably more useful for test models rather than production libraries.
    // In the limit we should get trebuchet/catapult to take this project as input
    // and configure all the targets itself,
    // and copy from build/polymorph/java/* to where it's actually checked in to the repo.
    //
    // If some other team needs a missing language, they should be able to build it themselves
    // using their own project that references this one.
    // The metadata that Smithy/Polymorph will add to artifacts means that
    // polymorphing can discover polymorphed dependencies
    // without caring who built them.
    //
    // This is already true for using AWS SDKs as polymorph dependencies:
    // Crypto Tools is creating Dafny SDKs without AWS directly supporting it.
    targets {
      java {}
      rust {}
      python {}
      go {}
      net {}
    }

    // Pluggable mechanism for templating things like project files.
    // Follows the Smithy example: something like smithy-typescript
    // will create a simple, functional package.json,
    // whereas aws-sdk-js-v3 codegen will create a much richer one
    // with AWS branding/licensing/etc.
    conventions.set("aws.cryptography")
}

dependencies {
    // Smithy dependencies
    // TODO: Should be build-time only somehow? smithyBuild(...) doesn't work.
    implementation("software.amazon.smithy:smithy-model:1.28.0")
    implementation("software.amazon.smithy:smithy-aws-traits:1.28.0")
    implementation("software.amazon.smithy:smithy-rules-engine:1.28.0")

    // Smithy build plugin for library SDK codegen.
    // TODO: If we're really clever this could just be dafny-ssdk-codegen?
    smithyBuild("software.amazon.smithy.dafny:dafny-library-codegen:0.1.0")

    // Dafny API generated from Smithy model.
    // Will be a full project also using the Dafny Gradle plugin,
    // generated by the software.amazon.smithy.gradle.smithy-jar plugin,
    // and therefore located in build/smithyprojections/source/dafny-library-codegen/AwsCryptographicMaterialProviders-API.
    // This is very similar to the workflow for building Smithy-based servers, 
    // ala https://smithy.io/2.0/tutorials/full-stack-tutorial.html#generating-the-server-sdk
    api("aws.cryptography.materialProviders:Dafny-AwsCryptographicMaterialProviders-API:1.0-SNAPSHOT")
    api("aws.cryptography.keyStore:Dafny-AwsCryptographyKeyStore-API:1.0-SNAPSHOT")

    // Polymorphed dependencies.
    //
    // Most of these will already be transitive Dafny-*-API dependencies.
    // Polymorphing will automatically map them to their target language equivalent,
    // based on the metadata that Smithy/Polymorph adds.
    // In practice these will often be inside the same repo and configured as sister Gradle projects,
    // so it's easy to build recursively when you want to.
    //
    // Mapping of polymorph dependencies will be pluggable and have fallback behavior:
    // * The default case is using convention to map the Smithy service name to the target language library artifact name,
    //   but then verifying the artifact is correct by looking for the Smithy/Polymorph-attached metadata.
    // * If that fails, use target language repository indexing to look for a package that includes a known class/type,
    //   like software.amazon.cryptography.primitives.AtomicPrimitives.
    //   This allows polymorph libraries to be merged as they are polymorphed,
    //   while still allowing source dependencies to the components.
    // * Can also provide custom indexing, if you really need to fallback to registering with some system
    //   whenever a polymorphed library is published somewhere,
    //   but better to avoid additional stateful systems.
    //
    // This mapping can be cached locally by Gradle, and/or locked down with some kind of lock file.\
    //
    // Dafny itself could use a similar approach, if Dafny projects have an identity
    // and a convention for mapping those identifiers to target language ecosystems.

    implementation("software.amazon.awssdk:Dafny-DynamoDB:1.0-SNAPSHOT")
    implementation("software.amazon.awssdk:Dafny-KMS:1.0-SNAPSHOT")
    implementation("aws.cryptography.primitives:Dafny-AwsCryptographicPrimitives:1.0-SNAPSHOT")
}