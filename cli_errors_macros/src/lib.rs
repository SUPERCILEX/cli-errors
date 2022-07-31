#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Proc macro to correctly handle the exit code and error message returned from
/// [CliResult][cli_errors::CliResult].
#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    if input.sig.ident != "main" {
        input
            .sig
            .ident
            .span()
            .unwrap()
            .error("cli_errors::main must be used on the main method.")
            .emit();
    }

    let wrapped_code = input.block;
    TokenStream::from(quote! {
        fn main() {
            if let Err(e) = wrapped_main() {
                if let Some(source) = e.source {
                    eprintln!("{:?}", source);
                }
                std::process::exit(e.code);
            }
        }

        fn wrapped_main() -> cli_errors::CliResult<()> {
            #wrapped_code
        }
    })
}
