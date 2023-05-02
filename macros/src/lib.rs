#![feature(proc_macro_quote)]

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, DeriveInput},
};

#[proc_macro_derive(CommandArgument)]
pub fn command_argument_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let output = quote! {
        impl nvim_oxi::conversion::FromObject for #name {
            fn from_object(object: nvim_oxi::Object) -> std::result::Result<Self, nvim_oxi::conversion::Error> {
                Ok(match object.kind() {
                    nvim_oxi::ObjectKind::Dictionary => Self::deserialize(nvim_oxi::serde::Deserializer::new(object))
                        .map_err(Into::<nvim_oxi::conversion::Error>::into)?,
                    _ => Self::default(),
                })
            }
        }

        impl nvim_oxi::lua::Poppable for #name {
            unsafe fn pop(lstate: *mut nvim_oxi::lua::ffi::lua_State) -> Result<Self, nvim_oxi::lua::Error> {
                let obj = Object::pop(lstate)?;
                <Self as nvim_oxi::conversion::FromObject>::from_object(obj)
                    .map_err(nvim_oxi::lua::Error::pop_error_from_err::<Self, _>)
            }
        }
    };

    TokenStream::from(output)
}
