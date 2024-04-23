use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
use std::collections::HashMap;

use quote::{format_ident, quote, quote_spanned};

// use oxyroot;

use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Fields, GenericParam, Generics};
use syn::{Data, DeriveInput};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(oxyroot))]
struct GOpts {
    // The struct ident.
    // ident: syn::Ident,
    // rename: Option<String>,
    branch_prefix: Option<String>,

    // Receives the body of the struct or enum. We don't care about
    // struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), FOpts>,
    // The type's generics. You'll need these any time your trait is expected
    // to work with types that declare generics.
    // generics: syn::Generics,
}

#[derive(FromField, Default, Debug)]
#[darling(default, attributes(oxyroot))]
struct FOpts {
    rename: Option<String>,
    branch_prefix: Option<String>,
    absolute_name: Option<String>,

    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,
}

#[derive(Default, Debug)]
struct OptionByField {
    renames: HashMap<Ident, String>,
    local_branch_prefix: HashMap<Ident, String>,
    global_branch_prefix: HashMap<Ident, String>,
    absolute_names: HashMap<Ident, String>,
}

///
/// Derive macro in order to read struct data from TTree. Branch names and types  are deduced from fields.
/// ## Basic usage
/// ```no_run
/// use oxyroot::{ReadFromTree, RootFile};
///
/// #[derive(ReadFromTree)]
/// struct MyStruct {
///     a: i32,     // will be read from branch "a" as 32 bits integer
///     s: String,  // will be read from branch "s" as String
/// }
/// let tree = RootFile::open("in.root").unwrap().get_tree("tree").unwrap();
/// MyStruct::from_tree(&tree).unwrap().map(|m: MyStruct | {  /* do something with m */ });
/// ```
///
/// ## Nested structures
/// ```no_run
/// use oxyroot::{ReadFromTree, RootFile};
///
/// #[derive(ReadFromTree)]
/// struct NestedStruct {
///     a: i32,     // will be read from branch "s.a" as 32 bits integer
///     s: String,  // will be read from branch "s.s" as String
/// }
///
/// #[derive(ReadFromTree)]
/// struct MyStruct {
///     s: NestedStruct,  // will be constructed from branch "s.a" and "s.s"
///     i: i32,           // will be read from branch "i" as 32 bits integer
/// }
/// let tree = RootFile::open("in.root").unwrap().get_tree("tree").unwrap();
/// MyStruct::from_tree(&tree).unwrap().map(|m: MyStruct | {  /* do something with m */ });
/// ```
///
///
///
///
/// ## Customisation
/// This macro can be customized with the following attributes:
/// - By using attribute `#[oxyroot(rename = "...")]`, it is possible to use different branch name:
/// ```no_run
/// use oxyroot::ReadFromTree;
///
/// #[derive(ReadFromTree)]
///  struct MyStruct {
///      #[oxyroot(rename = "b")]
///      a: i32,     // will be read from branch *"b"* as 32 bits integer
///      s: String,  // will be read from branch "s" as String
///  }
/// ```
///
/// - By using attribute `#[oxyroot(branch_prefix = "...")]` directly on the struct, it is possible to
/// globally (i.e. for all fields) prefix all branch names:
/// ```no_run
/// use oxyroot::ReadFromTree;
///
/// #[derive(ReadFromTree)]
/// #[oxyroot(branch_prefix = "branch_")]
///  struct MyStruct {
///      a: i32,     // will be read from branch *"branch_a"* as 32 bits integer
///      s: String,  // will be read from branch *"branch_s"* as String
///  }
/// ```
///
/// - By using attribute `#[oxyroot(branch_prefix = "...")]` on a field, it is possible to
/// locally prefix branch name:
/// ```no_run
/// use oxyroot::ReadFromTree;
///
/// #[derive(ReadFromTree)]
///  struct MyStruct {
///      a: i32,     // will be read from branch *"a"* as 32 bits integer
///     #[oxyroot(branch_prefix = "branch_")]
///      s: String,  // will be read from branch *"branch_s"* as String
///  }
/// ```
///
/// - By using attribute `#[oxyroot(absolute_name = "...")]` on a field, it is possible to
/// precisely set the branch name, even in a nested struct:
/// ```no_run
/// use oxyroot::ReadFromTree;
///
/// #[derive(ReadFromTree)]
///  struct MyStruct {
///      a: i32,     // will be read from branch *"a"* as 32 bits integer
///     #[oxyroot(absolute_name = "branch_s")]
///      s: String,  // will be read from branch *"branch_s"* as String
///  }
/// ```
/// - In combination with `#[oxyroot(branch_prefix = "...")]`:
/// ```no_run
/// use oxyroot::ReadFromTree;
///
/// #[derive(ReadFromTree)]
/// #[oxyroot(branch_prefix = "branch_")]
/// struct MyStruct {
///    a: i32,     // will be read from branch *"branch_a"* as 32 bits integer
///    c: f64,     // will be read from branch *"branch_c"* as 64 bits float
///   #[oxyroot(absolute_name = "zweig_s")]
///   s: String,  // will be read from branch *"zweig_s"* as String
///
/// }
///
///
#[proc_macro_derive(ReadFromTree, attributes(oxyroot))]
pub fn derive_ead_from_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // eprintln!("ast: {:#?}", input);

    let opts = GOpts::from_derive_input(&input).expect("Wrong options");
    // eprintln!("ropts: {:#?}", opts);
    let mut opts_by_fiels = OptionByField::default();

    let data = opts
        .data
        .as_ref()
        .take_struct()
        .expect("should be struct")
        .fields;
    for f in data.iter() {
        let original_name = f.ident.as_ref().expect("de");
        let final_name = match &f.rename {
            None => original_name.to_string(),

            Some(i) => i.to_string(),
        };
        opts_by_fiels
            .renames
            .insert(original_name.clone(), final_name);

        if let Some(l) = &f.branch_prefix {
            opts_by_fiels
                .local_branch_prefix
                .insert(original_name.clone(), l.to_string());
        }

        if let Some(g) = &opts.branch_prefix {
            opts_by_fiels
                .global_branch_prefix
                .insert(original_name.clone(), g.to_string());
        }

        if let Some(a) = &f.absolute_name {
            opts_by_fiels
                .absolute_names
                .insert(original_name.clone(), a.to_string());
        }
    }

    // eprintln!("opts_by_fiels: {:#?}", opts_by_fiels);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let func = write_func_for_readtotree(&input.data, &opts_by_fiels);
    let stru = write_struct_for_readtotree(&input.data);
    let next = write_next_for_readtotree(&input.data);

    let iterator_name = format_ident!("{name}Iterator");

    let expanded = quote!(

        impl<'a> #impl_generics  #ty_generics #where_clause oxyroot::ReadFromTree<'a> for #name{
            fn from_branch_tree(tree: &'a oxyroot::ReaderTree, branch_name: oxyroot::BranchName) -> oxyroot::Result<impl Iterator<Item = #name> +'a >{
                struct #iterator_name<'a>  {
                   #stru
                }

                impl<'a> #iterator_name<'a> {
                    fn new(tree: &'a oxyroot::ReaderTree, branch_name: oxyroot::BranchName) -> oxyroot::Result<Self> {
                        use oxyroot::ReadFromTree;
                        #func
                    }
                }

                impl Iterator for #iterator_name<'_> {
                    type Item = #name;
                    fn next(&mut self) -> Option<Self::Item> {
                        Some(#name { #next })
                }
            }
                Ok(#iterator_name::new(tree, branch_name)?)
            }
        }

    );

    expanded.into()
}

// Add a bound `T: Marshaler` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(oxyroot::ReadFromTree));
        }
    }
    generics
}

fn write_struct_for_readtotree(data: &Data) -> TokenStream {
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

fn write_func_for_readtotree(data: &Data, opts_by_fiels: &OptionByField) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let branch_names = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let branch_name_ident = format_ident!("bn_{field_name}");
                    let branch_name = match opts_by_fiels.renames.get(f.ident.as_ref().unwrap()) {
                        None => field_name.to_string(),
                        Some(s) => s.to_string(),
                    };

                    let mut let_b =
                        quote_spanned!(f.span() => let b = branch_name.make_child(#branch_name));

                    let global_prefix = opts_by_fiels
                        .global_branch_prefix
                        .get(f.ident.as_ref().unwrap());

                    if let Some(l) = global_prefix {
                        let_b = quote_spanned!(f.span() => #let_b.with_prefix(#l));
                    }

                    let absolute_name = opts_by_fiels.absolute_names.get(f.ident.as_ref().unwrap());

                    if let Some(a) = absolute_name {
                        let_b = quote_spanned!(f.span() => #let_b.make_absolute(#a));
                    }

                    let local_prefix = opts_by_fiels
                        .local_branch_prefix
                        .get(f.ident.as_ref().unwrap());

                    if let Some(l) = local_prefix {
                        let_b = quote_spanned!(f.span() => #let_b.with_prefix(#l));
                    }

                    quote_spanned!(f.span() => let #branch_name_ident =
                        {
                            #let_b;
                            b
                        };
                    )
                });

                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let _branch_name = match  opts_by_fiels.renames.get(f.ident.as_ref().unwrap()) {
                        None => {  field_name.to_string()}
                        Some(s) => {s.to_string()}
                    };
                    let field_type = &f.ty;


                    let branch_name_ident = format_ident!("bn_{field_name}");


                    quote_spanned! {
                        f.span() =>  #field_name:Box::new(<#field_type>::from_branch_tree(tree, #branch_name_ident.into())?)     ,
                    }
                });
                quote!(  #(#branch_names)*   Ok(Self{  #(#recurse)* }))
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

fn write_next_for_readtotree(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
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

///
/// Derive macro in order to write struct data into a TTree. Branch names and types are deduced from fields.
/// ```no_run
/// use oxyroot::{WriteToTree, RootFile};
/// #[derive(WriteToTree)]
/// struct MyStruct {
///     a: i32,     // will be read from branch "a" as 32 bits integer
///     s: String,  // will be read from branch "s" String
/// }
/// let mut f = RootFile::create("out.root").unwrap();
/// let mut tree = oxyroot::WriterTree::new("tree");
/// let it = vec![MyStruct {a: 0,s: "a".to_string()}, MyStruct {a: 1,s: "b".to_string()}].into_iter();
/// MyStruct::to_tree(it, &mut tree).unwrap();
/// tree.write(&mut f).unwrap();
/// f.close().unwrap();
/// ```
///
#[proc_macro_derive(WriteToTree)]
pub fn derive_write_to_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // eprintln!("ast: {:#?}", input);

    //let opts = GOpts::from_derive_input(&input).expect("Wrong options");
    // eprintln!("opts: {:#?}", opts);
    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let send_recv = write_send_recv_for_write_to_tree(&input.data);
    let match_it_some = write_match_it_some_for_write_to_tree(&input.data);
    let match_it_none = write_match_it_none_for_write_to_tree(&input.data);
    let new_branch = write_new_branch_for_write_to_tree(&input.data);

    let expanded = quote!(
        impl #impl_generics  #ty_generics #where_clause oxyroot::WriteToTree for #name{
            fn to_branch_tree(
                mut it: impl Iterator<Item = #name> + 'static,
                tree: &mut oxyroot::WriterTree, branch_name: Option<&str>
            ) -> oxyroot::Result<()> {

                use std::cell::RefCell;
                use std::rc::Rc;

                struct Channel<T> {
                    current: Rc<RefCell<Option<T>>>,
                }

                struct Sender<T> {
                    channel: Channel<T>,
                }

                impl<T> Sender<T> {
                    fn send(&self, value: Option<T>) {
                        let mut current = self.channel.current.borrow_mut();
                        *current = value;
                    }
                }

                struct Receiver<T> {
                    channel: Channel<T>,
                }

                pub fn make_channel<T>() -> (Sender<T>, Receiver<T>) {
                    let current = Rc::new(RefCell::new(None));
                    let channel = Channel {
                        current: current.clone(),
                    };

                    let sender = Sender { channel: channel };
                    let receiver = Receiver {
                        channel: Channel {
                            current: current.clone(),
                        },
                    };
                    (sender, receiver)
                }

                impl<T> Iterator for Receiver<T> {
                    type Item = T;

                    fn next(&mut self) -> Option<Self::Item> {
                        let mut current = self.channel.current.borrow_mut();
                        let ret = current.take();
                        ret
                    }
                }

                #send_recv

                let func = move |s: oxyroot::StateCallBack| {


                    match s {
                        oxyroot::StateCallBack::Before => {
                            match it.next() {
                                None => {
                                    #match_it_none
                                }
                                Some(struct_instance) => {
                                    #match_it_some
                                }
                            };
                        }
                        oxyroot::StateCallBack::Branch(_) => {}
                        oxyroot::StateCallBack::After => {}
                    }
                };

                #new_branch
                tree.add_callback(Box::new(func));

                Ok(())

            }

        }

    );

    expanded.into()
}

fn write_send_recv_for_write_to_tree(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let _field_type = &f.ty;

                    let sender_name = format_ident!("sender_{field_name}");
                    let recv_name = format_ident!("recv_{field_name}");

                    quote_spanned! {
                        f.span() => let (#sender_name, #recv_name) = make_channel();
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

fn write_match_it_some_for_write_to_tree(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let _field_type = &f.ty;

                    let sender_name = format_ident!("sender_{field_name}");
                    let _recv_name = format_ident!("recv_{field_name}");

                    quote_spanned! {
                        f.span() => #sender_name.send(Some(struct_instance.#field_name));
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

fn write_match_it_none_for_write_to_tree(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let _field_type = &f.ty;

                    let sender_name = format_ident!("sender_{field_name}");
                    let _recv_name = format_ident!("recv_{field_name}");

                    quote_spanned! {
                        f.span() => #sender_name.send(None);
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

fn write_new_branch_for_write_to_tree(data: &Data) -> TokenStream {
    match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;

                    let _sender_name = format_ident!("sender_{field_name}");
                    let recv_name = format_ident!("recv_{field_name}");

                    quote_spanned! {
                        f.span() => <#field_type>::to_branch_tree(#recv_name.into_iter(), tree, stringify!(#field_name).into())?;
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
