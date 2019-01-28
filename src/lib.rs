extern crate proc_macro;

#[proc_macro]
pub fn glsl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ((format!(r#""{}""#, input)).as_str()).parse().unwrap()
}
