package software.amazon.polymorph.smithygo.codegen;

import java.util.Arrays;
import java.util.Objects;
import java.util.Optional;
import software.amazon.smithy.codegen.core.CodegenException;
import software.amazon.smithy.model.Model;
import software.amazon.smithy.model.node.ObjectNode;
import software.amazon.smithy.model.shapes.ServiceShape;
import software.amazon.smithy.model.shapes.ShapeId;

public class GoSettings {

  private static final String SERVICE = "service";
  private static final String MODULE_NAME = "module";
  private static final String MODULE_DESCRIPTION = "moduleDescription";
  private static final String MODULE_VERSION = "moduleVersion";
  private static final String GENERATE_GO_MOD = "generateGoMod";
  private static final String GO_DIRECTIVE = "goDirective";

  private ShapeId service;
  private String moduleName;
  private String moduleDescription = "";
  private String moduleVersion;
  private Boolean generateGoMod = false;
  private ShapeId protocol;

  /**
   * Create a settings object from a configuration object node.
   *
   * @param config Config object to load.
   * @return Returns the extracted settings.
   */
  public static GoSettings from(ObjectNode config) {
    GoSettings settings = new GoSettings();
    config.warnIfAdditionalProperties(
      Arrays.asList(
        SERVICE,
        MODULE_NAME,
        MODULE_DESCRIPTION,
        MODULE_VERSION,
        GENERATE_GO_MOD,
        GO_DIRECTIVE
      )
    );

    settings.setService(config.expectStringMember(SERVICE).expectShapeId());
    settings.setModuleName(config.expectStringMember("moduleName").getValue());
    settings.setModuleDescription(
      config.getStringMemberOrDefault(
        MODULE_DESCRIPTION,
        settings.getModuleName() + " client"
      )
    );
    settings.setModuleVersion(
      config.getStringMemberOrDefault(MODULE_VERSION, null)
    );
    settings.setGenerateGoMod(
      config.getBooleanMemberOrDefault(GENERATE_GO_MOD, false)
    );
    return settings;
  }

  /**
   * Gets the id of the service that is being generated.
   *
   * @return Returns the service id.
   * @throws NullPointerException if the service has not been set.
   */
  public ShapeId getService() {
    return Objects.requireNonNull(service, SERVICE + " not set");
  }

  /**
   * Gets the corresponding {@link ServiceShape} from a model.
   *
   * @param model Model to search for the service shape by ID.
   * @return Returns the found {@code Service}.
   * @throws NullPointerException if the service has not been set.
   * @throws CodegenException     if the service is invalid or not found.
   */
  public ServiceShape getService(Model model) {
    return model
      .getShape(getService())
      .orElseThrow(() ->
        new CodegenException("Service shape not found: " + getService())
      )
      .asServiceShape()
      .orElseThrow(() ->
        new CodegenException("Shape is not a Service: " + getService())
      );
  }

  /**
   * Sets the service to generate.
   *
   * @param service The service to generate.
   */
  public void setService(ShapeId service) {
    this.service = Objects.requireNonNull(service);
  }

  /**
   * Gets the required module name for the module that will be generated.
   *
   * @return Returns the module name.
   * @throws NullPointerException if the module name has not been set.
   */
  public String getModuleName() {
    return Objects.requireNonNull(moduleName, MODULE_NAME + " not set");
  }

  /**
   * Sets the name of the module to generate.
   *
   * @param moduleName The name of the module to generate.
   */
  public void setModuleName(String moduleName) {
    this.moduleName = Objects.requireNonNull(moduleName);
  }

  /**
   * Gets the optional module description for the module that will be generated.
   *
   * @return Returns the module description.
   */
  public String getModuleDescription() {
    return moduleDescription;
  }

  /**
   * Sets the description of the module to generate.
   *
   * @param moduleDescription The description of the module to generate.
   */
  public void setModuleDescription(String moduleDescription) {
    this.moduleDescription = Objects.requireNonNull(moduleDescription);
  }

  /**
   * Gets the optional module version for the module that will be generated.
   *
   * @return Returns the module version.
   */
  public Optional<String> getModuleVersion() {
    return Optional.ofNullable(moduleVersion);
  }

  /**
   * Sets the version of the module to generate.
   *
   * @param moduleVersion The version of the module to generate.
   */
  public void setModuleVersion(String moduleVersion) {
    if (moduleVersion != null) {
      this.moduleVersion = moduleVersion;
    }
  }

  /**
   * Gets the flag for generating go.mod file.
   *
   * @return Returns if go.mod will be generated (true) or not (false)
   */
  public Boolean getGenerateGoMod() {
    return generateGoMod;
  }

  /**
   * Sets the flag for generating go.mod file.
   *
   * @param generateGoMod If go.mod will be generated (true) or not (false)
   */
  public void setGenerateGoMod(Boolean generateGoMod) {
    this.generateGoMod = Objects.requireNonNull(generateGoMod);
  }

  /**
   * Gets the configured protocol to generate.
   *
   * @return Returns the configured protocol.
   */
  public ShapeId getProtocol() {
    return protocol;
  }

  /**
   * Sets the protocol to generate.
   *
   * @param protocol Protocols to generate.
   */
  public void setProtocol(ShapeId protocol) {
    this.protocol = Objects.requireNonNull(protocol);
  }
}
