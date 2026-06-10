use std::ffi::{c_char, c_void};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MsType {
    String,
    Integer,
    Bool,
    Float,
    List,
    Ref,
    Unknown,
    Error,
}

#[repr(C)]
pub struct MsValue {
    pub value_type: MsType,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct MsList {
    pub length: usize,
    pub items: *mut *mut MsValue,
}

#[repr(C)]
pub struct MsError {
    pub message: *mut c_char,
}

pub type MsNative = extern "C" fn(
    args: *const *const MsValue,
    argc: usize,
    error: *mut *mut MsError,
) -> *mut MsValue;

pub type MsCallFunction = extern "C" fn(
    executor: *mut c_void,
    name: *const c_char,
    args: *const *const MsValue,
    argc: usize,
    error: *mut *mut MsError,
) -> *mut MsValue;

#[repr(C)]
pub struct MsInterpreterState {
    pub executor: *mut c_void,
    pub add_function: extern "C" fn(executor: *mut c_void, name: *const c_char, func: MsNative),
    pub allocate_value: extern "C" fn(v_type: MsType) -> *mut MsValue,
    pub free_value: extern "C" fn(ptr: *mut MsValue),
    pub allocate_string: extern "C" fn(s: *const c_char) -> *mut c_char,
    pub free_string: extern "C" fn(ptr: *mut c_char),
    pub allocate_list: extern "C" fn(length: usize) -> *mut MsList,
    pub free_list: extern "C" fn(ptr: *mut MsList),
    pub call_function: MsCallFunction,
}
