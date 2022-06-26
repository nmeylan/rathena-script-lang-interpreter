use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::io::{Stdout, Write};
use std::rc::Rc;
use std::default::Default;

use std::sync::Arc;
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
    Variable(Rc<Variable>),
    Instance(Rc<Instance>),
    Array(Rc<Array>),
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

    pub fn get_array(&self) -> Option<Rc<Array>> {
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
    heap: RefCell<HashMap<u64, RefCell<HashMap<u64, HeapEntry, NoopHasher>>, NoopHasher>>,
    constants_pool: RefCell<HashMap<u64, Constant, NoopHasher>>,
    classes_pool: RefCell<HashMap<String, Rc<Class>>>,
    native_pool: HashMap<u64, Native, NoopHasher>,
    native_method_handler: Box<dyn NativeMethodHandler>,
}

pub trait NativeMethodHandler {
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
            classes_pool: RefCell::new(Default::default()),
        }
    }

    pub fn bootstrap(vm: Arc<Vm>, mut classes: Vec<ClassFile>) {
        for class in classes.iter_mut() {
            for function in class.functions() {
                let chunk = &mut function.chunk.clone();
                vm.extend_constant_pool(std::mem::take(&mut chunk.constants_storage.borrow_mut()));
            }
            let class_rc = vm.register_class(class);
            Self::init_class(vm.clone(), class_rc).unwrap();
        }
    }

    pub fn execute_main_script(vm: Arc<Vm>) -> Result<(), RuntimeError> {
        let mut program = Thread::new(vm.clone(), vm.debug_flag);
        program.run_main(vm.classes_pool.borrow().get("_MainScript").as_ref().unwrap().new_instance()).map_err(|e| {
            println!("{}", e);
            e
        })
    }
    pub fn execute_class(vm: Arc<Vm>, class_name: String) -> Result<(), RuntimeError> {
        let mut instance = vm.classes_pool.borrow().get(&class_name).as_ref().unwrap().new_instance();
        let class = vm.get_class(&instance.class_name);
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInstanceInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let mut program = Thread::new(vm.clone(), vm.debug_flag);
            program.run_function(class.clone(), &mut Some(&mut instance), init_function)?;
        }
        let debug_flag = vm.debug_flag;
        let mut program = Thread::new(vm, debug_flag);
        program.run_main(instance).map_err(|e| {
            println!("{}", e);
            e
        })
    }

    pub fn init_class(vm: Arc<Vm>, class: Rc<Class>) -> Result<(), RuntimeError> {
        let maybe_init_function = class.functions_pool.get(&Vm::calculate_hash(&"_OnInit".to_string()));
        if let Some(init_function) = maybe_init_function {
            let debug_flag = vm.debug_flag;
            let mut program = Thread::new(vm, debug_flag);
            return program.run_function(class.clone(), &mut None, init_function).map_err(|e| {
                println!("{}", e);
                e
            });
        }
        Ok(())
    }

    pub fn register_class(&self, class: &mut ClassFile) -> Rc<Class> {
        let mut functions_pool: HashMap<u64, Function, NoopHasher> = Default::default();
        let mut sources: HashMap<u64, Vec<CompilationDetail>, NoopHasher> = Default::default();
        for function in class.functions() {
            let chunk_rc = function.chunk.clone();
            let chunk: &Chunk = chunk_rc.borrow();
            let function_reference = Vm::calculate_hash(&function.name);
            functions_pool.insert(function_reference,
                                  Function::from_chunk(function.name.clone(), chunk.clone()));
            sources.insert(function_reference, chunk.compilation_details.take());
        }
        let class_rc = Rc::new(Class::new(class.name.clone(), functions_pool, sources,
                                          class.static_variables.borrow().clone(),
                                          class.instance_variables.borrow().clone()));
        self.classes_pool.borrow_mut().insert(class.name.clone(), class_rc.clone());
        class_rc
    }

    pub fn extend_constant_pool(&self, constant_pool: HashMap<u64, Constant, NoopHasher>) {
        let mut constant_pool_ref_mut = self.constants_pool.borrow_mut();
        constant_pool_ref_mut.extend(constant_pool);
    }

    pub fn get_from_constant_pool(&self, reference: u64) -> Option<Constant> {
        let constant_pool_ref = self.constants_pool.borrow();
        constant_pool_ref.get(&reference).cloned()
    }

    pub fn get_value_ref_from_heap_entry(&self, owner_reference: u64, reference: u64) -> Result<ValueRef, TemporaryRuntimeError> {
        if self.heap.borrow().get(&owner_reference).is_none() {
            return Err(TemporaryRuntimeError::new_string(format!("Can't retrieve value from heap entry, because there is nothing on the heap for owner {}", owner_reference)));
        }
        let heap_ref = self.heap.borrow();
        let owner_entries = heap_ref.get(&owner_reference).unwrap().borrow();
        let heap_entry = owner_entries.get(&reference).ok_or_else(|| TemporaryRuntimeError::new(format!("Can't find heap entry in VM HEAP pool for given reference ({}, {})", owner_reference, reference).as_str()))?;
        if let Some(value_ref) = heap_entry.value_ref() {
            Ok(value_ref)
        } else {
            Err(TemporaryRuntimeError::new("Can't retrieve value from heap entry, because heap entry is not a variable. Probably a reference to an array is being used as function arguments."))
        }
    }

    pub fn allocate_array_if_needed(&self, owner_reference: u64, reference: u64, value_type: ValueType) {
        if self.heap.borrow().get(&owner_reference).is_none() {
            self.heap.borrow_mut().insert(owner_reference, Default::default());
        }
        let heap_ref = self.heap.borrow();
        let mut owner_entries = heap_ref.get(&owner_reference).unwrap().borrow_mut();
        if owner_entries.get(&reference).is_none() {
            owner_entries.insert(reference, HeapEntry::Array(Rc::new(Array::new(reference, value_type))));
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
        self.constants_pool.borrow_mut().insert(hash, constant);
        hash
    }

    pub(crate) fn array_from_heap_reference(&self, owner_reference: u64, reference: u64) -> Result<Rc<Array>, TemporaryRuntimeError> {
        if let Some(owner_entries) = self.heap.borrow().get(&owner_reference) {
            return Ok(owner_entries.borrow().get(&reference).cloned().unwrap().get_array().unwrap());
        }
        Err(TemporaryRuntimeError::new_string(format!("Array was not found from heap reference ({},{})", owner_reference, reference)))
    }

    pub fn get_from_native_pool(&self, reference: u64) -> Option<&Native> {
        self.native_pool.get(&reference)
    }

    pub fn get_class(&self, name: &String) -> Rc<Class> {
        self.classes_pool.borrow().get(name).unwrap().clone()
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
            for (reference, constant) in self.constants_pool.borrow().iter() {
                match constant {
                    Constant::String(_v) => writeln!(out, "({}) \"{}\"", reference, constant).unwrap(),
                    Constant::Number(_v) => writeln!(out, "({}) {}", reference, constant).unwrap(),
                }
            }
        }
        if self.debug_flag & DebugFlag::Heap.value() == DebugFlag::Heap.value() {
            writeln!(out, "========= VM Heap =========").unwrap();
            for (owner_reference, owner_entries) in self.heap.borrow().iter() {
                for (reference, owner_entry) in owner_entries.borrow().iter() {
                    writeln!(out, "[{}]({}) {:?}", owner_reference, reference, owner_entry).unwrap();
                }
            }
        }
    }
}