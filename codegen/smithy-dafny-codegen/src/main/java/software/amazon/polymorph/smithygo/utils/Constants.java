package software.amazon.polymorph.smithygo.utils;

import java.util.Map;
import software.amazon.polymorph.traits.PositionalTrait;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.utils.StringUtils;

public class Constants {

  public static final String DAFNY_RUNTIME_GO_LIBRARY_MODULE =
    "github.com/dafny-lang/DafnyRuntimeGo/v4";
  private static final Map<String, String> DEFAULT_VALUES = Map.ofEntries(
    Map.<String, String>entry("int32", "0"),
    Map.<String, String>entry("string", ""),
    Map.<String, String>entry("[]byte", "[]byte(\"\")"),
    Map.<String, String>entry("int64", "0"),
    Map.<String, String>entry("float64", "0"),
    Map.<String, String>entry("bool", "false")
  );

  /**
   * Gets the default value for a specific Smithy type.
   *
   * @param smithyType The Smithy type to get the default value for
   * @return The default value for the specified type, or null if the type is not found
   */
  public static String getDefaultValueForSmithyType(final String smithyType) {
    return DEFAULT_VALUES.getOrDefault(smithyType, "nil");
  }

  // TODO: Is it possible to make this function name shorter and in camelCase?
  /**
   * Generates a function name for memberShapes.
   * Generates private function for all shape excepts memberShape whose containerShape has positional trait
   *
   * @param memberShape The visiting MemberShape
   * @param suffix A string to be appended at the end of the generated function name
   * @param model The smithy model being used
   * @return A string representing the generated function name
   */
  public static String funcNameGenerator(
    final MemberShape memberShape,
    final String suffix,
    final Model model
  ) {
    String funcName = funcNameGenerator(memberShape, suffix);
    final Shape containerShape = model.expectShape(memberShape.getContainer());
    // membershape inside a container shape with positional trait has to be exposed.
    if (containerShape.hasTrait(PositionalTrait.class)) {
      funcName = StringUtils.capitalize(funcName);
    }
    return funcName;
  }

  /**
   * Generates a function name for memberShapes.
   * Always generates private function for all shape
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
