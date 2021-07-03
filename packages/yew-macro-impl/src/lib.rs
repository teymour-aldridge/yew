//! This crate provides Yew's procedural macro `html!` which allows using JSX-like syntax
//! for generating html and the `Properties` derive macro for deriving the `Properties` trait
//! for components.
//!
//! ```
//! use yew::prelude::*;
//!
//! struct Component {
//!   link: ComponentLink<Self>,
//! }
//!
//! #[derive(Clone, Properties)]
//! struct Props {
//!     prop: String,
//! };
//!
//! # enum Msg { Submit }
//! #
//! # impl yew::Component for Component {
//! #     type Message = Msg;
//! #     type Properties = Props;
//! #     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn view(&self) -> Html {
//! #
//! // ...
//!
//! html! {
//!   <div>
//!     <button onclick=self.link.callback(|_| Msg::Submit)>
//!       { "Submit" }
//!     </button>
//!     <>
//!       <Component prop="first" />
//!       <Component prop="second" />
//!     </>
//!   </div>
//! }
//! #
//! #     }
//! # }
//! #
//! # fn main() {}
//! ```
//!
//! Please refer to [https://github.com/yewstack/yew](https://github.com/yewstack/yew) for how to set this up.

mod classes;
mod derive_props;
mod html_tree;
mod props;
mod stringify;

use derive_props::DerivePropsInput;
use html_tree::{HtmlRoot, HtmlRootVNode};
use proc_macro2::TokenStream;
use props::PropsMacroInput;
use quote::ToTokens;
use syn::buffer::Cursor;
use syn::parse2;

trait Peek<'a, T> {
    fn peek(cursor: Cursor<'a>) -> Option<(T, Cursor<'a>)>;
}

trait PeekValue<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}

fn non_capitalized_ascii(string: &str) -> bool {
    if !string.is_ascii() {
        false
    } else if let Some(c) = string.bytes().next() {
        c.is_ascii_lowercase()
    } else {
        false
    }
}

/// Combine multiple `syn` errors into a single one.
/// Returns `Result::Ok` if the given iterator is empty
fn join_errors(mut it: impl Iterator<Item = syn::Error>) -> syn::Result<()> {
    it.next().map_or(Ok(()), |mut err| {
        for other in it {
            err.combine(other);
        }
        Err(err)
    })
}

#[no_mangle]
pub extern "C" fn derive_props(input: TokenStream) -> TokenStream {
    match parse2::<DerivePropsInput>(input) {
        Ok(t) => t.into_token_stream(),
        Err(e) => e.into_compile_error(),
    }
}

#[no_mangle]
pub extern "C" fn html_nested(input: TokenStream) -> TokenStream {
    match parse2::<HtmlRoot>(input) {
        Ok(t) => t.into_token_stream(),
        Err(e) => e.into_compile_error(),
    }
}

#[no_mangle]
pub extern "C" fn html(input: TokenStream) -> TokenStream {
    match parse2::<HtmlRootVNode>(input) {
        Ok(t) => t.into_token_stream(),
        Err(e) => e.into_compile_error(),
    }
}

#[no_mangle]
pub extern "C" fn props(input: TokenStream) -> TokenStream {
    match parse2::<PropsMacroInput>(input) {
        Ok(t) => t.into_token_stream(),
        Err(e) => e.into_compile_error(),
    }
}

#[no_mangle]
pub extern "C" fn classes(input: TokenStream) -> TokenStream {
    match parse2::<classes::Classes>(input) {
        Ok(t) => t.into_token_stream(),
        Err(e) => e.into_compile_error(),
    }
}
