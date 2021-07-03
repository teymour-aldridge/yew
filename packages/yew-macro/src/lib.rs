use watt::WasmMacro;
use proc_macro::TokenStream;

static EXECUTE: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("yew_macro_impl.wasm");

#[proc_macro_derive(Properties, attributes(prop_or, prop_or_else, prop_or_default))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    EXECUTE.proc_macro_derive("derive_props", input)
}

#[proc_macro]
pub fn html_nested(input: TokenStream) -> TokenStream {
    EXECUTE.proc_macro("html_nested", input)
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    EXECUTE.proc_macro("html", input)
}

#[proc_macro]
pub fn props(input: TokenStream) -> TokenStream {
    EXECUTE.proc_macro("props", input)
}

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    EXECUTE.proc_macro("classes", input)
}
