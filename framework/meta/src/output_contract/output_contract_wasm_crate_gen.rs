use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use super::OutputContract;

const PREFIX_AUTO_GENERATED: &str =
    "// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

";

const NUM_INIT: usize = 1;
const NUM_ASYNC_CB: usize = 1;

const PREFIX_NO_STD: &str = "
#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

";

impl OutputContract {
    /// Makes sure that all the necessary wasm crate directories exist.
    pub fn create_wasm_crate_dir(&self) {
        fs::create_dir_all(PathBuf::from(&self.wasm_crate_path()).join("src")).unwrap();
    }

    /// Generates the wasm crate lib.rs source, st the given path.
    pub fn generate_wasm_src_lib_file(&self) {
        let lib_path = format!("{}/src/lib.rs", &self.wasm_crate_path());
        let mut wasm_lib_file = File::create(lib_path).unwrap();
        self.write_wasm_src_lib_contents(&mut wasm_lib_file);
    }

    fn write_wasm_src_lib_contents(&self, wasm_lib_file: &mut File) {
        let explicit_endpoint_names = self.endpoint_names();
        wasm_lib_file
            .write_all(PREFIX_AUTO_GENERATED.as_bytes())
            .unwrap();
        self.write_stat_comments(wasm_lib_file);
        wasm_lib_file.write_all(PREFIX_NO_STD.as_bytes()).unwrap();

        let full_macro_name = if self.external_view {
            "mx_sc_wasm_adapter::external_view_endpoints!"
        } else {
            "mx_sc_wasm_adapter::endpoints!"
        };

        let mut all_endpoint_names = explicit_endpoint_names;
        if self.abi.has_callback {
            all_endpoint_names.push("callBack".to_string());
        }
        for promise_callback in &self.abi.promise_callbacks {
            all_endpoint_names.push(promise_callback.name.to_string());
        }

        let contract_module_name = self.abi.get_crate_name_for_code();
        write_endpoints_macro(
            full_macro_name,
            wasm_lib_file,
            &contract_module_name,
            all_endpoint_names.iter(),
        );

        if !self.abi.has_callback {
            write_wasm_empty_callback_macro(wasm_lib_file);
        }
    }
}

fn write_stat_comment(wasm_lib_file: &mut File, label: &str, number: usize) {
    writeln!(wasm_lib_file, "// {label:<35} {number:3}").unwrap();
}

impl OutputContract {
    /// Writing some nicely formatted comments breaking down all exported functions.
    fn write_stat_comments(&self, wasm_lib_file: &mut File) {
        write_stat_comment(wasm_lib_file, "Init:", NUM_INIT);
        write_stat_comment(wasm_lib_file, "Endpoints:", self.abi.endpoints.len());
        if self.abi.has_callback {
            write_stat_comment(wasm_lib_file, "Async Callback:", NUM_ASYNC_CB);
        } else {
            write_stat_comment(wasm_lib_file, "Async Callback (empty):", NUM_ASYNC_CB);
        }
        if !self.abi.promise_callbacks.is_empty() {
            write_stat_comment(
                wasm_lib_file,
                "Promise callbacks:",
                self.abi.promise_callbacks.len(),
            );
        }
        let total =
            self.abi.endpoints.len() + NUM_INIT + NUM_ASYNC_CB + self.abi.promise_callbacks.len();

        write_stat_comment(wasm_lib_file, "Total number of exported functions:", total);
    }
}

fn write_endpoints_macro<'a, I>(
    full_macro_name: &str,
    wasm_lib_file: &mut File,
    contract_module_name: &str,
    endpoint_names: I,
) where
    I: Iterator<Item = &'a String>,
{
    writeln!(wasm_lib_file, "{full_macro_name} {{").unwrap();
    writeln!(wasm_lib_file, "    {contract_module_name}").unwrap();
    writeln!(wasm_lib_file, "    (").unwrap();
    for endpoint_name in endpoint_names {
        writeln!(wasm_lib_file, "        {endpoint_name}").unwrap();
    }
    writeln!(wasm_lib_file, "    )").unwrap();
    writeln!(wasm_lib_file, "}}").unwrap();
}

fn write_wasm_empty_callback_macro(wasm_lib_file: &mut File) {
    writeln!(wasm_lib_file).unwrap();
    writeln!(wasm_lib_file, "mx_sc_wasm_adapter::empty_callback! {{}}").unwrap();
}
