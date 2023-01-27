// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package software.amazon.polymorph;

import software.amazon.polymorph.smithydotnet.localServiceWrapper.LocalServiceWrappedCodegen;
import software.amazon.polymorph.smithydotnet.localServiceWrapper.LocalServiceWrappedConversionCodegen;
import software.amazon.polymorph.smithydotnet.localServiceWrapper.LocalServiceWrappedShimCodegen;
import software.amazon.polymorph.smithyjava.generator.library.JavaLibrary;
import software.amazon.polymorph.utils.ModelUtils;

import org.apache.commons.cli.CommandLine;
import org.apache.commons.cli.DefaultParser;
import org.apache.commons.cli.HelpFormatter;
import org.apache.commons.cli.Option;
import org.apache.commons.cli.Options;
import org.apache.commons.cli.ParseException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import software.amazon.polymorph.smithydafny.DafnyApiCodegen;
import software.amazon.polymorph.smithydotnet.AwsSdkShimCodegen;
import software.amazon.polymorph.smithydotnet.AwsSdkTypeConversionCodegen;
import software.amazon.polymorph.smithydotnet.ServiceCodegen;
import software.amazon.polymorph.smithydotnet.ShimCodegen;
import software.amazon.polymorph.smithydotnet.TypeConversionCodegen;
import software.amazon.polymorph.smithyjava.generator.awssdk.JavaAwsSdkV1;
import software.amazon.polymorph.utils.TokenTree;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.loader.ModelAssembler;
import software.amazon.smithy.model.shapes.ServiceShape;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Arrays;

public class CodegenCli {
    private static final Logger logger = LoggerFactory.getLogger(CodegenCli.class);

    public static void main(String[] args) {
        Optional<CliArguments> cliArgumentsOptional = Optional.empty();
        try {
            cliArgumentsOptional = CliArguments.parse(args);
        } catch (ParseException e) {
            logger.error("Command-line arguments could not be parsed", e);
        }
        if (cliArgumentsOptional.isEmpty()) {
            printHelpMessage();
            System.exit(0);
        }
        final CliArguments cliArguments = cliArgumentsOptional.get();

        try {
            if (cliArguments.outputDotnetDir.isPresent()) {
                Files.createDirectories(cliArguments.outputDotnetDir.get());
            }
            if (cliArguments.outputJavaDir.isPresent()) {
                Files.createDirectories(cliArguments.outputJavaDir.get());
            }
        } catch (IOException e) {
            e.printStackTrace();
            System.exit(1);
        }

        final ModelAssembler assembler = new ModelAssembler();

        assembler.addImport(cliArguments.modelPath);
        Arrays
                .stream(cliArguments.dependentModelPaths)
                .forEach(assembler::addImport);

        final Model model = assembler
                .assemble()
                .unwrap();

        final ServiceShape serviceShape = ModelUtils.serviceFromNamespace(model, cliArguments.namespace);
        final List<String> messages = new ArrayList<>(3);

        if (cliArguments.outputJavaDir.isPresent()) {
            final Path outputJavaDir = cliArguments.outputJavaDir.get();
            if (cliArguments.awsSdkStyle) {
                messages.add(javaAwsSdkV1(outputJavaDir, serviceShape, model));
            } else {
                messages.add(javaLocalService(outputJavaDir, serviceShape, model));
            }
        }

        if (cliArguments.outputDotnetDir.isPresent()) {
            final Path outputDotnetDir = cliArguments.outputDotnetDir.get();
            if (cliArguments.awsSdkStyle) {
                messages.add(netAwsSdk(outputDotnetDir, serviceShape, model, cliArguments.dependentModelPaths));
            } else if (cliArguments.outputLocalServiceTest.isPresent()) {
                messages.add(netWrappedLocalService(outputDotnetDir, serviceShape, model, cliArguments.dependentModelPaths));
            } else {
                messages.add(netLocalService(outputDotnetDir, serviceShape, model));
            }
        }

        if (cliArguments.outputDafny) {
            final DafnyApiCodegen dafnyApiCodegen = new DafnyApiCodegen(
                    model,
                    serviceShape,
                    cliArguments.modelPath,
                    cliArguments.includeDafnyFile.get(),
                    cliArguments.dependentModelPaths
            );

            if (cliArguments.outputLocalServiceTest.isPresent()) {
                writeTokenTreesIntoDir(dafnyApiCodegen.generateWrappedAbstractServiceModule(
                        cliArguments.outputLocalServiceTest.get()),
                        cliArguments.outputLocalServiceTest.get()
                );
                messages.add("Dafny that tests a local service generated in %s".formatted(cliArguments.outputLocalServiceTest.get()));
            } else {
                // The Smithy model and the Dafny code are expected to be in the same location.
                // This simplifies the process of including the various Dafny files.
                writeTokenTreesIntoDir(dafnyApiCodegen.generate(), cliArguments.modelPath);
                messages.add("Dafny code generated in %s".formatted(cliArguments.modelPath));
            }
        }

        logger.info("\n");
        messages.forEach(logger::info);
    }

    private static String javaLocalService(Path outputJavaDir, ServiceShape serviceShape, Model model) {
        final JavaLibrary javaLibrary = new JavaLibrary(model, serviceShape);
        writeTokenTreesIntoDir(javaLibrary.generate(), outputJavaDir);
        return "Java code generated in %s".formatted(outputJavaDir);
    }

    //TODO: Figure out a nice way to differentiate AWS SDK Java V1 from AWS SDK Java V2
    // Or maybe we just hard code one or the other and call that good enough
    static String javaAwsSdkV1(Path outputJavaDir, ServiceShape serviceShape, Model model) {
        final JavaAwsSdkV1 javaShimCodegen = JavaAwsSdkV1.createJavaAwsSdkV1(serviceShape, model);
        writeTokenTreesIntoDir(javaShimCodegen.generate(), outputJavaDir);
        return "Java code generated in %s".formatted(outputJavaDir);
    }

    static String netLocalService(Path outputNetDir, ServiceShape serviceShape, Model model) {
        final ServiceCodegen service = new ServiceCodegen(model, serviceShape);
        writeTokenTreesIntoDir(service.generate(), outputNetDir);

        final ShimCodegen shim = new ShimCodegen(model, serviceShape);
        writeTokenTreesIntoDir(shim.generate(), outputNetDir);

        final TypeConversionCodegen conversion = new TypeConversionCodegen(model, serviceShape);
        writeTokenTreesIntoDir(conversion.generate(), outputNetDir);
        return ".NET code generated in %s".formatted(outputNetDir);
    }

    static String netWrappedLocalService(Path outputNetDir, ServiceShape serviceShape, Model model, Path[] dependentModelPaths) {
        final LocalServiceWrappedCodegen service = new LocalServiceWrappedCodegen(model, serviceShape);
        writeTokenTreesIntoDir(service.generate(), outputNetDir);

        final LocalServiceWrappedShimCodegen wrappedShim = new LocalServiceWrappedShimCodegen(
                model, serviceShape, dependentModelPaths);
        writeTokenTreesIntoDir(wrappedShim.generate(), outputNetDir);

        final TypeConversionCodegen conversion = new LocalServiceWrappedConversionCodegen(model, serviceShape);
        writeTokenTreesIntoDir(conversion.generate(), outputNetDir);
        return ".NET code generated in %s".formatted(outputNetDir);
    }

    static String netAwsSdk(Path outputNetDir, ServiceShape serviceShape, Model model, Path[] dependentModelPaths) {
        final AwsSdkShimCodegen dotnetShimCodegen = new AwsSdkShimCodegen(
                model, serviceShape, dependentModelPaths);
        writeTokenTreesIntoDir(dotnetShimCodegen.generate(), outputNetDir);

        final TypeConversionCodegen conversion = new AwsSdkTypeConversionCodegen(model, serviceShape);
        writeTokenTreesIntoDir(conversion.generate(), outputNetDir);
        return ".NET code generated in %s".formatted(outputNetDir);
    }

    private static Options getCliOptions() {
        return new Options()
          .addOption(Option.builder("h")
            .longOpt("help")
            .desc("print help message")
            .build())
          .addOption(Option.builder("m")
            .longOpt("model")
            .desc("directory for the model file[s] (.smithy format). Also the Dafny output directory.")
            .hasArg()
            .required()
            .build())
          .addOption(Option.builder("d")
            .longOpt("dependent-model")
            .desc("directory for dependent model file[s] (.smithy format)")
            .hasArg()
            .required()
            .build())
          .addOption(Option.builder("n")
            .longOpt("namespace")
            .desc("smithy namespace to generate code for, such as 'com.foo'")
            .hasArg()
            .required()
            .build())
          .addOption(Option.builder()
            .longOpt("output-dotnet")
            .desc("<optional> output directory for generated .NET files")
            .hasArg()
            .build())
          .addOption(Option.builder()
            .longOpt("output-java")
            .desc("<optional> output directory for generated Java files")
            .hasArg()
            .build())
          .addOption(Option.builder()
            .longOpt("aws-sdk")
            .desc("<optional> generate AWS SDK-style API and shims")
            .build())
          .addOption(Option.builder()
            .longOpt("output-local-service-test")
            .desc("<optional> output directory for generated Dafny that tests a local service")
            .hasArg()
            .build())
          .addOption(Option.builder()
            .longOpt("output-dafny")
            .desc("<optional> generate Dafny code")
            .build())
          .addOption(Option.builder()
            .longOpt("include-dafny")
            .desc("<optional> file to be include in the Dafny model file")
            .hasArg()
            .build());
    }

    private static void printHelpMessage() {
        new HelpFormatter().printHelp("smithy-polymorph", getCliOptions());
    }

    private record CliArguments(
            Path modelPath,
            Path[] dependentModelPaths,
            String namespace,
            Optional<Path> outputDotnetDir,
            Optional<Path> outputJavaDir,
            boolean awsSdkStyle,
            Optional<Path> outputLocalServiceTest,
            boolean outputDafny,
            Optional<Path> includeDafnyFile
    ) {
        /**
         * @param args arguments to parse
         * @return parsed arguments, or {@code Optional.empty()} if help should be printed
         * @throws ParseException if command line arguments are invalid
         */
        static Optional<CliArguments> parse(String[] args) throws ParseException {
            final DefaultParser parser = new DefaultParser();
            final CommandLine commandLine = parser.parse(getCliOptions(), args);
            if (commandLine.hasOption("h")) {
                return Optional.empty();
            }

            final Path modelPath = Path.of(commandLine.getOptionValue('m'));

            final Path[] dependentModelPaths = Arrays
              .stream(commandLine.getOptionValues('d'))
              .map(Path::of)
              .toArray(Path[]::new);

            final String namespace = commandLine.getOptionValue('n');
            Optional<Path> outputDotnetDir = Optional.empty();
            if (commandLine.hasOption("output-dotnet")) {
                outputDotnetDir = Optional.of(Paths.get(commandLine.getOptionValue("output-dotnet"))
                        .toAbsolutePath().normalize());
            }

            Optional<Path> outputJavaDir = Optional.empty();
            if (commandLine.hasOption("output-java")) {
                 outputJavaDir = Optional.of(Paths.get(commandLine.getOptionValue("output-java"))
                         .toAbsolutePath().normalize());
            }

            final boolean awsSdkStyle = commandLine.hasOption("aws-sdk");
            if (awsSdkStyle && commandLine.hasOption("output-local-service-test")) {
                throw new ParseException("Can not have aws sdk style and test a local service.");
            }

            Optional<Path> outputLocalServiceTest = Optional.empty();
            if (commandLine.hasOption("output-local-service-test")) {
                outputLocalServiceTest = Optional.of(Paths.get(commandLine.getOptionValue("output-local-service-test"))
                                                    .toAbsolutePath().normalize());
            }

            final boolean outputDafny = commandLine.hasOption("output-dafny");
            Optional<Path> includeDafnyFile = Optional.empty();
            if (outputDafny) {
                if (!commandLine.hasOption("include-dafny")) {
                    // if outputing dafny, an include file is required
                    // if outputting dafny, an include file is required
                    throw new ParseException("Dafny requires an include file.");
                }
                includeDafnyFile = Optional.of(Paths.get(commandLine.getOptionValue("include-dafny"))
                                                       .toAbsolutePath().normalize());
            }
            return Optional.of(new CliArguments(
                    modelPath,
                    dependentModelPaths,
                    namespace,
                    outputDotnetDir,
                    outputJavaDir,
                    awsSdkStyle,
                    outputLocalServiceTest,
                    outputDafny,
                    includeDafnyFile
            ));
        }
    }

    private static void writeToFile(final String text, final File file) {
        try {
            /*if (!file.createNewFile()) {
                logger.warn("Overwriting existing file {}", file);
            }*/
            final FileWriter fileWriter = new FileWriter(file);
            fileWriter.write(text);
            if (!text.endsWith("\n")) {
                fileWriter.write("\n");
            }
            fileWriter.close();
        } catch (IOException e) {
            logger.error("Could not write to file {}", file);
            e.printStackTrace();
        }
    }

    private static final String COPYRIGHT_HEADER = """
            // Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
            // SPDX-License-Identifier: Apache-2.0
            """;

    public static final String GENERATED_HEADER = """
            // Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
            """;

    private static void writeTokenTreesIntoDir(final Map<Path, TokenTree> tokenTreeMap, final Path outputDir) {
        for (Map.Entry<Path, TokenTree> entry : tokenTreeMap.entrySet()) {
            Path localPath = entry.getKey();
            TokenTree tokens = entry.getValue();
            final Path outputPath = Path.of(outputDir.toString(), localPath.toString());
            try {
                Files.createDirectories(outputPath.getParent());
                final String content = COPYRIGHT_HEADER + GENERATED_HEADER + tokens.toString();
                writeToFile(content, outputPath.toFile());
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}
