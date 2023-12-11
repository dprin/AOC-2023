use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, Ident, Token, LitInt, ExprLit, parse_macro_input};
use std::fmt::Display;

const PARAMS: u32 = 5;

#[derive(Default)]
struct MacroInput {
    year: u32,
    day: u32,
    part: u32,
    test_input_name: Option<Ident>,
    test_answer: Option<ExprLit>,
}

impl Display for MacroInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inp = match &self.test_input_name {
            Some(val) => format!("{}", val),
            None => "None".to_string()
        };

        write!(f, "MacroInput {{ year: {}, day: {}, part: {}, test_input_name: {}, test_answer: {} }}", self.year, self.day, self.part, inp, "doesn't work :(")
    }
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = MacroInput::default();

        for i in 0..PARAMS {
            let var: Ident = input.parse()?;
            input.parse::<Token!(=)>()?;

            match var.to_string().as_str() {
                "Year" => {ret.year = input.parse::<LitInt>()?.base10_parse::<u32>()?},
                "Day" => {ret.day = input.parse::<LitInt>()?.base10_parse::<u32>()?},
                "Part" => {ret.part = input.parse::<LitInt>()?.base10_parse::<u32>()?},
                "Test" => {ret.test_input_name = Some(input.parse::<Ident>()?)},
                "Answer" => {ret.test_answer = Some(input.parse::<ExprLit>()?)},
                s => panic!("Invalid parameter: {}",  s),
            };
            
            if i != PARAMS - 1 {
                input.parse::<Token!(,)>()?;
            }
        }

        if ret.test_answer.is_none() || ret.test_input_name.is_none() {
            panic!("Please enter an test input and answer")
        }

        Ok(ret)
    }
}

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr}");
    println!("{item}");
    
    let attr = parse_macro_input!(attr as MacroInput);
    let test_name: Ident = attr.test_input_name.clone().unwrap();

    let main = quote! {
        fn main() {
            println!("Hello from macro!\n {}", #test_name);
        }
    };
    
    println!("{attr}");

    main.into()
}
