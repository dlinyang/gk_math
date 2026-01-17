use proc_macro::*;
use quote::quote;
use syn::*;

pub fn vector_math_impl(input: TokenStream) -> TokenStream {
    println!("{}", input);

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields)=> &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "VectorMath only use to struct (e.g struct Vec3{ x:f32, y:f32, z:f32})"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                name,
                "VectorMath only for struct"
            )
            .to_compile_error()
            .into();
        }
    };
    
    let field_names: Vec<_> = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    let field_count = field_types.len();
    let field_index = (0..field_count).collect::<Vec<usize>>();

    if field_types.len() == 0 {
        return syn::Error::new_spanned(
            name,
            format!("no field in structure: {}", name)
        )
        .to_compile_error()
        .into();
    }
    
    let elem_ty: &syn::Type = field_types[0];

    // 检查字段类型是否为基本类型
    for field_type in &field_types {
        let type_str = quote!(#field_type).to_string();
        let primitive_types = [
            "i8", "i16", "i32", "i64", "i128", "isize",
            "u8", "u16", "u32", "u64", "u128", "usize",
            "f32", "f64",
        ];
        
        if !primitive_types.contains(&type_str.as_str()) {
            return syn::Error::new_spanned(
                field_type,
                format!("field must be primitive type, except: {}", type_str)
            )
            .to_compile_error()
            .into();
        }

        
        if type_str != quote!(#elem_ty).to_string() {
            return  syn::Error::new_spanned(
                field_type, 
                format!("type must be same type {}", type_str)
            )
            .to_compile_error()
            .into();
        }
    }

    let cross_impl = if field_count == 3 {
        quote!{
            impl #name {        
                /// 三维向量的叉积   
                #[inline]
                pub fn cross(a: &Self, b: &Self) -> Self {
                    #name::new(
                        a[1] * b[2] - a[2] * b[1],
                        a[2] * b[0] - a[0] * b[2],
                        a[0] * b[1] - a[1] * b[0],
                    )
                }
            }
        }
    }  else {
        quote!()
    };

    // 生成代码
    let expanded = quote! {
        impl #name {
            #[inline]
            pub fn new(#(#field_names : #field_types),*) -> Self {
                Self {
                    #(#field_names: #field_names),*
                }
            }

            /// 计算向量的点积
            #[inline]
            pub fn dot(&self, other: &Self) -> #elem_ty {
                #(self.#field_names * other.#field_names) + *
            }
            
            /// 计算向量的长度平方（避免开方运算）
            #[inline]
            pub fn length_squared(&self) -> #elem_ty {
                self.dot(self)
            }
            
            /// 计算向量的长度
            #[inline]
            pub fn length(&self) -> #elem_ty {
                self.length_squared().sqrt()
            }
            
            /// 返回归一化（单位长度）的向量
            #[inline]
            pub fn normalized(&self) -> Self {
                Self {
                    #(#field_names: self.#field_names / self.length(),)*
                }
            }
            
            /// 向量是否为零向量
            #[inline]
            pub fn is_zero(&self) -> bool {
                #(self.#field_names == <#elem_ty as Default>::default())&&*
            }
            
            /// 获取最小元素值
            pub fn min_element(&self) -> #elem_ty {
                let mut min = self[0];
                #(
                    if self.#field_names < min {
                        min = self.#field_names;
                    }
                )*
                min
            }
            
            /// 获取最大元素值
            pub fn max_element(&self) -> #elem_ty {
                let mut max = self[0];
                #(
                    if self.#field_names > max {
                        max = self.#field_names;
                    }
                )*
                max
            }
            
            /// 对向量的每个元素应用函数
            pub fn map<F>(self, f: F) -> #name
            where
                F: Fn(#elem_ty) -> #elem_ty
            {
                #name{
                    #(#field_names: f(self.#field_names)),*
                }
            }
        }

        #cross_impl

        impl From<[#elem_ty;#field_count]> for #name {
            fn from(item: [#elem_ty;#field_count]) -> Self {
                Self {
                    #(#field_names: item[#field_index]),*
                }
            }
        }

        // 实现加法
        impl std::ops::Add for #name {
            type Output = Self;
            
            fn add(self, rhs: Self) -> Self::Output {
                #name::new(
                    #(self.#field_names + rhs.#field_names),*
                )
            }
        }
        
        // 实现加法赋值
        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                #(self.#field_names += rhs.#field_names;)*
            }
        }
        
        // 实现减法
        impl std::ops::Sub for #name {
            type Output = Self;
            
            fn sub(self, rhs: Self) -> Self::Output {
                #name::new(
                    #(self.#field_names - rhs.#field_names),*
                )
            }
        }
        
        // 实现减法赋值
        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                #(self.#field_names -= rhs.#field_names;)*
            }
        }
        
        // 实现向量取反
        impl std::ops::Neg for #name {
            type Output = Self;
            
            fn neg(self) -> Self::Output {
                #name::new(
                    #(-self.#field_names),*
                )
            }
        }
        
        // 实现标量乘法（右侧为标量）
        impl std::ops::Mul<#elem_ty> for #name {
            type Output = Self;
            
            fn mul(self, rhs: #elem_ty) -> Self::Output {
                #name::new(
                    #(self.#field_names * rhs),*
                )
            }
        }
        
        // 实现标量乘法赋值
        impl std::ops::MulAssign<#elem_ty> for #name {
            fn mul_assign(&mut self, rhs: #elem_ty) {
                #(self.#field_names *= rhs;)*
            }
        }
        
        // 实现标量乘法（左侧为标量）
        impl std::ops::Mul<#name> for #elem_ty {
            type Output = #name;
            
            fn mul(self, rhs: #name) -> Self::Output {
                #name::new(
                    #(self * rhs.#field_names),*
                )
            }
        }
        
        // 实现标量除法
        impl std::ops::Div<#elem_ty> for #name {
            type Output = Self;
            
            fn div(self, rhs: #elem_ty) -> Self::Output {
                #name::new(
                    #(self.#field_names / rhs),*
                )
            }
        }
        
        // 实现标量除法赋值
        impl std::ops::DivAssign<#elem_ty> for #name {
            fn div_assign(&mut self, rhs: #elem_ty) {
                #(self.#field_names /= rhs;)*
            }
        }
        
        // 实现相等比较
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                #(self.#field_names == other.#field_names)&&*
            }
        }
        
        // 实现大小比较（逐元素比较）
        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                // 逐元素比较，如果所有元素都相等返回 Equal
                // 如果所有对应元素都满足 <= 或 >=，返回相应的 Ordering
                // 否则返回 None（不可比较）
                
                let mut result = std::cmp::Ordering::Equal;
                
                #(
                    match self.#field_names.partial_cmp(&other.#field_names) {
                        Some(std::cmp::Ordering::Less) => {
                            if result == std::cmp::Ordering::Greater {
                                return None;
                            }
                            result = std::cmp::Ordering::Less;
                        },
                        Some(std::cmp::Ordering::Greater) => {
                            if result == std::cmp::Ordering::Less {
                                return None;
                            }
                            result = std::cmp::Ordering::Greater;
                        },
                        Some(std::cmp::Ordering::Equal) => {},
                        None => return None,
                    }
                )*
                
                Some(result)
            }
            
            fn lt(&self, other: &Self) -> bool {
                #(self.#field_names < other.#field_names)&&*
            }
            
            fn le(&self, other: &Self) -> bool {
                #(self.#field_names <= other.#field_names)&&*
            }
            
            fn gt(&self, other: &Self) -> bool {
                #(self.#field_names > other.#field_names)&&*
            }
            
            fn ge(&self, other: &Self) -> bool {
                #(self.#field_names >= other.#field_names)&&*
            }
        }
        
        // 实现调试输出
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} [", stringify!(#name))?;
                #(
                    write!(f, "{:?} ", self.#field_names)?;
                )*
                write!(f, "]")
            }
        }
        
        // 实现显示输出
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")?;
                #(
                    write!(f, "{} ", self.#field_names)?;
                )*
                write!(f, "]")
            }
        }
        
        // 实现索引访问
        impl std::ops::Index<usize> for #name {
            type Output = #elem_ty;
            
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #(
                        #field_index => &self.#field_names,
                    )*
                    _ => panic!("Index {} out of bounds for {}", index, stringify!(#name)),
                }
            }
        }
        
        impl std::ops::IndexMut<usize> for #name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    #(
                        #field_index => &mut self.#field_names,
                    )*
                    _ => panic!("Index {} out of bounds for {}", index, stringify!(#name)),
                }
            }
        }
    };
     
    TokenStream::from(expanded)
}