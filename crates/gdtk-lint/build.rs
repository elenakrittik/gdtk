fn main() {
    println!("cargo::rerun-if-changed=lints/");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=schema.graphql");

    let lints = jwalk::WalkDir::new("lints")
        .into_iter()
        .filter_map(|path| path.ok())
        .filter(|path| path.file_type().is_file())
        .filter_map(|path| path.path().to_str().map(|path| "../".to_owned() + path))
        .collect::<Vec<_>>();

    let len = lints.len();

    let mod_ = quote::quote! {
        use crate::types::LintQuery;

        pub(crate) fn get_builtin_lints() -> [LintQuery<'static>; #len] {
            [
                #(ron::from_str(include_str!(#lints)).unwrap(),)*
            ]
        }
    };

    let mod_: syn::File = syn::parse2(mod_).unwrap();
    let pretty = prettyplease::unparse(&mod_);

    std::fs::write("src/builtin.rs", pretty).unwrap();
}
