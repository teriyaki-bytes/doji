
use deno_core::op;

#[op(fast)]
pub fn load_fsr(www: String) -> Result<(), deno_core::error::AnyError> {
    println!("Called from doji_ts: {www}");
    // if Path::new(&www).exists() {
    //     return Err(deno_core::error::custom_error(
    //         "PathNotExist",
    //         format!("Path at {www} does not exist or cannot be accessed"),
    //     ));
    // }
    // https://github.com/denoland/deno/blob/main/core/examples/ts_module_loader.rs
    Ok(())
}
