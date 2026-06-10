# mallubind

Native C-ABI bindings for writing plugins and libraries for the **Malluscript** programming language.

`mallubind` provides a safe, idiomatic Rust wrapper over the raw C-ABI exposed by the Malluscript Runtime. It handles automatic conversion of Malluscript's internal types (`Integer`, `Float`, `String`, `List`) across the FFI boundary, making it incredibly easy to extend Malluscript with high-performance native modules.

## Getting Started

To create a new Malluscript plugin:

1. Create a new Rust library crate:
   ```bash
   cargo new --lib my_malluscript_plugin
   ```

2. Update your `Cargo.toml` to build a dynamic C library and add `mallubind`:
   ```toml
   [package]
   name = "my_malluscript_plugin"
   version = "0.1.0"
   edition = "2024"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   mallubind = "0.1.0"
   ```

## Example Plugin

Here is a simple example of a plugin that exposes an `add` function to Malluscript.

```rust
use mallubind::{InterpreterState, Value, malluscript_native};

// Define your native functions using the `malluscript_native!` macro.
malluscript_native!(add, reg, |args: &[Value]| {
    let a = args.get(0).and_then(|v| v.as_integer()).unwrap_or(0);
    let b = args.get(1).and_then(|v| v.as_integer()).unwrap_or(0);
    
    // Return the result back to Malluscript!
    Ok(Value::Integer(a + b))
});

// The entry point required by Malluscript.
// This function MUST be named `register_mallubind` and use `#[no_mangle]`.
#[no_mangle]
pub extern "C" fn register_mallubind(registry: *const mallubind::ffi::MsInterpreterState) {
    let reg = InterpreterState::new(registry);
    
    // Register the native functions into the module's execution space
    reg.register("add", add);
}
```

### Calling your plugin from Malluscript
Once compiled into a `.dll` (Windows) or `.so` (Linux), you can use it in your Malluscript code:

```malluscript
my_malluscript_plugin my_plugin ennu ulppeduthuka;

result = my_plugin.add<10, 20>;
result ezhuthuka; // Outputs: 30
```

## Cross-Boundary Calling (Plugin -> Malluscript)

`mallubind` also supports executing functions defined in the parent Malluscript environment directly from your native Rust plugin!

```rust
// Inside a native function:
if let Err(e) = reg.call("my_script_function", &[Value::Integer(42)]) {
    eprintln!("Failed to call script function: {}", e);
}
```

