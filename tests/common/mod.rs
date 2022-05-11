use std::sync::Arc;
use ragnarok_script_interpreter::lang::value::{Value};
use ragnarok_script_interpreter::lang::vm::{NativeMethodHandler, Vm};

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub value: Value,
}
pub struct VmHook {
    hook: Box<dyn Fn(Event)>
}
impl NativeMethodHandler for VmHook {
    fn handle(&self, native: &ragnarok_script_interpreter::lang::value::Native, params: Vec<Value>) {
        if native.name.eq("println") {
            println!("{}", params.iter().map(|p| p.string_value().clone()).collect::<Vec<String>>().join(" "));
            return;
        }
        if native.name.eq("vm_dump_var") {
            if params.len() != 2 {
                println!("vm_dump_var takes exactly 2 args: first arg is the name of the variable for display purpose, second is the variable with its scope and type.");
                return;
            }
            (self.hook)(Event {
                name: params[0].string_value().clone(),
                value: params[1].clone(),
            });
            return;
        }
    }
}

pub fn setup_vm<F>(hook: F) -> Arc<Vm> where F: 'static + Fn(Event) {
    let mut vm_hook = VmHook{hook: Box::new(hook)};
    let mut vm = Vm::new(Box::new(vm_hook));
    Arc::new(vm)
}