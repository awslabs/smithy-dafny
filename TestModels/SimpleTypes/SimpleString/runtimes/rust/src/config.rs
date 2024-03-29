// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Configuration for a simple service client.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Config {
    behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
}
impl Config {
    /// Constructs a config builder.
    pub fn builder() -> Builder {
        Builder::default()
    }
    /// Converts this config back into a builder so that it can be tweaked.
    pub fn to_builder(&self) -> Builder {
        Builder {
            behavior_version: self.behavior_version.clone(),
        }
    }
}
/// Builder for creating a `Config`.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Builder {
    pub(crate) behavior_version: ::std::option::Option<crate::config::BehaviorVersion>,
}
impl ::std::default::Default for Builder {
    fn default() -> Self {
        Self {
            behavior_version: ::std::default::Default::default(),
        }
    }
}
impl Builder {
    /// Constructs a config builder.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
    ///
    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
    /// all operations might be the ideal behavior but could break existing applications.
    ///
    /// # Examples
    ///
    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
    /// ```no_run
    /// use simple::config::BehaviorVersion;
    ///
    /// let config = simple::Config::builder()
    ///     .behavior_version(BehaviorVersion::latest())
    ///     // ...
    ///     .build();
    /// let client = simple::Client::from_conf(config);
    /// ```
    ///
    /// Customizing behavior major version:
    /// ```no_run
    /// use simple::config::BehaviorVersion;
    ///
    /// let config = simple::Config::builder()
    ///     .behavior_version(BehaviorVersion::v2023_11_09())
    ///     // ...
    ///     .build();
    /// let client = simple::Client::from_conf(config);
    /// ```

    pub fn behavior_version(mut self, behavior_version: crate::config::BehaviorVersion) -> Self {
        self.set_behavior_version(Some(behavior_version));
        self
    }

    /// Sets the [`behavior major version`](crate::config::BehaviorVersion).
    ///
    /// Over time, new best-practice behaviors are introduced. However, these behaviors might not be backwards
    /// compatible. For example, a change which introduces new default timeouts or a new retry-mode for
    /// all operations might be the ideal behavior but could break existing applications.
    ///
    /// # Examples
    ///
    /// Set the behavior major version to `latest`. This is equivalent to enabling the `behavior-version-latest` cargo feature.
    /// ```no_run
    /// use simple::config::BehaviorVersion;
    ///
    /// let config = simple::Config::builder()
    ///     .behavior_version(BehaviorVersion::latest())
    ///     // ...
    ///     .build();
    /// let client = simple::Client::from_conf(config);
    /// ```
    ///
    /// Customizing behavior major version:
    /// ```no_run
    /// use simple::config::BehaviorVersion;
    ///
    /// let config = simple::Config::builder()
    ///     .behavior_version(BehaviorVersion::v2023_11_09())
    ///     // ...
    ///     .build();
    /// let client = simple::Client::from_conf(config);
    /// ```

    pub fn set_behavior_version(&mut self, behavior_version: Option<crate::config::BehaviorVersion>) -> &mut Self {
        self.behavior_version = behavior_version;
        self
    }

    /// Convenience method to set the latest behavior major version
    ///
    /// This is equivalent to enabling the `behavior-version-latest` Cargo feature
    pub fn behavior_version_latest(mut self) -> Self {
        self.set_behavior_version(Some(crate::config::BehaviorVersion::latest()));
        self
    }
    /// Builds a [`Config`].
    #[allow(unused_mut)]
    pub fn build(mut self) -> Config {
        Config {
            behavior_version: self.behavior_version,
        }
    }
}
