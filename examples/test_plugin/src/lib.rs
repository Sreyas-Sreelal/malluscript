use mallubind::{InterpreterState, Value, malluscript_native};

malluscript_native!(add, reg, |args: &[Value]| {
    let a = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let b = args.get(1).and_then(|v| v.as_integer()).unwrap_or(0);
    Ok(Value::Integer(a + b))
});

malluscript_native!(concat, reg, |args: &[Value]| {
    let a = args.get(0).and_then(|v| v.as_string()).unwrap_or("");
    let b = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
    let mut res = String::new();
    res.push_str(a);
    res.push_str(b);
    Ok(Value::String(res))
});

malluscript_native!(reverse_list, reg, |args: &[Value]| {
    if let Some(list) = args.get(0).and_then(|v| v.as_list()) {
        let mut reversed = list.clone();
        reversed.reverse();
        Ok(Value::List(reversed))
    } else {
        Ok(Value::Unknown)
    }
});

malluscript_native!(invoke_malluscript, reg, |args: &[Value]| {
    if let Some(name) = args.get(0).and_then(|v| v.as_string()) {
        let call_args = if args.len() > 1 {
            &args[1..]
        } else {
            &[]
        };
        match reg.call(name, call_args) {
            Ok(val) => Ok(val),
            Err(e) => {
                eprintln!("invoke_malluscript error: {}", e);
                Err(format!("Failed to call {}: {}", name, e))
            }
        }
    } else {
        Err("Expected function name as first argument".into())
    }
});

#[no_mangle]
pub extern "C" fn register_mallubind(registry: *const mallubind::ffi::MsInterpreterState) {
    let reg = InterpreterState::new(registry);
    
    reg.register("add", add);
    reg.register("concat", concat);
    reg.register("reverse_list", reverse_list);
    reg.register("invoke_malluscript", invoke_malluscript);
}

