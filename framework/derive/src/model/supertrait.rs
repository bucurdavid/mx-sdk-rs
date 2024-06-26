use syn::{punctuated::Punctuated, token::PathSep};

/// Path to a Rust module containing a contract module.
pub type ModulePath = Punctuated<syn::PathSegment, PathSep>;

#[derive(Clone, Debug)]
pub struct Supertrait {
    pub full_path: syn::Path,
    #[allow(dead_code)]
    pub trait_name: syn::PathSegment,
    pub module_path: ModulePath,
}
