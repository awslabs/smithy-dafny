use std::rc::Rc;
use std::cell::UnsafeCell;
use dafny_runtime::rcmut;
use crate::r#_Wrappers_Compile::Result;

pub mod internal_ExternDefinitions_Compile {

    use crate::conversions::*;
    use crate::types::*;
    use crate::implementation_from_dafny::_OrphanedResource_Compile::*;
    use crate::implementation_from_dafny::_ExternDefinitions_Compile::_default;
    use crate::simple::orphaned::internaldafny::types::*;


    impl _default {
        pub fn InitializeOrphanedStructure(
            uninitialized_structure: &Rc<crate::simple::orphaned::internaldafny::types::OrphanedStructure>,
        ) -> Rc<crate::simple::orphaned::internaldafny::types::OrphanedStructure> {
            // Rc::clone(uninitialized_structure)
            let native_structure = orphaned_structure::from_dafny(uninitialized_structure.clone());
            // I don't think generated Rust objects have a "toBuilder" method.
            // Ideally, this extern would convert the Dafny structure to native,
            // then set this property on the converted native structure.
            // But that is sort of outside the scope of this TestModel.
            // The fact that the from/to_dafny conversions above and below exist are the important things.
            let native_structure_new = crate::types::OrphanedStructure::builder().set_string_value(Some(
                "the extern MUST use Smithy-generated conversions to set this value in the native structure".to_string()
            )).build().unwrap();
            return orphaned_structure::to_dafny(&native_structure_new);
        }

        pub fn CallNativeOrphanedResource(
            dafny_resource: &Object<dyn IOrphanedResource>,
        ) -> Rc<Result<Rc<crate::simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput>, Rc<Error>>>
        {
            // let cloned_dafny_resource: Object<dyn IOrphanedResource> = Rc::new(*dafny_resource.clone());
            // let cloned_dafny_resource: Object<dyn IOrphanedResource> = Rc::clone(dafny_resource) as Object<dyn IOrphanedResource>;
            // let native_resource = orphaned_resource::from_dafny(cloned_dafny_resource).inner.borrow();
            // let cloned_dafny_resource: Object<dyn IOrphanedResource> = Rc::new(*Rc::clone(dafny_resource));
            // let native_resource_ref = orphaned_resource::from_dafny(Object(Some(dafny_runtime::rcmut::RcMut::new(Box::new(*dafny_resource) as Box<dyn IOrphanedResource>)));
            // let native_resource_ref = orphaned_resource::from_dafny(Object::new(Box::new(*dafny_resource) as Box<dyn IOrphanedResource>));
            // let native_resource_ref = orphaned_resource::from_dafny(dafny_runtime::Object<IOrphanedResource>(*(dafny_resource as &dyn IOrphanedResource)));

            let wrap = crate::conversions::orphaned_resource::IOrphanedResourceDafnyWrapper {
                obj: dafny_resource.clone(),
            };
            let native_resource_ref = crate::types::orphaned_resource::OrphanedResourceRef {
              inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
            };
            
            // let rc_trait_object: Rc<std::cell::UnsafeCell<dyn IOrphanedResource>> = Rc::new(std::cell::UnsafeCell::new((*dafny_resource)));
            // let option_trait_object: Option<Rc<std::cell::UnsafeCell<dyn IOrphanedResource>>> = Some(rc_trait_object);
            // let native_resource_ref = orphaned_resource::from_dafny(Object(option_trait_object));
            
            
            let native_resource = native_resource_ref.inner.borrow();
            let native_output = native_resource.orphaned_resource_operation(
                crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput{
                    some_string : std::option::Option::Some (
                        "the extern MUST provide this string to the native resource's operation".to_string()
                    )
                }
            );
            
            let dafny_output = orphaned_resource_operation::_orphaned_resource_operation_output::to_dafny(native_output.unwrap());

            ::std::rc::Rc::new(Result::<
                Rc<crate::simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput>,
                Rc<Error>
            >::Success {
                value: dafny_output,
            })
        }

        pub fn CallNativeOrphanedError(
            dafny_error: &Rc<Error>,
        ) -> Rc<Error> {
            // Rc::clone(uninitialized_structure)
            let native_error = crate::conversions::error::from_dafny(dafny_error.clone());
            // I don't think generated Rust objects have a way for me to update the message
            // on a pre-existing object (i.e. public fields or a .toBuilder).
            // Ideally, this extern would convert the Dafny structure to native,
            // then set this property on the converted native structure.
            // But that is sort of outside the scope of this TestModel.
            // The fact that the from/to_dafny conversions above and below exist are the important things.
            let native_error_new = crate::types::error::Error::OrphanedError {
               message : "the extern MUST set this string using the catch-all error converter, NOT the orphaned error-specific converter".to_string()
            };
            return crate::conversions::error::to_dafny(native_error_new);
        }
    }

}