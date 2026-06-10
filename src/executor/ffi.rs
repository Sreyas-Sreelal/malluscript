use crate::executor::datatype::DataTypes;
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;

pub use mallubind::ffi::*;

#[no_mangle]
pub extern "C" fn ms_allocate_string(s: *const c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(s);
        let s_dup = CString::new(c_str.to_bytes()).unwrap();
        s_dup.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn ms_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn ms_allocate_list(length: usize) -> *mut MsList {
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        vec.push(ptr::null_mut());
    }
    vec.shrink_to_fit();
    let mut vec = std::mem::ManuallyDrop::new(vec);
    let list = Box::new(MsList {
        length,
        items: vec.as_mut_ptr(),
    });
    Box::into_raw(list)
}

#[no_mangle]
pub extern "C" fn ms_free_list(ptr: *mut MsList) {
    if !ptr.is_null() {
        unsafe {
            let list = Box::from_raw(ptr);
            let items = Vec::from_raw_parts(list.items, list.length, list.length);
            for item in items {
                ms_free_value(item);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn ms_allocate_value(v_type: MsType) -> *mut MsValue {
    let val = Box::new(MsValue {
        value_type: v_type,
        data: ptr::null_mut(),
    });
    Box::into_raw(val)
}

#[no_mangle]
pub extern "C" fn ms_free_value(ptr: *mut MsValue) {
    if !ptr.is_null() {
        unsafe {
            let val = Box::from_raw(ptr);
            match val.value_type {
                MsType::String => ms_free_string(val.data as *mut c_char),
                MsType::Integer => drop(Box::from_raw(val.data as *mut i64)),
                MsType::Float => drop(Box::from_raw(val.data as *mut f64)),
                MsType::Bool => drop(Box::from_raw(val.data as *mut bool)),
                MsType::List => ms_free_list(val.data as *mut MsList),
                _ => {}
            }
        }
    }
}

pub fn datatype_to_ms_value(dt: &DataTypes) -> *mut MsValue {
    match dt {
        DataTypes::Integer(i) => {
            let v = ms_allocate_value(MsType::Integer);
            unsafe {
                (*v).data = Box::into_raw(Box::new(*i)) as *mut c_void;
            }
            v
        }
        DataTypes::Float(f) => {
            let v = ms_allocate_value(MsType::Float);
            unsafe {
                (*v).data = Box::into_raw(Box::new(*f)) as *mut c_void;
            }
            v
        }
        DataTypes::Bool(b) => {
            let v = ms_allocate_value(MsType::Bool);
            unsafe {
                (*v).data = Box::into_raw(Box::new(*b)) as *mut c_void;
            }
            v
        }
        DataTypes::String(s) => {
            let v = ms_allocate_value(MsType::String);
            let c_str = CString::new(s.clone()).unwrap();
            unsafe {
                (*v).data = c_str.into_raw() as *mut c_void;
            }
            v
        }
        DataTypes::List(items) => {
            let v = ms_allocate_value(MsType::List);
            let list_ptr = ms_allocate_list(items.len());
            unsafe {
                let items_ptr = (*list_ptr).items;
                for (i, item) in items.iter().enumerate() {
                    let item_v = datatype_to_ms_value(item);
                    *items_ptr.add(i) = item_v;
                }
                (*v).data = list_ptr as *mut c_void;
            }
            v
        }
        _ => ms_allocate_value(MsType::Unknown),
    }
}

pub fn ms_value_to_datatype(val: *const MsValue) -> Result<DataTypes, String> {
    if val.is_null() {
        return Err("Null pointer returned".into());
    }
    unsafe {
        let v = &*val;
        match v.value_type {
            MsType::Integer => {
                if !v.data.is_null() {
                    Ok(DataTypes::Integer(*(v.data as *const i64)))
                } else {
                    Err("Null data for Integer".into())
                }
            }
            MsType::Float => {
                if !v.data.is_null() {
                    Ok(DataTypes::Float(*(v.data as *const f64)))
                } else {
                    Err("Null data for Float".into())
                }
            }
            MsType::Bool => {
                if !v.data.is_null() {
                    Ok(DataTypes::Bool(*(v.data as *const bool)))
                } else {
                    Err("Null data for Bool".into())
                }
            }
            MsType::String => {
                if !v.data.is_null() {
                    let s = CStr::from_ptr(v.data as *const c_char)
                        .to_string_lossy()
                        .into_owned();
                    Ok(DataTypes::String(s))
                } else {
                    Err("Null data for String".into())
                }
            }
            MsType::List => {
                if !v.data.is_null() {
                    let list = &*(v.data as *const MsList);
                    let mut items = Vec::with_capacity(list.length);
                    for i in 0..list.length {
                        let item_ptr = *list.items.add(i);
                        let dt = ms_value_to_datatype(item_ptr)?;
                        items.push(dt);
                    }
                    Ok(DataTypes::List(items))
                } else {
                    Err("Null data for List".into())
                }
            }
            MsType::Unknown => Ok(DataTypes::Unknown),
            MsType::Error => Err("Plugin returned error".into()),
            _ => Err("Unsupported return type".into()),
        }
    }
}

#[no_mangle]
pub extern "C" fn ms_call_function(
    executor: *mut c_void,
    name: *const c_char,
    args: *const *const MsValue,
    argc: usize,
    error: *mut *mut MsError,
) -> *mut MsValue {
    unsafe {
        let exec = &mut *(executor as *mut crate::executor::Executor);
        let fn_name = CStr::from_ptr(name).to_string_lossy().into_owned();

        let mut rust_args = Vec::new();
        if argc > 0 && !args.is_null() {
            let slice = std::slice::from_raw_parts(args, argc);
            for &arg_ptr in slice {
                match ms_value_to_datatype(arg_ptr) {
                    Ok(dt) => rust_args.push(dt),
                    Err(e) => {
                        if !error.is_null() {
                            let msg = CString::new(e).unwrap();
                            let err_box = Box::new(MsError {
                                message: msg.into_raw(),
                            });
                            *error = Box::into_raw(err_box);
                        }
                        return ptr::null_mut();
                    }
                }
            }
        }

        match exec.call_function(&fn_name, rust_args) {
            Ok(dt) => datatype_to_ms_value(&dt),
            Err(e) => {
                if !error.is_null() {
                    let err_str = format!("{:?}", e);
                    let msg = CString::new(err_str).unwrap();
                    let err_box = Box::new(MsError {
                        message: msg.into_raw(),
                    });
                    *error = Box::into_raw(err_box);
                }
                ptr::null_mut()
            }
        }
    }
}
