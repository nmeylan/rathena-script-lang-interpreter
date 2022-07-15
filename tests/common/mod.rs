use std::sync::Arc;
use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::{Compiler};
use rathena_script_lang_interpreter::lang::error::CompilationError;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Value};
use rathena_script_lang_interpreter::lang::vm::{NativeMethodHandler, Vm};

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

pub struct VmHook {
    pub hook: Box<dyn Fn(Event) + Send + Sync>,
}

impl NativeMethodHandler for VmHook {
    fn handle(&self, native: &rathena_script_lang_interpreter::lang::value::Native, params: Vec<Value>, program: &Thread, call_frame: &CallFrame) {
        if native.name.eq("println") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string()}
                }
            }).collect::<Vec<String>>().join(" "));
            return;
        }
        if native.name.eq("vm_dump_var") {
            if params.len() != 2 {
                println!("vm_dump_var takes exactly 2 args: first arg is the name of the variable for display purpose, second is the variable with its scope and type.");
                return;
            }
            (self.hook)(Event {
                name: params[0].string_value().unwrap().clone(),
                value: params[1].clone(),
            });
        }
        if native.name.eq("vm_dump_locals") {
            call_frame.locals().for_each(|(_, var)| {
                if !var.value_ref.is_set() {
                    return;
                }
                let maybe_local_variable_value = program.vm.get_from_constant_pool(var.value_ref.get_ref())
                    .map(|constant| {
                        constant.value()
                    });
                if maybe_local_variable_value.is_some() {
                    (self.hook)(Event {
                        name: var.name.clone(),
                        value: program.vm.get_from_constant_pool(var.value_ref.get_ref()).map(|constant| constant.value()).unwrap(),
                    });
                }
            });
        }
    }
}

pub fn setup_vm(debug_flag: u16) -> Arc<Vm> {
    let vm = Vm::new("native_functions_list.txt", debug_flag);
    Arc::new(vm)
}

pub fn compile_script(script: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
    Compiler::compile_script("test_script".to_string(), script, "native_functions_list.txt", debug_flag).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e));
        e
    })
}


pub fn compile(script: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
    Compiler::compile("test_script".to_string(), script, "native_functions_list.txt", debug_flag).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e));
        e
    })
}
