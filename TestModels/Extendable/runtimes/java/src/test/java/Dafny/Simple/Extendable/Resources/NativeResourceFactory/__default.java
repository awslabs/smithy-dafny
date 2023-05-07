package simple.extendable.internaldafny.nativeresourcefactory;

import simple.extendable.resources.ToDafny;

import simple.extendable.resources.internaldafny.types.IExtendableResource;
import simple.extendable.resources.NativeResource;

public class __default {
    public static IExtendableResource DafnyFactory() {
        return ToDafny.ExtendableResource(NativeResource.NativeFactory());
    }
}
