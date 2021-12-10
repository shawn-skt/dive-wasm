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

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    // let host_hello = Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
    //     println!("Got {} from WebAssembly", param);
    //     println!("my host state is: {}", caller.data());
    // });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = Instance::new(&mut store, &module, &[])?;
    let hello = instance.get_typed_func::<(), f32, _>(&mut store, "add_one")?;

    // And finally we can call the wasm!
    let ans = hello.call(&mut store, ())?;
    println!("arm wasmtime f32:{:?}", ans.to_be_bytes());

    Ok(())
}