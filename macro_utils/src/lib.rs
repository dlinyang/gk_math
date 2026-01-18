mod vector;
use vector::vector_math_impl;

mod tool;
use tool::mat_vec_mul_impl;

use proc_macro::*;


#[proc_macro_derive(VectorMath)]
pub fn vector_vector_drive(input: TokenStream) -> TokenStream {
    vector_math_impl(input)
}

#[proc_macro_attribute]
pub fn mat_vec_mul(input: TokenStream, item: TokenStream) -> TokenStream {
    mat_vec_mul_impl(input, item)
}