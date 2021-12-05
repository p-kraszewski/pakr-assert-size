
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input,  ItemStruct, LitInt};

#[derive(Debug)]
struct ExpSize {
    size: usize,
}

impl Parse for ExpSize {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse::<LitInt>()?.base10_parse::<usize>()?;
        Ok(ExpSize { size: value })
    }
}

/// The attribute `#[assert_size(USIZE)]` performs **compile-time** check, if the
/// structure it is attached to has the exact size in bytes.
///
/// It uses the newly stabilized usage of `panic!` in const context to perform
/// check and early bailout.
///
/// Check does not pollute namespace, it is expanded to
/// ```
/// # struct StructName{}
/// # const ExpectedSize: usize=0;
/// #
/// const _: () = assert!(
///    std::mem::size_of::<StructName>() == ExpectedSize,
///    "size of 'StructName' is not ExpectedSize bytes"
/// );
/// ```
///
/// # Examples
///
/// Success (real size matches expected):
///
/// ```
/// # use pakr_assert_size::*;
///
/// #[repr(C, packed)]
/// #[assert_size(16)]
/// struct A {
///     field1: u64,
///     field2: u64,
/// }
/// ```
///
/// Success (real size matches expected):
///
/// ```
/// # use pakr_assert_size::*;
///
/// #[assert_size(24)]
/// #[repr(C, packed)]
/// struct B {
///     field1: u64,
///     field2: u64,
///     field3: u64,
/// }
/// ```
///
/// Failure (real size is 24 bytes, expected is 32 bytes):
/// ```compile_fail
/// # use pakr_assert_size::*;
///
/// #[assert_size(32)]
/// #[repr(C, packed)]
/// struct C {
///     field1: u64,
///     field2: u64,
///     field3: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn assert_size(attr: TokenStream, item: TokenStream) -> TokenStream {
    let size = parse_macro_input!(attr as ExpSize);
    let size = size.size;

    let struct_item = parse_macro_input!(item as ItemStruct);
    let id = struct_item.ident.clone();

    let message = format!("size of '{}' is not {} bytes", id, size);

    let checker = quote! {
        const _: () = assert!(std::mem::size_of::<#id>() == #size, #message);
        #struct_item
    };

    TokenStream::from(checker)
}

/// The attribute `#[assert_size_fits(USIZE)]` performs **compile-time** check, if the
/// structure it is attached is not bigger than given amount of bytes.
///
/// Check does not pollute namespace, it is expanded to
/// ```
/// # struct StructName{}
/// # const ExpectedSize: usize=0;
/// #
/// const _: () = assert!(
///    std::mem::size_of::<StructName>() <= ExpectedSize,
///    "'StructName' does not fit in ExpectedSize bytes"
/// );
/// ```
///
/// # Examples
///
/// Success (real size matches expected):
///
/// ```
/// # use pakr_assert_size::*;
///
/// #[repr(C, packed)]
/// #[assert_size_fits(16)]
/// struct A {
///     field1: u64,
///     field2: u64,
/// }
/// ```
///
/// Success (real size is less than expected):
///
/// ```
/// # use pakr_assert_size::*;
///
/// #[assert_size_fits(32)]
/// #[repr(C, packed)]
/// struct B {
///     field1: u64,
///     field2: u64,
///     field3: u64,
/// }
/// ```
///
/// Failure (real size is 24 bytes, exceeding maximum of 16 bytes):
/// ```compile_fail
/// # use pakr_assert_size::*;
///
/// #[assert_size_fits(16)]
/// #[repr(C, packed)]
/// struct C {
///     field1: u64,
///     field2: u64,
///     field3: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn assert_size_fits(attr: TokenStream, item: TokenStream) -> TokenStream {
    let size = parse_macro_input!(attr as ExpSize);
    let size = size.size;

    let struct_item = parse_macro_input!(item as ItemStruct);
    let id = struct_item.ident.clone();

    let message = format!("'{}' does not fit in {} bytes", id, size);

    let checker = quote! {
        const _: () = assert!(std::mem::size_of::<#id>() <= #size, #message);
        #struct_item
    };

    TokenStream::from(checker)
}
