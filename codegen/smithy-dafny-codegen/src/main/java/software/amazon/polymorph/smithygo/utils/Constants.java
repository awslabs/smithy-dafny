package software.amazon.polymorph.smithygo.utils;

import java.util.Map;
import software.amazon.smithy.model.shapes.MemberShape;

public class Constants {

  public static final String DAFNY_RUNTIME_GO_LIBRARY_MODULE =
    "github.com/dafny-lang/DafnyRuntimeGo/v4";
  
  private static final Map<String, String> DEFAULT_VALUES = Map.of(
    "int32", "0",
    "string", "",
    "[]byte", "[0]",
    "int64", "0",
    "float64", "0",
    "bool", "false"
  );

  /**
   * Gets the default value for a specific Smithy type.
   *
   * @param smithyType The Smithy type to get the default value for
   * @return The default value for the specified type, or null if the type is not found
   */
  public static String getDefaultValueForSmithyType(String smithyType) {
    return DEFAULT_VALUES.getOrDefault(smithyType, "nil");
  }

  // TODO: Is it possible to make this function name shorter and in camelCase?
  /**
   * Generates a function name for shape visitors for AWS SDK and localservice.
   *
   * @param memberShape The visiting MemberShape
   * @param suffix A string to be appended at the end of the generated function name
   * @return A string representing the generated function name
   */
  public static String funcNameGenerator(
    final MemberShape memberShape,
    final String suffix
  ) {
    return memberShape
      .getId()
      .toString()
      .replaceAll("[.$#]", "_")
      .concat("_")
      .concat(suffix);
  }
}
