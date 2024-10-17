(function() {var implementors = {
"aws_smithy_runtime":[["impl&lt;B&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"aws_smithy_runtime/client/http/body/minimum_throughput/struct.MinimumThroughputDownloadBody.html\" title=\"struct aws_smithy_runtime::client::http::body::minimum_throughput::MinimumThroughputDownloadBody\">MinimumThroughputDownloadBody</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>&lt;Data = <a class=\"struct\" href=\"bytes/bytes/struct.Bytes.html\" title=\"struct bytes::bytes::Bytes\">Bytes</a>, Error = <a class=\"type\" href=\"aws_smithy_runtime_api/box_error/type.BoxError.html\" title=\"type aws_smithy_runtime_api::box_error::BoxError\">BoxError</a>&gt;,</div>"]],
"aws_smithy_types":[["impl <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"aws_smithy_types/body/struct.SdkBody.html\" title=\"struct aws_smithy_types::body::SdkBody\">SdkBody</a>"]],
"http_body":[],
"http_body_util":[["impl&lt;B&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.BodyStream.html\" title=\"struct http_body_util::BodyStream\">BodyStream</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>,</div>"],["impl&lt;B&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.Limited.html\" title=\"struct http_body_util::Limited\">Limited</a>&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>,\n    B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,</div>"],["impl&lt;B, F, B2&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/combinators/struct.MapFrame.html\" title=\"struct http_body_util::combinators::MapFrame\">MapFrame</a>&lt;B, F&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"struct\" href=\"http_body/frame/struct.Frame.html\" title=\"struct http_body::frame::Frame\">Frame</a>&lt;B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Data\" title=\"type http_body::Body::Data\">Data</a>&gt;) -&gt; <a class=\"struct\" href=\"http_body/frame/struct.Frame.html\" title=\"struct http_body::frame::Frame\">Frame</a>&lt;B2&gt;,\n    B2: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;B, F, E&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/combinators/struct.MapErr.html\" title=\"struct http_body_util::combinators::MapErr\">MapErr</a>&lt;B, F&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(B::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>) -&gt; E,</div>"],["impl&lt;B: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.Collected.html\" title=\"struct http_body_util::Collected\">Collected</a>&lt;B&gt;"],["impl&lt;D&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.Full.html\" title=\"struct http_body_util::Full\">Full</a>&lt;D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;D, E&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/combinators/struct.BoxBody.html\" title=\"struct http_body_util::combinators::BoxBody\">BoxBody</a>&lt;D, E&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;D, E&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/combinators/struct.UnsyncBoxBody.html\" title=\"struct http_body_util::combinators::UnsyncBoxBody\">UnsyncBoxBody</a>&lt;D, E&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;D: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.Empty.html\" title=\"struct http_body_util::Empty\">Empty</a>&lt;D&gt;"],["impl&lt;L, R, Data&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"enum\" href=\"http_body_util/enum.Either.html\" title=\"enum http_body_util::Either\">Either</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>&lt;Data = Data&gt;,\n    R: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>&lt;Data = Data&gt;,\n    L::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,\n    R::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/error/trait.Error.html\" title=\"trait core::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>&gt;&gt;,\n    Data: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;S, D, E&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/struct.StreamBody.html\" title=\"struct http_body_util::StreamBody\">StreamBody</a>&lt;S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"http_body/frame/struct.Frame.html\" title=\"struct http_body::frame::Frame\">Frame</a>&lt;D&gt;, E&gt;&gt;,\n    D: <a class=\"trait\" href=\"bytes/buf/buf_impl/trait.Buf.html\" title=\"trait bytes::buf::buf_impl::Buf\">Buf</a>,</div>"],["impl&lt;T, F&gt; <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a> for <a class=\"struct\" href=\"http_body_util/combinators/struct.WithTrailers.html\" title=\"struct http_body_util::combinators::WithTrailers\">WithTrailers</a>&lt;T, F&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"http_body/trait.Body.html\" title=\"trait http_body::Body\">Body</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"http/header/map/struct.HeaderMap.html\" title=\"struct http::header::map::HeaderMap\">HeaderMap</a>, T::<a class=\"associatedtype\" href=\"http_body/trait.Body.html#associatedtype.Error\" title=\"type http_body::Body::Error\">Error</a>&gt;&gt;&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()