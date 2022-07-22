use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::default::Default;
use std::mem;

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::Compiler;
use rathena_script_lang_interpreter::lang::error::CompilationError;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Value, ValueType};
use rathena_script_lang_interpreter::lang::vm::{NativeMethodHandler, Vm};

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

pub struct VmHook {
    pub hook: Box<dyn Fn(Event) + Send + Sync>,
    pub global_variable_store: Mutex<Vec<GlobalVariableEntry>>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct GlobalVariableEntry {
    pub name: String,
    pub value: Value,
    pub scope: String,
    pub index: Option<usize>
}

impl VmHook {
    pub fn new(hook: Box<dyn Fn(Event) + Send + Sync>) -> Self {
        Self {
            hook,
            global_variable_store: Default::default(),
        }
    }

    pub fn find_global_by_name_and_scope(&self, name: &String, scope: &String) -> Option<GlobalVariableEntry> {
        self.global_variable_store.lock().unwrap().iter().find(|entry| &entry.name == name && &entry.scope == scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None)).cloned()
    }

    pub fn remove_global_by_name_and_scope(&self, name: &String, scope: &String) {
        let position = self.global_variable_store.lock().unwrap().iter().position(|entry| &entry.name == name && &entry.scope == scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None));
        if let Some(position) = position {
            self.global_variable_store.lock().unwrap().remove(position);
        }
    }

    pub fn find_global_array_entries(&self, name: &String, scope: &String) -> Vec<GlobalVariableEntry> {
        self.global_variable_store.lock().unwrap().iter().filter(|entry| &entry.name == name && &entry.scope == scope
            && entry.index.is_some()).map(|e| e.clone()).collect::<Vec<GlobalVariableEntry>>()
    }
}

impl NativeMethodHandler for VmHook {
    fn handle(&self, native: &rathena_script_lang_interpreter::lang::value::Native, params: Vec<Value>, thread: &Thread, call_frame: &CallFrame) {
        if native.name.eq("println") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
            return;
        } else if native.name.eq("vm_dump_var") {
            if params.len() != 2 {
                println!("vm_dump_var takes exactly 2 args: first arg is the name of the variable for display purpose, second is the variable with its scope and type.");
                return;
            }
            (self.hook)(Event {
                name: params[0].string_value().unwrap().clone(),
                value: params[1].clone(),
            });
        } else if native.name.eq("vm_dump_locals") {
            call_frame.locals().for_each(|(_, var)| {
                if !var.value_ref.is_set() {
                    return;
                }
                let maybe_local_variable_value = thread.vm.get_from_constant_pool(var.value_ref.get_ref())
                    .map(|constant| {
                        constant.value()
                    });
                if maybe_local_variable_value.is_some() {
                    (self.hook)(Event {
                        name: var.name.clone(),
                        value: thread.vm.get_from_constant_pool(var.value_ref.get_ref()).map(|constant| constant.value()).unwrap(),
                    });
                }
            });
        } else if native.name.eq("setglobalvariable") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            println!("{}", variable_name);
            self.remove_global_by_name_and_scope(variable_name, variable_scope);
            self.global_variable_store.lock().unwrap().push(
                GlobalVariableEntry {
                    name: variable_name.clone(),
                    value: params[2].clone(),
                    scope: variable_scope.clone(),
                    index: None
                }
            );
        } else if native.name.eq("getglobalvariable") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            println!("{}", variable_name);
            let entry = self.find_global_by_name_and_scope(variable_name, variable_scope);
            if let Some(entry) = entry {
                thread.push_constant_on_stack(entry.value.clone());
            } else {
                panic!("getglobalvariable: can't find variable {} with scope {}", variable_name, variable_scope);
            }
        } else if native.name.eq("setglobalarray") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            let mut index = 2;
            loop {
                if index >= params.len(){
                    break;
                }
                let array_index = params[index].number_value().unwrap();
                let value = params[index + 1].clone();
                self.global_variable_store.lock().unwrap().push(
                    GlobalVariableEntry {
                        name: variable_name.clone(),
                        value,
                        scope: variable_scope.clone(),
                        index: Some(array_index as usize)
                    }
                );
                index += 2;
            }
        } else if native.name.eq("getglobalarray") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            let array_entries = self.find_global_array_entries(variable_name, variable_scope);
            for entry in array_entries.iter() {
                thread.push_constant_on_stack(entry.value.clone());
                thread.push_constant_on_stack(Value::Number(Some(entry.index.unwrap() as i32)));
            }
            thread.push_constant_on_stack(Value::Number(Some((array_entries.len() * 2) as i32)));
        } else {
            panic!("native not handled {}", native.name);
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
