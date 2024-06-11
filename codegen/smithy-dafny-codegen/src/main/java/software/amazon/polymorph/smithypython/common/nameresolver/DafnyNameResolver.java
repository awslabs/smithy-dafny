// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package software.amazon.polymorph.smithypython.common.nameresolver;

import org.assertj.core.util.Strings;
import software.amazon.polymorph.smithypython.awssdk.nameresolver.AwsSdkNameResolver;
import software.amazon.polymorph.traits.LocalServiceTrait;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.ResourceShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.StringShape;
import software.amazon.smithy.model.shapes.UnionShape;
import software.amazon.smithy.model.traits.EnumTrait;
import software.amazon.smithy.model.traits.ErrorTrait;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonWriter;
import software.amazon.smithy.utils.CaseUtils;

import static software.amazon.polymorph.smithydafny.DafnyNameResolver.dafnyTypesModuleName;

/**
 * Contains utility functions that map Smithy shapes to an expected corresponding string generated
 * by Dafny's Python compiler. i.e. strings in this file match behavior of Dafny-generated code.
 */
public class DafnyNameResolver {

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `types`
   * module from the same Dafny project for the provided Shape. ex. example.namespace.ExampleShape
   * -> "ExampleNamespaceTypes"
   *
   * @param shape
   * @return
   */
  public static String getDafnyPythonTypesModuleNameForShape(Shape shape, GenerationContext context) {
    return getDafnyPythonTypesModuleNameForShape(shape.getId(), context);
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `types`
   * module from the same Dafny project for the provided Shape. ex. example.namespace.ExampleShape
   * -> "ExampleNamespaceTypes"
   *
   * @param shapeId
   * @return
   */
  public static String getDafnyPythonTypesModuleNameForShape(ShapeId shapeId, GenerationContext context) {
    return getDafnyTypesModuleNameForSmithyNamespace(shapeId.getNamespace(), context);
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `index`
   * module from the same Dafny project for the provided Shape. ex. example.namespace.ExampleShape
   * -> "ExampleNamespace"
   *
   * @param shape
   * @return
   */
  public static String getDafnyPythonIndexModuleNameForShape(Shape shape, GenerationContext context) {
    return getDafnyPythonIndexModuleNameForShape(shape.getId(), context);
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `index`
   * module from the same Dafny project for the provided Shape. ex. example.namespace.ExampleShape
   * -> "ExampleNamespace"
   *
   * @param shapeId
   * @return
   */
  public static String getDafnyPythonIndexModuleNameForShape(ShapeId shapeId, GenerationContext context) {
    return getDafnyIndexModuleNameForSmithyNamespace(shapeId.getNamespace(), context);
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `index`
   * module from the same Dafny project for the provided smithyNamespace. ex.
   * example.namespace -> "example_module.internaldafny.generated"
   *
   * @param smithyNamespace
   * @return
   */
  public static String getDafnyGeneratedPathForSmithyNamespace(String smithyNamespace) {
    // If this is an AWS SDK shape, rewrite its namespace to match the Dafny extern namespace
    return SmithyNameResolver.getPythonModuleNameForSmithyNamespace(smithyNamespace) + ".internaldafny.generated";
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `index`
   * module from the same Dafny project for the provided smithyNamespace. ex.
   * example.namespace.ExampleShape -> "example_module.internaldafny.generated.ExampleNamespace"
   *
   * @param smithyNamespace
   * @return
   */
  public static String getDafnyIndexModuleNameForSmithyNamespace(String smithyNamespace, GenerationContext context) {
    String indexModuleName = "";

    if (AwsSdkNameResolver.isAwsSdkNamespace(smithyNamespace)) {
      // If this is an AWS SDK shape, rewrite its namespace to match the Dafny extern namespace
      // ex. Com.Amazonaws.Kms -> Com_Amazonaws_Kms
      String[] namespaceSegments = smithyNamespace.split("\\.");
      StringBuilder output = new StringBuilder();
      for (String segment : namespaceSegments) {
        output.append(CaseUtils.toPascalCase(segment));
        if (!segment.equals(namespaceSegments[namespaceSegments.length - 1])) {
          output.append("_");
        }
      }
      indexModuleName = output.toString();
    } else {
      // For localService, index module name SHOULD be the defined sdkId on the localService trait
      for (ServiceShape serviceShape : context.model().getServiceShapes()) {
        if (smithyNamespace.equals(serviceShape.getId().getNamespace())) {
          indexModuleName = serviceShape.expectTrait(LocalServiceTrait.class).getSdkId();
          break;
        }
      }
    }

    if (Strings.isNullOrEmpty(indexModuleName)) {
      throw new IllegalArgumentException("Could not determine index module name for " + smithyNamespace);
    }
    return getDafnyGeneratedPathForSmithyNamespace(smithyNamespace) + "." +
            indexModuleName;
  }

  /**
   * Returns the name of the Python module containing Dafny-generated Python code from the `types`
   * module from the same Dafny project for the provided smithyNamespace. ex.
   * example.namespace -> "example_module.internaldafny.generated.ExampleNamespaceTypes"
   *
   * @param smithyNamespace
   * @return
   */
  public static String getDafnyTypesModuleNameForSmithyNamespace(String smithyNamespace, GenerationContext context) {
    return getDafnyGeneratedPathForSmithyNamespace(smithyNamespace) + "." + dafnyTypesModuleName(smithyNamespace);
  }

  /**
   * Returns a String representing the corresponding Dafny type for the provided shape. This MUST
   * NOT be used for errors; for errors use `getDafnyTypeForError`. ex.
   * example.namespace.ExampleShape -> "DafnyExampleShape"
   *
   * @param shapeId
   * @return
   */
  public static String getDafnyTypeForShape(ShapeId shapeId) {
    if (SmithyNameResolver.isUnitShape(shapeId)) {
      // Dafny models Unit shapes as the Python `None` type
      return "None";
    } else {
      // Catch-all: Return `Dafny[shapeName]`
      return "Dafny" + shapeId.getName();
    }
  }

  /**
   * Returns a String representing the Dafny-generated Python type corresponding to the provided
   * Shape. ex. example.namespace.ExampleShape -> "DafnyExampleShape"
   *
   * @param shape
   * @return
   */
  public static String getDafnyTypeForShape(Shape shape) {
    return getDafnyTypeForShape(shape.getId());
  }

  /**
   * Returns a String representing the Dafny-generated Python type corresponding to the provided
   * EnumShape. ex. example.namespace.ExampleShape -> "DafnyExampleShape"
   *
   * @param stringShape
   * @param enumValue
   * @return
   */
  public static String getDafnyTypeForStringShapeWithEnumTrait(
      StringShape stringShape, String enumValue) {
    if (!stringShape.hasTrait(EnumTrait.class) || !stringShape.isStringShape()) {
      throw new IllegalArgumentException(
          "Argument is not a StringShape with EnumTrait: " + stringShape.getId());
    }

    return stringShape.getId().getName() + "_" + enumValue.replace("_", "__");
  }

  public static void importDafnyTypeForStringShapeWithEnumTrait(
      PythonWriter writer, StringShape stringShape, String enumValue, GenerationContext context) {
    if (!stringShape.hasTrait(EnumTrait.class)) {
      throw new IllegalArgumentException(
          "Argument is not a StringShape with EnumTrait: " + stringShape.getId());
    }

    // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
    // dependencies
    writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(stringShape.getId().getNamespace()) + ".module_");
    writer.addStdlibImport(
        getDafnyTypesModuleNameForSmithyNamespace(stringShape.getId().getNamespace(), context),
        getDafnyTypeForStringShapeWithEnumTrait(stringShape, enumValue));
  }

  /**
   * Imports the Dafny-generated Python type corresponding to the provided shape. ex.
   * example.namespace.ExampleShape -> "from example_namespace_internaldafny_types import
   * DafnyExampleShape"
   *
   * @param shape
   * @return
   */
  private static void importDafnyTypeForShape(
      PythonWriter writer, Shape shape, GenerationContext context) {
    importDafnyTypeForShape(writer, shape.getId(), context);
  }

  /**
   * Calls writer.addImport to import the corresponding Dafny type for the provided Smithy ShapeId.
   * DafnyExampleShape"
   *
   * @param writer
   * @param shapeId
   */
  public static void importDafnyTypeForShape(
      PythonWriter writer, ShapeId shapeId, GenerationContext context) {

    if (SmithyNameResolver.isUnitShape(shapeId)) {
      // No corresponding Dafny type for unit. Dafny uses "None", which does not need to be imported
      return;
    }

    else if (context.model().expectShape(shapeId).hasTrait(ErrorTrait.class)) {
      importDafnyTypeForError(writer, shapeId, context);
    }

    else {
      // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
      // dependencies
      writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(shapeId.getNamespace()) + ".module_");
      String name = shapeId.getName();
      writer.addStdlibImport(
          getDafnyPythonTypesModuleNameForShape(shapeId, context),
          name.replace("_", "__") + "_" + name.replace("_", "__"),
          getDafnyTypeForShape(shapeId));
    }
  }

  /**
   * Returns a String representing the client interface type for the provided serviceShape as Dafny
   * models the interface type. ex. example.namespace.ExampleService -> "IExampleServiceClient"
   *
   * @param serviceShape
   * @return
   */
  public static String getDafnyClientInterfaceTypeForServiceShape(ServiceShape serviceShape) {
    if (AwsSdkNameResolver.isAwsSdkShape(serviceShape)) {
      return "I" + AwsSdkNameResolver.clientNameForService(serviceShape);
    } else {
      return "I" + getDafnyClientTypeForServiceShape(serviceShape);
    }
  }

  /**
   * Returns a String representing the client interface type for the provided serviceShape as Dafny
   * models the interface type. ex. example.namespace.ExampleService -> "ExampleServiceClient"
   *
   * @param serviceShape
   * @return
   */
  public static String getDafnyClientTypeForServiceShape(ServiceShape serviceShape) {
    return serviceShape.getId().getName() + "Client";
  }

  /**
   * Returns a String representing the interface type for the provided resourceShape as Dafny models
   * the interface type. ex. example.namespace.ExampleResource -> "IExampleResource"
   *
   * @param resourceShape
   * @return
   */
  public static String getDafnyInterfaceTypeForResourceShape(ResourceShape resourceShape) {
    return "I" + resourceShape.getId().getName();
  }

  /**
   * Imports the Dafny-generated Python type corresponding to the provided resourceShape.
   *
   * @param resourceShape ex. example.namespace.ExampleResource -> "from
   *     example_namespace_internaldafny_types import IExampleResource"
   * @return
   */
  public static void importDafnyTypeForResourceShape(
      PythonWriter writer, ResourceShape resourceShape, GenerationContext context) {
    // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
    // dependencies
    writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(resourceShape.getId().getNamespace()) + ".module_");
    writer.addStdlibImport(
        getDafnyPythonTypesModuleNameForShape(resourceShape.getId(), context),
        getDafnyInterfaceTypeForResourceShape(resourceShape));
  }

  /**
   * Imports the Dafny-generated Python type corresponding to the provided serviceShape. ex.
   * example.namespace.ExampleService -> "from example_namespace_internaldafny_types import
   * IExampleServiceClient"
   *
   * @param serviceShape
   * @return
   */
  public static void importDafnyTypeForServiceShape(
      PythonWriter writer, ServiceShape serviceShape, GenerationContext context) {
    // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
    // dependencies
    writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(serviceShape.getId().getNamespace()) + ".module_");
    writer.addStdlibImport(
        getDafnyPythonTypesModuleNameForShape(serviceShape.getId(), context),
        getDafnyClientInterfaceTypeForServiceShape(serviceShape));
  }

  /**
   * Returns a String representing the corresponding Dafny type for the provided Error shape. This
   * MUST ONLY be used for errors; for other shapes use `getDafnyTypeForShape`. ex.
   * example.namespace.ExampleError -> "Error_ExampleError"
   *
   * @param shape
   * @return
   */
  public static String getDafnyTypeForError(Shape shape) {
    return getDafnyTypeForError(shape.getId());
  }

  /**
   * Returns a String representing the Dafny-generated Python type corresponding to the provided
   * error shape. ex. example.namespace.ExampleError -> "Error_ExampleError"
   *
   * @param shapeId
   * @return
   */
  public static String getDafnyTypeForError(ShapeId shapeId) {
    return "Error_" + shapeId.getName();
  }

  public static String escapeShapeName(String name) {
    if ("none".equalsIgnoreCase(name)) {
      return name + "_";
    }
    return name.replace("_", "__");
  }

  /**
   * Returns a String representing the corresponding Dafny type for the provided UnionShape and one
   * of its MemberShapes. This MUST ONLY be used for unions and their members; for other shapes use
   * `getDafnyTypeForShape`. ex. example.namespace.ExampleUnion:IntegerValue ->
   * "ExampleUnion_IntegerValue"
   *
   * @param unionShape
   * @param memberShape
   * @return
   */
  public static String getDafnyTypeForUnion(UnionShape unionShape, MemberShape memberShape) {
    return unionShape.getId().getName().replace("_", "__")
        + "_"
        + memberShape.getMemberName().replace("_", "__");
  }

  /**
   * Imports the Dafny-generated Python type corresponding to the provided unionShape. ex.
   * example.namespace.ExampleUnion:IntegerValue -> "from example_namespace_internaldafny_types
   * import ExampleUnion_IntegerValue"
   *
   * @param unionShape
   * @return
   */
  public static void importDafnyTypeForUnion(
      PythonWriter writer, UnionShape unionShape, MemberShape memberShape, GenerationContext context) {
    writer.addStdlibImport(
        getDafnyPythonTypesModuleNameForShape(unionShape, context),
        getDafnyTypeForUnion(unionShape, memberShape));
  }

  /**
   * Calls writer.addImport to import the corresponding Dafny type for the provided Smithy ShapeId.
   * This MUST ONLY be used for errors; for other shapes use `importDafnyTypeForShape`. ex.
   * example.namespace.ExampleUnion:IntegerValue -> "from example_namespace_internaldafny_types
   * import ExampleUnion_IntegerValue"
   *
   * @param writer
   * @param shapeId
   */
  public static void importDafnyTypeForError(
      PythonWriter writer, ShapeId shapeId, GenerationContext context) {
    if (!context.model().expectShape(shapeId).hasTrait(ErrorTrait.class)) {
      throw new IllegalArgumentException(
          "Must provide an error shape to importDafnyTypeForError. Provided " + shapeId);
    }
    // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
    // dependencies
    writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(shapeId.getNamespace()) + ".module_");
    writer.addStdlibImport(
        getDafnyPythonTypesModuleNameForShape(shapeId, context), getDafnyTypeForError(shapeId));
  }

  /**
   * Imports the generic Dafny error type for the provided namespace. ex. example.namespace -> "from
   * example_namespace_internaldafny_types import Error"
   *
   * @param writer
   * @param namespace
   */
  public static void importGenericDafnyErrorTypeForNamespace(
      PythonWriter writer, String namespace, GenerationContext context) {
    // When generating a Dafny import, must ALWAYS first import module_ to avoid circular
    // dependencies
    writer.addStdlibImport(getDafnyGeneratedPathForSmithyNamespace(namespace) + ".module_");
    writer.addStdlibImport(getDafnyTypesModuleNameForSmithyNamespace(namespace, context), "Error");
  }
}
