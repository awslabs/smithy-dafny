package software.amazon.polymorph.smithygo.awssdk;

import java.util.HashSet;
import java.util.Set;
import java.util.logging.Logger;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.knowledge.KnowledgeIndex;
import software.amazon.smithy.model.knowledge.NeighborProviderIndex;
import software.amazon.smithy.model.knowledge.NullableIndex;
import software.amazon.smithy.model.neighbor.NeighborProvider;
import software.amazon.smithy.model.neighbor.Relationship;
import software.amazon.smithy.model.neighbor.RelationshipType;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.ShapeType;
import software.amazon.smithy.model.shapes.ToShapeId;
import software.amazon.smithy.model.traits.EnumTrait;
import software.amazon.smithy.model.traits.StreamingTrait;
import software.amazon.smithy.utils.SetUtils;

/**
 * This class is modelled on the codegen/GoPointableIndex but has specific logic for dafny - aws-sdk shapes.
 */
public class AwsSdkGoPointableIndex implements KnowledgeIndex {

  public static final NullableIndex.CheckMode DEFAULT_CHECKMODE =
    NullableIndex.CheckMode.CLIENT_ZERO_VALUE_V1_NO_INPUT;

  private static final Logger LOGGER = Logger.getLogger(
    AwsSdkGoPointableIndex.class.getName()
  );

  // All types that are Go value types
  private static final Set<ShapeType> INHERENTLY_VALUE = SetUtils.of(
    ShapeType.BLOB,
    ShapeType.LIST,
    ShapeType.SET,
    ShapeType.MAP,
    ShapeType.UNION,
    ShapeType.DOCUMENT
  );

  // All types that are Go pointer types
  private static final Set<ShapeType> INHERENTLY_POINTABLE = SetUtils.of(
    ShapeType.BIG_DECIMAL,
    ShapeType.BIG_INTEGER
  );

  // All types that cannot be dereferenced
  private static final Set<ShapeType> INHERENTLY_NONDEREFERENCABLE =
    SetUtils.of(
      // built in slice/map
      ShapeType.BLOB,
      ShapeType.LIST,
      ShapeType.SET,
      ShapeType.MAP,
      // Interfaces
      ShapeType.UNION,
      ShapeType.DOCUMENT,
      // known pointer types.
      ShapeType.BIG_DECIMAL,
      ShapeType.BIG_INTEGER
    );

  // All types types that are comparable to nil
  private static final Set<ShapeType> INHERENTLY_NILLABLE = SetUtils.of(
    // built in slice/map
    ShapeType.BLOB,
    ShapeType.LIST,
    ShapeType.SET,
    ShapeType.MAP,
    // Interfaces
    ShapeType.UNION,
    ShapeType.DOCUMENT,
    // known pointer types.
    ShapeType.BIG_DECIMAL,
    ShapeType.BIG_INTEGER
  );

  private final Model model;
  private final NullableIndex nullableIndex;
  private final NullableIndex.CheckMode checkMode;
  private final Set<ShapeId> pointableShapes = new HashSet<>();
  private final Set<ShapeId> nillableShapes = new HashSet<>();
  private final Set<ShapeId> dereferencableShapes = new HashSet<>();

  public AwsSdkGoPointableIndex(
    Model model,
    NullableIndex.CheckMode checkMode
  ) {
    this.model = model;
    this.nullableIndex = NullableIndex.of(model);
    this.checkMode = checkMode;

    for (Shape shape : model.toSet()) {
      if (shape.asMemberShape().isPresent()) {
        MemberShape member = shape.asMemberShape().get();
        Shape targetShape = model.expectShape(member.getTarget());

        if (isMemberPointable(member, targetShape)) {
          pointableShapes.add(shape.getId());
        }
        if (isMemberNillable(member, targetShape)) {
          nillableShapes.add(shape.getId());
        }
        if (isMemberDereferencable(member, targetShape)) {
          dereferencableShapes.add(shape.getId());
        }
      } else {
        if (isShapePointable(shape)) {
          pointableShapes.add(shape.getId());
          nillableShapes.add(shape.getId());
        }
        if (isShapeNillable(shape)) {
          nillableShapes.add(shape.getId());
        }
        if (isShapeDereferencable(shape)) {
          dereferencableShapes.add(shape.getId());
        }
      }
    }
  }

  public AwsSdkGoPointableIndex(Model model) {
    this(model, DEFAULT_CHECKMODE);
  }

  public static AwsSdkGoPointableIndex of(Model model) {
    return model.getKnowledge(
      AwsSdkGoPointableIndex.class,
      AwsSdkGoPointableIndex::new
    );
  }

  public static AwsSdkGoPointableIndex of(
    Model model,
    NullableIndex.CheckMode checkMode
  ) {
    return model.getKnowledge(
      AwsSdkGoPointableIndex.class,
      model1 -> new AwsSdkGoPointableIndex(model1, checkMode)
    );
  }

  private boolean isMemberDereferencable(
    MemberShape member,
    Shape targetShape
  ) {
    return (
      !INHERENTLY_NONDEREFERENCABLE.contains(targetShape.getType()) &&
      isMemberPointable(member, targetShape)
    );
  }

  private boolean isMemberNillable(MemberShape member, Shape targetShape) {
    return (
      INHERENTLY_NILLABLE.contains(targetShape.getType()) ||
      isMemberPointable(member, targetShape)
    );
  }

  private boolean isMemberPointable(MemberShape member, Shape targetShape) {
    // Streamed blob shapes are never pointers because they are interfaces
    if (isBlobStream(targetShape)) {
      return false;
    }

    if (
      INHERENTLY_VALUE.contains(targetShape.getType()) ||
      isShapeEnum(targetShape)
    ) {
      return false;
    }

    return nullableIndex.isMemberNullable(member, checkMode);
  }

  private boolean isShapeDereferencable(Shape shape) {
    return (
      !INHERENTLY_NONDEREFERENCABLE.contains(shape.getType()) &&
      isShapePointable(shape)
    );
  }

  private boolean isShapeNillable(Shape shape) {
    return (
      INHERENTLY_NILLABLE.contains(shape.getType()) || isShapePointable(shape)
    );
  }

  private boolean isShapePointable(Shape shape) {
    // All operation input and output shapes are pointable.
    if (isOperationStruct(shape)) {
      return true;
    }

    // Streamed blob shapes are never pointers because they are interfaces
    if (isBlobStream(shape)) {
      return false;
    }

    if (shape.isServiceShape()) {
      return true;
    }

    // This is odd because its not a go type but a function with receiver
    if (shape.isOperationShape()) {
      return false;
    }

    if (INHERENTLY_POINTABLE.contains(shape.getType())) {
      return true;
    }

    if (INHERENTLY_VALUE.contains(shape.getType()) || isShapeEnum(shape)) {
      return false;
    }

    return nullableIndex.isNullable(shape);
  }

  private boolean isShapeEnum(Shape shape) {
    return (
      (shape.getType() == ShapeType.STRING &&
        shape.hasTrait(EnumTrait.class)) ||
      shape.getType() == ShapeType.ENUM ||
      shape.getType() == ShapeType.INT_ENUM
    );
  }

  private boolean isBlobStream(Shape shape) {
    return (
      shape.getType() == ShapeType.BLOB && shape.hasTrait(StreamingTrait.ID)
    );
  }

  public boolean isOperationStruct(Shape shape) {
    NeighborProvider provider = NeighborProviderIndex
      .of(model)
      .getReverseProvider();
    for (Relationship relationship : provider.getNeighbors(shape)) {
      RelationshipType relationshipType = relationship.getRelationshipType();
      if (
        relationshipType == RelationshipType.INPUT ||
        relationshipType == RelationshipType.OUTPUT
      ) {
        return true;
      }
    }

    return false;
  }

  /**
   * Returns if the shape should be generated as a Go pointer type or not.
   *
   * @param shape the shape to check if should be pointable type.
   * @return if the shape is should be a Go pointer type.
   */
  public final boolean isPointable(ToShapeId shape) {
    return pointableShapes.contains(shape.toShapeId());
  }

  /**
   * Returns if the Go type generated for the shape is comparable to nil.
   *
   * @param shape the shape to check
   * @return if the shape's go type is comparable to nil
   */
  public final boolean isNillable(ToShapeId shape) {
    return nillableShapes.contains(shape.toShapeId());
  }

  /**
   * Returns if the Go type generated for the shape can be dereferenced.
   *
   * @param shape the shape to check
   * @return if the shape's go type is dereferencable
   */
  public final boolean isDereferencable(ToShapeId shape) {
    return dereferencableShapes.contains(shape.toShapeId());
  }
}
