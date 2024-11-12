package software.amazon.polymorph.smithygo.utils;

import software.amazon.polymorph.traits.PositionalTrait;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.utils.CaseUtils;

public class Constants {

  public static final String DAFNY_RUNTIME_GO_LIBRARY_MODULE =
    "github.com/dafny-lang/DafnyRuntimeGo/v4";

  // TODO: Is it possible to make this function name shorter and in camelCase?
  /**
   * Generates a function name for shape visitors for AWS SDK and localservice.
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
    String memberShapeId = memberShape.getId().toString();
    Shape containerShape = model.expectShape(memberShape.getContainer());
    // membershape inside a container shape with positional trait has to be exposed.
    if (containerShape.hasTrait(PositionalTrait.class)) {
      memberShapeId = CaseUtils.toPascalCase(memberShapeId);
    }
    return memberShapeId.replaceAll("[.$#]", "_").concat("_").concat(suffix);
  }

  /**
   * Generates a function name for shape visitors for AWS SDK and localservice.
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
