extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, ItemFn, LitStr, Result, Token};

struct ExecuteArgs {
    before: Option<LitStr>,
    after: Option<LitStr>,
}

impl Parse for ExecuteArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut before = None;
        let mut after = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let lit: LitStr = input.parse()?;

            if ident == "before" {
                before = Some(lit);
            } else if ident == "after" {
                after = Some(lit);
            } else {
                return Err(syn::Error::new(ident.span(), "Expected 'before' or 'after'"));
            }

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(ExecuteArgs { before, after })
    }
}

#[proc_macro_attribute]
pub fn execute(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExecuteArgs);
    let input = parse_macro_input!(input as ItemFn);

    let before_code = if let Some(code) = args.before {
        let tokens: proc_macro2::TokenStream = code
            .value()
            .parse()
            .expect("Can't parse before code to TokenStream");
        quote! { #tokens }
    } else {
        quote!()
    };

    let after_code = if let Some(code) = args.after {
        let tokens: proc_macro2::TokenStream = code
            .value()
            .parse()
            .expect("Can't parse after code to TokenStream");
        quote! { #tokens }
    } else {
        quote!()
    };

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let result = quote! {
        #(#attrs)*
        #vis #sig {
            #before_code
            let result = #block;
            #after_code
            result
        }
    };

    TokenStream::from(result)
}