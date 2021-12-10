use std::error::Error;
use wasmer::{imports, wat2wasm, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmtime::*;
use anyhow::Result;

fn main() {
    let nan = 0.0_f32 / 0.0;
    let nan2 = -0.0_f32 / 0.0;
    println!("nan signaling: {:?}", nan.to_be_bytes());
    println!("nan2 signaling: {:?}", nan2.to_be_bytes());
    println!("nan + 0: {:?}", (nan + 0 as f32).to_be_bytes());
    println!("nan - 0: {:?}", (nan - 0 as f32).to_be_bytes());
    println!("nan * 0: {:?}", (nan * 0 as f32).to_be_bytes());
    println!("nan / 0: {:?}", (nan / 0 as f32).to_be_bytes());
    println!("nan + nan: {:?}", (nan + nan2).to_be_bytes());
    println!("nan - nan: {:?}", (nan - nan2).to_be_bytes());
    println!("nan * nan: {:?}", (nan * nan2).to_be_bytes());
    println!("nan / nan: {:?}", (nan / nan2).to_be_bytes());
    println!("nan + 2.0: {:?}", (nan + 2 as f32).to_be_bytes());
    println!("nan - 2: {:?}", (nan - 2 as f32).to_be_bytes());
    println!("nan * 2: {:?}", (nan * 2 as f32).to_be_bytes());
    println!("nan / 2: {:?}", (nan / 2 as f32).to_be_bytes());
    println!("change in byte");
    let mut nanb = nan.to_be_bytes();
    let mut nanb2 = nan2.to_be_bytes();
    nanb[3] = 1 as u8;
    nanb2[3] = 255 as u8;
    println!("change last byte: {:?}", nanb);
    println!("change last byte into 255: {:?}", nanb);
    let nan3 = f32::from_be_bytes(nanb2);
    println!("nan: {}", f32::NAN != f32::NAN);
    println!("nan quiet: {:?}", nan3);
    println!("nan3 + 0: {:?}", (nan3 + 0 as f32).to_be_bytes());
    println!("nan3 - 0: {:?}", (nan3 - 0 as f32).to_be_bytes());
    println!("nan3 * 0: {:?}", (nan3 * 0 as f32).to_be_bytes());
    println!("nan3 / 0: {:?}", (nan3 / 0 as f32).to_be_bytes());
    println!("nan3 + nan3: {:?}", (nan3 + nan3).to_be_bytes());
    println!("nan3 - nan3: {:?}", (nan3 - nan3).to_be_bytes());
    println!("nan3 * nan3: {:?}", (nan3 * nan3).to_be_bytes());
    println!("nan3 / nan3: {:?}", (nan3 / nan3).to_be_bytes());
    println!("NAn / nan: {:?}", (f32::NAN / f32::NAN));
    println!("Nan f64 as f32: {:?}", (f64::NAN as f32).to_be_bytes());
    println!("Nan f64 : {:?}", (f64::NAN).to_be_bytes());
    println!("Nan f32 as f64: {:?}", (f32::NAN as f64).to_be_bytes());
    println!("Nan / Nan: {:?}", (0.0f32 / f64::NAN as f32));
    println!("nan3 + 2.0: {:?}", (nan3 + 2 as f32).to_be_bytes());
    println!("nan3 - 2: {:?}", (nan3 - 2 as f32).to_be_bytes());
    println!("nan3 * 2: {:?}", (nan3 * 2 as f32).to_be_bytes());
    println!("nan3 / 2: {:?}", (nan3 / 2 as f32).to_be_bytes());
}

// f32 rust test
#[test]
fn test() {
    let result: f32 = 0.0 / 0.0;
    println!("{:?}", result.to_be_bytes());
}

// f64 wat wasmer test, the same for f32, not runnable for darwin
#[test]
fn test_all() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = wat2wasm(
        br#"
        (module
            (type $test_t (func (param f64) (result f64)))
            (func $test_f (type $test_t)  (param $value f64) (result f64)
              local.get $value
              f64.const 0.0
              f64.div)
            (export "test" (func $test_f))
          )"#,
    )?;

    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    let test = instance
        .exports
        .get_function("test")?
        .native::<f64, f64>()?;

    println!("Calling `test` function...");
    let result = test.call(0.0)?;
    println!("{:?}", result.to_be_bytes());

    println!("Results of `test`: {:?}", result);

    Ok(())
}

// f32 wat wasmtime test, not runnable for darwin
#[test]
fn wasmtime_test() -> Result<()>{
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    // Create a `Linker` and define our host function in it:
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host", "hello", |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    })?;

    // Use the `linker` to instantiate the module, which will automatically
    // resolve the imports of the module using name-based resolution.
    let mut store = Store::new(&engine, 0);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), (), _>(&mut store, "hello")?;
    hello.call(&mut store, ())?;

    Ok(())
}
