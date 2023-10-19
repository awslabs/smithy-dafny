/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *   http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */

package software.amazon.polymorph.smithypython.awssdk;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;
import software.amazon.polymorph.smithypython.common.Constants;
import software.amazon.polymorph.smithypython.common.nameresolver.DafnyNameResolver;
import software.amazon.polymorph.smithypython.common.nameresolver.SmithyNameResolver;
import software.amazon.polymorph.smithypython.common.nameresolver.Utils;
import software.amazon.polymorph.smithypython.localservice.shapevisitor.DafnyToSmithyShapeVisitor;
import software.amazon.polymorph.smithypython.localservice.shapevisitor.SmithyToDafnyShapeVisitor;
import software.amazon.polymorph.traits.LocalServiceTrait;
import software.amazon.smithy.codegen.core.Symbol;
import software.amazon.smithy.codegen.core.SymbolReference;
import software.amazon.smithy.codegen.core.WriterDelegator;
import software.amazon.smithy.model.shapes.OperationShape;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.Shape;
import software.amazon.smithy.model.shapes.ShapeId;
import software.amazon.smithy.model.shapes.StructureShape;
import software.amazon.smithy.model.traits.ErrorTrait;
import software.amazon.smithy.python.codegen.ApplicationProtocol;
import software.amazon.smithy.python.codegen.CodegenUtils;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonWriter;
import software.amazon.smithy.python.codegen.SmithyPythonDependency;
import software.amazon.smithy.python.codegen.integration.ProtocolGenerator;
import software.amazon.smithy.utils.CodeSection;
import software.amazon.smithy.utils.SmithyUnstableApi;

/**
 * This will implement any handling of components outside the request
 * body and error handling.
 */
@SmithyUnstableApi
public abstract class DafnyPythonAwsSdkProtocolGenerator implements ProtocolGenerator {

  public static ApplicationProtocol DAFNY_PYTHON_AWS_SDK_PROTOCOL = new ApplicationProtocol(
      // Define the `dafny-python-aws-sdk-protocol` ApplicationProtocol.
      // ApplicationProtocol is distinct from the Protocols used in ProtocolGenerators.
      // We define an ApplicationProtocol that will be used by all Dafny-Python AWS SDK protocols.
      // The ApplicationProtocol is used within our code to determine which code should be generated.
      // The `null`s reflect that this ApplicationProtocol does not have request
      //   or response object types, since it does not use Smithy-generated clients.
      "dafny-python-aws-sdk-application-protocol",
      null,
      null
  );

  @Override
  public ApplicationProtocol getApplicationProtocol() {
    return DAFNY_PYTHON_AWS_SDK_PROTOCOL;
  }

  @Override
  public void generateRequestSerializers(GenerationContext context) { }

  @Override
  public void generateResponseDeserializers(GenerationContext context) { }

}
