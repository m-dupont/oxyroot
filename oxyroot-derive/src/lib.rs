use proc_macro2::{Ident, TokenStream};

use quote::{format_ident, quote, quote_spanned, ToTokens};

use oxyroot;

use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Fields, GenericParam, Generics};
use syn::{Data, DeriveInput};

#[proc_macro_derive(FromRootTree)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // eprintln!("ast: {:#?}", input);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let func = write_func(&input.data);
    let stru = write_struct(&input.data);
    let next = write_next(&input.data);

    let iterator_name = format_ident!("{name}Iterator");

    let expanded = quote!(
        impl #impl_generics #name #ty_generics #where_clause {
            fn from_tree<'a>(tree: &'a oxyroot::ReaderTree) -> impl Iterator<Item = #name> +'a {
                struct #iterator_name<'a>  {
                   #stru
                }

                impl<'a> #iterator_name<'a> {
                    fn new(tree: &'a oxyroot::ReaderTree) -> Self {
                        #func
                    }
                }

                impl Iterator for #iterator_name<'_> {
                    type Item = #name;
                    fn next(&mut self) -> Option<Self::Item> {
                        Some(#name { #next })
                }
            }
                #iterator_name::new(tree)
            }
        }

    );

    expanded.into()
}

// Add a bound `T: Marshaler` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(oxyroot::Marshaler));
        }
    }
    generics
}

fn write_struct(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote_spanned! {
                        f.span() => #field_name: Box<dyn Iterator<Item=#field_type> + 'a>,
                    }
                });
                quote!(#(#recurse)*)
            }
            Fields::Unnamed(_) => {
                unimplemented!("Unnamed")
            }
            Fields::Unit => {
                unimplemented!("Unit")
            }
        },
        Data::Enum(_) => {
            unimplemented!("Enum")
        }
        Data::Union(_) => {
            unimplemented!("Union")
        }
    }
}

fn write_func(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote_spanned! {
                        f.span() => #field_name:Box::new(tree.branch(stringify!(#field_name)).unwrap().as_iter::<#field_type>().expect("wrong type")),
                    }
                });
                quote!(  Self{  #(#recurse)* })
            }
            Fields::Unnamed(_) => {
                unimplemented!("Unnamed")
            }
            Fields::Unit => {
                unimplemented!("Unit")
            }
        },
        Data::Enum(_) => {
            unimplemented!("Enum")
        }
        Data::Union(_) => {
            unimplemented!("Union")
        }
    }
}

fn write_next(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote_spanned! {
                        f.span() => #field_name: self.#field_name.next()?,
                    }
                });
                quote!(  #(#recurse)* )
            }
            Fields::Unnamed(_) => {
                unimplemented!("Unnamed")
            }
            Fields::Unit => {
                unimplemented!("Unit")
            }
        },
        Data::Enum(_) => {
            unimplemented!("Enum")
        }
        Data::Union(_) => {
            unimplemented!("Union")
        }
    }
}
