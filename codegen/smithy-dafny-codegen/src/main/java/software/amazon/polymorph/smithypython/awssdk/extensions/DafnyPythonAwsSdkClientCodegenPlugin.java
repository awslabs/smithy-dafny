
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
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

package software.amazon.polymorph.smithypython.awssdk.extensions;

import software.amazon.polymorph.smithypython.awssdk.DafnyAwsSdkProtocolTrait;
import software.amazon.polymorph.smithypython.wrappedlocalservice.WrappedLocalServiceTrait;
import software.amazon.polymorph.traits.LocalServiceTrait;
import software.amazon.smithy.build.PluginContext;
import software.amazon.smithy.build.SmithyBuildPlugin;
import software.amazon.smithy.codegen.core.directed.CodegenDirector;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.transform.ModelTransformer;
import software.amazon.smithy.python.codegen.GenerationContext;
import software.amazon.smithy.python.codegen.PythonSettings;
import software.amazon.smithy.python.codegen.PythonWriter;
import software.amazon.smithy.python.codegen.integration.PythonIntegration;
import software.amazon.smithy.utils.SmithyUnstableApi;

/**
 * Plugin to trigger Smithy-Dafny Python code generation.
 * This differs from the PythonClientCodegenPlugin by not calling
 *     runner.performDefaultCodegenTransforms();
 * and
 *     runner.createDedicatedInputsAndOutputs();
 * These methods transform the model in ways that the model does not align with
 *   the generated Dafny code.
 */
@SmithyUnstableApi
public final class DafnyPythonAwsSdkClientCodegenPlugin implements SmithyBuildPlugin {
  @Override
  public String getName() {
    return "dafny-python-aws-sdk-client-codegen";
  }

  @Override
  public void execute(PluginContext context) {
    CodegenDirector<PythonWriter, PythonIntegration, GenerationContext, PythonSettings> runner
        = new CodegenDirector<>();

    PythonSettings settings = PythonSettings.from(context.getSettings());
    settings.setProtocol(DafnyAwsSdkProtocolTrait.ID);
    runner.settings(settings);
    runner.directedCodegen(new DirectedDafnyPythonAwsSdkCodegen());
    runner.fileManifest(context.getFileManifest());
    runner.service(settings.getService());
    runner.model(context.getModel());
    runner.integrationClass(PythonIntegration.class);

    // Add a DafnyAwsSdkLocal to the service as a contextual indicator that code generation requires
    //   wrapped local service generation
    ServiceShape serviceShape = context.getModel().expectShape(settings.getService()).asServiceShape().get();
    runner.model(addAwsSdkProtocolTrait(context.getModel(), serviceShape));

    runner.run();
  }

  public static Model addAwsSdkProtocolTrait(Model model, ServiceShape serviceShape) {
    return ModelTransformer.create().mapShapes(model, shape -> {
      if (shape instanceof ServiceShape && shape.hasTrait(LocalServiceTrait.class)) {
        return serviceShape.toBuilder().addTrait(DafnyAwsSdkProtocolTrait.builder().build()).build();
      } else {
        return shape;
      }
    });
  }
}
