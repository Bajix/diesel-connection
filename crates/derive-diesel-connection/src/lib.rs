use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PoolContext)]
pub fn derive_pool_context(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let expanded = quote! {
    impl diesel_connection::PoolContext for #ident {
      fn pool() -> &'static diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Self::Connection>> {
        #[diesel_connection::static_init::dynamic(0)]

        static POOL: diesel::r2d2::Pool<
          diesel::r2d2::ConnectionManager<<#ident as ConnectionInfo>::Connection>
        > = <#ident as diesel_connection::ConnectionInfo>::create_pool().expect("Invalid database url");

        unsafe { &POOL }
      }
    }
  };

  expanded.into()
}
