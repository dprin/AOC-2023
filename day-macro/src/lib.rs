use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::cell::RefCell;
use std::fmt::Debug;
use syn::{parse::Parse, parse_macro_input, Ident, LitInt, Token};
use syn::{Expr, ItemFn};

thread_local! {
    static STORED_FUNCTIONS: RefCell<Vec<Function>> = RefCell::new(Vec::new());
}

struct Function {
    args: MacroInput,
    func: String,
}

#[derive(Default)]
struct ExecuteInput {
    year: u32,
    day: u32,
}

#[derive(Default, Debug, Clone)]
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
        
        ret.year = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        input.parse::<Token!(,)>()?;
        ret.day = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        input.parse::<Token!(,)>()?;
        ret.part = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        
        if !input.is_empty() {
            input.parse::<Token!(,)>()?;
            ret.test_input_name = Some(input.parse::<Ident>()?);
            input.parse::<Token!(,)>()?;
            ret.test_answer = Some(input.parse::<Expr>()?);
        }        

        Ok(ret)
    }
}

impl Parse for ExecuteInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = ExecuteInput::default();

        ret.year = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        input.parse::<Token!(,)>()?;
        ret.day = input.parse::<LitInt>()?.base10_parse::<u32>()?;

        Ok(ret)
    }
}

fn generate_test(params: &MacroInput, fn_name: &Ident) -> TokenStream {
    let test_name = format_ident!("test_day_{}_part_{}", &params.day, &params.part);

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

fn generate_functions(day: &u32) -> TokenStream {
    let mut ret = TokenStream::new();
    let day: u32 = *day;

    STORED_FUNCTIONS.with_borrow(|v| {
        for f in v.into_iter().filter(|func| func.args.day == day) {
            let func = format_ident!("{}", f.func);
            let day = f.args.day;
            let part = f.args.part;

            let call: TokenStream = quote! {
               println!("Day {} | Part {}: {}", #day, #part, #func(&data));
            }
            .into();

            ret.extend(call);
        }
    });

    ret
}

#[proc_macro]
pub fn execute_day(tokens: TokenStream) -> TokenStream {
    let ExecuteInput { year, day } = parse_macro_input!(tokens as ExecuteInput);
    let funcs = proc_macro2::TokenStream::from(generate_functions(&day));

    quote! {
        fn main() {
            let data = utils::fetch_input(#year, #day);

            #funcs
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as MacroInput);
    let item = parse_macro_input!(item as ItemFn);

    STORED_FUNCTIONS.with_borrow_mut(|v| {
        v.push(Function {
            args: attr.clone(),
            func: item.sig.ident.clone().to_string(), // this is the only fix to stop getting my
                                                      // macro from seg_fauling
        })
    });

    let mut ret: TokenStream = TokenStream::new();

    if attr.test_answer.is_some() {
        ret.extend(&mut generate_test(&attr, &item.sig.ident).into_iter());
    }

    ret.extend(TokenStream::from(item.into_token_stream()).into_iter());

    ret
}
