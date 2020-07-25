extern crate proc_macro;

mod vector;

use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn interrupt(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Adapted from https://github.com/rust-embedded/cortex-m-rt/blob/master/macros/src/lib.rs
    let f = syn::parse_macro_input!(input as syn::ItemFn);
    let args: Vec<_> = args.into_iter().collect();

    let fspan = f.span();
    let ident = f.sig.ident;
    let ident_s = ident.to_string();
    let attrs = f.attrs;
    let block = f.block;
    let stmts = block.stmts;
    let unsafety = f.sig.unsafety;

    let chip = if let Some(tree) = args.get(0) {
        if let proc_macro::TokenTree::Ident(ident) = tree {
            ident.to_string()
        } else {
            return syn::parse::Error::new(
                proc_macro2::Span::call_site(),
                "#[interrupt(chip)]: chip must be an ident",
            )
            .to_compile_error()
            .into();
        }
    } else {
        return syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            "#[interrupt(chip)] needs a chip argument",
        )
        .to_compile_error()
        .into();
    };

    let valid_signature = f.sig.constness.is_none()
        && f.vis == syn::Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            syn::ReturnType::Default => true,
            syn::ReturnType::Type(_, ref ty) => match **ty {
                syn::Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                syn::Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return syn::parse::Error::new(
            fspan,
            "`#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let vect = if let Some(v) = vector::lookup_vector(&chip, &ident_s) {
        v
    } else {
        return syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            &format!("Chip `{}` or interrupt `{}` unknown", chip, ident_s),
        )
        .to_compile_error()
        .into();
    };
    let vector = format!("__vector_{}", vect);
    let vector_ident = syn::Ident::new(&vector, proc_macro2::Span::call_site());

    quote::quote! (
        #[no_mangle]
        pub unsafe extern "avr-interrupt" fn #vector_ident() {
            #ident();
        }

        #(#attrs)*
        #[allow(non_snake_case)]
        #unsafety fn #ident() {
            #(#stmts)*
        }
    )
    .into()
}
