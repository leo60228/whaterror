use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::Span;
use proc_macro_error::*;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    Expr, Ident, ItemFn, ReturnType,
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

    let attr = parse_macro_input!(attr as MacroArgs);
    let inner_main = parse_macro_input!(item as ItemFn);

    let expr = attr.expr;

    if let Some(constness) = &inner_main.sig.constness {
        emit_error!(constness.span(), "const fns are not supported");
    }

    if let Some(asyncness) = &inner_main.sig.asyncness {
        emit_error!(asyncness.span(), "async fns are not supported");
    }

    if !inner_main.sig.generics.params.is_empty() || inner_main.sig.generics.where_clause.is_some()
    {
        emit_error!(
            inner_main.sig.generics.span(),
            "generic functions are not supported"
        );
    }

    if !inner_main.sig.inputs.is_empty() {
        emit_error!(inner_main.sig.inputs.span(), "arguments are not supported");
    }

    if let Some(variadic) = &inner_main.sig.variadic {
        emit_error!(variadic.span(), "variadic functions are not supported");
    }

    if !inner_main.attrs.is_empty() {
        emit_warning!(
            try_fold1(inner_main.attrs.iter().map(Spanned::span), |a, b| a.join(b))
                .unwrap_or_else(Span::call_site),
            "attributes may have unexpected behavior"
        );
    }

    let whaterror_str = proc_macro_crate::crate_name("whaterror")
        .map_err(|x| Diagnostic::new(Level::Error, x))
        .unwrap_or_abort();

    let whaterror = Ident::new(&whaterror_str, Span::call_site());

    abort_if_dirty();

    let ident = &inner_main.sig.ident;

    let mut outer_main = inner_main.clone();
    outer_main.sig.output = ReturnType::Default;
    outer_main.block = Box::new(parse_quote! {{
        extern crate #whaterror as whaterror;

        #inner_main

        let ret = #ident();
        if <_ as whaterror::Termination<_>>::failed(&ret) {
            <_ as whaterror::Termination<_>>::handle(ret, #expr);
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
