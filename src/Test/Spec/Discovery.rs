// Native binaries have no dynamic module loading, so discovery cannot scan a
// directory of compiled modules. `specs` lists them explicitly instead.
pub fn Test_Spec_Discovery_getSpecs() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(|_pattern| {
        crate::Value::Func1(purust_core::Func1::Shared(std::rc::Rc::new(
            |_unit| -> crate::UnknownType {
                panic!("Test.Spec.Discovery: spec discovery is not supported by the native backend; list your specs explicitly")
            },
        )))
    })))
}
