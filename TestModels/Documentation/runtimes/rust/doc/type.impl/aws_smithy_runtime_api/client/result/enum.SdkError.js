(function() {var type_impls = {
"documentation":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-Debug-for-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; <a class=\"trait\" href=\"documentation/_Wrappers_Compile/trait.Debug.html\" title=\"trait documentation::_Wrappers_Compile::Debug\">Debug</a> for SdkError&lt;E, R&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"documentation/_Wrappers_Compile/trait.Debug.html\" title=\"trait documentation::_Wrappers_Compile::Debug\">Debug</a>,\n    R: <a class=\"trait\" href=\"documentation/_Wrappers_Compile/trait.Debug.html\" title=\"trait documentation::_Wrappers_Compile::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"documentation/_Wrappers_Compile/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"documentation/_Wrappers_Compile/struct.Formatter.html\" title=\"struct documentation::_Wrappers_Compile::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"documentation/_Wrappers_Compile/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","documentation::error::SdkError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-Display-for-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for SdkError&lt;E, R&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"documentation/_Wrappers_Compile/struct.Formatter.html\" title=\"struct documentation::_Wrappers_Compile::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","documentation::error::SdkError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Error-for-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-Error-for-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> for SdkError&lt;E, R&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static,\n    R: <a class=\"trait\" href=\"documentation/_Wrappers_Compile/trait.Debug.html\" title=\"trait documentation::_Wrappers_Compile::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.source\" class=\"method trait-impl\"><a href=\"#method.source\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.source\" class=\"fn\">source</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;(dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + 'static)&gt;</h4></section></summary><div class='docblock'>The lower-level source of this error, if any. <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.source\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.description\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/error.rs.html#110\">source</a></span><a href=\"#method.description\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.description\" class=\"fn\">description</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a></h4></section></summary><span class=\"item-info\"><div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated since 1.42.0: use the Display impl or to_string()</span></div></span><div class='docblock'> <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.description\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.cause\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/error.rs.html#120\">source</a></span><a href=\"#method.cause\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.cause\" class=\"fn\">cause</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a>&gt;</h4></section></summary><span class=\"item-info\"><div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated since 1.33.0: replaced by Error::source, which can support downcasting</span></div></span></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.provide\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/error.rs.html#183\">source</a><a href=\"#method.provide\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.provide\" class=\"fn\">provide</a>&lt;'a&gt;(&amp;'a self, request: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/error/struct.Request.html\" title=\"struct core::error::Request\">Request</a>&lt;'a&gt;)</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>error_generic_member_access</code>)</span></div></span><div class='docblock'>Provides type based access to context intended for error reports. <a href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html#method.provide\">Read more</a></div></details></div></details>","Error","documentation::error::SdkError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CBuildError%3E-for-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-From%3CBuildError%3E-for-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"documentation/error/struct.BuildError.html\" title=\"struct documentation::error::BuildError\">BuildError</a>&gt; for SdkError&lt;E, R&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: <a class=\"struct\" href=\"documentation/error/struct.BuildError.html\" title=\"struct documentation::error::BuildError\">BuildError</a>) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<BuildError>","documentation::error::SdkError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ProvideErrorMetadata-for-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-ProvideErrorMetadata-for-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; <a class=\"trait\" href=\"documentation/error/trait.ProvideErrorMetadata.html\" title=\"trait documentation::error::ProvideErrorMetadata\">ProvideErrorMetadata</a> for SdkError&lt;E, R&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"documentation/error/trait.ProvideErrorMetadata.html\" title=\"trait documentation::error::ProvideErrorMetadata\">ProvideErrorMetadata</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.meta\" class=\"method trait-impl\"><a href=\"#method.meta\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"documentation/error/trait.ProvideErrorMetadata.html#tymethod.meta\" class=\"fn\">meta</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"documentation/error/struct.ErrorMetadata.html\" title=\"struct documentation::error::ErrorMetadata\">ErrorMetadata</a></h4></section></summary><div class='docblock'>Returns error metadata, which includes the error code, message,\nrequest ID, and potentially additional information.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.code\" class=\"method trait-impl\"><a href=\"#method.code\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"documentation/error/trait.ProvideErrorMetadata.html#method.code\" class=\"fn\">code</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the error code if it’s available.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.message\" class=\"method trait-impl\"><a href=\"#method.message\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"documentation/error/trait.ProvideErrorMetadata.html#method.message\" class=\"fn\">message</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the error message, if there is one.</div></details></div></details>","ProvideErrorMetadata","documentation::error::SdkError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SdkError%3CE,+R%3E\" class=\"impl\"><a href=\"#impl-SdkError%3CE,+R%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;E, R&gt; SdkError&lt;E, R&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.construction_failure\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">construction_failure</a>(\n    source: impl <a class=\"trait\" href=\"documentation/_StandardLibrary_Compile/_UInt_Compile/trait.Into.html\" title=\"trait documentation::_StandardLibrary_Compile::_UInt_Compile::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,\n) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class=\"docblock\"><p>Construct a <code>SdkError</code> for a construction failure</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.timeout_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">timeout_error</a>(\n    source: impl <a class=\"trait\" href=\"documentation/_StandardLibrary_Compile/_UInt_Compile/trait.Into.html\" title=\"trait documentation::_StandardLibrary_Compile::_UInt_Compile::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,\n) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class=\"docblock\"><p>Construct a <code>SdkError</code> for a timeout</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.dispatch_failure\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">dispatch_failure</a>(source: <a class=\"struct\" href=\"documentation/error/struct.ConnectorError.html\" title=\"struct documentation::error::ConnectorError\">ConnectorError</a>) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class=\"docblock\"><p>Construct a <code>SdkError</code> for a dispatch failure with a <a href=\"documentation/error/struct.ConnectorError.html\" title=\"struct documentation::error::ConnectorError\"><code>ConnectorError</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.response_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">response_error</a>(\n    source: impl <a class=\"trait\" href=\"documentation/_StandardLibrary_Compile/_UInt_Compile/trait.Into.html\" title=\"trait documentation::_StandardLibrary_Compile::_UInt_Compile::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,\n    raw: R,\n) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class=\"docblock\"><p>Construct a <code>SdkError</code> for a response error</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.service_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">service_error</a>(source: E, raw: R) -&gt; SdkError&lt;E, R&gt;</h4></section></summary><div class=\"docblock\"><p>Construct a <code>SdkError</code> for a service failure</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_service_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">into_service_error</a>(self) -&gt; E<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + CreateUnhandledError + 'static,\n    R: <a class=\"trait\" href=\"documentation/_Wrappers_Compile/trait.Debug.html\" title=\"trait documentation::_Wrappers_Compile::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Returns the underlying service error <code>E</code> if there is one</p>\n<p>If the <code>SdkError</code> is not a <code>ServiceError</code> (for example, the error is a network timeout),\nthen it will be converted into an unhandled variant of <code>E</code>. This makes it easy to match\non the service’s error response while simultaneously bubbling up transient failures.\nFor example, handling the <code>NoSuchKey</code> error for S3’s <code>GetObject</code> operation may look as\nfollows:</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">match </span>sdk_err.into_service_error() {\n    GetObjectError::NoSuchKey(<span class=\"kw\">_</span>) =&gt; {\n        <span class=\"comment\">// handle NoSuchKey\n    </span>}\n    err @ <span class=\"kw\">_ </span>=&gt; <span class=\"kw\">return </span><span class=\"prelude-val\">Err</span>(err),\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_service_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">as_service_error</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;E</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Returns a reference underlying service error <code>E</code> if there is one</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">if </span>sdk_err.as_service_error().map(|e|e.is_not_found()) == <span class=\"prelude-val\">Some</span>(<span class=\"bool-val\">true</span>) {\n    <span class=\"macro\">println!</span>(<span class=\"string\">\"the object doesn't exist\"</span>);\n    <span class=\"comment\">// return None, or handle this error specifically\n</span>}\n<span class=\"comment\">// ... handle other error cases, happy path, etc.</span></code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_source\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">into_source</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;, SdkError&lt;E, R&gt;&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Converts this error into its error source.</p>\n<p>If there is no error source, then <code>Err(Self)</code> is returned.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.raw_response\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">raw_response</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;R</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Return a reference to this error’s raw response, if it contains one. Otherwise, return <code>None</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_service_error\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">map_service_error</a>&lt;E2&gt;(self, map: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(E) -&gt; E2) -&gt; SdkError&lt;E2, R&gt;</h4></section></summary><div class=\"docblock\"><p>Maps the service error type in <code>SdkError::ServiceError</code></p>\n</div></details></div></details>",0,"documentation::error::SdkError"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()