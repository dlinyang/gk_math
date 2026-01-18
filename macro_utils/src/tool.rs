use proc_macro::*;
use quote::quote;
use syn::*;

pub fn mat_vec_mul_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("impl Mul<{}> for {}", attr, item);
    let vec_ty = parse_macro_input!(attr as syn::Ident);

    let item = parse_macro_input!(item as ItemType);

    let mat_ty = &item.ident;

    println!("{}", quote!{ #vec_ty, #mat_ty}.to_string());

    let generics = extract_mat_generics(&item.ty)
        .expect("Expected Mat<T, ROWS, COLS> type");
    
    let (_elem_ty, _rows, cols) = generics;

    let cols: usize = quote!{#cols}.to_string().parse().unwrap();

    let index = (0..cols).collect::<Vec<usize>>();

    let mut vec_ty_mul = Vec::new();
    vec_ty_mul.resize(cols, &vec_ty);

    let expanded = quote! {
        #item

        impl std::ops::Mul<#vec_ty> for #mat_ty {
            type Output = #vec_ty;
            fn mul(self, rhs: #vec_ty) -> Self::Output {
                #vec_ty::new(
                    #(#vec_ty_mul::from(self[#index]).dot(&rhs)),*
                )
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_mat_generics(ty: &Type) -> Option<(syn::Type, syn::Expr, syn::Expr)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Mat" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    let mut args_iter = args.args.iter();
                    
                    let elem_type = match args_iter.next()? {
                        GenericArgument::Type(ty) => ty.clone(),
                        _ => return None,
                    };
                    
                    let rows = match args_iter.next()? {
                        GenericArgument::Const(expr) => expr.clone(),
                        _ => return None,
                    };
                    
                    let cols = match args_iter.next()? {
                        GenericArgument::Const(expr) => expr.clone(),
                        _ => return None,
                    };
                    
                    return Some((elem_type, rows, cols));
                }
            }
        }
    }
    None
}