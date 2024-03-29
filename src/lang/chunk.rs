use std::borrow::Borrow;
use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::lang::compiler::CompilationDetail;

use crate::lang::noop_hasher::NoopHasher;
use crate::lang::value::{Constant, Scope, ValueType, Variable};
use crate::lang::vm::{MAIN_FUNCTION, Vm};

#[derive(Serialize, Deserialize,Debug)]
pub struct ClassFile {
    pub name: String,
    pub reference: u64,
    pub defined_in_file_name: String,
    pub defined_at_line: usize,
    pub functions: RefCell<Vec<Rc<FunctionDefinition>>>,
    pub instance_variables: RefCell<HashMap<u64, Variable, NoopHasher>>,
    pub static_variables: RefCell<HashMap<u64, Variable, NoopHasher>>,
    // state
    #[serde(skip)]
    pub state: Option<ClassFileState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassFileState {
    pub(crate) current_declared_function_index: RefCell<usize>,
    pub(crate) called_functions: RefCell<Vec<Rc<(String, CompilationDetail)>>>,
}

impl Default for ClassFileState {
    fn default() -> Self {
        Self {
            current_declared_function_index: RefCell::new(0),
            called_functions: RefCell::new(vec![]),
        }
    }
}

impl ClassFile {
    pub fn new(name: String, file_name: String, line: usize) -> Self {
        Self {
            name,
            reference: 0,
            defined_in_file_name: file_name,
            defined_at_line: line,
            functions: RefCell::new(vec![]),
            instance_variables: RefCell::new(Default::default()),
            static_variables: RefCell::new(Default::default()),
            state: Some(Default::default()),
        }
    }
    pub fn new_with_main_function(name: String, file_name: String, line: usize) -> Self {
        Self {
            name,
            reference: 0,
            defined_in_file_name: file_name,
            defined_at_line: line,
            functions: RefCell::new(vec![Rc::new(FunctionDefinition::new(MAIN_FUNCTION.to_string()))]),
            instance_variables: RefCell::new(Default::default()),
            static_variables: RefCell::new(Default::default()),
            state: Some(Default::default()),
        }
    }
    pub fn add_function(&self, function: FunctionDefinition) -> usize {
        self.functions.borrow_mut().push(Rc::new(function));
        *self.state.as_ref().unwrap().current_declared_function_index.borrow_mut() = self.functions.borrow().len() - 1;
        self.functions.borrow().len()
    }

    pub fn add_instance_variable(&self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.instance_variables.borrow_mut().insert(hash, variable);
        hash
    }

    pub fn add_static_variable(&self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.static_variables.borrow_mut().insert(hash, variable);
        hash
    }
    pub fn set_current_declared_function_index(&self, index: usize) {
        *self.state.as_ref().unwrap().current_declared_function_index.borrow_mut() = index;
    }
    pub fn add_called_function(&self, called_function: (String, CompilationDetail)) {
        self.state.as_ref().unwrap().called_functions.borrow_mut().push(Rc::new(called_function));
    }
    pub fn functions(&self) -> Vec<Rc<FunctionDefinition>> {
        self.functions.borrow().iter().cloned().collect::<Vec<Rc<FunctionDefinition>>>()
    }
    pub fn called_functions(&self) -> Vec<Rc<(String, CompilationDetail)>> {
        self.state.as_ref().unwrap().called_functions.borrow().iter().cloned().collect::<Vec<Rc<(String, CompilationDetail)>>>()
    }
    pub fn current_declared_function(&self) -> Rc<FunctionDefinition> {
        let i = self.state.as_ref().unwrap().current_declared_function_index();
        self.functions.borrow().get(i).unwrap().clone()
    }

    pub fn is_inside_a_main_function(&self) -> bool {
        self.state.as_ref().unwrap().current_declared_function_index() == 0
    }

    pub fn get_label(&self, label_name: &String) -> Option<Rc<Label>> {
        let functions = self.functions.borrow();
        let main_function: &Rc<FunctionDefinition> = functions.get(0).as_ref().unwrap();
        main_function.get_label(label_name)
    }

    pub fn get_function_returned_type(&self, function_name: &String) -> Option<ValueType> {
        let functions = self.functions.borrow();
        if let Some(function) = functions.iter().find(|f| &f.name == function_name) {
            function.returned_type.borrow().clone()
        } else {
            None
        }
    }

    pub fn insert_callsub(&self, label: String) {
        let functions = self.functions.borrow();
        let main_function = functions.get(0).unwrap();
        main_function.insert_callsub(label);
    }

    pub fn load_variable(&self, variable: &Variable, scope: Scope) -> Result<u64, String> {
        let cell = RefCell::new(Default::default());
        let variables = match scope {
            Scope::Npc => self.static_variables.borrow(),
            Scope::Instance => self.instance_variables.borrow(),
            _ => cell.borrow()
        };
        let maybe_found = variables.iter().find(|(_reference, local)| *local == variable);
        if let Some((reference, _)) = maybe_found.as_ref() {
            Ok(**reference)
        } else {
            Err(String::from("Undefined variable"))
        }
    }

    pub fn set_reference(&mut self) {
        self.reference = Vm::calculate_hash(self);
    }
}

impl Hash for ClassFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.functions.borrow().len().hash(state);
        if self.functions.borrow().len() > 0 {
            self.functions.borrow()[0].hash(state);
        }
        self.static_variables.borrow().len().hash(state);
    }
}

impl ClassFileState {
    pub fn current_declared_function_index(&self) -> usize {
        *self.current_declared_function_index.borrow()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub chunk: Rc<Chunk>,
    #[serde(skip)]
    pub(crate) state: Option<FunctionDefinitionState>,
    pub(crate) returned_type: RefCell<Option<ValueType>>,
}

impl FunctionDefinition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            chunk: Default::default(),
            state: Some(Default::default()),
            returned_type: RefCell::new(None),
        }
    }
    pub fn new_with_chunk(name: String, chunk: Chunk) -> Self {
        Self {
            name,
            chunk: Rc::new(chunk),
            state: Some(Default::default()),
            returned_type: RefCell::new(None),
        }
    }
    pub fn declared_labels(&self) -> Vec<Rc<Label>> {
        self.state.as_ref().unwrap().declared_labels.borrow().iter().map(|(_, label)| label.clone()).collect::<Vec<Rc<Label>>>()
    }

    pub fn insert_label(&self, label: Label) {
        self.state.as_ref().unwrap().declared_labels.borrow_mut().insert(label.name.clone(), Rc::new(label));
    }

    pub fn insert_callsub(&self, label: String) {
        self.state.as_ref().unwrap().callsub.borrow_mut().insert(label);
    }

    pub fn callsub_contains(&self, label: &String) -> bool {
        self.state.as_ref().unwrap().callsub.borrow().contains(label)
    }

    pub fn get_label(&self, label_name: &String) -> Option<Rc<Label>> {
        let declared_labels = self.state.as_ref().unwrap().declared_labels.borrow();
        declared_labels.get(label_name).cloned()
    }

    pub fn set_returned_type(&self, returned_type: Option<ValueType>) {
        *self.returned_type.borrow_mut() = returned_type;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FunctionDefinitionState {
    declared_labels: RefCell<HashMap<String, Rc<Label>>>,
    callsub: RefCell<HashSet<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Label {
    pub(crate) name: String,
    pub(crate) first_op_code_index: usize,
    pub(crate) last_op_code_index: usize,
}

impl PartialEq for FunctionDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for FunctionDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

impl Display for FunctionDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}()", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub op_codes: RefCell<Vec<OpCode>>,
    #[serde(skip)]
    pub compilation_details: RefCell<Vec<CompilationDetail>>,
    pub locals: RefCell<HashMap<u64, Variable, NoopHasher>>,
    pub constants_storage: RefCell<HashMap<u64, Constant, NoopHasher>>,
    // state
    #[serde(skip)]
    label_gotos_op_code_indices: RefCell<HashMap<String, Vec<(usize, CompilationDetail)>>>,
    // key are label name, values are goto op code that goto this label
    #[serde(skip)]
    current_block_state: RefCell<usize>,
    #[serde(skip)]
    block_states: RefCell<Vec<BlockState>>,
    #[serde(skip)]
    chunk_state: RefCell<ChunkState>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            op_codes: RefCell::new(vec![]),
            compilation_details: RefCell::new(vec![]),
            locals: RefCell::new(HashMap::with_hasher(NoopHasher::default())),
            constants_storage: RefCell::new(HashMap::with_hasher(NoopHasher::default())),
            label_gotos_op_code_indices: RefCell::new(Default::default()),
            current_block_state: RefCell::new(0),
            block_states: RefCell::new(vec![]),
            chunk_state: RefCell::new(Default::default()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockState {
    // Switch/Loop: store all "break" op_code indices, in order to update jump index to complete switch/for/while/do-while statements
    pub break_op_code_indices: RefCell<Vec<usize>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChunkState {
    pub local_setd: HashSet<u64>,
    pub dynamically_defined_variable: HashSet<u64>,
    pub undefined_variables: HashSet<u64>,
}

impl Default for BlockState {
    fn default() -> Self {
        Self {
            break_op_code_indices: RefCell::new(vec![]),
        }
    }
}

impl Chunk {
    pub fn last_op_code_index(&self) -> usize {
        if self.op_codes.borrow().len() == 0 {
            return 0;
        }
        self.op_codes.borrow().len() - 1
    }

    pub fn set_op_code_at(&self, index: usize, op_code: OpCode) {
        self.op_codes.borrow_mut()[index] = op_code;
    }

    pub fn get_op_code_at(&self, index: usize) -> OpCode {
        self.op_codes.borrow()[index].clone()
    }
    pub fn clone_op_code_at(&self, index: usize) -> (OpCode, CompilationDetail) {
        (self.op_codes.borrow()[index].clone(), self.compilation_details.borrow()[index].clone())
    }
    pub fn insert_op_code_at(&self, index: usize, op_code: OpCode, compilation_details: CompilationDetail) {
        self.op_codes.borrow_mut().insert(index, op_code);
        self.compilation_details.borrow_mut().insert(index, compilation_details);
    }

    pub fn emit_op_code(&self, op_code: OpCode, compilation_details: CompilationDetail) -> usize {
        self.op_codes.borrow_mut().push(op_code);
        self.compilation_details.borrow_mut().push(compilation_details);
        self.last_op_code_index()
    }

    pub fn add_constant(&self, constant: Constant) -> u64 {
        let hash = Vm::calculate_hash(&constant);
        self.constants_storage.borrow_mut().insert(hash, constant);
        hash
    }

    pub fn add_local(&self, variable: Variable) -> u64 {
        let hash = Vm::calculate_hash(&variable);
        self.locals.borrow_mut().insert(hash, variable);
        hash
    }

    pub fn load_local(&self, variable: &Variable) -> Result<u64, String> {
        let locals = self.locals.borrow();
        let maybe_found = locals.iter().find(|(_reference, local)| *local == variable);
        if let Some((reference, _)) = maybe_found.as_ref() {
            Ok(**reference)
        } else {
            Err(String::from("Undefined variable"))
        }
    }

    pub fn push_goto_index(&self, label: String, index: usize, compilation_detail: CompilationDetail) {
        let mut ref_mut = self.label_gotos_op_code_indices.borrow_mut();
        let gotos_op_code_indices = ref_mut.entry(label).or_insert(vec![]);
        gotos_op_code_indices.push((index, compilation_detail));
    }
    pub fn drop_goto_indices(&self) -> HashMap<String, Vec<(usize, CompilationDetail)>> {
        mem::take(&mut self.label_gotos_op_code_indices.borrow_mut())
    }

    pub fn add_new_block_state(&self) -> usize {
        *self.current_block_state.borrow_mut() = self.block_states.borrow().len();
        self.block_states.borrow_mut().push(Default::default());
        *self.current_block_state.borrow()
    }

    pub fn drop_block_state(&self) -> BlockState {
        let state = self.block_states.borrow_mut().pop().unwrap();
        state
    }

    pub fn push_block_break_index(&self, index: usize) {
        let block_state_ref_mut = self.block_states.borrow_mut();
        let block_state = block_state_ref_mut.last().unwrap();
        block_state.break_op_code_indices.borrow_mut().push(index);
    }

    pub fn local_setd_len(&self) -> usize {
        self.chunk_state.borrow().local_setd.len()
    }
    pub fn add_local_setd(&self, reference: u64) {
        self.chunk_state.borrow_mut().local_setd.insert(reference);
    }
    pub fn add_dynamically_defined_variable(&self, reference: u64) {
        self.chunk_state.borrow_mut().dynamically_defined_variable.insert(reference);
    }
    pub fn dynamically_defined_variable_contains(&self, reference: u64) -> bool {
        self.chunk_state.borrow().dynamically_defined_variable.contains(&reference)
    }

    pub fn add_undefined_variable(&self, reference: u64) {
        self.chunk_state.borrow_mut().undefined_variables.insert(reference);
    }

    pub fn undefined_variables_len(&self) -> usize {
        self.chunk_state.borrow().undefined_variables.len()
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OpCode {
    LoadConstant(u64),
    LoadValue, // Read reference from stack, push its constant reference to the stack
    StoreGlobal,
    LoadGlobal,
    // To handle edge case for function using variable reference. Some function takes variable (not variable name) which should be evaluated as variable ref
    AssignVariable(u64),
    StoreLocal(u64),
    LoadLocal(u64),
    StoreInstance(u64),
    LoadInstance(u64),
    StoreStatic(u64),
    LoadStatic(u64),
    DefineFunction(u64),
    ArrayStore,
    ArrayLoad,
    GlobalArrayStore,
    GlobalArrayLoad,
    CallNative { reference: u64, argument_count: usize },
    CallFunction { reference: u64, argument_count: usize },
    Equal,
    SwitchCompare,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    BitAnd,
    BitOr,
    BitNot,
    Relational(Relational),
    Add,
    NumericOperation(NumericOperation),
    // OpCode index to jump to. Use it to jump to any opcode
    Jump(usize),
    // OpCode index to jump to. Use it to jump to a given label defined in main function.
    Goto(usize),
    // OpCode index to jump to. Using goto instead of jump allow to break function
    Call,
    Return(bool),
    If(usize),
    // OpCode index to jump to when condition is evaluated to false.
    Else,
    End,
    Command,
    Noop, // Use by compiler in case it need to remove already generated op code. WE can't remove them as we may break jump index, so we replace with noop
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Relational {
    GT,
    GTE,
    LT,
    LTE,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum NumericOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    RightShift,
    LeftShift,
}