use quote::quote;
use proc_macro::TokenStream;
use syn::{Error, FnArg, ItemFn, PatType};

#[proc_macro_attribute]
pub fn space(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse ast from token stream
    let ast = syn::parse::<ItemFn>(input).expect("Place this attribute above a function");

    // Signature
    let name = ast.sig.ident;
    let input = match ast.sig.inputs.first() {
        Some(FnArg::Typed(PatType { pat, ty, .. })) => quote! {
            let #pat = ::space_lib::common::deserialize::<#ty>(__input_bytes).unwrap();
        },
        _ => Error::new(name.span(), "expected one argument").to_compile_error(),
    };

    // Body
    let body = match ast.block.stmts.as_slice() {
        [head @ .., tail] => {
            let head = head.iter();
            quote! {
                #(#head);*

                let output = #tail;

                // Serialize output
                let __output_bytes = ::space_lib::common::serialize(&output).unwrap().leak();
                ::std::boxed::Box::new(::space_lib::SpaceSlice {
                    len: __output_bytes.len(),
                    ptr: __output_bytes.as_mut_ptr(),
                })
            }
        }
        _ => Error::new(name.span(), "body is empty").to_compile_error(),
    };

    quote! {
        #[no_mangle]
        fn #name(ptr: usize) -> ::std::boxed::Box<::space_lib::SpaceSlice> {
            // Deserialize input
            let __input_bytes = unsafe {
                let len = *(ptr as *const usize);
                let data = (ptr + 4) as *mut u8;
                ::std::slice::from_raw_parts(data, len)
            };

            #input

            #body
        }
    }.into()
}
