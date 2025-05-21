use quote::quote;
use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(building))]
struct BuildingStructAttributes {
    faction: String,
    building_name: String,
}

fn building_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // Parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Extract struct attributes
    let BuildingStructAttributes {
        faction,
        building_name,
    } = deluxe::extract_attributes(&mut ast)?;

    // Define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let config_name = format!("{}{}Config", faction, building_name);

    // Generate
    Ok(quote! {
        struct #config_name {}

        impl #impl_generics Building for #ident #type_generics #where_clause {
            fn build_unit(&self) -> &'static str{
                println!("{} {}", #faction, #building_name);

                #faction
            }
        }
    })
}

#[proc_macro_derive(Building, attributes(building))]
pub fn building_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    building_derive_macro2(item.into()).unwrap().into()
}
