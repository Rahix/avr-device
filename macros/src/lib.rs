// Adapted from https://github.com/rust-embedded/cortex-m-rt/blob/master/macros/src/lib.rs

extern crate proc_macro;

mod vector;

use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn entry(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut f = syn::parse_macro_input!(input as syn::ItemFn);

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.vis == syn::Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            syn::ReturnType::Default => false,
            syn::ReturnType::Type(_, ref ty) => matches!(**ty, syn::Type::Never(_)),
        };

    if !valid_signature {
        return syn::parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            "This attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let (statics, stmts) = match extract_static_muts(f.block.stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    // Rename the function so it is not callable
    f.sig.ident = syn::Ident::new(
        &format!("__avr_device_rt_{}", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;

        // Note that we use an explicit `'static` lifetime for the entry point arguments. This makes
        // it more flexible, and is sound here, since the entry will not be called again, ever.
        syn::parse::<syn::FnArg>(
            quote::quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &'static mut #ty).into(),
        )
        .unwrap()
    }));
    f.block.stmts = stmts;

    let tramp_ident = syn::Ident::new(
        &format!("{}_trampoline", f.sig.ident),
        proc_macro2::Span::call_site(),
    );
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote::quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    &mut #ident
                }
            }
        })
        .collect::<Vec<_>>();

    quote::quote! (
        #[cfg(not(any(doc, target_arch = "avr")))]
        compile_error!(
            "Ensure that you are using an AVR target! You may need to change \
       directories or pass a --target flag to cargo. See
       https://github.com/Rahix/avr-device/pull/41 for more details."
        );

        #[doc(hidden)]
        #[export_name = "main"]
        pub unsafe extern "C" fn #tramp_ident() {
            #ident(
                #(#resource_args),*
            )
        }

        #[doc(hidden)]
        #f
    )
    .into()
}

#[proc_macro_attribute]
pub fn interrupt(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut f: syn::ItemFn = syn::parse(input).expect("`#[interrupt]` must be applied to a function");
    let args: Vec<_> = args.into_iter().collect();

    let fspan = f.span();
    let ident = f.sig.ident.clone();
    let ident_s = ident.to_string();

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

    let (statics, stmts) = match extract_static_muts(f.block.stmts.iter().cloned()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    f.sig.ident = syn::Ident::new(&format!("__avr_device_rt_{}", f.sig.ident), proc_macro2::Span::call_site());
    f.sig.inputs.extend(statics.iter().map(|statik| {
        let ident = &statik.ident;
        let ty = &statik.ty;
        let attrs = &statik.attrs;
        syn::parse::<syn::FnArg>(quote::quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &mut #ty).into())
            .unwrap()
    }));
    f.block.stmts = stmts;

    let tramp_ident = syn::Ident::new(&format!("{}_trampoline", f.sig.ident), proc_macro2::Span::call_site());
    let ident = &f.sig.ident;

    let resource_args = statics
        .iter()
        .map(|statik| {
            let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
            let ident = &statik.ident;
            let ty = &statik.ty;
            let expr = &statik.expr;
            quote::quote! {
                #(#cfgs)*
                {
                    #(#attrs)*
                    static mut #ident: #ty = #expr;
                    &mut #ident
                }
            }
        })
        .collect::<Vec<_>>();

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
    let vector_ident_s = vector_ident.to_string();

    quote::quote! (
        #[doc(hidden)]
        #[export_name = #vector_ident_s]
        pub unsafe extern "avr-interrupt" fn #tramp_ident() {
            #ident(
                #(#resource_args),*
            )
        }

        #[doc(hidden)]
        #f
    )
    .into()
}

/// Extracts `static mut` vars from the beginning of the given statements
fn extract_static_muts(
    stmts: impl IntoIterator<Item = syn::Stmt>,
) -> Result<(Vec<syn::ItemStatic>, Vec<syn::Stmt>), syn::parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut seen = std::collections::HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];
    while let Some(stmt) = istmts.next() {
        match stmt {
            syn::Stmt::Item(syn::Item::Static(var)) => {
                if var.mutability.is_some() {
                    if seen.contains(&var.ident) {
                        return Err(syn::parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }

                    seen.insert(var.ident.clone());
                    statics.push(var);
                } else {
                    stmts.push(syn::Stmt::Item(syn::Item::Static(var)));
                }
            }
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);

    Ok((statics, stmts))
}

fn extract_cfgs(attrs: Vec<syn::Attribute>) -> (Vec<syn::Attribute>, Vec<syn::Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &syn::Attribute, name: &str) -> bool {
    attr.style == syn::AttrStyle::Outer && attr.path.is_ident(name)
}
