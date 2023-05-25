package simple.extendable.resources.internaldafny.wrapped;

import simple.extendable.resources.SimpleExtendableResources;
import simple.extendable.resources.ToNative;
import simple.extendable.resources.wrapped.TestSimpleExtendableResources;

import simple.extendable.resources.internaldafny.types.IExtendableResource;
import simple.extendable.resources.internaldafny.types.ISimpleExtendableResourcesClient;
import simple.extendable.resources.internaldafny.types.Error;

import simple.extendable.resources.internaldafny.types.SimpleExtendableResourcesConfig;
import simple.extendable.resources.NativeResource;
import Wrappers_Compile.Result;

import static simple.extendable.resources.internaldafny.nativeresourcefactory.__default.DafnyFactory;

public class __default extends _ExternBase___default {
    public static Result<ISimpleExtendableResourcesClient, Error> WrappedSimpleExtendableResources(SimpleExtendableResourcesConfig config) {
        TestUnwrapExtendable();
        simple.extendable.resources.model.SimpleExtendableResourcesConfig wrappedConfig = ToNative.SimpleExtendableResourcesConfig(config);
        simple.extendable.resources.SimpleExtendableResources impl = SimpleExtendableResources.builder().SimpleExtendableResourcesConfig(wrappedConfig).build();
        TestSimpleExtendableResources wrappedClient = TestSimpleExtendableResources.builder().impl(impl).build();
        return Result.create_Success(wrappedClient);
    }

    /**
     * We have not developed the ability to call ToNative from Dafny source code at this time.
     * But we need to test this un-wrapping, so this is written in native code until we figure that out.
     */
    public static void TestUnwrapExtendable() {
        IExtendableResource dafnyWrappingNativeWrappingDafny = DafnyFactory();
        simple.extendable.resources.IExtendableResource nativeWrappingDafny = ToNative.ExtendableResource(dafnyWrappingNativeWrappingDafny);
        if (!(nativeWrappingDafny instanceof NativeResource)) {
            throw new AssertionError(
                    "Polymorph MUST generate conversion methods " +
                            "capable of wrapping & un-wrapping" +
                            "these native resources.");
        }
    }
}
