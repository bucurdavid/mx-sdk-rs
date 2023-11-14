use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use toml::{value::Table, Value};

use crate::cmd::contract::sc_config::ContractVariant;

pub const CARGO_TOML_DEPENDENCIES: &str = "dependencies";
pub const CARGO_TOML_DEV_DEPENDENCIES: &str = "dev-dependencies";

/// Contains an in-memory representation of a Cargo.toml file.
///
/// Implementation notes:
///
/// - Currently contains a raw toml tree, but in principle it could also work with a cargo_toml::Manifest.
/// - It keeps an ordered representation, thanks to the `toml` `preserve_order` feature.
#[derive(Clone, Debug)]
pub struct CargoTomlContents {
    pub path: PathBuf,
    pub toml_value: toml::Value,
}

impl CargoTomlContents {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref();
        let cargo_toml_content = fs::read(path_ref).expect("failed to open Cargo.toml file");
        let cargo_toml_content_str =
            String::from_utf8(cargo_toml_content).expect("error decoding Cargo.toml utf-8");
        let toml_value = cargo_toml_content_str
            .parse::<toml::Value>()
            .unwrap_or_else(|e| {
                panic!(
                    "failed to parse Cargo.toml toml format, path:{}, error: {e}",
                    path_ref.display()
                )
            });
        CargoTomlContents {
            path: path_ref.to_owned(),
            toml_value,
        }
    }

    pub fn new() -> Self {
        CargoTomlContents {
            path: PathBuf::new(),
            toml_value: toml::Value::Table(Table::new()),
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let cargo_toml_content_str =
            toml::to_string_pretty(&self.toml_value).expect("failed to format Cargo.toml contents");
        let mut file = std::fs::File::create(path).expect("failed to create Cargo.toml file");
        file.write_all(cargo_toml_content_str.as_bytes())
            .expect("failed to write Cargo.toml contents to file");
    }

    pub fn package_name(&self) -> String {
        self.toml_value
            .get("package")
            .expect("missing package in Cargo.toml")
            .get("name")
            .expect("missing package name in Cargo.toml")
            .as_str()
            .expect("package name not a string value")
            .to_string()
    }

    pub fn package_edition(&self) -> String {
        self.toml_value
            .get("package")
            .expect("missing package in Cargo.toml")
            .get("edition")
            .expect("missing package name in Cargo.toml")
            .as_str()
            .expect("package name not a string value")
            .to_string()
    }

    pub fn mvx_dependency_version(&self) -> String {
        let toml_value = self.dependency("multiversx-sc").unwrap().to_owned();

        if let Value::Table(table) = toml_value {
            if let Some(version) = table.get("version") {
                return version.to_string().replace("\"", "");
            }
        }
        panic!("could not find multiversx-sc dependency version in cargo toml")
    }

    pub fn mvx_dependency_path(&self) -> String {
        let toml_value = self.dependency("multiversx-sc").unwrap().to_owned();

        if let Value::Table(table) = toml_value {
            if let Some(path) = table.get("path") {
                return path.to_string().replace("\"", "");
            }
        }

        //if path is missing then don't print at all
        panic!("could not find multiversx-sc dependency path in cargo toml")
    }

    pub fn get_adapter_dependencies(&self) -> (String, String) {
        (self.mvx_dependency_version(), self.mvx_dependency_path())
    }

    pub fn add_deps(
        &mut self,
        crate_name: String,
        adapter_version: &String,
        adapter_path: &String,
    ) {
        let mut crate_deps = toml::map::Map::new();
        crate_deps.insert("path".to_string(), toml::Value::String("..".to_string()));

        let mut adapter_deps = toml::map::Map::new();
        adapter_deps.insert(
            "version".to_string(),
            toml::Value::String(adapter_version.to_string()),
        );
        adapter_deps.insert(
            "path".to_string(),
            toml::Value::String(change_from_base_to_adapter_path(adapter_path)),
        );

        let mut toml_table_adapter = toml::map::Map::new();
        toml_table_adapter.insert(
            "multiversx-sc-wasm-adapter".to_string(),
            toml::Value::Table(adapter_deps),
        );

        let mut toml_table_crate = toml::map::Map::new();
        toml_table_crate.insert(crate_name, toml::Value::Table(crate_deps));

        toml_table_crate.extend(toml_table_adapter);

        self.toml_value
            .as_table_mut()
            .expect("add deps cargo toml error wasm adapter")
            .insert(
                "dependencies".to_string(),
                toml::Value::Table(toml_table_crate),
            );
    }

    /// Assumes that a package section already exists.
    pub fn change_package_name(&mut self, new_package_name: String) {
        let package = self
            .toml_value
            .get_mut("package")
            .expect("missing package in Cargo.toml");
        package
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("name".to_string(), toml::Value::String(new_package_name));
    }

    pub fn dependencies_table(&self) -> Option<&Table> {
        if let Some(deps) = self.toml_value.get(CARGO_TOML_DEPENDENCIES) {
            deps.as_table()
        } else if let Some(deps) = self.toml_value.get(CARGO_TOML_DEV_DEPENDENCIES) {
            deps.as_table()
        } else {
            None
        }
    }

    pub fn dependency(&self, dep_name: &str) -> Option<&Value> {
        if let Some(deps_map) = self.dependencies_table() {
            deps_map.get(dep_name)
        } else {
            None
        }
    }

    pub fn has_dependencies(&self) -> bool {
        self.toml_value.get(CARGO_TOML_DEPENDENCIES).is_some()
    }

    pub fn dependencies_mut(&mut self) -> &mut Table {
        self.toml_value
            .get_mut(CARGO_TOML_DEPENDENCIES)
            .unwrap_or_else(|| panic!("no dependencies found in crate {}", self.path.display()))
            .as_table_mut()
            .expect("malformed crate Cargo.toml")
    }

    pub fn has_dev_dependencies(&self) -> bool {
        self.toml_value.get(CARGO_TOML_DEV_DEPENDENCIES).is_some()
    }

    pub fn dev_dependencies_mut(&mut self) -> &mut Table {
        self.toml_value
            .get_mut(CARGO_TOML_DEV_DEPENDENCIES)
            .unwrap_or_else(|| panic!("no dependencies found in crate {}", self.path.display()))
            .as_table_mut()
            .expect("malformed crate Cargo.toml")
    }

    pub fn add_lib(&mut self) {
        let mut value = toml::map::Map::new();
        let array = vec![toml::Value::String("cdylib".to_string())];
        let members = toml::Value::Array(array);
        value.insert("crate-type".to_string(), members);

        self.toml_value
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("lib".to_string(), toml::Value::Table(value));
    }

    pub fn add_package_info(
        &mut self,
        name: String,
        version: String,
        current_edition: String,
        publish: bool,
    ) {
        let mut value = toml::map::Map::new();
        value.insert("name".to_string(), Value::String(name));
        value.insert("version".to_string(), Value::String(version));
        value.insert("edition".to_string(), Value::String(current_edition));
        value.insert("publish".to_string(), Value::Boolean(publish));

        self.toml_value
            .as_table_mut()
            .expect("malformed package in Cargo.toml / add_package")
            .insert("package".to_string(), toml::Value::Table(value));
    }

    pub fn add_contract_variant_profile(&mut self, contract: &ContractVariant) {
        let contract_profile = contract.settings.contract_variant_profile.clone();

        let mut profile_props = toml::map::Map::new();
        profile_props.insert(
            "codegen-units".to_string(),
            Value::Integer(contract_profile.codegen_units.into()),
        );
        profile_props.insert(
            "opt-level".to_string(),
            Value::String(contract_profile.opt_level),
        );
        profile_props.insert("lto".to_string(), Value::Boolean(contract_profile.lto));
        profile_props.insert("debug".to_string(), Value::Boolean(contract_profile.debug));
        profile_props.insert("panic".to_string(), Value::String(contract_profile.panic));

        let mut toml_table = toml::map::Map::new();
        toml_table.insert("release".to_string(), toml::Value::Table(profile_props));

        self.toml_value
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("profile".to_string(), toml::Value::Table(toml_table));
    }

    pub fn add_workspace(&mut self, members: &[&str]) {
        let array: Vec<toml::Value> = members
            .iter()
            .map(|s| toml::Value::String(s.to_string()))
            .collect();
        let members_toml = toml::Value::Array(array);

        let mut workspace = toml::Value::Table(toml::map::Map::new());
        workspace
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("members".to_string(), members_toml);

        self.toml_value
            .as_table_mut()
            .expect("malformed package in Cargo.toml")
            .insert("workspace".to_string(), workspace);
    }

    pub fn local_dependency_paths(&self, ignore_deps: &[&str]) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(deps_map) = self.dependencies_table() {
            for (key, value) in deps_map {
                if ignore_deps.contains(&key.as_str()) {
                    continue;
                }

                if let Some(path) = value.get("path") {
                    result.push(path.as_str().expect("path is not a string").to_string());
                }
            }
        }
        result
    }

    pub fn change_features_for_parent_crate_dep(&mut self, features: &[String]) {
        let deps_mut = self.dependencies_mut();
        for (_, dep) in deps_mut {
            if is_dep_path_above(dep) {
                let feature_values = features
                    .iter()
                    .map(|feature| Value::String(feature.clone()))
                    .collect();
                dep.as_table_mut()
                    .expect("malformed crate Cargo.toml")
                    .insert("features".to_string(), Value::Array(feature_values));
            }
        }
    }
}

/// Checks that path == ".." in a depdency.
fn is_dep_path_above(dep: &Value) -> bool {
    if let Some(path) = dep.get("path") {
        if let Some(s) = path.as_str() {
            return s == "..";
        }
    }

    false
}

fn change_from_base_to_adapter_path(base_path: &String) -> String {
    format!("../{}", base_path.replace("base", "wasm-adapter"))
}

mod tests {

    #[test]
    fn test_change_from_base_to_adapter_path() {
        let base_path = "../../../framework/base".to_string();
        let adapter_path = "../../../../framework/wasm-adapter".to_string();
        assert_eq!(super::change_from_base_to_adapter_path(&base_path), adapter_path);
    }
}
