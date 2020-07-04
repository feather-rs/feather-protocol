//! Procedural macro to read in a protocol definition
//! and output code to read/write each packet.
//!
//! This macro works in stages, inspired by rustc/typical
//! compiler architecture:
//! AST: `syn::ItemMod` containing all the packet structs and field attributes.
//! High-level intermediate representation (HIR): a more processed version of the `ItemMod`.
//! Mid-level intermediate representation (MIR): a lower level structure intended for use
//! by the code generator (the next stage). Different versions of packets are "monomorphized".
//! Code generator: writes out final Rust code.
//!
//! Transitioning from one stage to another is referred to as "lowering."

use quote::quote;
use syn::ItemMod;

mod codegen;
mod hir;
mod mir;

#[proc_macro_attribute]
pub fn packets(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: ItemMod = syn::parse_macro_input!(input);

    let _ = codegen::generate(todo!());

    let tokens = quote! {
        mod __packets {
            pub mod send {}
            pub mod recv {}
        }
    };

    tokens.into()
}
