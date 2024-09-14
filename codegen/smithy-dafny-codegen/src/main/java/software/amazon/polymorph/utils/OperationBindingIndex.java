package software.amazon.polymorph.utils;

import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.knowledge.KnowledgeIndex;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ResourceShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.ToShapeId;
import software.amazon.smithy.utils.SetUtils;

public class OperationBindingIndex implements KnowledgeIndex {

  private final Map<ShapeId, Set<Shape>> bindingShapesForOperation =
    new HashMap();
  private final Map<ShapeId, Set<OperationShape>> operationBindings =
    new HashMap();

  public OperationBindingIndex(Model model) {
    for (final ServiceShape service : model.getServiceShapes()) {
      for (final ShapeId operationId : service.getOperations()) {
        final OperationShape operationShape = model.expectShape(
          operationId,
          OperationShape.class
        );
        operationBindings
          .computeIfAbsent(service.getId(), id -> new HashSet<>())
          .add(operationShape);
        bindingShapesForOperation
          .computeIfAbsent(operationId, id -> new HashSet<>())
          .add(service);
      }
    }

    for (final ResourceShape resource : model.getResourceShapes()) {
      for (final ShapeId operationId : resource.getOperations()) {
        final OperationShape operationShape = model.expectShape(
          operationId,
          OperationShape.class
        );
        operationBindings
          .computeIfAbsent(resource.getId(), id -> new HashSet<>())
          .add(operationShape);
        bindingShapesForOperation
          .computeIfAbsent(operationId, id -> new HashSet<>())
          .add(resource);
      }
    }
  }

  public Set<Shape> getBindingShapes(ToShapeId operation) {
    return SetUtils.copyOf(
      bindingShapesForOperation.getOrDefault(
        operation.toShapeId(),
        Collections.emptySet()
      )
    );
  }

  public Set<OperationShape> getOperations(ToShapeId bindingShape) {
    return SetUtils.copyOf(
      operationBindings.getOrDefault(
        bindingShape.toShapeId(),
        Collections.emptySet()
      )
    );
  }
}
