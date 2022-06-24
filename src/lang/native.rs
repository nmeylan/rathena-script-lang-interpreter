use std::env::var;
use std::mem;
use crate::lang::call_frame::CallFrame;
use crate::lang::class::{Class, Instance};
use crate::lang::error::RuntimeError;
use crate::lang::stack::StackEntry;
use crate::lang::stack::StackEntry::ConstantPoolReference;
use crate::lang::thread::Thread;
use crate::lang::value::{Native, Scope, Value, Variable};
use crate::lang::vm::{Hashcode, Vm};

pub(crate) fn handle_native_method(thread: &Thread, native: &Native, class: &Class, instance: &mut Option<&mut Instance>, call_frame: &mut CallFrame, arguments: Vec<Value>, arguments_ref: Vec<Option<u64>>) -> Result<(), RuntimeError> {
    match native.name.as_str() {
        "getarg" => {
            getarg(thread, call_frame, &arguments)?
        }
        "getarraysize" => {
            getarraysize(thread, &arguments)?
        }
        "cleararray" => {
            cleararray(thread, &arguments)?
        }
        "setarray" => {
            setarray(thread, &arguments, &arguments_ref)?
        }
        "getelementofarray" => {
            getelementofarray(thread, &arguments)?
        }
        "deletearray" => {
            deletearray(thread, &arguments)?
        }
        "inarray" => {
            inarray(thread, &arguments, arguments_ref)?
        }
        "copyarray" => {
            copyarray(thread, arguments)?
        }
        "setd" => {
            let variable_identifier = arguments[0].string_value();
            let variable_value = arguments_ref[1].clone();
            let variable = Variable::from_string(variable_identifier);
            if !variable.value_ref.borrow().value_type.match_value(&arguments[1]) {
                return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(),
                                                    format!("setd - tried to assign a {} to a variable declared as {}",
                                                            arguments[1].display_type(), variable.value_ref.borrow().value_type.display_type())));
            }
            let variable_reference = Vm::calculate_hash(&variable);
            let owner_reference = if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Instance) {
                instance.as_ref().unwrap().hash_code()
            } else if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Npc) {
                class.hash_code()
            } else {
                call_frame.hash_code()
            };
            thread.stack.push(ConstantPoolReference(variable_value.unwrap()));
            thread.set_variable(call_frame, class, instance, &variable, owner_reference)?;
            if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Instance) {
                // instance.unwrap().variables.insert(variable_reference, variable);
                let mut mutable_instance = instance.as_mut().unwrap();
                mutable_instance.variables.insert(variable_reference, variable);
            } else if mem::discriminant(&variable.scope) == mem::discriminant(&Scope::Npc) {
                class.insert_variable(variable_reference, variable);
            } else {
                call_frame.locals.insert(variable_reference, variable);
            };
        }
        _ => {
            return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), format!("Native function {} is not handled yet!", native.name)));
        }
    }
    Ok(())
}

fn copyarray(thread: &Thread, arguments: Vec<Value>) -> Result<(), RuntimeError> {
    let (destination_owner_reference, destination_reference, _, destination_index) = arguments[0].array_entry_value();
    let (source_array_owner_reference, source_array_reference, _, source_array_index) = arguments[1].array_entry_value();
    let count = arguments[2].number_value();
    let destination_array = thread.vm.array_from_heap_reference(*destination_owner_reference, *destination_reference).unwrap();
    let source_array = thread.vm.array_from_heap_reference(*source_array_owner_reference, *source_array_reference).unwrap();
    if destination_array.value_type != source_array.value_type {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(),
                                            thread.stack_traces.clone(),
                                            format!("copyarray - tried to assign an array of {} (second argument) to an array of {}",
                                                    source_array.value_type.display_type(), destination_array.value_type.display_type())));
    }
    destination_array.copyarray(source_array, *destination_index, *source_array_index, count as usize)
        .map_err(|err| RuntimeError::from_temporary(thread.current_source_line.clone(), thread.stack_traces.clone(), err))
}

fn inarray(thread: &Thread, arguments: &Vec<Value>, arguments_ref: Vec<Option<u64>>) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = if arguments[0].is_reference() {
        arguments[0].reference_value()
    } else {
        let (owner_reference, reference, _, _) = arguments[0].array_entry_value();
        (*owner_reference, *reference)
    };
    let reference_to_find = arguments_ref[1].unwrap();
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let index = array.index_of(reference_to_find);
    let index_constant_ref = thread.vm.add_in_constant_pool(Value::Number(Some(index as i32)));
    thread.stack.push(StackEntry::ConstantPoolReference(index_constant_ref));
    Ok(())
}

fn deletearray(thread: &Thread, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value();
    let size = arguments[1].number_value() as usize;
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    array.remove(*index, size);
    Ok(())
}

fn getelementofarray(thread: &Thread, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = arguments[0].reference_value();
    let index = arguments[1].number_value() as usize;
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let reference = array.get(index).map_err(|err| RuntimeError::from_temporary(thread.current_source_line.clone(), thread.stack_traces.clone(), err))?;
    thread.stack.push(StackEntry::ConstantPoolReference(reference.unwrap()));
    Ok(())
}

fn setarray(thread: &Thread, arguments: &Vec<Value>, arguments_ref: &Vec<Option<u64>>) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value();
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    // first parameters of setarray is already assigned to index, and thus is not part of arguments.
    // so we assign arguments starting at index + 1;
    let mut index = index + 1; // setarray .@a[0], assignment, arguments.
    for (i, array_reference) in arguments_ref.iter().enumerate() { // arguments are in reverse order
        if array_reference.is_some() {
            if !array.value_type.match_value(&arguments[i]) {
                return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(),
                                                    format!("setarray - tried to assign {} ({}th arguments) to an array of {}",
                                                            arguments[i].display_type(), i + 2, array.value_type.display_type())));
            }
            array.assign(index, array_reference.unwrap());
            index += 1;
        }
    }
    Ok(())
}

fn cleararray(thread: &Thread, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let (owner_reference, reference, _, index) = arguments[0].array_entry_value();
    let value = arguments[1].clone();
    let size = arguments[2].number_value();
    let array = thread.vm.array_from_heap_reference(*owner_reference, *reference).unwrap();
    if !array.value_type.match_value(&value) {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(),
                                            thread.stack_traces.clone(),
                                            format!("cleararray - tried to assign {} (second argument) to an array of {}",
                                                    value.display_type(), array.value_type.display_type())));
    }
    let value_reference = thread.vm.add_in_constant_pool(value);
    array.assign_multiple(*index, size as usize, value_reference);
    Ok(())
}

fn getarraysize(thread: &Thread, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let (owner_reference, reference) = arguments[0].reference_value();
    let array = thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
    let len = array.len();
    let reference = thread.vm.add_in_constant_pool(Value::Number(Some(len as i32)));
    thread.stack.push(StackEntry::ConstantPoolReference(reference));
    Ok(())
}

fn getarg(thread: &Thread, call_frame: &CallFrame, arguments: &Vec<Value>) -> Result<(), RuntimeError> {
    let index = arguments[0].number_value() as usize;
    if arguments.len() == 1 && index > (call_frame.arguments_count - 1) {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), format!("Can't call getarg({}) which is greater than number of arguments provided: {}. Maximum allow index is {}. Consider calling getarg with a default value: getarg({}, DEFAULT_VALUE)", index, call_frame.arguments_count, call_frame.arguments_count - 1, index)));
    } else if arguments.len() == 2 && index > (call_frame.arguments_count - 1) {
        let value = arguments[1].clone();
        let reference = thread.vm.add_in_constant_pool(value);
        thread.stack.push(StackEntry::ConstantPoolReference(reference));
    } else if arguments.len() > 2 {
        return Err(RuntimeError::new_string(thread.current_source_line.clone(), thread.stack_traces.clone(), String::from("Can't call getarg with more than 2 arguments")));
    } else {
        let stack_entry = thread.stack.peek(call_frame.stack_pointer + index)?;
        thread.stack.push(stack_entry);
    }
    Ok(())
}