// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

plugins {
    id("software.amazon.smithy").version("0.6.0")
}

repositories {
    mavenLocal()
    mavenCentral()
}

dependencies {
    implementation("software.amazon.smithy:smithy-model:1.28.0")
    implementation("software.amazon.smithy:smithy-aws-traits:1.28.0")
    implementation("software.amazon.smithy:smithy-rules-engine:1.28.0")

    // Must be built and published to the local Maven repo
    implementation("software.amazon.smithy.dafny:smithy-dafny-codegen:0.1.0")
}

configure<software.amazon.smithy.gradle.SmithyExtension> {
    // Uncomment this to use a custom projection when building the JAR.
    // projection = "foo"
}

// Uncomment to disable creating a JAR.
tasks["jar"].enabled = false

tasks.register("polymorphDafny") {
    dependsOn("build")
    doLast {
        // if needed, specify a projection to use instead
        // default (no projection) is "source"
        val projectionName = "source"
        copy {
            from(layout.buildDirectory.dir("smithyprojections/" + project.name + "/" + projectionName + "/dafny-client-codegen/Model/"))
            into("model")
        }
        exec {
            // need to adjust the relative import, since we're copying it away
            // the commandLine method does not play nice with sed,
            // so we have to execute it through bash :(
            commandLine("bash", "-c", "sed '4s|../../../../../../../../dafny-dependencies/StandardLibrary/src/Index.dfy|../../../dafny-dependencies/StandardLibrary/src/Index.dfy|' model/ComAmazonawsS3Types.dfy > model/tmp && mv model/tmp model/ComAmazonawsS3Types.dfy")
        }
    }
}

tasks.register("polymorphDotnet") {
    dependsOn("build")
    doLast {
        // if needed, specify a projection to use instead
        // default (no projection) is "source"
        val projectionName = "source"
        copy {
            // build plugin calls it "dotnet" and CLI calls it "net"
            from(layout.buildDirectory.dir("smithyprojections/" + project.name + "/" + projectionName + "/dafny-client-codegen/runtimes/dotnet"))
            into("runtimes/net")
        }
    }
}

buildscript {
    val smithyVersion: String by project

    repositories {
        mavenCentral()
    }
    dependencies {
        "classpath"("software.amazon.smithy:smithy-cli:$smithyVersion")
    }
}
