mod vector;
use vector::vector_math_impl;

use proc_macro::*;


#[proc_macro_derive(VectorMath)]
pub fn vector_vector_drive(input: TokenStream) -> TokenStream {
    vector_math_impl(input)
}