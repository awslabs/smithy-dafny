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

package software.amazon.polymorph.smithygo.codegen;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.TreeSet;
import software.amazon.smithy.codegen.core.SymbolDependency;
import software.amazon.smithy.codegen.core.SymbolDependencyContainer;
import software.amazon.smithy.utils.SetUtils;
import software.amazon.smithy.utils.SmithyBuilder;

public final class GoDependency implements SymbolDependencyContainer {

  private final Type type;
  private final String sourcePath;
  private final String importPath;
  private final String alias;
  private final String version;
  private final Set<GoDependency> dependencies;
  private final SymbolDependency symbolDependency;

  private GoDependency(Builder builder) {
    this.type = SmithyBuilder.requiredState("type", builder.type);
    this.sourcePath =
      !this.type.equals(Type.STANDARD_LIBRARY)
        ? SmithyBuilder.requiredState("sourcePath", builder.sourcePath)
        : "";
    this.importPath =
      SmithyBuilder.requiredState("importPath", builder.importPath);
    this.alias = builder.alias;
    this.version = SmithyBuilder.requiredState("version", builder.version);
    this.dependencies = builder.dependencies;

    this.symbolDependency =
      SymbolDependency
        .builder()
        .dependencyType(this.type.toString())
        .packageName(this.sourcePath)
        .version(this.version)
        .build();
  }

  /**
   * Get the the set of {@link GoDependency} required by this dependency.
   *
   * @return the set of dependencies
   */
  public Set<GoDependency> getGoDependencies() {
    return this.dependencies;
  }

  /**
   * Get the symbol dependency representing the dependency.
   *
   * @return the symbol dependency
   */
  public SymbolDependency getSymbolDependency() {
    return symbolDependency;
  }

  /**
   * Get the type of dependency.
   *
   * @return the dependency type
   */
  public Type getType() {
    return type;
  }

  /**
   * Get the source code path of the dependency.
   *
   * @return the dependency source code path
   */
  public String getSourcePath() {
    return sourcePath;
  }

  /**
   * Get the import path of the dependency.
   *
   * @return the import path of the dependency
   */
  public String getImportPath() {
    return importPath;
  }

  /**
   * Get the alias of the module to use.
   *
   * @return the alias
   */
  public String getAlias() {
    return alias;
  }

  /**
   * Get the version of the dependency.
   *
   * @return the version
   */
  public String getVersion() {
    return version;
  }

  @Override
  public List<SymbolDependency> getDependencies() {
    Set<SymbolDependency> symbolDependencySet = new TreeSet<>(
      SetUtils.of(getSymbolDependency())
    );

    symbolDependencySet.addAll(
      resolveDependencies(getGoDependencies(), new HashSet<>(SetUtils.of(this)))
    );

    return new ArrayList<>(symbolDependencySet);
  }

  private Set<SymbolDependency> resolveDependencies(
    Set<GoDependency> goDependencies,
    Set<GoDependency> processed
  ) {
    Set<SymbolDependency> symbolDependencies = new TreeSet<>();
    if (goDependencies.size() == 0) {
      return symbolDependencies;
    }

    Set<GoDependency> dependenciesToResolve = new TreeSet<>();
    for (GoDependency dependency : goDependencies) {
      if (processed.contains(dependency)) {
        continue;
      }
      processed.add(dependency);
      symbolDependencies.add(dependency.getSymbolDependency());
      dependenciesToResolve.addAll(dependency.getGoDependencies());
    }

    symbolDependencies.addAll(
      resolveDependencies(dependenciesToResolve, processed)
    );

    return symbolDependencies;
  }

  public static Builder builder() {
    return new Builder();
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    } else if (!(o instanceof GoDependency)) {
      return false;
    }

    GoDependency other = (GoDependency) o;

    return (
      this.type.equals(other.type) &&
      this.sourcePath.equals(other.sourcePath) &&
      this.importPath.equals(other.importPath) &&
      this.alias.equals(other.alias) &&
      this.version.equals(other.version) &&
      this.dependencies.equals(other.dependencies)
    );
  }

  @Override
  public int hashCode() {
    return Objects.hash(
      type,
      sourcePath,
      importPath,
      alias,
      version,
      dependencies
    );
  }

  /**
   * Represents a dependency type.
   */
  public enum Type {
    STANDARD_LIBRARY,
    DEPENDENCY;

    @Override
    public String toString() {
      switch (this) {
        case STANDARD_LIBRARY:
          return "stdlib";
        case DEPENDENCY:
          return "dependency";
        default:
          return "unknown";
      }
    }
  }

  /**
   * Get {@link GoDependency} representing the provided module description.
   *
   * @param sourcePath the root source path for the given code
   * @param importPath the import path of the package
   * @param version    the version of source module
   * @param alias      a default alias to use when importing the package, can be null
   * @return the dependency
   */
  public static GoDependency moduleDependency(
    String sourcePath,
    String importPath,
    String version,
    String alias
  ) {
    Builder builder = GoDependency
      .builder()
      .type(Type.DEPENDENCY)
      .sourcePath(sourcePath)
      .importPath(importPath)
      .version(version);
    if (alias != null) {
      builder.alias(alias);
    }
    return builder.build();
  }

  /**
   * Get {@link GoDependency} representing the standard library import description.
   *
   * @param importPath the import path of the package
   * @param version    the version of source module
   * @return the dependency
   */
  public static GoDependency standardLibraryDependency(
    String importPath,
    String version
  ) {
    return GoDependency
      .builder()
      .type(Type.STANDARD_LIBRARY)
      .importPath(importPath)
      .version(version)
      .build();
  }

  /**
   * Get {@link GoDependency} representing the standard library import description.
   *
   * @param importPath the import path of the package
   * @param version    the version of source module
   * @param alias      the alias for stdlib dependency
   * @return the dependency
   */
  public static GoDependency standardLibraryDependency(
    String importPath,
    String version,
    String alias
  ) {
    Builder builder = GoDependency
      .builder()
      .type(Type.STANDARD_LIBRARY)
      .importPath(importPath)
      .version(version);

    if (alias != null) {
      builder.alias(alias);
    }
    return builder.build();
  }

  /**
   * Builder for {@link GoDependency}.
   */
  public static final class Builder implements SmithyBuilder<GoDependency> {

    private Type type;
    private String sourcePath;
    private String importPath;
    private String alias;
    private String version;
    private final Set<GoDependency> dependencies = new TreeSet<>();

    private Builder() {}

    /**
     * Set the dependency type.
     *
     * @param type dependency type
     * @return the builder
     */
    public Builder type(Type type) {
      this.type = type;
      return this;
    }

    /**
     * Set the source path.
     *
     * @param sourcePath the source path root
     * @return the builder
     */
    public Builder sourcePath(String sourcePath) {
      this.sourcePath = sourcePath;
      return this;
    }

    /**
     * Set the import path.
     *
     * @param importPath the import path
     * @return the builder
     */
    public Builder importPath(String importPath) {
      this.importPath = importPath;
      return this;
    }

    /**
     * Set the dependency alias.
     *
     * @param alias the alias
     * @return the builder
     */
    public Builder alias(String alias) {
      this.alias = alias;
      return this;
    }

    /**
     * Set the dependency version.
     *
     * @param version the version
     * @return the builder
     */
    public Builder version(String version) {
      this.version = version;
      return this;
    }

    /**
     * Set the collection of {@link GoDependency} required by this dependency.
     *
     * @param dependencies a collection of dependencies
     * @return the builder
     */
    public Builder dependencies(Collection<GoDependency> dependencies) {
      this.dependencies.clear();
      this.dependencies.addAll(dependencies);
      return this;
    }

    /**
     * Add a dependency on another {@link GoDependency}.
     *
     * @param dependency the dependency
     * @return the builder
     */
    public Builder addDependency(GoDependency dependency) {
      this.dependencies.add(dependency);
      return this;
    }

    /**
     * Remove a dependency on another {@link GoDependency}.
     *
     * @param dependency the dependency
     * @return the builder
     */
    public Builder removeDependency(GoDependency dependency) {
      this.dependencies.remove(dependency);
      return this;
    }

    @Override
    public GoDependency build() {
      return new GoDependency(this);
    }
  }
}
