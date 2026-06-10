pub mod ffi;

use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr;

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Unknown,
}

impl Value {
    pub fn as_integer(&self) -> Option<i64> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Value::List(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub unsafe fn from_raw(ptr: *const ffi::MsValue) -> Self {
        if ptr.is_null() {
            return Value::Unknown;
        }
        unsafe {
            let val = &*ptr;
            match val.value_type {
                ffi::MsType::Integer => {
                    if !val.data.is_null() {
                        Value::Integer(*(val.data as *const i64))
                    } else {
                        Value::Unknown
                    }
                }
                ffi::MsType::String => {
                    if !val.data.is_null() {
                        let s = CStr::from_ptr(val.data as *const c_char)
                            .to_string_lossy()
                            .into_owned();
                        Value::String(s)
                    } else {
                        Value::Unknown
                    }
                }
                ffi::MsType::Bool => {
                    if !val.data.is_null() {
                        Value::Bool(*(val.data as *const bool))
                    } else {
                        Value::Unknown
                    }
                }
                ffi::MsType::List => {
                    if !val.data.is_null() {
                        let list = &*(val.data as *const ffi::MsList);
                        let mut items = Vec::with_capacity(list.length);
                        for i in 0..list.length {
                            let item_ptr = *list.items.add(i);
                            items.push(Value::from_raw(item_ptr));
                        }
                        Value::List(items)
                    } else {
                        Value::Unknown
                    }
                }
                _ => Value::Unknown,
            }
        }
    }

    pub unsafe fn into_raw(self, reg: &ffi::MsInterpreterState) -> *mut ffi::MsValue {
        unsafe {
            match self {
                Value::Integer(i) => {
                    let v = (reg.allocate_value)(ffi::MsType::Integer);
                    (*v).data = Box::into_raw(Box::new(i)) as *mut c_void;
                    v
                }
                Value::String(s) => {
                    let c_str = CString::new(s).unwrap_or_else(|_| CString::new("").unwrap());
                    let v = (reg.allocate_value)(ffi::MsType::String);
                    (*v).data = (reg.allocate_string)(c_str.as_ptr()) as *mut c_void;
                    v
                }
                Value::Bool(b) => {
                    let v = (reg.allocate_value)(ffi::MsType::Bool);
                    (*v).data = Box::into_raw(Box::new(b)) as *mut c_void;
                    v
                }
                Value::List(items) => {
                    let v = (reg.allocate_value)(ffi::MsType::List);
                    let list_ptr = (reg.allocate_list)(items.len());
                    let items_ptr = (*list_ptr).items;
                    for (i, item) in items.into_iter().enumerate() {
                        let item_v = item.into_raw(reg);
                        *items_ptr.add(i) = item_v;
                    }
                    (*v).data = list_ptr as *mut c_void;
                    v
                }
                Value::Unknown => (reg.allocate_value)(ffi::MsType::Unknown),
            }
        }
    }
}


#[macro_export]
macro_rules! malluscript_native {
    ($name:ident, $reg_ident:ident, $handler:expr) => {
        extern "C" fn $name(
            args: *const *const mallubind::ffi::MsValue,
            argc: usize,
            _error: *mut *mut mallubind::ffi::MsError,
        ) -> *mut mallubind::ffi::MsValue {
            unsafe {
                let $reg_ident = mallubind::InterpreterState::current_registry();

                let mut rust_args = Vec::with_capacity(argc);
                if argc > 0 && !args.is_null() {
                    let slice = std::slice::from_raw_parts(args, argc);
                    for &arg_ptr in slice {
                        rust_args.push(mallubind::Value::from_raw(arg_ptr));
                    }
                }

                let result: Result<mallubind::Value, String> = $handler(&rust_args);

                match result {
                    Ok(val) => val.into_raw($reg_ident.raw()),
                    Err(_) => ($reg_ident.raw().allocate_value)(mallubind::ffi::MsType::Unknown),
                }
            }
        }
    };
}

#[derive(Copy, Clone)]
pub struct InterpreterState {
    raw: *const ffi::MsInterpreterState,
}

static mut CURRENT_REGISTRY: *const ffi::MsInterpreterState = ptr::null();

impl InterpreterState {
    pub fn new(raw: *const ffi::MsInterpreterState) -> Self {
        unsafe {
            CURRENT_REGISTRY = raw;
        }
        InterpreterState { raw }
    }

    pub fn current_registry() -> Self {
        unsafe {
            InterpreterState {
                raw: CURRENT_REGISTRY,
            }
        }
    }

    pub fn raw(&self) -> &ffi::MsInterpreterState {
        unsafe { &*self.raw }
    }

    pub fn register(&self, name: &str, func: ffi::MsNative) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            ((*self.raw).add_function)((*self.raw).executor, c_name.as_ptr(), func);
        }
    }

    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        let c_name = CString::new(name).map_err(|e| e.to_string())?;
        unsafe {
            let mut c_args = Vec::with_capacity(args.len());
            let mut args_ptrs = Vec::with_capacity(args.len());
            for arg in args {
                let c_arg = arg.clone().into_raw(self.raw());
                c_args.push(c_arg);
                args_ptrs.push(c_arg as *const ffi::MsValue);
            }

            let mut error_ptr: *mut ffi::MsError = std::ptr::null_mut();

            let result_ptr = ((*self.raw).call_function)(
                (*self.raw).executor,
                c_name.as_ptr(),
                args_ptrs.as_ptr(),
                args_ptrs.len(),
                &mut error_ptr,
            );

            for c_arg in c_args {
                ((*self.raw).free_value)(c_arg);
            }

            if !error_ptr.is_null() {
                let msg = CStr::from_ptr((*error_ptr).message)
                    .to_string_lossy()
                    .into_owned();
                return Err(msg);
            }

            let result = Value::from_raw(result_ptr);
            ((*self.raw).free_value)(result_ptr);

            Ok(result)
        }
    }
}
