/*
 * Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */

/*
 * Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */

package software.amazon.polymorph.smithygo.codegen.knowledge;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.TreeSet;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.knowledge.KnowledgeIndex;
import software.amazon.smithy.model.knowledge.TopDownIndex;
import software.amazon.smithy.model.neighbor.Walker;
import software.amazon.smithy.model.shapes.MemberShape;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.ToShapeId;
import software.amazon.smithy.model.traits.HttpLabelTrait;
import software.amazon.smithy.model.traits.RequiredTrait;
import software.amazon.smithy.utils.SetUtils;

/**
 * Provides a knowledge index of which service operations and shapes require validation helpers.
 */
public class GoValidationIndex implements KnowledgeIndex {

  private final Map<ShapeId, Set<ShapeId>> serviceToOperationMap =
    new HashMap<>();
  private final Map<ShapeId, Set<ShapeId>> serviceValidationHelpers =
    new HashMap<>();

  public GoValidationIndex(Model model) {
    TopDownIndex topDownIndex = model.getKnowledge(TopDownIndex.class);
    Walker walker = new Walker(model);

    model
      .shapes(ServiceShape.class)
      .forEach(serviceShape -> {
        // Go uses unique input shapes per operation so we can index using the input shape as our key
        Map<Shape, OperationShape> inputShapeToOperation = new HashMap<>();
        Set<ShapeId> requireValidationHelpers = new TreeSet<>();

        // First pass is to collect member containers that contain members requiring validation
        Set<OperationShape> operations = topDownIndex.getContainedOperations(
          serviceShape
        );
        operations.forEach(operationShape -> {
          Shape inputShape = model.expectShape(operationShape.getInput().get());
          GoValidationIndex.walkValidationTree(
            walker,
            inputShape,
            shape -> {
              if (shape.isMemberShape()) {
                Shape container = model.expectShape(
                  ((MemberShape) shape).getContainer()
                );
                if (
                  isRequiredParameter(
                    model,
                    (MemberShape) shape,
                    inputShape.equals(container)
                  )
                ) {
                  inputShapeToOperation.put(inputShape, operationShape);
                  requireValidationHelpers.add(container.toShapeId());
                }
              }
            }
          );
        });

        // 2nd step is final all containers that reference the initial containers which require validation until
        // we've discovered all intermediate containing types
        inputShapeToOperation
          .keySet()
          .forEach(input -> {
            Set<ShapeId> helpers = new TreeSet<>();
            do {
              GoValidationIndex.walkValidationTree(
                walker,
                input,
                shape -> {
                  if (shape.isMemberShape()) {
                    MemberShape memberShape = shape.asMemberShape().get();
                    Shape container = model.expectShape(
                      memberShape.getContainer()
                    );
                    Shape target = model.expectShape(memberShape.getTarget());
                    if (
                      requireValidationHelpers.contains(target.toShapeId()) &&
                      !requireValidationHelpers.contains(container.toShapeId())
                    ) {
                      helpers.add(container.toShapeId());
                    }
                  }
                }
              );
              if (helpers.isEmpty()) {
                break;
              }
              requireValidationHelpers.addAll(helpers);
              helpers.clear();
            } while (true);
          });

        serviceToOperationMap.put(
          serviceShape.toShapeId(),
          new TreeSet<>(
            inputShapeToOperation
              .values()
              .stream()
              .map(OperationShape::toShapeId)
              .sorted()
              .collect(Collectors.toSet())
          )
        );
        serviceValidationHelpers.put(
          serviceShape.toShapeId(),
          requireValidationHelpers
        );
      });
  }

  public static GoValidationIndex of(Model model) {
    return model.getKnowledge(GoValidationIndex.class, GoValidationIndex::new);
  }

  /**
   * Get the set of operations that require validation.
   *
   * @param service service to find operations for
   * @return operations requiring validation
   */
  public Set<ShapeId> getOperationsRequiringValidation(ToShapeId service) {
    return serviceToOperationMap.getOrDefault(
      service.toShapeId(),
      SetUtils.of()
    );
  }

  /**
   * Get a set of shapes that require validation helpers.
   *
   * @param service service to find operations for
   * @return operations requiring validation
   */
  public Set<ShapeId> getShapesRequiringValidationHelpers(ToShapeId service) {
    return serviceValidationHelpers.getOrDefault(
      service.toShapeId(),
      SetUtils.of()
    );
  }

  /**
   * Returns whether the given shape requires a validation helper.
   *
   * @param shape the shape to check
   * @return whether the shape requires a validation helper
   */
  public boolean isValidationHelperRequired(ToShapeId shape) {
    return serviceValidationHelpers.containsKey(shape.toShapeId());
  }

  /**
   * Checks whether a {@link MemberShape} has any validation constraints.
   *
   * @param model                the model
   * @param shape                the {@link MemberShape} to check
   * @param validateHttpBindings whether http bindings should be checked for additional implicit constraints
   * @return whether the {@link MemberShape} has validation costraints
   */
  public static boolean hasValidation(
    Model model,
    MemberShape shape,
    boolean validateHttpBindings
  ) {
    return isRequiredParameter(model, shape, validateHttpBindings);
  }

  /**
   * Checks whether a {@link MemberShape} is marked as being required explicitly or implicitly.
   *
   * @param model                the model
   * @param shape                the {@link MemberShape} to check
   * @param validateHttpBindings whether http bindings should be checked for additional implicit constraints
   * @return whether the {@link MemberShape} is a required parameter
   */
  public static boolean isRequiredParameter(
    Model model,
    MemberShape shape,
    boolean validateHttpBindings
  ) {
    Optional<RequiredTrait> requiredTrait = shape.getMemberTrait(
      model,
      RequiredTrait.class
    );
    return (
      requiredTrait.isPresent() ||
      (validateHttpBindings &&
        shape.getMemberTrait(model, HttpLabelTrait.class).isPresent())
    );
  }

  private static void walkValidationTree(
    Walker walker,
    Shape shape,
    Consumer<Shape> visitor
  ) {
    walker
      .walkShapes(
        shape,
        relationship -> {
          switch (relationship.getRelationshipType()) {
            case STRUCTURE_MEMBER:
            case UNION_MEMBER:
            case MAP_VALUE:
            case LIST_MEMBER:
            case SET_MEMBER:
            case MEMBER_TARGET:
              return true;
            default:
              return false;
          }
        }
      )
      .forEach(visitor::accept);
  }
}
