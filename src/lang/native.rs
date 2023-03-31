use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::mem;
use std::sync::Arc;

use crate::lang::array::Array;
use crate::lang::call_frame::CallFrame;
use crate::lang::class::{Class, Instance};
use crate::lang::error::RuntimeError;
use crate::lang::stack::StackEntry;
use crate::lang::stack::StackEntry::ConstantPoolReference;
use crate::lang::thread::Thread;
use crate::lang::value::{Native, Scope, Value, Variable};
use crate::lang::vm::{Hashcode, NativeMethodHandler, Vm};

pub(crate) fn handle_native_method(thread: &Thread, native: &Native, class: &Class, instance: &mut Option<Arc<Instance>>, call_frame: &mut CallFrame, arguments: Vec<Value>, arguments_ref: Vec<Option<u64>>, native_method_handler: &&dyn NativeMethodHandler) -> Result<(), RuntimeError> {
    match native.name.as_str() {
        "getargcount" => {
            thread.push_constant_on_stack(Value::new_number(call_frame.arguments_count as i32))
        }
        "getarg" => {
            getarg(thread, call_frame, &arguments)?
        }
        "getarraysize" => {
            getarraysize(thread, &arguments)?
        }
        "implode" => {
            implode(thread, &arguments)?
        }
        "cleararray" => {
            cleararray(thread, call_frame, native_method_handler, &arguments, &class)?
        }
        "setarray" => {
            setarray(thread, call_frame, native_method_handler, &arguments, &arguments_ref, &class)?
        }
        "getelementofarray" => {
            getelementofarray(thread, &arguments)?
        }
        "deletearray" => {
            deletearray(thread, call_frame, native_method_handler, &arguments, &class)?
        }
        "inarray" => {
            inarray(thread, &arguments, arguments_ref)?
        }
        "copyarray" => {
            copyarray(thread, call_frame, native_method_handler, arguments, &class)?
        }
        "setd" => {
            setd(thread, class, instance, call_frame, &arguments, arguments_ref, native_method_handler)?
        }
        "getd" => {
            getd(thread, class, instance, call_frame, arguments, native_method_handler)?
        }
        "getvariableofnpc" => {
            getvariableofnpc(thread, instance, call_frame, arguments)?
        },
        // stdlib
        "pow" => {
            let value = arguments[0].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "Pow first argument should be a number"))?;
            let exponent = arguments[1].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "Pow second argument should be a number"))?;
            let res = value.pow(exponent as u32);
            thread.push_constant_on_stack(Value::new_number(res));
        }
        "rand" => {
            let min = arguments[0].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            let max = arguments[1].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            let res = (RandomState::new().build_hasher().finish() % max as u64) + min as u64; // this random function is crap, should do the job for random in npc
            thread.push_constant_on_stack(Value::new_number(res as i32));
        }
        "min" => {
            let arg1 = arguments[0].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            let arg2 = arguments[1].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            thread.push_constant_on_stack(Value::new_number(arg1.min(arg2)));
        }
        "max" => {
            let arg1 = arguments[0].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            let arg2 = arguments[1].number_value().map_err(|err|
                thread.new_runtime_from_temporary(err, "rand first argument should be a number"))?;
            thread.push_constant_on_stack(Value::new_number(arg1.max(arg2)));
        }
        _ => {
            return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), format!("Native function {} is not handled yet!", native.name)));
        }
    }
    Ok(())
}

fn getvariableofnpc(thread: &Thread, instance: &Option<Arc<Instance>>, call_frame: &mut CallFrame, arguments: Vec<Value>) -> Result<(), RuntimeError> {
    let variable_identifier = arguments[0].string_value().map_err(|err| thread.new_runtime_from_temporary(err, "getvariableofnpc first argument should be a variable"))?;
    let variable_from_string = Variable::from_string(variable_identifier);
    let variable_reference = Vm::calculate_hash(&variable_from_string);
    let class_name = arguments[1].string_value().map_err(|err| thread.new_runtime_from_temporary(err, "getvariableofnpc second argument should be NPC name"))?;
    let class = thread.vm.get_class(class_name);
    let variable = class.get_variable(variable_reference)
        .ok_or_else(|| RuntimeError::new(thread.current_source_line.clone(), thread.stack_traces.clone(),
                                         format!("Variable {} is not declared in NPC {}", variable_identifier, class_name).as_str()))?;
    if variable_from_string.value_ref.borrow().is_array() {
        load_array_index_value(thread, class.as_ref(), instance, call_frame, variable_identifier, &variable_from_string, variable_reference)?;
    } else {
        let reference = variable.value_ref.borrow().reference().ok_or_else(|| RuntimeError::new(thread.current_source_line.clone(), thread.stack_traces.clone(),
                                                                                                format!("Variable {} in NPC {} is not initialized", variable_identifier, class_name).as_str()))?;
        thread.stack.push(ConstantPoolReference(reference));
    }
    Ok(())
}

fn getd(thread: &Thread, class: &Class, instance: &Option<Arc<Instance>>, call_frame: &mut CallFrame, arguments: Vec<Value>, native_method_handler: &&dyn NativeMethodHandler) -> Result<(), RuntimeError> {
    let variable_identifier = arguments[0].string_value().map_err(|err| thread.new_runtime_from_temporary(err, "getd first argument should be an expression producing a variable name"))?;
    let variable_from_string = Variable::from_string(variable_identifier);
    let variable_reference = Vm::calculate_hash(&variable_from_string);
    if mem::discriminant(&variable_from_string.scope) == mem::discriminant(&Scope::Instance) {
        let instance = instance.as_ref().unwrap();
        thread.stack.push(StackEntry::VariableReference((variable_from_string.scope.clone(), instance.hash_code(), variable_reference)));
    } else if mem::discriminant(&variable_from_string.scope) == mem::discriminant(&Scope::Npc) {
        thread.stack.push(StackEntry::VariableReference((variable_from_string.scope.clone(), class.hash_code(), variable_reference)));
    } else if mem::discriminant(&variable_from_string.scope) == mem::discriminant(&Scope::Local) {
        thread.stack.push(StackEntry::VariableReference((variable_from_string.scope.clone(), call_frame.hash_code(), variable_reference)));
    } else {
        thread.load_global(class, &mut None, call_frame, native_method_handler, &variable_from_string, variable_reference)?
    };
    if variable_from_string.value_ref.borrow().is_array() {
        // pop HeapReference, we don't need it on stack as we already have all reference to be able to load array.
        // When variable is an array, stack has been pushed with a HeapReference when calling load_xxxxx_variable functions
        thread.stack.pop().map_err(|err| thread.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
        load_array_index_value(thread, class, instance, call_frame, variable_identifier, &variable_from_string, variable_reference)?;
    }
    Ok(())
}

fn load_array_index_value(thread: &Thread, class: &Class, instance: &Option<Arc<Instance>>, call_frame: &mut CallFrame, variable_identifier: &str, variable_from_string: &Variable, variable_reference: u64) -> Result<(), RuntimeError> {
    let owner_reference = if mem::discriminant(&variable_from_string.scope) == mem::discriminant(&Scope::Instance) {
        instance.as_ref().unwrap().hash_code()
    } else if mem::discriminant(&variable_from_string.scope) == mem::discriminant(&Scope::Npc) {
        class.hash_code()
    } else {
        call_frame.hash_code()
    };
    let array = thread.vm.array_from_heap_reference(owner_reference, variable_reference).unwrap();
    let array_index = array_index_from_string(variable_identifier);

    thread.stack.push(ConstantPoolReference(array.get(array_index)
        .map_err(|err| RuntimeError::from_temporary(thread.current_source_line.clone(), thread.stack_traces.clone(), err))?.unwrap()));

    Ok(())
}

fn setd(thread: &Thread, class: &Class, instance: &mut Option<Arc<Instance>>, call_frame: &mut CallFrame, arguments: &[Value], arguments_ref: Vec<Option<u64>>, native_method_handler: &&dyn NativeMethodHandler) -> Result<(), RuntimeError> {
    let variable_identifier = arguments[0].string_value().map_err(|err| thread.new_runtime_from_temporary(err, "setd first argument should be an expression producing a variable name"))?;
    let variable_value = arguments_ref[1];
    let variable = Variable::from_string(variable_identifier);
    let variable_reference = Vm::calculate_hash(&variable);
    let owner_reference = if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Instance) {
        instance.as_ref().unwrap().hash_code()
    } else if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Npc) {
        class.hash_code()
    } else {
        call_frame.hash_code()
    };
    // to simulate the behavior when we assign via "set" and "=", where right expression of assigment is a constant reference, just before assigning the variable.
    thread.stack.push(ConstantPoolReference(variable_value.unwrap()));
    thread.variable_assign_reference(class, instance, call_frame, native_method_handler, variable.clone(), owner_reference)?;
    // When it is an array, we simulate ArrayStore OpCode. ArrayStore OpCode contains index for assignment, here we need to retrieve from first argument of "setd"
    if variable.value_ref.borrow().is_array() {
        let array = thread.vm.array_from_heap_reference(owner_reference, variable_reference);
        let array_index = array_index_from_string(variable_identifier);
        let array = array.unwrap();
        array.assign(array_index, variable_value.unwrap(), thread.array_update_callback(call_frame, native_method_handler, array.clone(), &class));
    }
    Ok(())
}

fn array_index_from_string(variable_identifier: &str) -> usize {
    let opening_bracket_index = variable_identifier.chars().position(|c| c == '[').unwrap();
    let closing_bracket_index = variable_identifier.chars().position(|c| c == ']').unwrap();

    variable_identifier[opening_bracket_index + 1..closing_bracket_index].parse::<usize>().unwrap()
}

fn copyarray(thread: &Thread, call_frame: &mut CallFrame, native_method_handler: &&dyn NativeMethodHandler, arguments: Vec<Value>, class: &Class) -> Result<(), RuntimeError> {
    let (destination_owner_reference, destination_reference, _, destination_index) = arguments[0].array_entry_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "copyarray first argument should be an array element"))?;
    let (source_array_owner_reference, source_array_reference, _, source_array_index) = arguments[1].array_entry_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "copyarray second argument should be an array element"))?;
    let count = arguments[2].number_value().map_err(|err| thread.new_runtime_from_temporary(err, "copyarray third argument should be  theamount of data to copy"))?;
    let destination_array = thread.vm.array_from_heap_reference(*destination_owner_reference, *destination_reference).unwrap();
    let source_array = thread.vm.array_from_heap_reference(*source_array_owner_reference, *source_array_reference).unwrap();
    if destination_array.value_type != source_array.value_type {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(),
                                            thread.stack_traces.clone(),
                                            format!("copyarray - tried to assign an array of {} (second argument) to an array of {}",
                                                    source_array.value_type.display_type(), destination_array.value_type.display_type())));
    }
    destination_array.copyarray(source_array, *destination_index, *source_array_index, count as usize, thread.array_update_callback(call_frame, native_method_handler, destination_array.clone(), &class))
        .map_err(|err| RuntimeError::from_temporary(thread.current_source_line.clone(), thread.stack_traces.clone(), err))
}

fn inarray(thread: &Thread, arguments: &[Value], arguments_ref: Vec<Option<u64>>) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = if arguments[0].is_reference() {
        arguments[0].reference_value().map_err(|err|
            thread.new_runtime_from_temporary(err, "inarray first argument should be array name"))?
    } else {
        let (owner_reference, reference, _, _) = arguments[0].array_entry_value().map_err(|err|
            thread.new_runtime_from_temporary(err, "inarray first argument should be array name"))?;
        (*owner_reference, *reference)
    };
    let reference_to_find = arguments_ref[1].unwrap();
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let index = array.index_of(reference_to_find);
    let index_constant_ref = thread.vm.add_in_constant_pool(Value::Number(Some(index as i32)));
    thread.stack.push(StackEntry::ConstantPoolReference(index_constant_ref));
    Ok(())
}

fn deletearray(thread: &Thread, call_frame: &mut CallFrame, native_method_handler: &&dyn NativeMethodHandler, arguments: &[Value], class: &Class) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "deletearray first argument should be array name"))?;
    let size = arguments[1].number_value().map_err(|err| thread.new_runtime_from_temporary(err, "deletearray second argument should be number of element to delete"))? as usize;
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    array.remove(*index, size, thread.array_remove_items_callback(call_frame, native_method_handler, array.clone(), &class));
    Ok(())
}

fn getelementofarray(thread: &Thread, arguments: &[Value]) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = arguments[0].reference_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "getelementofarray first argument should be array name"))?;
    let index = arguments[1].number_value().map_err(|err| thread.new_runtime_from_temporary(err, "getelementofarray second argument should be array index"))? as usize;
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let reference = array.get(index).map_err(|err| RuntimeError::from_temporary(thread.current_source_line.clone(), thread.stack_traces.clone(), err))?;
    thread.stack.push(StackEntry::ConstantPoolReference(reference.unwrap()));
    Ok(())
}

fn setarray(thread: &Thread, call_frame: &mut CallFrame, native_method_handler: &&dyn NativeMethodHandler, arguments: &[Value], arguments_ref: &[Option<u64>], class: &Class) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "setarray first argument should be array name"))?;
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    // first parameters of setarray is already assigned to index, and thus is not part of arguments.
    // so we assign arguments starting at index + 1;
    let mut index = index + 1; // setarray .@a[0], assignment, arguments.
    for (i, array_reference) in arguments_ref.iter().enumerate() { // arguments are in reverse order
        if i == 0 {
            continue;
        }
        if array_reference.is_some() {
            if !array.value_type.match_value(&arguments[i]) {
                return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(),
                                                    format!("setarray - tried to assign {} ({}th arguments) to an array of {}",
                                                            arguments[i].display_type(), i + 2, array.value_type.display_type())));
            }
            array.assign::<Box<dyn Fn(Array)>>(index, array_reference.unwrap(), None);
            index += 1;
        }
    }
    let maybe_callback = thread.array_update_callback(call_frame, native_method_handler, array.clone(), &class);
    if let Some(callback) = maybe_callback {
        callback(array.as_ref().clone());
    }
    Ok(())
}

fn cleararray(thread: &Thread, call_frame: &mut CallFrame, native_method_handler: &&dyn NativeMethodHandler, arguments: &[Value], class: &Class) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "cleararray first argument should be array name"))?;
    let value = arguments[1].clone();
    let size = arguments[2].number_value().map_err(|err| thread.new_runtime_from_temporary(err, "cleararray third argument should be number of values to set"))?;
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    if !array.value_type.match_value(&value) {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(),
                                            thread.stack_traces.clone(),
                                            format!("cleararray - tried to assign {} (second argument) to an array of {}",
                                                    value.display_type(), array.value_type.display_type())));
    }
    let value_reference = thread.vm.add_in_constant_pool(value);
    array.assign_multiple(*index, size as usize, value_reference, thread.array_update_callback(call_frame, native_method_handler, array.clone(), &class));
    Ok(())
}

fn getarraysize(thread: &Thread, arguments: &[Value]) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = arguments[0].reference_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "getarraysize first argument should be array name"))?;
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let len = array.len();
    let reference = thread.vm.add_in_constant_pool(Value::Number(Some(len as i32)));
    thread.stack.push(StackEntry::ConstantPoolReference(reference));
    Ok(())
}

fn implode(thread: &Thread, arguments: &[Value]) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = arguments[0].reference_value().map_err(|err|
        thread.new_runtime_from_temporary(err, "implode first argument should be array name"))?;
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let array_values_guard = array.values.read().unwrap();
    let joined_values = array_values_guard.iter().enumerate()
        .map(|(index, _)| array_values_guard.get(index).unwrap()).filter(|v| v.is_some())
        .map(|constant_ref| thread.vm.get_from_constant_pool(constant_ref.unwrap()).unwrap().value().string_value().unwrap().clone())
        .collect::<Vec<String>>()
        .join(arguments[1].string_value().map_err(|_| thread.new_runtime_error(String::from("implode second argument should be a string")))?);
    let reference = thread.vm.add_in_constant_pool(Value::new_string(joined_values));
    thread.stack.push(StackEntry::ConstantPoolReference(reference));
    Ok(())
}

fn getarg(thread: &Thread, call_frame: &CallFrame, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let index = arguments[0].number_value().map_err(|err| thread.new_runtime_from_temporary(err, "getarg first argument should be number of argument"))? as usize;
    if arguments.len() == 1 && index > (call_frame.arguments_count - 1) {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), format!("Can't call getarg({}) which is greater than number of arguments provided: {}. Maximum allow index is {}. Consider calling getarg with a default value: getarg({}, DEFAULT_VALUE)", index, call_frame.arguments_count, call_frame.arguments_count - 1, index)));
    } else if arguments.len() == 2 && index > (call_frame.arguments_count - 1) {
        let value = arguments[1].clone();
        let reference = thread.vm.add_in_constant_pool(value);
        thread.stack.push(StackEntry::ConstantPoolReference(reference));
    } else if arguments.len() > 2 {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), String::from("Can't call getarg with more than 2 arguments")));
    } else {
        let stack_entry = thread.stack.peek(call_frame.stack_pointer + index).map_err(|err| thread.new_runtime_from_temporary(err.clone(), err.message.as_str()))?;
        thread.stack.push(stack_entry);
    }
    Ok(())
}