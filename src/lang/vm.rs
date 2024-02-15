use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{Stdout, Write};

use std::default::Default;



use std::sync::{Arc, RwLock};
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{Chunk, ClassFile, FunctionDefinition};
use crate::lang::class::{Class, Function, Instance};
use crate::lang::array::{Array};
use crate::lang::compiler::CompilationDetail;
use crate::lang::error::{RuntimeError, TemporaryRuntimeError};

use crate::lang::noop_hasher::NoopHasher;
use crate::lang::thread::Thread;

use crate::lang::value::{Constant, Native, Value, ValueRef, ValueType, Variable};
use crate::util::file::read_lines;

pub const MAIN_FUNCTION: &str = "_main";

#[derive(Clone)]
pub struct StaticNativeFunction<'vm> {
    pub(crate) name: &'vm str,
    pub(crate) return_type: Option<ValueType>,
    pub(crate) min_arguments: usize,
    pub(crate) max_arguments: usize,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub(crate) name: String,
    pub(crate) return_type: Option<ValueType>,
    pub(crate) min_arguments: usize,
    pub(crate) max_arguments: usize,
}

pub const NATIVE_FUNCTIONS: &[StaticNativeFunction] = &[
    StaticNativeFunction { name: "getarg", return_type: None, min_arguments: 1, max_arguments: 2 },
    StaticNativeFunction { name: "getargcount", return_type: Some(ValueType::Number), min_arguments: 0, max_arguments: 0 },
    StaticNativeFunction { name: "getarraysize", return_type: Some(ValueType::Number), min_arguments: 1, max_arguments: 1 },
    StaticNativeFunction { name: "cleararray", return_type: None, min_arguments: 3, max_arguments: 3 },
    StaticNativeFunction { name: "setarray", return_type: None, min_arguments: 2, max_arguments: 255 },
    StaticNativeFunction { name: "getelementofarray", return_type: None, min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "deletearray", return_type: None, min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "inarray", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "copyarray", return_type: None, min_arguments: 3, max_arguments: 3 },
    StaticNativeFunction { name: "setd", return_type: None, min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "getd", return_type: None, min_arguments: 1, max_arguments: 1 },
    StaticNativeFunction { name: "getvariableofnpc", return_type: None, min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "getglobalarrayref", return_type: None, min_arguments: 1, max_arguments: 1 },
    // stdlib
    StaticNativeFunction { name: "pow", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "rand", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "min", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "max", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
    StaticNativeFunction { name: "implode", return_type: Some(ValueType::String), min_arguments: 2, max_arguments: 2 },
];

impl NativeFunction {
    pub(crate) fn from_vm_native(vm_native: &crate::lang::vm::StaticNativeFunction) -> Self {
        NativeFunction::new(
            vm_native.name.to_string(),
            vm_native.return_type.clone(),
            vm_native.min_arguments,
            vm_native.max_arguments,
        )
    }

    fn new(name: String, return_type: Option<ValueType>, min_arguments: usize, max_arguments: usize) -> Self {
        Self {
            name,
            return_type,
            min_arguments,
            max_arguments,
        }
    }
}

pub enum DebugFlag {
    None,
    All,
    // VM
    Native,
    Function,
    Class,
    Instance,
    Heap,
    Constant,
    // Thread
    Execution,
    Stack,
    // CallFrame
    OpCode,
    LocalsVariable,
}

impl DebugFlag {
    pub fn value(&self) -> u16 {
        match self {
            DebugFlag::None => 0,
            DebugFlag::All => 0xFFFF,
            DebugFlag::Native => 2,
            DebugFlag::Function => 4,
            DebugFlag::Class => 8,
            DebugFlag::Instance => 16,
            DebugFlag::Heap => 32,
            DebugFlag::Execution => 64,
            DebugFlag::Stack => 128,
            DebugFlag::OpCode => 256,
            DebugFlag::LocalsVariable => 512,
            DebugFlag::Constant => 1024,
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum HeapEntry {
    Variable(Arc<Variable>),
    Instance(Arc<Instance>),
    Array(Arc<Array>),
}

impl HeapEntry {
    pub fn value_ref(&self) -> Option<ValueRef> {
        match self {
            HeapEntry::Variable(var) => {
                Some(var.value_ref.borrow().clone())
            }
            _ => None
        }
    }

    pub fn get_array(&self) -> Option<Arc<Array>> {
        match self {
            HeapEntry::Array(array) => Some(array.clone()),
            _ => None
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, HeapEntry::Array(_))
    }
}

pub trait Hashcode {
    fn hash_code(&self) -> u64;
}

pub struct Vm {
    debug_flag: u16,
    heap: RwLock<HashMap<u64, RwLock<HashMap<u64, HeapEntry, NoopHasher>>, NoopHasher>>,
    constants_pool: RwLock<HashMap<u64, Constant, NoopHasher>>,
    classes_pool: RwLock<HashMap<String, Arc<Class>>>,
    native_pool: HashMap<u64, Native, NoopHasher>,
}

pub trait NativeMethodHandler: Send + Sync {
    fn handle(&self, native: &Native, _params: Vec<Value>, _program: &Thread, _call_frame: &CallFrame, source_line: &CompilationDetail, _class_name: String) {
        panic!("Native function {}", native.name);
    }
}

impl Vm {
    pub fn new(native_function_list_file_path: &str, debug_flag: u16) -> Vm {
        let mut native_pool: HashMap<u64, Native, NoopHasher> = Default::default();
        for native in Vm::collect_native_functions(native_function_list_file_path).iter() {
            native_pool.insert(Self::calculate_hash(&native.name), Native { name: native.name.clone() });
        }

        for native in NATIVE_FUNCTIONS.iter() {
            native_pool.insert(Self::calculate_hash(&native.name.to_string()), Native { name: native.name.to_string() });
        }
        Self {
            debug_flag,
            heap: Default::default(),
            constants_pool: Default::default(),
            native_pool,
            classes_pool: RwLock::new(Default::default()),
        }
    }

    pub fn bootstrap(vm: Arc<Vm>, mut classes: Vec<ClassFile>, native_method_handler: Box<&dyn NativeMethodHandler>) {
        for class in classes.iter_mut() {
            for function in class.functions() {
                let chunk = &mut function.chunk.clone();
                vm.extend_constant_pool(std::mem::take(&mut chunk.constants_storage.borrow_mut()));
            }
            let class_rc = vm.register_class(class);
            Self::bootstrap_class(vm.clone(), class_rc, native_method_handler.clone()).unwrap();
        }
    }

    pub fn execute_main_script(vm: Arc<Vm>, native_method_handler: Box<&dyn NativeMethodHandler>, thread_constants: Vec<u32>) -> Result<(), RuntimeError> {
        let mut thread = Thread::new(vm.clone(), vm.debug_flag);
        Self::set_thread_constants(thread_constants, &mut thread);
        let instance = vm.classes_pool.read().unwrap().get("_MainScript").as_ref().unwrap().new_instance();
        thread.run_main(Arc::new(instance), native_method_handler).map_err(|e| {
            println!("{}", e);
            e
        })
    }

    pub fn repl(vm: Arc<Vm>, class_file: &ClassFile, native_method_handler: Box<&dyn NativeMethodHandler>, thread_constants: Vec<u32>) -> Result<(), RuntimeError> {
        let mut thread = Thread::new(vm.clone(), vm.debug_flag);
        Self::set_thread_constants(thread_constants, &mut thread);
        let main_function_hash = Vm::calculate_hash(&String::from("_main"));
        let functions_def = class_file.functions.borrow();
        let main_function_def: &FunctionDefinition = functions_def.get(0).unwrap();
        let class_arc = vm.define_class(class_file, false, false);
        let instance = class_arc.new_instance();
        let function = class_arc.functions_pool.get(&main_function_hash).unwrap();
        vm.extend_constant_pool(main_function_def.chunk.constants_storage.borrow().clone());
        thread.run_function(class_arc.clone(), &mut Some(Arc::new(instance)), function, native_method_handler, vec![]).map_err(|e| {
            e
        })
    }

    pub fn run_main_function(vm: Arc<Vm>, class_reference: u64, instance_reference: u64, native_method_handler: Box<&dyn NativeMethodHandler>, thread_constants: Vec<u32>) -> Result<(), RuntimeError> {
        let instance = vm.get_instance_from_heap(class_reference, instance_reference).map_err(|e| RuntimeError::from_temporary(CompilationDetail::new_empty(), vec![], e))?;
        let debug_flag = vm.debug_flag;
        let mut thread = Thread::new(vm, debug_flag);
        Self::set_thread_constants(thread_constants, &mut thread);
        thread.run_main(instance, native_method_handler).map_err(|e| {
            println!("{}", e);
            e
        })
    }

    fn set_thread_constants(thread_constants: Vec<u32>, thread: &mut Thread) {
        let mut i = 0;
        for thread_constant in thread_constants {
            thread.set_constant(i, thread_constant);
            i += 1;
        }
    }

    pub fn create_instance(vm: Arc<Vm>, class_name: String, native_method_handler: Box<&dyn NativeMethodHandler>, constructor_args: Vec<Value>) -> Result<(u64, u64), RuntimeError> {
        let instance = Arc::new(vm.classes_pool.read().unwrap().get(&class_name).as_ref().unwrap().new_instance());
        let class = vm.get_class(&instance.class_name);
        vm.store_instance_on_heap(class.hash_code(), instance.hash_code(), instance.clone());
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInstanceInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let mut thread = Thread::new(vm.clone(), vm.debug_flag);
            thread.run_function(class.clone(), &mut Some(instance.clone()), init_function, native_method_handler, constructor_args)?;
        }
        Ok((class.hash_code(), instance.hash_code()))
    }

    pub fn bootstrap_class(vm: Arc<Vm>, class: Arc<Class>, native_method_handler: Box<&dyn NativeMethodHandler>) -> Result<(), RuntimeError> {
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let debug_flag = vm.debug_flag;
            let mut thread = Thread::new(vm, debug_flag);
            return thread.run_function(class.clone(), &mut None, init_function, native_method_handler, vec![]).map_err(|e| {
                println!("{}", e);
                e
            });
        }
        Ok(())
    }

    fn define_class(&self, class_file: &ClassFile, take_chunk: bool, keep_source: bool) -> Arc<Class> {
        let mut functions_pool: HashMap<u64, Function, NoopHasher> = Default::default();
        let mut sources: HashMap<u64, Vec<CompilationDetail>, NoopHasher> = Default::default();
        for function in class_file.functions().iter() {
            let chunk_rc = function.chunk.clone();
            let chunk: &Chunk = chunk_rc.borrow();
            let function_reference = Vm::calculate_hash(&function.name);
            functions_pool.insert(function_reference,
                                  Function::from_chunk(function.name.clone(), chunk.clone()));
            if keep_source {
                if take_chunk {
                    sources.insert(function_reference, chunk.compilation_details.take());
                } else {
                    sources.insert(function_reference, chunk.compilation_details.borrow().clone());
                }
            }
        }
        Arc::new(Class::new(class_file.name.clone(), class_file.reference, functions_pool, sources,
                                           class_file.static_variables.borrow().clone(),
                                           class_file.instance_variables.borrow().clone()))
    }


    pub fn register_class(&self, class_file: &mut ClassFile) -> Arc<Class> {
        let class_arc = self.define_class(class_file, true, true);
        self.classes_pool.write().unwrap().insert(class_file.name.clone(), class_arc.clone());
        class_arc
    }

    pub fn extend_constant_pool(&self, constant_pool: HashMap<u64, Constant, NoopHasher>) {
        let mut constant_pool_ref_mut = self.constants_pool.write().unwrap();
        constant_pool_ref_mut.extend(constant_pool);
    }

    pub fn get_from_constant_pool(&self, reference: u64) -> Option<Constant> {
        let constant_pool_ref = self.constants_pool.read().unwrap();
        constant_pool_ref.get(&reference).cloned()
    }

    pub fn get_value_ref_from_heap_entry(&self, owner_reference: u64, reference: u64) -> Result<ValueRef, TemporaryRuntimeError> {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            return Err(TemporaryRuntimeError::new_string(format!("Can't retrieve value from heap entry, because there is nothing on the heap for owner {}", owner_reference)));
        }
        let heap_ref = self.heap.read().unwrap();
        let owner_entries = heap_ref.get(&owner_reference).unwrap().read().unwrap();
        let heap_entry = owner_entries.get(&reference).ok_or_else(|| TemporaryRuntimeError::new(format!("Can't find heap entry in VM HEAP pool for given reference ({}, {})", owner_reference, reference).as_str()))?;
        if let Some(value_ref) = heap_entry.value_ref() {
            Ok(value_ref)
        } else {
            Err(TemporaryRuntimeError::new("Can't retrieve value from heap entry, because heap entry is not a variable. Probably a reference to an array is being used as function arguments."))
        }
    }

    pub fn allocate_array_if_needed(&self, owner_reference: u64, reference: u64, value_type: ValueType, variable: &Variable) {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            self.heap.write().unwrap().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.read().unwrap();
        let mut owner_entries = heap_ref.get(&owner_reference).unwrap().write().unwrap();
        if owner_entries.get(&reference).is_none() {
            owner_entries.insert(reference, HeapEntry::Array(Arc::new(Array::new(reference, value_type, variable.scope.clone(), variable.name.clone()))));
        }
    }

    pub fn store_instance_on_heap(&self, owner_reference: u64, reference: u64, instance: Arc<Instance>) {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            self.heap.write().unwrap().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.read().unwrap();
        let mut owner_entries = heap_ref.get(&owner_reference).unwrap().write().unwrap();
        owner_entries.insert(reference, HeapEntry::Instance(instance));
    }

    pub fn get_instance_from_heap(&self, owner_reference: u64, reference: u64) -> Result<Arc<Instance>, TemporaryRuntimeError> {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            self.heap.write().unwrap().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.read().unwrap();
        let owner_entries = heap_ref.get(&owner_reference).unwrap().read().unwrap();
        let entry = owner_entries.get(&reference).ok_or(TemporaryRuntimeError::new_string(format!("Heap entry is not an instance for owner reference {} and reference {}", owner_reference, reference)))?;
        match entry {
            HeapEntry::Instance(entry) => Ok(entry.clone()),
            _x => Err(TemporaryRuntimeError::new_string(format!("Heap entry does not contain an instance for owner reference {} and reference {}", owner_reference, reference))),
        }
    }

    pub fn add_in_constant_pool(&self, value: Value) -> u64 {
        let constant = match value {
            Value::String(str) => Constant::String(str.unwrap()),
            Value::Number(num) => Constant::Number(num.unwrap()),
            Value::ArrayEntry(array_entry) => array_entry.unwrap().2.unwrap(),
            x => panic!("add_in_constant_pool not match - only value of type String or Number can be added, but was {:?}", x)
        };
        let hash = Self::calculate_hash(&constant);
        self.constants_pool.write().unwrap().insert(hash, constant);
        hash
    }

    pub fn array_from_heap_reference(&self, owner_reference: u64, reference: u64) -> Result<Arc<Array>, TemporaryRuntimeError> {
        if let Some(owner_entries) = self.heap.read().unwrap().get(&owner_reference) {
            return Ok(owner_entries.read().unwrap().get(&reference).cloned().unwrap().get_array().unwrap());
        }
        Err(TemporaryRuntimeError::new_string(format!("Array was not found from heap reference ({},{})", owner_reference, reference)))
    }

    pub fn get_from_native_pool(&self, reference: u64) -> Option<&Native> {
        self.native_pool.get(&reference)
    }

    pub fn get_global_class(&self, _reference: &u64) -> Arc<Class> {
        self.get_class(&String::from("_Global"))
    }

    pub fn get_class(&self, name: &String) -> Arc<Class> {
        self.classes_pool.read().unwrap().get(name).unwrap().clone()
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    pub fn collect_native_functions(native_function_list_file_path: &str) -> Vec<NativeFunction> {
        let mut native_functions: Vec<NativeFunction> = vec![];
        let result = read_lines(native_function_list_file_path);
        if result.is_err() {
            panic!("{}", result.err().unwrap());
        }
        if let Ok(lines) = result {
            for line in lines.flatten() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('/') {
                    continue;
                }
                let split = line.split(',');
                let split: Vec<&str> = split.map(|item| item.trim()).collect();
                let min_arguments = split[1].parse::<usize>().unwrap();
                let max_arguments = split[2].parse::<usize>().unwrap();
                let return_type = if split.len() > 3 {
                    match split[3] {
                        "Number" | "number" => Some(ValueType::Number),
                        "String" | "string" => Some(ValueType::String),
                        _ => None
                    }
                } else {
                    None
                };
                native_functions.push(NativeFunction { name: split[0].to_string(), return_type, min_arguments, max_arguments});
            }
        } else {
            panic!()
        }
        native_functions
    }

    pub fn dump(&self, out: &mut Stdout) {
        if self.debug_flag & DebugFlag::Native.value() == DebugFlag::Native.value() {
            writeln!(out, "========= VM Native functions =========").unwrap();
            for (reference, native) in self.native_pool.borrow().iter() {
                writeln!(out, "[{}]{}()", reference, native.name).unwrap();
            }
        }
        if self.debug_flag & DebugFlag::Constant.value() == DebugFlag::Constant.value() {
            writeln!(out, "========= VM Constants Pool =========").unwrap();
            for (reference, constant) in self.constants_pool.read().unwrap().iter() {
                match constant {
                    Constant::String(_v) => writeln!(out, "({}) \"{}\"", reference, constant).unwrap(),
                    Constant::Number(_v) => writeln!(out, "({}) {}", reference, constant).unwrap(),
                }
            }
        }
        if self.debug_flag & DebugFlag::Heap.value() == DebugFlag::Heap.value() {
            writeln!(out, "========= VM Heap =========").unwrap();
            for (owner_reference, owner_entries) in self.heap.read().unwrap().iter() {
                for (reference, owner_entry) in owner_entries.read().unwrap().iter() {
                    writeln!(out, "[{}]({}) {:?}", owner_reference, reference, owner_entry).unwrap();
                }
            }
        }
    }
}