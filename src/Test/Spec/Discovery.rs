// AOT spec discovery. The backend registers every module exporting a nullary
// `spec` inside the generated main; `getSpecs` filters the registered names
// with the runtime pattern. This preserves the upstream contract (a module is
// discovered when its name matches the pattern) without dynamic module
// loading, and new fixtures are picked up by rebuilding alone.
//
// The registry stores no values: it keeps the module name and a function
// pointer that builds the spec value. `getSpecs` calls the pointer on the
// caller's thread, so the Rc/Arc carriers are created where they are used and
// the table itself stays `Send + Sync` under both runtimes.
use std::rc::Rc;

use fancy_regex::Regex;

type SpecAccessor = fn() -> crate::UnknownType;

static REGISTERED_SPECS: std::sync::Mutex<Vec<(String, SpecAccessor)>> =
    std::sync::Mutex::new(Vec::new());

/// Called from the generated `main` for every spec module of the program.
pub fn purust_register_spec(name: String, spec: SpecAccessor) {
    REGISTERED_SPECS.lock().unwrap().push((name, spec));
}

fn spec_record(name: String, spec: crate::UnknownType) -> crate::UnknownType {
    let mut fields = purust_core::RecordFields::new();
    fields.insert("name".to_owned(), crate::Value::String(name));
    fields.insert("spec".to_owned(), spec);
    crate::Value::DynamicRecord(perceus_ptr::PerceusPtr::new(fields))
}

pub fn Test_Spec_Discovery_getSpecs() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(|pattern| {
        let pattern = pattern.unwrap_string();
        // An invalid pattern throws in the JavaScript implementation too.
        let regex = Regex::new(&pattern).expect("Test.Spec.Discovery: invalid pattern");
        let specs = REGISTERED_SPECS
            .lock()
            .unwrap()
            .iter()
            .filter(|(name, _)| regex.is_match(name).unwrap_or(false))
            .map(|(name, accessor)| spec_record(name.clone(), accessor()))
            .collect::<Vec<_>>();
        crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_on_error| {
            let specs = specs.clone();
            crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |on_success| {
                on_success.unwrap_func1()(purust_core::mk_array(specs.clone()));
                // Canceler, like the no-op returned by the JavaScript FFI.
                crate::Value::Func1(purust_core::Func1::Static(|_| crate::Value::Unit))
            })))
        })))
    })))
}
