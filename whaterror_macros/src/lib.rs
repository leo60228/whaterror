use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::*;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_quote,
    spanned::Spanned,
    Expr, ExprBlock, ExprClosure, Ident, ItemFn, ReturnType, TypeTuple,
};

fn try_fold1<I, F>(mut iter: I, f: F) -> Option<I::Item>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item) -> Option<I::Item>,
{
    let elem = iter.next()?;
    iter.try_fold(elem, f)
}

struct MacroArgs {
    expr: Expr,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = Expr::parse(input)?;

        Ok(Self { expr })
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn whaterror(attr: ProcTokenStream, item: ProcTokenStream) -> ProcTokenStream {
    dummy::set_dummy(item.clone().into());

    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    let args = if attr.is_empty() {
        emit_call_site_error!("missing arguments");
        None
    } else {
        let span = SpanRange::from_tokens(&attr);

        match syn::parse2::<MacroArgs>(attr) {
            Ok(args) => Some(args),
            Err(err) => {
                emit_error!(span, "{}", err);
                None
            }
        }
    };

    let inner_main_fn = syn::parse2::<ItemFn>(item).unwrap_or_abort();

    if let Some(constness) = &inner_main_fn.sig.constness {
        emit_error!(constness.span(), "const fns are not supported");
    }

    if let Some(asyncness) = &inner_main_fn.sig.asyncness {
        emit_error!(asyncness.span(), "async fns are not supported");
    }

    if !inner_main_fn.sig.generics.params.is_empty()
        || inner_main_fn.sig.generics.where_clause.is_some()
    {
        emit_error!(
            inner_main_fn.sig.generics.span(),
            "generic functions are not supported"
        );
    }

    if !inner_main_fn.sig.inputs.is_empty() {
        emit_error!(
            inner_main_fn.sig.inputs.span(),
            "arguments are not supported"
        );
    }

    if !inner_main_fn.attrs.is_empty() {
        emit_warning!(
            try_fold1(inner_main_fn.attrs.iter().map(Spanned::span), |a, b| a
                .join(b))
            .unwrap_or_else(Span::call_site),
            "attributes may have unexpected behavior"
        );
    }

    abort_if_dirty();

    let args = args.unwrap();
    let expr = args.expr;

    let mut outer_main = inner_main_fn.clone();

    let inner_main = ExprClosure {
        attrs: vec![],
        asyncness: None,
        movability: None,
        capture: Some(Default::default()),
        or1_token: Default::default(),
        inputs: Default::default(),
        or2_token: Default::default(),
        output: match inner_main_fn.sig.output {
            ReturnType::Default => ReturnType::Type(
                Default::default(),
                Box::new(
                    TypeTuple {
                        paren_token: Default::default(),
                        elems: Default::default(),
                    }
                    .into(),
                ),
            ),
            x @ ReturnType::Type(_, _) => x,
        },
        body: Box::new(
            ExprBlock {
                attrs: vec![],
                label: None,
                block: *inner_main_fn.block,
            }
            .into(),
        ),
    };

    let inner = Ident::new("inner", Span::mixed_site());
    let thunk = Ident::new("thunk", Span::mixed_site());

    outer_main.sig.output = ReturnType::Default;
    outer_main.block = Box::new(parse_quote! {{
        let #inner = #inner_main;
        let #thunk = || #expr;

        {
            use ::whaterror::{Termination, FatalError, terminate};

            // help out type inference
            fn handle<R: Termination<H>, H>(result: R, handler: fn() -> H) {
                if let Err(e) = result.into_result() {
                    e.handle(handler());
                    terminate(cfg!(test));
                }
            }

            handle(#inner(), #thunk);
        }
    }});

    outer_main.into_token_stream().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
