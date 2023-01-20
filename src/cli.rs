use std::env;
use std::rc::Rc;

use anyhow::Error;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

use crate::runtime::TypescriptModuleLoader;
use crate::server::fsr;


pub fn parse() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if let Some(arg) = args.get(i) {
            match arg.as_str() {
                "run" => {
                    let path = if let Some(e) = args.get(i + 1) {
                        e.as_str()
                    } else {
                        "."
                    };
                    doji_runtime(path).unwrap()
                }
                _ => continue,
            }
        } else {
            return;
        }
    }
}

pub(crate) fn doji_runtime(start: &str) -> Result<(), Error> {
    let ext = Extension::builder("fsr")
        .ops(vec![fsr::load_fsr::decl()])
        .build();

    // let mut runtime = JsRuntime::new(RuntimeOptions {
    //     extensions: vec![ext],
    //     module_loader: Some(Rc::new(FsModuleLoader)),
    //     ..Default::default()
    // });

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        module_loader: Some(Rc::new(TypescriptModuleLoader)),
        ..Default::default()
    });

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // let input = Path::new(start);
    // let (file_name, _) = input
    //     .file_name()
    //     .unwrap()
    //     .to_str()
    //     .unwrap()
    //     .split_once('.')
    //     .unwrap();

    let main_module = deno_core::resolve_path(start)?;
    // let file_contents = fs::read_to_string(start).unwrap();
    // runtime.execute_script(file_name, &file_contents).unwrap();

    let future = async move {
        let mod_id = js_runtime.load_main_module(&main_module, None).await?;
        let result = js_runtime.mod_evaluate(mod_id);
        js_runtime.run_event_loop(false).await?;
        result.await?
    };

    runtime.block_on(future)
}
