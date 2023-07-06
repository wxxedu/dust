use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Item, ItemEnum, ItemFn, ItemStruct, ItemUnion,
    Visibility,
};

#[proc_macro_attribute]
pub fn dust(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut data = parse_macro_input!(input as Item);
    match data {
        Item::Enum(ref mut data) => dust_enum(data),
        Item::Fn(ref mut data) => dust_fn(data),
        Item::Struct(ref mut data) => dust_struct(data),
        Item::Union(ref mut data) => dust_union(data),
        _ => panic!(),
    }
    .into()
}

fn dust_enum(data: &mut ItemEnum) -> TokenStream {
    data.vis = Visibility::Inherited;
    quote! {
        #[repr(C)]
        pub #data
    }
}

fn dust_struct(data: &mut ItemStruct) -> TokenStream {
    data.vis = Visibility::Inherited;
    quote! {
        #[repr(C)]
        pub #data
    }
}

fn dust_fn(data: &mut ItemFn) -> TokenStream {
    data.vis = Visibility::Inherited;
    quote! {
        #[no_mangle]
        pub extern "C" #data
    }
}

fn dust_union(data: &mut ItemUnion) -> TokenStream {
    data.vis = Visibility::Inherited;
    quote! {
        #[repr(C)]
        pub #data
    }
}
