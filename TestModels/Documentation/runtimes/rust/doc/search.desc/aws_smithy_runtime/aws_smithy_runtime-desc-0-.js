searchState.loadedDescShard("aws_smithy_runtime", 0, "Runtime support logic and types for smithy-rs generated …\nRuntime support logic for generated clients.\nCache for entries that have an expiration time.\nA data structure for persisting and sharing state between …\nSmithy auth scheme implementations.\nUtility to simplify config building for config and config …\nRuntime plugins that provide defaults for clients.\nBuilt-in DNS resolver implementations.\nCode for applying endpoints to a request.\nBuilt-in Smithy HTTP clients and connectors.\nSmithy identity used by auth and signing.\nInterceptors for Smithy clients.\nThe client orchestrator implementation\nSmithy code related to retry handling and token buckets.\nStalled stream protection for clients\nSmithy support-code for code generated waiters.\nThe <code>NoAuthRuntimePlugin</code> and supporting code.\nAuth scheme ID for “no auth”.\nA <code>RuntimePlugin</code> that registers a “no auth” identity …\nThe “no auth” auth scheme.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>NoAuthRuntimePlugin</code>.\nCreates a new <code>NoAuthScheme</code>.\nUtility to simplify config building and config overrides.\nReturns a mutable reference to the latest config.\nReturns the argument unchanged.\nConstruct a new <code>Resolver</code> in <em>initial mode</em>.\nCalls <code>U::from(self)</code>.\nReturns true if in <em>initial mode</em>.\nReturns true if the latest config has <code>T</code> set.\nReturns true if <code>T</code> is set anywhere.\nThe async sleep implementation.\nConstruct a new <code>Resolver</code> in <em>override mode</em>.\nResolves the value <code>T</code> with fallback\nReturns a mutable reference to the latest runtime …\nThe async sleep implementation.\nArguments for the <code>default_plugins</code> method.\nRuntime plugin that provides a default connector.\nRuntime plugin that registers the default identity cache …\nAll default plugins.\nRuntime plugin that sets the default retry strategy, …\nRuntime plugin that provides a default async sleep …\nRuntime plugin that sets the default stalled stream …\nRuntime plugin that provides a default time source.\nRuntime plugin that sets the default timeout config (no …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCreates a new <code>DefaultPluginParams</code>.\nSets the behavior major version.\nSets the retry partition name.\nApply <code>endpoint</code> to <code>uri</code>\nHTTP body and body-wrapper types\nInterceptor for connection poisoning.\nRuntimePlugin to ensure that the amount of data received …\nA body-wrapping type that ensures data is being streamed …\nAn error returned when a body did not have the expected …\nRuntime plugin that enforces response bodies match their …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a runtime plugin which installs Content-Length …\nUse <code>MinimumThroughputDownloadBody</code> instead.\nA body-wrapping type that ensures data is being streamed …\nThroughput representation for use when configuring …\nReturns the argument unchanged.\nReturns the argument unchanged.\nAn implementation of v0.4 <code>http_body::Body</code> for …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new throughput with the given bytes read and time …\nCreate a new minimum throughput body.\nCreate a new throughput in bytes per second.\nCreate a new throughput in kilobytes per second.\nCreate a new throughput in megabytes per second.\nOptions for a <code>MinimumThroughputBody</code>.\nA collection of options for configuring a …\nA builder for <code>MinimumThroughputBodyOptions</code>\nBuild this builder, producing a …\nCreate a new builder.\nNot used. Always returns <code>Duration::from_millis(500)</code>.\nNo longer used. The check interval is now based on the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe throughput check grace period.\nSet the amount of time that throughput my fall below …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe minimum acceptable throughput\nSet the minimum allowable throughput.\nCreate a new <code>MinimumThroughputBodyOptionsBuilder</code>.\nNo longer used. The check interval is now based on the …\nSet the amount of time that throughput my fall below …\nSet the minimum allowable throughput.\nConvert this struct into a builder.\nState for a middleware that will monitor and manage …\nAn interceptor for poisoning connections in response to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the associated connection metadata.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>ConnectionPoisoningInterceptor</code>.\nCreate a new connection monitor.\nSet the retriever that will capture the <code>hyper</code> connection.\nIdentity cache configuration.\nBuilder for lazy identity caching.\nAmount of time before the actual identity expiration time …\nBuilds a <code>SharedIdentityCache</code> from this builder.\nDefault expiration time to set on an identity if it doesn…\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConfigure a lazy identity cache.\nTimeout for identity resolution.\nCreate a new builder.\nIdentity resolver implementation for “no auth”.\nCreate an identity cache that does not cache any resolved …\nAmount of time before the actual identity expiration time …\nDefault expiration time to set on an identity if it doesn…\nTimeout for identity resolution.\nSet the async sleep implementation for this cache.\nSet the time source for this cache.\nSet the async sleep implementation for this cache.\nSet the time source for this cache.\nIdentity for the <code>NoAuthScheme</code> auth scheme.\nIdentity resolver for the <code>NoAuthScheme</code> auth scheme.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>NoAuthIdentity</code>.\nCreates a new <code>NoAuthIdentityResolver</code>.\nInterceptor that maps the request with a given function.\nInterceptor that mutates the request with a given function.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>MapRequestInterceptor</code>.\nCreates a new <code>MutateRequestInterceptor</code>.\nStop the orchestrator before transmitting the request\nDon’t stop orchestration early\nAllows for returning early at different points during …\nDefines types that implement a trait for endpoint …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nOrchestrates the execution of a request and handling of a …\nSame as <code>invoke</code>, but allows for returning early at …\nUtility for making one-off unmodeled requests with the …\nAn endpoint resolver that uses a static URI.\nEmpty params to be used with <code>StaticUriEndpointResolver</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a resolver that resolves to <code>http://localhost:{port}</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>StaticUriEndpointResolverParams</code>.\nCreate a resolver that resolves to the given URI.\nOrchestrates execution of a HTTP request without any …\nBuilder for <code>Operation</code>.\nCreates an <code>Operation</code> from the builder.\nReturns a new <code>OperationBuilder</code> for the <code>Operation</code>.\nConfigures the deserializer for the builder.\nConfigures the a deserializer implementation for the …\nConfigures the endpoint URL for the builder.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConfigures the http client for the builder.\nConfigures the interceptor for the builder.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nInvokes this <code>Operation</code> with the given <code>input</code> and returns …\nCreates a new <code>OperationBuilder</code>.\nDisables auth for the operation.\nDisables the retry for the operation.\nConfigures the operation name for the builder.\nConfigures the retry classifier for the builder.\nConfigures the runtime plugin for the builder.\nConfigures the serializer for the builder.\nConfigures the service name for the builder.\nConfigures the sleep for the builder.\nConfigures stalled stream protection with the given config.\nConfigures the standard retry for the builder.\nConfigures the time source for the builder.\nConfigures the timeout configuration for the builder.\nRegisters the <code>ConnectionPoisoningInterceptor</code>.\nRate limiter for adaptive retry.\nRepresents a partition for the rate limiter, e.g. an …\nRepresents the retry partition, e.g. an endpoint, a region\nToken bucket used for standard and adaptive retry.\nSmithy retry classifiers.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a <code>ClientRateLimiterPartition</code> from the given …\nCreates a new <code>ClientRateLimiter</code>\nCreates a new <code>TokenBucket</code> with the given initial quota.\nCreates a new <code>RetryPartition</code> from the given <code>name</code>.\nSmithy retry strategies.\nA retry classifier that will treat HTTP response with …\nA retry classifier for checking if an error is modeled as …\nClassifies response, timeout, and connector errors as …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>ModeledAsRetryableClassifier</code>\nCreate a new <code>TransientErrorClassifier</code>\nGiven a <code>Vec&lt;u16&gt;</code> where the <code>u16</code>s represent status codes, …\nReturn the priority of this retry classifier.\nReturn the priority of this retry classifier.\nReturn the priority of this retry classifier.\nGiven an iterator of retry classifiers and an interceptor …\nA retry strategy that never retries.\nRetry strategy with exponential backoff, max attempts, and …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new <code>NeverRetryStrategy</code>.\nCreate a new standard retry strategy with the given config.\nEnable stalled stream protection for both request and …\nEnable stalled stream protection for request bodies.\nEnable stalled stream protection for response bodies.\nAdds stalled stream protection when sending requests …\nStalled stream protection can be enable for request …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new stalled stream protection interceptor.\nWaiter acceptor state\nA <code>failure</code> acceptor matched the response.\nNone of the modeled acceptors matched the response.\nA <code>retry</code> acceptor matched the response.\nA <code>success</code> acceptor matched the response.\nOrchestrates waiting via polling with jittered exponential …\nBuilder for <code>WaiterOrchestrator</code>.\nSet the acceptor function for the waiter.\nAttaches a tracing span with a semi-unique waiter ID …\nBuild a waiter orchestrator.\nReturns a builder for the waiter orchestrator.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSet the maximum delay time for the waiter.\nSet the maximum total wait time for the waiter.\nSet the minimum delay time for the waiter.\nSet the operation function for the waiter.\nOrchestrates waiting via polling with jittered exponential …\nSet the async sleep implementation the waiter will use to …\nSet the time source the waiter will use.\nExpiry-aware cache\nReturns the argument unchanged.\nAttempts to refresh the cached value with the given future.\nCalls <code>U::from(self)</code>.\nCreates <code>ExpiringCache</code> with the given <code>buffer_time</code>.\nIf the value is expired, clears the cache. Otherwise, …\nA data structure for persisting and sharing state between …\nReturns the argument unchanged.\nGets the value for the given partition key.\nGets the value for the given partition key, initializing …\nGets the value for the given partition key, initializing …\nCalls <code>U::from(self)</code>.\nCreates a new <code>StaticPartitionMap</code>.")