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
    String funcNameWithOutSuffix = memberShape.getId().toString().replaceAll("[.$#]", "_");
    Shape containerShape = model.expectShape(memberShape.getContainer());
    if (containerShape.hasTrait(PositionalTrait.class)) {
      funcNameWithOutSuffix = CaseUtils.toPascalCase(
        funcNameWithOutSuffix
      );
    }
    return funcNameWithOutSuffix.concat("_").concat(suffix);
  }
}
