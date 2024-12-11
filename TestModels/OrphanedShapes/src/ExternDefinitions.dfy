// include "../Model/SimpleOrphanedTypes.dfy"

// module ExternDefinitions {

//   import opened Wrappers
//   import Types = SimpleOrphanedTypes

//   // Dafny-Go compiler maybe-bug:
//   // A module needs at least one non-extern method for the Dafny-Go compiler generate the CompanionStruct_Default___ type
//   method EmptyMethod() { }

//   // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
//   // This will ensure that OrphanedStructure and its conversions are generated,
//   // even though OrphanedStructure is "orphaned".

//   // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
//   // The Smithy model does not know about this operation,
//   //   so it doesn't register OrphanedStructure as an operation shape.
//   // If this operation were on the Smithy model,
//   //   both the operation and OrphanedStructure would no longer be orphaned,
//   //   and wouldn't be useful in an "orphaned shapes" TestModel.
//   // Putting all usage of the orphaned shape outside the model's knowledge
//   //   lets us test orphaned shape model/conversion generation.
//   method {:extern} InitializeOrphanedStructure( input: Types.OrphanedStructure )
//     returns (output: Types.OrphanedStructure)

//   // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
//   // This will ensure that OrphanedStructure and its conversions are generated,
//   // even though OrphanedStructure is "orphaned".

//   // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
//   // The Smithy model does not know about this operation,
//   //   so it doesn't register OrphanedStructure as an operation shape.
//   // If this operation were on the Smithy model,
//   //   both the operation and OrphanedStructure would no longer be orphaned,
//   //   and wouldn't be useful in an "orphaned shapes" TestModel.
//   // Putting all usage of the orphaned shape outside the model's knowledge
//   //   lets us test orphaned shape model/conversion generation.
//   method {:extern} CallNativeOrphanedResource( input: Types.IOrphanedResource )
//     returns (output: Result<Types.OrphanedResourceOperationOutput, Types.Error>)

//   // This extern MUST use Smithy-generated type conversions to initialize the input in the native OrphanedStructure.
//   // This will ensure that OrphanedStructure and its conversions are generated,
//   // even though OrphanedStructure is "orphaned".

//   // OrphanedStructure is orphaned because InitializeOrphanedStructure is not a modelled operation.
//   // The Smithy model does not know about this operation,
//   //   so it doesn't register OrphanedStructure as an operation shape.
//   // If this operation were on the Smithy model,
//   //   both the operation and OrphanedStructure would no longer be orphaned,
//   //   and wouldn't be useful in an "orphaned shapes" TestModel.
//   // Putting all usage of the orphaned shape outside the model's knowledge
//   //   lets us test orphaned shape model/conversion generation.
//   method {:extern} CallNativeOrphanedError( input: Types.Error )
//     returns (output: Types.Error )
// }