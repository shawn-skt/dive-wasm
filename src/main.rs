use anyhow::Result;
use wasmtime::*;

// wasmtime test
fn main() -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let wat = r#"
        (module
            (func $add_one_f (result f32)
              f32.const 0.0
              f32.const 0.0
              f32.div)
            (export "add_one" (func $add_one_f))
          )"#;
    let module = Module::new(&engine, wat)?;

    let mut store = Store::new(&engine, 4);
    
    let instance = Instance::new(&mut store, &module, &[])?;
    let hello = instance.get_typed_func::<(), f32, _>(&mut store, "add_one")?;

    // And finally we can call the wasm!
    let ans = hello.call(&mut store, ())?;
    println!("arm wasmtime f32:{:?}", ans.to_be_bytes());

    Ok(())
}