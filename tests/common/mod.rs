use std::sync::{Arc, Mutex};
use std::default::Default;

use std::mem;

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::{CompilationDetail, Compiler};
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
    pub hook_handle_native: Option<Box<dyn Fn(String, &Thread) -> bool + Send + Sync>>,
    pub global_variable_store: Mutex<Vec<GlobalVariableEntry>>,
    pub failed_on_missing_native: bool,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct GlobalVariableEntry {
    pub name: String,
    pub value: Value,
    pub scope: String,
    pub index: Option<usize>,
}

impl VmHook {
    pub fn new(hook: Box<dyn Fn(Event) + Send + Sync>) -> Self {
        Self {
            hook,
            hook_handle_native: None,
            global_variable_store: Default::default(),
            failed_on_missing_native: true,
        }
    }
    pub fn new_with_behavior_on_missing_native(hook: Box<dyn Fn(Event) + Send + Sync>, failed_on_missing_native: bool) -> Self {
        Self {
            hook,
            hook_handle_native: None,
            global_variable_store: Default::default(),
            failed_on_missing_native,
        }
    }
    pub fn new_with_custom_handler(hook: Box<dyn Fn(Event) + Send + Sync>, hook_handle_native: Box<dyn Fn(String, &Thread) -> bool + Send + Sync>) -> Self {
        Self {
            hook,
            hook_handle_native: Some(hook_handle_native),
            global_variable_store: Default::default(),
            failed_on_missing_native: true,
        }
    }

    pub fn push_global(&self, variable: GlobalVariableEntry) {
        self.remove_global_by_name_and_scope(&variable.name, &variable.scope, &variable.index);
        self.global_variable_store.lock().unwrap().push(variable);
    }

    pub fn find_global_by_name_and_scope(&self, name: &String, scope: &String) -> Option<GlobalVariableEntry> {
        self.global_variable_store.lock().unwrap().iter().find(|entry| entry.name == *name && entry.scope == *scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None)).cloned()
    }

    pub fn remove_global_by_name_and_scope(&self, name: &String, scope: &String, index: &Option<usize>) {
        let position = self.global_variable_store.lock().unwrap().iter().position(|entry| entry.name == *name && entry.scope == *scope
            && ((index.is_some() && entry.index.is_some() && index.unwrap() == entry.index.unwrap()) || index.is_none() && entry.index.is_none()));
        if let Some(position) = position {
            self.global_variable_store.lock().unwrap().remove(position);
        }
    }

    pub fn remove_global_by_name_and_scope_and_index(&self, name: &String, scope: &String, index: usize) {
        let position = self.global_variable_store.lock().unwrap().iter().position(|entry| &entry.name == name && &entry.scope == scope
            && entry.index.is_some() && *entry.index.as_ref().unwrap() == index);
        if let Some(position) = position {
            self.global_variable_store.lock().unwrap().remove(position);
        }
    }

    pub fn find_global_array_entries(&self, name: &String, scope: &String) -> Vec<GlobalVariableEntry> {
        self.global_variable_store.lock().unwrap().iter().filter(|entry| &entry.name == name && &entry.scope == scope
            && entry.index.is_some()).cloned().collect::<Vec<GlobalVariableEntry>>()
    }
}

impl NativeMethodHandler for VmHook {
    fn handle(&self, native: &rathena_script_lang_interpreter::lang::value::Native, params: Vec<Value>, thread: &Thread, call_frame: &CallFrame, source_line: &CompilationDetail, _class_name: String) {
        if self.hook_handle_native.is_some() {
            let should_continue = self.hook_handle_native.as_ref().unwrap()(native.name.clone(), thread);
            if !should_continue {
                return;
            }
        }
        if native.name.eq("println") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
            
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
            self.remove_global_by_name_and_scope(variable_name, variable_scope, &None);
            self.global_variable_store.lock().unwrap().push(
                GlobalVariableEntry {
                    name: variable_name.clone(),
                    value: params[2].clone(),
                    scope: variable_scope.clone(),
                    index: None,
                }
            );
        } else if native.name.eq("getglobalvariable") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            let entry = self.find_global_by_name_and_scope(variable_name, variable_scope);
            if let Some(entry) = entry {
                thread.push_constant_on_stack(entry.value);
            } else {
                panic!("getglobalvariable: can't find variable {} with scope {}", variable_name, variable_scope);
            }
        } else if native.name.eq("setglobalarray") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            let mut index = 2;
            loop {
                if index >= params.len() {
                    break;
                }
                let array_index = params[index].number_value().unwrap();
                let value = params[index + 1].clone();
                self.push_global(
                    GlobalVariableEntry {
                        name: variable_name.clone(),
                        value,
                        scope: variable_scope.clone(),
                        index: Some(array_index as usize),
                    }
                );
                index += 2;
            }
        } else if native.name.eq("input") {
            let variable_name = params[0].string_value().unwrap();
            if variable_name.ends_with('$') {
                thread.push_constant_on_stack(Value::new_string("Hello world from input".to_string()));
            } else {
                thread.push_constant_on_stack(Value::new_number(10));
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
        } else if native.name.eq("nativeacceptingarrayref"){
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "nativeacceptingarrayref first argument should be array")).unwrap();
            let _array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
        } else if native.name.eq("removeitemsglobalarray"){
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            let start_index = params[2].number_value().unwrap();
            let end_index = params[3].number_value().unwrap();
            for i in start_index..end_index {
                self.remove_global_by_name_and_scope_and_index(variable_name, variable_scope, i as usize);
            }
        } else {
            if self.failed_on_missing_native {
                panic!("native not handled {}", native.name);
            } else {
                thread.push_constant_on_stack(Value::Number(Some(1)));
                // println!("native not handled {}", native.name);
            }
        }
    }
}

pub fn setup_vm(debug_flag: u16) -> Arc<Vm> {
    let vm = Vm::new("native_functions_list.txt", debug_flag);
    Arc::new(vm)
}

pub fn compile_script(script: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
    Compiler::compile_script("_MainScript".to_string(), script, "native_functions_list.txt", debug_flag).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e));
        e
    })
}


pub fn compile(script: &str, debug_flag: u64) -> Result<Vec<ClassFile>, Vec<CompilationError>> {
    Compiler::compile("_MainScript".to_string(), script, "native_functions_list.txt", debug_flag).map_err(|e| {
        e.iter().for_each(|e| println!("\n{}", e));
        e
    })
}
