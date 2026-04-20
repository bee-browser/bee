#[proc_macro_derive(Trace)]
pub fn derive_trace(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    do_derive_trace(input).into()
}

fn do_derive_trace(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ty_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = trace_fields(&input.data);

    if body.is_empty() {
        quote::quote! {
            impl #impl_generics jsgc::Trace for #ty_name #ty_generics #where_clause {
                #[inline]
                fn trace(&self, _visits: &mut jsgc::VisitList) {}
            }
        }
    } else {
        quote::quote! {
            impl #impl_generics jsgc::Trace for #ty_name #ty_generics #where_clause {
                #[inline]
                fn trace(&self, visits: &mut jsgc::VisitList) {
                    #body
                }
            }
        }
    }
}

fn trace_fields(data: &syn::Data) -> proc_macro2::TokenStream {
    match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote::quote! {
                        self.#name.trace(visits);
                    }
                });
                quote::quote! {
                    #(#recurse)*
                }
            }
            syn::Fields::Unnamed(_) => unimplemented!("unnamed fields are not supported yet"),
            syn::Fields::Unit => quote::quote!(),
        },
        syn::Data::Enum(_) => unimplemented!("enums are not supported yet"),
        syn::Data::Union(_) => unreachable!("unions are not sypported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct Simple {
                string: Handle<String>,
                object: HandleMut<Object>,
            }
        };

        let actual = do_derive_trace(input);

        let expected = quote::quote! {
            impl jsgc::Trace for Simple {
                #[inline]
                fn trace(&self, visits: &mut jsgc::VisitList) {
                    self.string.trace(visits);
                    self.object.trace(visits);
                }
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_unit() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct Unit;
        };

        let actual = do_derive_trace(input);

        let expected = quote::quote! {
            impl jsgc::Trace for Unit {
                #[inline]
                fn trace(&self, _visits: &mut jsgc::VisitList) {}
            }
        };

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
