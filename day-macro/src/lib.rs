use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::fmt::Debug;
use syn::Expr;
use syn::{parse::Parse, parse_macro_input, Ident, LitInt, Token};

#[derive(Default, Debug)]
struct MacroInput {
    year: u32,
    day: u32,
    part: u32,
    test_input_name: Option<Ident>,
    test_answer: Option<Expr>,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = MacroInput::default();

        while !input.is_empty() {
            let var: Ident = input.parse()?;
            input.parse::<Token!(=)>()?;

            match var.to_string().as_str() {
                "Year" => ret.year = input.parse::<LitInt>()?.base10_parse::<u32>()?,
                "Day" => ret.day = input.parse::<LitInt>()?.base10_parse::<u32>()?,
                "Part" => ret.part = input.parse::<LitInt>()?.base10_parse::<u32>()?,
                "Test" => ret.test_input_name = Some(input.parse::<Ident>()?),
                "Answer" => ret.test_answer = Some(input.parse::<Expr>()?),
                s => panic!("Invalid parameter: {}", s),
            };

            if !input.is_empty() {
                input.parse::<Token!(,)>()?;
            }
        }

        if ret.test_answer.is_none() ^ ret.test_input_name.is_none() {
            panic!("Please enter either both test and answer or no test and no answer");
        }

        Ok(ret)
    }
}

fn generate_test(params: &MacroInput, fn_name: &Ident) -> TokenStream {
    let test_name = format_ident!("test_part_{}", &params.part);

    let input_name = params.test_input_name.clone().unwrap();

    let answer = match params.test_answer.clone().unwrap() {
        Expr::Lit(l) => l,
        _ => panic!("Expected a literal for answer"),
    };

    let fn_input: Ident = format_ident!("{}", fn_name);

    quote! {
        #[test]
        fn #test_name () {
            assert_eq!(#fn_input(&String::from(#input_name)), #answer);
        }
    }
    .into()
}

fn generate_main() -> TokenStream {
    quote! {
        fn main() {

        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as MacroInput);
    let item = parse_macro_input!(item as syn::ItemFn);

    let mut ret: TokenStream = TokenStream::new();

    // println!("{:#?}", attr);
    // println!("{:#?}", item);

    if attr.test_answer.is_some() {
        ret.extend(&mut generate_test(&attr, &item.sig.ident).into_iter());
    }
    ret.extend(&mut generate_main().into_iter());
    ret.extend(TokenStream::from(item.into_token_stream()).into_iter());

    ret.into()
}
