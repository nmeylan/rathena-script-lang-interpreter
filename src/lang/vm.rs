use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{Stdout, Write};
use std::rc::Rc;
use std::default::Default;
use std::fmt::format;
use std::os::linux::raw::stat;

use std::sync::{Arc, RwLock};
use crate::lang::call_frame::CallFrame;
use crate::lang::chunk::{Chunk, ClassFile};
use crate::lang::class::{Array, Class, Function, Instance};
use crate::lang::compiler::CompilationDetail;
use crate::lang::error::{RuntimeError, TemporaryRuntimeError};

use crate::lang::noop_hasher::NoopHasher;
use crate::lang::thread::Thread;

use crate::lang::value::{Constant, Native, Value, ValueRef, ValueType, Variable};

pub const MAIN_FUNCTION: &str = "_main";

#[derive(Clone)]
pub struct NativeFunction<'vm> {
    pub(crate) name: &'vm str,
    pub(crate) return_type: Option<ValueType>,
    pub(crate) min_arguments: usize,
    pub(crate) max_arguments: usize,
}

pub const NATIVE_FUNCTIONS: &[NativeFunction] = &[
    NativeFunction { name: "getarraysize", return_type: Some(ValueType::Number), min_arguments: 1, max_arguments: 1 },
    NativeFunction { name: "getarg", return_type: None, min_arguments: 1, max_arguments: 2 },
    NativeFunction { name: "cleararray", return_type: None, min_arguments: 3, max_arguments: 3 },
    NativeFunction { name: "setarray", return_type: None, min_arguments: 2, max_arguments: 255 },
    NativeFunction { name: "getelementofarray", return_type: None, min_arguments: 2, max_arguments: 2 },
    NativeFunction { name: "deletearray", return_type: None, min_arguments: 2, max_arguments: 2 },
    NativeFunction { name: "inarray", return_type: None, min_arguments: 2, max_arguments: 2 },
    NativeFunction { name: "copyarray", return_type: None, min_arguments: 3, max_arguments: 3 },
    NativeFunction { name: "setd", return_type: None, min_arguments: 2, max_arguments: 2 },
    NativeFunction { name: "getd", return_type: None, min_arguments: 1, max_arguments: 1 },
    NativeFunction { name: "getvariableofnpc", return_type: None, min_arguments: 2, max_arguments: 2 },
    // stdlib
    NativeFunction { name: "pow", return_type: Some(ValueType::Number), min_arguments: 2, max_arguments: 2 },
];


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
    native_method_handler: Box<dyn NativeMethodHandler>,
}

pub trait NativeMethodHandler: Send + Sync {
    fn handle(&self, native: &Native, _params: Vec<Value>, _program: &Thread, _call_frame: &CallFrame) {
        panic!("Native function {}", native.name);
    }
}

impl Vm {
    pub fn new(native_method_handler: Box<dyn NativeMethodHandler>, debug_flag: u16) -> Vm {
        let mut native_pool: HashMap<u64, Native, NoopHasher> = Default::default();
        native_pool.insert(Self::calculate_hash(&"print".to_string()), Native { name: "println".to_string() });
        native_pool.insert(Self::calculate_hash(&"vm_dump_var".to_string()), Native { name: "vm_dump_var".to_string() });
        native_pool.insert(Self::calculate_hash(&"vm_dump_locals".to_string()), Native { name: "vm_dump_locals".to_string() });

        for native in NATIVE_FUNCTIONS.iter() {
            native_pool.insert(Self::calculate_hash(&native.name.to_string()), Native { name: native.name.to_string() });
        }
        Self {
            debug_flag,
            heap: Default::default(),
            constants_pool: Default::default(),
            native_method_handler,
            native_pool,
            classes_pool: RwLock::new(Default::default()),
        }
    }

    pub fn bootstrap(vm: Arc<Vm>, mut classes: Vec<ClassFile>) {
        for class in classes.iter_mut() {
            for function in class.functions() {
                let chunk = &mut function.chunk.clone();
                vm.extend_constant_pool(std::mem::take(&mut chunk.constants_storage.borrow_mut()));
            }
            let class_rc = vm.register_class(class);
            Self::bootstrap_class(vm.clone(), class_rc).unwrap();
        }
    }

    pub fn execute_main_script(vm: Arc<Vm>) -> Result<(), RuntimeError> {
        let mut program = Thread::new(vm.clone(), vm.debug_flag);
        let instance = vm.classes_pool.read().unwrap().get("_MainScript").as_ref().unwrap().new_instance();
        program.run_main(Arc::new(instance)).map_err(|e| {
            println!("{}", e);
            e
        })
    }

    pub fn run_main_function(vm: Arc<Vm>, class_reference: u64, instance_reference: u64) -> Result<(), RuntimeError> {
        let instance = vm.get_instance_from_heap(class_reference, instance_reference)?;
        let debug_flag = vm.debug_flag;
        let mut thread = Thread::new(vm, debug_flag);
        thread.run_main(instance).map_err(|e| {
            println!("{}", e);
            e
        })
    }

    pub fn create_instance(vm: Arc<Vm>, class_name: String) -> Result<(u64, u64), RuntimeError> {
        let mut instance = Arc::new(vm.classes_pool.read().unwrap().get(&class_name).as_ref().unwrap().new_instance());
        let class = vm.get_class(&instance.class_name);
        vm.store_instance_on_heap(class.hash_code(), instance.hash_code(), instance.clone());
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInstanceInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let mut thread = Thread::new(vm.clone(), vm.debug_flag);
            thread.run_function(class.clone(), &mut Some(instance.clone()), init_function)?;
        }
        Ok((class.hash_code(), instance.hash_code()))
    }

    pub fn bootstrap_class(vm: Arc<Vm>, class: Arc<Class>) -> Result<(), RuntimeError> {
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let debug_flag = vm.debug_flag;
            let mut thread = Thread::new(vm, debug_flag);
            return thread.run_function(class.clone(), &mut None, init_function).map_err(|e| {
                println!("{}", e);
                e
            });
        }
        Ok(())
    }

    pub fn register_class(&self, class_file: &mut ClassFile) -> Arc<Class> {
        let mut functions_pool: HashMap<u64, Function, NoopHasher> = Default::default();
        let mut sources: HashMap<u64, Vec<CompilationDetail>, NoopHasher> = Default::default();
        for function in class_file.functions() {
            let chunk_rc = function.chunk.clone();
            let chunk: &Chunk = chunk_rc.borrow();
            let function_reference = Vm::calculate_hash(&function.name);
            functions_pool.insert(function_reference,
                                  Function::from_chunk(function.name.clone(), chunk.clone()));
            sources.insert(function_reference, chunk.compilation_details.take());
        }
        let class_rc = Arc::new(Class::new(class_file.name.clone(), class_file.reference, functions_pool, sources,
                                          class_file.static_variables.borrow().clone(),
                                          class_file.instance_variables.borrow().clone()));
        self.classes_pool.write().unwrap().insert(class_file.name.clone(), class_rc.clone());
        class_rc
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

    pub fn allocate_array_if_needed(&self, owner_reference: u64, reference: u64, value_type: ValueType) {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            self.heap.write().unwrap().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.read().unwrap();
        let mut owner_entries = heap_ref.get(&owner_reference).unwrap().write().unwrap();
        if owner_entries.get(&reference).is_none() {
            owner_entries.insert(reference, HeapEntry::Array(Arc::new(Array::new(reference, value_type))));
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

    pub fn get_instance_from_heap(&self, owner_reference: u64, reference: u64) -> Result<Arc<Instance>, RuntimeError> {
        if self.heap.read().unwrap().get(&owner_reference).is_none() {
            self.heap.write().unwrap().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.read().unwrap();
        let mut owner_entries = heap_ref.get(&owner_reference).unwrap().read().unwrap();
        let entry = owner_entries.get(&reference).ok_or(RuntimeError::new_internal(format!("Heap entry is not an instance for owner reference {} and reference {}", owner_reference, reference)))?;
        match entry {
            HeapEntry::Instance(entry) => Ok(entry.clone()),
            x => Err(RuntimeError::new_internal(format!("Heap entry does not contain an instance for owner reference {} and reference {}", owner_reference, reference))),
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

    pub(crate) fn array_from_heap_reference(&self, owner_reference: u64, reference: u64) -> Result<Arc<Array>, TemporaryRuntimeError> {
        if let Some(owner_entries) = self.heap.read().unwrap().get(&owner_reference) {
            return Ok(owner_entries.read().unwrap().get(&reference).cloned().unwrap().get_array().unwrap());
        }
        Err(TemporaryRuntimeError::new_string(format!("Array was not found from heap reference ({},{})", owner_reference, reference)))
    }

    pub fn get_from_native_pool(&self, reference: u64) -> Option<&Native> {
        self.native_pool.get(&reference)
    }

    pub fn get_class(&self, name: &String) -> Arc<Class> {
        self.classes_pool.read().unwrap().get(name).unwrap().clone()
    }

    pub fn native_method_handler(&self) -> &Box<dyn NativeMethodHandler> {
        &self.native_method_handler
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
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