use karin_js::{option::*, Compiler};
use karinc::{hir::id::*, input::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: &str) -> Vec<String> {
    let input_tree = InputTree {
        hakos: vec![
            InputHako {
                id: HakoId::new(0),
                mods: vec![
                    InputMod {
                        id: ModId::new(0, 0),
                        path: "my_hako".into(),
                        source: input.to_string(),
                        submods: Vec::new(),
                    },
                ],
            },
        ],
    };
    let options = CompilerOptions {
        root_source_name: "index".to_string(),
        bundles: true,
        module: JsModule::Es,
    };
    let output = Compiler::compile(&input_tree, &options);
    output.files.into_iter().map(|file| {
        match file.source {
            Some(code) => code.source,
            None => "err".to_string(),
        }
    }).collect()
}
