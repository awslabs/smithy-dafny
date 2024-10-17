(function() {var implementors = {
"futures_core":[],
"futures_util":[["impl&lt;A, B&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"enum\" href=\"futures_util/future/enum.Either.html\" title=\"enum futures_util::future::Either\">Either</a>&lt;A, B&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    B: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>&lt;Item = A::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;,</div>"],["impl&lt;A, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>() -&gt; A&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.RepeatWith.html\" title=\"struct futures_util::stream::RepeatWith\">RepeatWith</a>&lt;F&gt;"],["impl&lt;B, St, S, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Scan.html\" title=\"struct futures_util::stream::Scan\">Scan</a>&lt;St, S, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut S</a>, St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;B&gt;&gt;,</div>"],["impl&lt;F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/future/struct.FlattenStream.html\" title=\"struct futures_util::future::FlattenStream\">FlattenStream</a>&lt;F&gt;<div class=\"where\">where\n    Flatten&lt;F, &lt;F as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html#associatedtype.Output\" title=\"type core::future::future::Future::Output\">Output</a>&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,</div>"],["impl&lt;F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/future/struct.IntoStream.html\" title=\"struct futures_util::future::IntoStream\">IntoStream</a>&lt;F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Once.html\" title=\"struct futures_util::stream::Once\">Once</a>&lt;F&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;Fut&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/future/struct.TryFlattenStream.html\" title=\"struct futures_util::future::TryFlattenStream\">TryFlattenStream</a>&lt;Fut&gt;<div class=\"where\">where\n    TryFlatten&lt;Fut, Fut::<a class=\"associatedtype\" href=\"futures_util/future/trait.TryFuture.html#associatedtype.Ok\" title=\"type futures_util::future::TryFuture::Ok\">Ok</a>&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>,</div>"],["impl&lt;Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Once.html\" title=\"struct futures_util::stream::Once\">Once</a>&lt;Fut&gt;"],["impl&lt;S: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Fuse.html\" title=\"struct futures_util::stream::Fuse\">Fuse</a>&lt;S&gt;"],["impl&lt;S: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.PollImmediate.html\" title=\"struct futures_util::stream::PollImmediate\">PollImmediate</a>&lt;S&gt;"],["impl&lt;St&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Cycle.html\" title=\"struct futures_util::stream::Cycle\">Cycle</a>&lt;St&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,</div>"],["impl&lt;St&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Flatten.html\" title=\"struct futures_util::stream::Flatten\">Flatten</a>&lt;St&gt;<div class=\"where\">where\n    Flatten&lt;St, St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,</div>"],["impl&lt;St&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Take.html\" title=\"struct futures_util::stream::Take\">Take</a>&lt;St&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TryFlatten.html\" title=\"struct futures_util::stream::TryFlatten\">TryFlatten</a>&lt;St&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a>,\n    &lt;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a> as <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a>&gt;::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>&gt;,</div>"],["impl&lt;St, E&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.ErrInto.html\" title=\"struct futures_util::stream::ErrInto\">ErrInto</a>&lt;St, E&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.MapErr.html\" title=\"struct futures_util::stream::MapErr\">MapErr</a>&lt;St, IntoFn&lt;E&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Inspect.html\" title=\"struct futures_util::stream::Inspect\">Inspect</a>&lt;St, F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Map.html\" title=\"struct futures_util::stream::Map\">Map</a>&lt;St, InspectFn&lt;F&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.InspectErr.html\" title=\"struct futures_util::stream::InspectErr\">InspectErr</a>&lt;St, F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Inspect.html\" title=\"struct futures_util::stream::Inspect\">Inspect</a>&lt;<a class=\"struct\" href=\"futures_util/stream/struct.IntoStream.html\" title=\"struct futures_util::stream::IntoStream\">IntoStream</a>&lt;St&gt;, InspectErrFn&lt;F&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.InspectOk.html\" title=\"struct futures_util::stream::InspectOk\">InspectOk</a>&lt;St, F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Inspect.html\" title=\"struct futures_util::stream::Inspect\">Inspect</a>&lt;<a class=\"struct\" href=\"futures_util/stream/struct.IntoStream.html\" title=\"struct futures_util::stream::IntoStream\">IntoStream</a>&lt;St&gt;, InspectOkFn&lt;F&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Map.html\" title=\"struct futures_util::stream::Map\">Map</a>&lt;St, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: FnMut1&lt;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.MapErr.html\" title=\"struct futures_util::stream::MapErr\">MapErr</a>&lt;St, F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Map.html\" title=\"struct futures_util::stream::Map\">Map</a>&lt;<a class=\"struct\" href=\"futures_util/stream/struct.IntoStream.html\" title=\"struct futures_util::stream::IntoStream\">IntoStream</a>&lt;St&gt;, MapErrFn&lt;F&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.MapOk.html\" title=\"struct futures_util::stream::MapOk\">MapOk</a>&lt;St, F&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"futures_util/stream/struct.Map.html\" title=\"struct futures_util::stream::Map\">Map</a>&lt;<a class=\"struct\" href=\"futures_util/stream/struct.IntoStream.html\" title=\"struct futures_util::stream::IntoStream\">IntoStream</a>&lt;St&gt;, MapOkFn&lt;F&gt;&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St, Fut&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TakeUntil.html\" title=\"struct futures_util::stream::TakeUntil\">TakeUntil</a>&lt;St, Fut&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.AndThen.html\" title=\"struct futures_util::stream::AndThen\">AndThen</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>&lt;Error = St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Filter.html\" title=\"struct futures_util::stream::Filter\">Filter</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.OrElse.html\" title=\"struct futures_util::stream::OrElse\">OrElse</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>&lt;Ok = St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.SkipWhile.html\" title=\"struct futures_util::stream::SkipWhile\">SkipWhile</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TakeWhile.html\" title=\"struct futures_util::stream::TakeWhile\">TakeWhile</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Then.html\" title=\"struct futures_util::stream::Then\">Then</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TryFilter.html\" title=\"struct futures_util::stream::TryFilter\">TryFilter</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TrySkipWhile.html\" title=\"struct futures_util::stream::TrySkipWhile\">TrySkipWhile</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>&lt;Ok = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>, Error = St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>&gt;,</div>"],["impl&lt;St, Fut, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TryTakeWhile.html\" title=\"struct futures_util::stream::TryTakeWhile\">TryTakeWhile</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>&lt;Ok = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a>, Error = St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>&gt;,</div>"],["impl&lt;St, Fut, F, T&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.FilterMap.html\" title=\"struct futures_util::stream::FilterMap\">FilterMap</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    F: FnMut1&lt;St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>, Output = Fut&gt;,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt;,</div>"],["impl&lt;St, Fut, F, T&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.TryFilterMap.html\" title=\"struct futures_util::stream::TryFilterMap\">TryFilterMap</a>&lt;St, Fut, F&gt;<div class=\"where\">where\n    St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,\n    Fut: <a class=\"trait\" href=\"futures_util/future/trait.TryFuture.html\" title=\"trait futures_util::future::TryFuture\">TryFuture</a>&lt;Ok = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;, Error = St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Error\" title=\"type futures_util::stream::TryStream::Error\">Error</a>&gt;,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(St::<a class=\"associatedtype\" href=\"futures_util/stream/trait.TryStream.html#associatedtype.Ok\" title=\"type futures_util::stream::TryStream::Ok\">Ok</a>) -&gt; Fut,</div>"],["impl&lt;St, U, F&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.FlatMap.html\" title=\"struct futures_util::stream::FlatMap\">FlatMap</a>&lt;St, U, F&gt;<div class=\"where\">where\n    Flatten&lt;<a class=\"struct\" href=\"futures_util/stream/struct.Map.html\" title=\"struct futures_util::stream::Map\">Map</a>&lt;St, F&gt;, U&gt;: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>,</div>"],["impl&lt;St1, St2&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Chain.html\" title=\"struct futures_util::stream::Chain\">Chain</a>&lt;St1, St2&gt;<div class=\"where\">where\n    St1: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,\n    St2: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>&lt;Item = St1::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;,</div>"],["impl&lt;St1, St2&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Select.html\" title=\"struct futures_util::stream::Select\">Select</a>&lt;St1, St2&gt;<div class=\"where\">where\n    St1: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,\n    St2: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>&lt;Item = St1::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;,</div>"],["impl&lt;St1, St2&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Zip.html\" title=\"struct futures_util::stream::Zip\">Zip</a>&lt;St1, St2&gt;<div class=\"where\">where\n    St1: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,\n    St2: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,</div>"],["impl&lt;St1, St2, Clos, State&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.SelectWithStrategy.html\" title=\"struct futures_util::stream::SelectWithStrategy\">SelectWithStrategy</a>&lt;St1, St2, Clos, State&gt;<div class=\"where\">where\n    St1: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>,\n    St2: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>&lt;Item = St1::<a class=\"associatedtype\" href=\"futures_util/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_util::stream::Stream::Item\">Item</a>&gt;,\n    Clos: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut State</a>) -&gt; <a class=\"enum\" href=\"futures_util/stream/enum.PollNext.html\" title=\"enum futures_util::stream::PollNext\">PollNext</a>,</div>"],["impl&lt;St: <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Skip.html\" title=\"struct futures_util::stream::Skip\">Skip</a>&lt;St&gt;"],["impl&lt;St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Enumerate.html\" title=\"struct futures_util::stream::Enumerate\">Enumerate</a>&lt;St&gt;"],["impl&lt;St: <a class=\"trait\" href=\"futures_util/stream/trait.Stream.html\" title=\"trait futures_util::stream::Stream\">Stream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Peekable.html\" title=\"struct futures_util::stream::Peekable\">Peekable</a>&lt;St&gt;"],["impl&lt;St: <a class=\"trait\" href=\"futures_util/stream/trait.TryStream.html\" title=\"trait futures_util::stream::TryStream\">TryStream</a> + <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a>&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.IntoStream.html\" title=\"struct futures_util::stream::IntoStream\">IntoStream</a>&lt;St&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Empty.html\" title=\"struct futures_util::stream::Empty\">Empty</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Pending.html\" title=\"struct futures_util::stream::Pending\">Pending</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Repeat.html\" title=\"struct futures_util::stream::Repeat\">Repeat</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;T, F, Fut, Item&gt; <a class=\"trait\" href=\"futures_util/stream/trait.FusedStream.html\" title=\"trait futures_util::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_util/stream/struct.Unfold.html\" title=\"struct futures_util::stream::Unfold\">Unfold</a>&lt;T, F, Fut&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(T) -&gt; Fut,\n    Fut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(Item, T)</a>&gt;&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()