(function() {var implementors = {
"aws_smithy_runtime":[["impl <a class=\"trait\" href=\"aws_smithy_runtime_api/client/interceptors/trait.Intercept.html\" title=\"trait aws_smithy_runtime_api::client::interceptors::Intercept\">Intercept</a> for <a class=\"struct\" href=\"aws_smithy_runtime/client/http/connection_poisoning/struct.ConnectionPoisoningInterceptor.html\" title=\"struct aws_smithy_runtime::client::http::connection_poisoning::ConnectionPoisoningInterceptor\">ConnectionPoisoningInterceptor</a>"],["impl <a class=\"trait\" href=\"aws_smithy_runtime_api/client/interceptors/trait.Intercept.html\" title=\"trait aws_smithy_runtime_api::client::interceptors::Intercept\">Intercept</a> for <a class=\"struct\" href=\"aws_smithy_runtime/client/stalled_stream_protection/struct.StalledStreamProtectionInterceptor.html\" title=\"struct aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor\">StalledStreamProtectionInterceptor</a>"],["impl&lt;F&gt; <a class=\"trait\" href=\"aws_smithy_runtime_api/client/interceptors/trait.Intercept.html\" title=\"trait aws_smithy_runtime_api::client::interceptors::Intercept\">Intercept</a> for <a class=\"struct\" href=\"aws_smithy_runtime/client/interceptors/struct.MutateRequestInterceptor.html\" title=\"struct aws_smithy_runtime::client::interceptors::MutateRequestInterceptor\">MutateRequestInterceptor</a>&lt;F&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(&amp;mut <a class=\"type\" href=\"aws_smithy_runtime_api/client/orchestrator/type.HttpRequest.html\" title=\"type aws_smithy_runtime_api::client::orchestrator::HttpRequest\">HttpRequest</a>) + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div>"],["impl&lt;F, E&gt; <a class=\"trait\" href=\"aws_smithy_runtime_api/client/interceptors/trait.Intercept.html\" title=\"trait aws_smithy_runtime_api::client::interceptors::Intercept\">Intercept</a> for <a class=\"struct\" href=\"aws_smithy_runtime/client/interceptors/struct.MapRequestInterceptor.html\" title=\"struct aws_smithy_runtime::client::interceptors::MapRequestInterceptor\">MapRequestInterceptor</a>&lt;F, E&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"type\" href=\"aws_smithy_runtime_api/client/orchestrator/type.HttpRequest.html\" title=\"type aws_smithy_runtime_api::client::orchestrator::HttpRequest\">HttpRequest</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"type\" href=\"aws_smithy_runtime_api/client/orchestrator/type.HttpRequest.html\" title=\"type aws_smithy_runtime_api::client::orchestrator::HttpRequest\">HttpRequest</a>, E&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div>"]],
"aws_smithy_runtime_api":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()