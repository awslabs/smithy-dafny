// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package software.amazon.polymorph.smithypython;

import static software.amazon.smithy.dafny.codegen.TestUtils.make;

import java.nio.file.Path;
import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;
import software.amazon.polymorph.CodegenEngine;
import software.amazon.polymorph.TestModelTest;
import software.amazon.polymorph.smithydafny.DafnyVersion;

class PythonTestModels extends TestModelTest {

  private static final Set<String> DISABLED_TESTS = new HashSet<>();

  static {
    DISABLED_TESTS.add("AggregateReferences");
    DISABLED_TESTS.add("CallingAWSSDKFromLocalService");
    DISABLED_TESTS.add("LanguageSpecificLogic");
    DISABLED_TESTS.add("MultipleModels");
    DISABLED_TESTS.add("SimpleTypes/BigDecimal");
    DISABLED_TESTS.add("SimpleTypes/BigInteger");
    DISABLED_TESTS.add("SimpleTypes/SimpleByte");
    DISABLED_TESTS.add("SimpleTypes/SimpleDocument");
    DISABLED_TESTS.add("SimpleTypes/SimpleFloat");
    DISABLED_TESTS.add("SimpleTypes/SimpleShort");
    DISABLED_TESTS.add("SimpleTypes/SimpleTimestamp");
    DISABLED_TESTS.add("Streaming");
    DISABLED_TESTS.add("SQSExtended");
    DISABLED_TESTS.add("aws-sdks/ddb-lite");
    DISABLED_TESTS.add("aws-sdks/glue");
    DISABLED_TESTS.add("aws-sdks/lakeformation");
    DISABLED_TESTS.add("aws-sdks/kms-lite");
    DISABLED_TESTS.add("aws-sdks/sqs");
    DISABLED_TESTS.add("aws-sdks/sqs-via-cli");
    //TODO: Add support for Recursive shapes.
    DISABLED_TESTS.add("RecursiveShape");
  }

  @ParameterizedTest
  @MethodSource("discoverTestModels")
  void testModelsForPython(String relativeTestModelPath) {
    Assumptions.assumeFalse(DISABLED_TESTS.contains(relativeTestModelPath));

    // The @streaming support depends on our subset of the Dafny standard libraries
    // which cannot be built for old versions of Dafny.
    if (
      relativeTestModelPath.endsWith("Streaming") ||
      relativeTestModelPath.endsWith("s3")
    ) {
      DafnyVersion dafnyVersion = CodegenEngine.getDafnyVersionFromDafny();
      if (dafnyVersion.compareTo(DafnyVersion.parse("4.9.0")) < 0) {
        Assumptions.assumeTrue(false);
      }
    }

    Path testModelPath = getTestModelPath(relativeTestModelPath);
    make(testModelPath, "setup_prettier");
    make(testModelPath, "polymorph_dafny");
    make(testModelPath, "polymorph_python");
    make(testModelPath, "transpile_python");
    make(testModelPath, "test_python");
  }
}
