use mallubind::{InterpreterState, Value, malluscript_native};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVERS: Mutex<HashMap<i64, TcpListener>> = Mutex::new(HashMap::new());
    static ref CLIENTS: Mutex<HashMap<i64, TcpStream>> = Mutex::new(HashMap::new());
    static ref SERVER_ID_COUNTER: Mutex<i64> = Mutex::new(0);
    static ref CLIENT_ID_COUNTER: Mutex<i64> = Mutex::new(0);
}

malluscript_native!(http_listen, reg, |args: &[Value]| {
    let port = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let address = format!("127.0.0.1:{}", port);
    
    match TcpListener::bind(&address) {
        Ok(listener) => {
            let mut counter = SERVER_ID_COUNTER.lock().unwrap();
            *counter += 1;
            let id = *counter;
            SERVERS.lock().unwrap().insert(id, listener);
            Ok(Value::Integer(id))
        }
        Err(_) => Ok(Value::Integer(-1)),
    }
});

malluscript_native!(http_accept, reg, |args: &[Value]| {
    let server_id = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let servers = SERVERS.lock().unwrap();
    
    if let Some(listener) = servers.get(&server_id) {
        if let Ok((stream, _)) = listener.accept() {
            let mut counter = CLIENT_ID_COUNTER.lock().unwrap();
            *counter += 1;
            let id = *counter;
            CLIENTS.lock().unwrap().insert(id, stream);
            
            if let Err(e) = reg.call("connected", &[Value::Integer(id)]) {
                eprintln!("[http_plugin] Note: Could not call 'connected' function: {}", e);
            }
            
            return Ok(Value::Integer(id));
        }
    }
    Ok(Value::Integer(-1))
});

malluscript_native!(http_read, reg, |args: &[Value]| {
    let client_id = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let mut clients = CLIENTS.lock().unwrap();
    
    if let Some(stream) = clients.get_mut(&client_id) {
        let mut buffer = [0; 1024];
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            let text = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            return Ok(Value::String(text));
        }
    }
    Ok(Value::String("".to_string()))
});

malluscript_native!(http_write, reg, |args: &[Value]| {
    let client_id = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let data = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
    
    let mut clients = CLIENTS.lock().unwrap();
    if let Some(stream) = clients.get_mut(&client_id) {
        let success = if stream.write_all(data.as_bytes()).is_ok() { 1 } else { 0 };
        return Ok(Value::Integer(success));
    }
    Ok(Value::Integer(0))
});

malluscript_native!(http_close, reg, |args: &[Value]| {
    let client_id = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let mut clients = CLIENTS.lock().unwrap();
    
    if let Some(mut stream) = clients.remove(&client_id) {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Ok(Value::Integer(1));
    }
    Ok(Value::Integer(0))
});

#[no_mangle]
pub extern "C" fn register_mallubind(registry: *const mallubind::ffi::MsInterpreterState) {
    let reg = InterpreterState::new(registry);
    reg.register("http_listen", http_listen);
    reg.register("http_accept", http_accept);
    reg.register("http_read", http_read);
    reg.register("http_write", http_write);
    reg.register("http_close", http_close);
}
