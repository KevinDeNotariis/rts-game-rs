use quote::quote;
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(spawnable_unit))]
struct SpawnableUnitStructAttributes {
    unit_type: String,
}

fn spawnable_unit_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // Parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Extract struct attributes
    let SpawnableUnitStructAttributes { unit_type } = deluxe::extract_attributes(&mut ast)?;

    // Define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // Generate
    Ok(quote! {
        impl #impl_generics SpawnableUnit for #ident #type_generics #where_clause {
            fn build_unit(&self) -> &'static str{
                println!("{}", #unit_type);
                #unit_type
            }
        }
    })
}

#[proc_macro_derive(SpawnableUnit, attributes(spawnable_unit))]
pub fn spawnable_unit_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    spawnable_unit_derive_macro2(item.into()).unwrap().into()
}
