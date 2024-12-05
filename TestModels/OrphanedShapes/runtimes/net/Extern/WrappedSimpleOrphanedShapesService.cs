// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
using Wrappers_Compile;
using Simple.OrphanedShapes;
using Simple.OrphanedShapes.Wrapped;
using TypeConversion = Simple.OrphanedShapes.TypeConversion;
namespace simple.orphanedshapes.internaldafny.wrapped
{
    public partial class __default
    {
        public static _IResult<types.ISimpleOrphanedShapesClient, types._IError> WrappedSimpleOrphanedShapes(types._ISimpleOrphanedShapesConfig config)
        {
            var wrappedConfig = TypeConversion.FromDafny_N6_simple__N14_orphanedShapes__S26_SimpleOrphanedShapesConfig(config);
            var impl = new SimpleOrphanedShapes(wrappedConfig);
            var wrappedClient = new SimpleOrphanedShapesShim(impl);
            return Result<types.ISimpleOrphanedShapesClient, types._IError>.create_Success(wrappedClient);
        }
    }
}
