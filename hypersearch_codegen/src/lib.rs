use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn hyperparameters(_header: TokenStream, input_struct: TokenStream) -> TokenStream {
    let orig_struct = parse_macro_input!(input_struct as ItemStruct);
    let field_names = orig_struct.fields.iter().map(|field| field.ident.as_ref().expect("Tuple structs are not allowed."));
    let field_names1 = orig_struct.fields.iter().map(|field| field.ident.as_ref().expect("Tuple structs are not allowed."));
    let field_names3 = orig_struct.fields.iter().map(|field| field.ident.as_ref().expect("Tuple structs are not allowed."));
    let field_names4 = orig_struct.fields.iter().map(|field| field.ident.as_ref().expect("Tuple structs are not allowed."));
    let field_names5 = orig_struct.fields.iter().map(|field| field.ident.as_ref().expect("Tuple structs are not allowed."));
    let num_fields = field_names4.count();
    let field_types = orig_struct.fields.iter().map(|field| &field.ty);
    let field_types1 = orig_struct.fields.iter().map(|field| {
        let type_string = field.ty.to_token_stream().to_string();
        assert!(type_string.len() > 5, "Only Vec hyperparams are supported currently");
        assert_eq!(type_string[..5], *"Vec <", "Only Vec hyperparams are supported currently");
        let type_string = proc_macro2::TokenStream::from_str(&type_string[5..type_string.len()-1].to_owned().trim().to_string().replace('"', "")).unwrap();
        quote!{#type_string}
    });


    let indexes = 0..num_fields;
    //let ref_types: Vec<proc_macro2::TokenStream> = field_names3.zip(field_types).map(|(ident, ty)| quote!{#ident: &'a #ty}).collect();

    let orig_struct_ident = orig_struct.ident.clone();
    let permutation_struct_ident = Ident::new(&format!("{}Permutations", orig_struct_ident), Span::call_site());

    TokenStream::from(quote!{
        // Original struct
        #orig_struct

        // Impl for original struct
        impl <'a>#orig_struct_ident {
            pub fn permutations(&'a self) -> #permutation_struct_ident <'a> {
                #permutation_struct_ident {
                    #(#field_names : &self.#field_names),*,

                    indexes: [0; #num_fields],
                    lens: [#(self.#field_names1.len()),*],
                    first: true,
                }
            }
        }

        // Permutation struct
        pub struct #permutation_struct_ident<'a> {
            #(
                #field_names3: &'a #field_types
            ),*,

            indexes: [usize; #num_fields],
            lens: [usize; #num_fields],
            first: bool,
        }

        // Iterator for permutation struct
        impl <'a>Iterator for #permutation_struct_ident<'a> {
            type Item = (
                #(#field_types1),*
            );

            fn next(&mut self) -> Option<Self::Item> {
                if !self.first {
                    // Iterate indexes
                    for (index, i) in self.indexes.iter_mut().enumerate().rev() {
                        if *i < self.lens[index] - 1 {
                            *i += 1;
                            break
                        } else {
                            if index == 0 {
                                self.indexes = [0; #num_fields];
                                return None;
                            }
                            *i = 0;
                        }
                    }
                } else {
                    self.first = false;
                }
                
                // Return values
                Some(
                    (#(
                        self . #field_names5 [ self.indexes[ #indexes ]]
                    ),*)
                )
            }
        }
    })
}