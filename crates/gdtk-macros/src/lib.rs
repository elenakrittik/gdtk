use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, ItemStruct};

#[derive(Debug, FromMeta)]
struct LintArgs {
    message: String,
    code: String,
    severity: syn::Ident,
}

impl LintArgs {
    fn from_tokens(tokens: TokenStream) -> Self {
        let meta = NestedMeta::parse_meta_list(tokens.into()).unwrap();
        Self::from_list(&meta).unwrap()
    }
}

#[proc_macro_attribute]
pub fn lint(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = LintArgs::from_tokens(args);
    let mut item = syn::parse_macro_input!(item as ItemStruct);

    let LintArgs {
        message,
        code,
        severity,
    } = args;

    // Add the diagnostic field to the struct
    if let syn::Fields::Named(fields) = &mut item.fields {
        let field_src = quote! { __diagnostics: Vec<miette::MietteDiagnostic> };
        let field = syn::Field::parse_named.parse2(field_src).unwrap();

        fields.named.push(field);
    }

    let ident = &item.ident;
    let generics = &item.generics;

    quote::quote! {
        #[derive(Default)]
        #item

        impl #generics #ident #generics {
            /// Begin a new report.
            pub fn report() -> miette::MietteDiagnostic {
                miette::MietteDiagnostic::new(#message)
                    .with_code(#code)
                    .with_severity(miette::Severity::#severity)
            }

            /// Submit the report.
            pub fn submit(&mut self, report: miette::MietteDiagnostic) {
                self.__diagnostics.push(report);
            }

            /// Consumes self and returns the collected diagnostics.
            pub fn into_diagnostics(self) -> Vec<miette::MietteDiagnostic> {
                self.__diagnostics
            }
        }
    }
    .into()
}
