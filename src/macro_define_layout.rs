/// This macro defines a data layout. Given such a layout, the [Field](crate::Field) or [FieldView](crate::FieldView) APIs can be used to access data based on it.
///
/// Data layouts define
/// - a name for the layout
/// - and endianness for its fields ([BigEndian](crate::BigEndian) or [LittleEndian](crate::LittleEndian))
/// - and an ordered collection of typed fields.
///
/// See [supported field types](crate#supported-field-types) for a list of supported field types.
///
/// # API
/// ```text
/// define_layout!(<<Name>>, <<Endianness>>, {
///   <<FieldName>>: <<FieldType>>,
///   <<FieldName>>: <<FieldType>>,
///   ...
/// });
/// ```
///
/// ## Field names
/// Field names can be any valid Rust identifiers, but it is recommended to avoid names that contain `storage`, `into_` or `_mut`.
/// This is because the [define_layout!] macro creates a [View class with several accessors](#struct-view) for each field that contain those identifier parts.
///
/// ## Example
/// ```
/// use binary_layout::prelude::*;
///
/// define_layout!(icmp_packet, BigEndian, {
///   packet_type: u8,
///   code: u8,
///   checksum: u16,
///   rest_of_header: [u8; 4],
///   data_section: [u8], // open ended byte array, matches until the end of the packet
/// });
/// ```
///
/// # Generated code
/// See [icmp_packet](crate::example::icmp_packet) for an example.
///
/// This macro will define a module for you with several members:
/// - For each field, there will be a struct containing
///   - metadata like [OFFSET](crate::Field::OFFSET) and [SIZE](crate::Field::SIZE) as rust `const`s
///   - data accessors for the [Field](crate::Field) API
/// - The module will also contain a `View` struct that offers the [FieldView](crate::FieldView) API.
///
/// This macro will also generate rustdoc documentation for everything it generates. One of the best ways to figure out
/// how to use the generated layouts is to read the rustdoc documentation that was generated for them.
///
/// ## Metadata Example
/// ```
/// use binary_layout::prelude::*;
///
/// define_layout!(my_layout, LittleEndian, {
///   field1: u16,
///   field2: u32,
/// });
/// assert_eq!(2, my_layout::field2::OFFSET);
/// assert_eq!(Some(4), my_layout::field2::SIZE);
/// ```
///
/// ## struct View
/// See [icmp_packet::View](crate::example::icmp_packet::View) for an example.
///
/// You can create views over a storage by calling `View::new`. Views can be created based on
/// - Immutable borrowed storage: `&[u8]`
/// - Mutable borrowed storage: `&mut [u8]`
/// - Owning storage: impl `AsRef<u8>` (for example: `Vec<u8>`)
///
/// The generated `View` struct will offer
/// - `View::new(storage)` to create a `View`
/// - `View::into_storage(self)` to destroy a `View` and return the storage held
///
/// and it will offer the following accessors for each field
/// - `${field_name}()`: Read access. This returns a [FieldView](crate::FieldView) instance with read access.
/// - `${field_name}_mut()`: Read access. This returns a [FieldView](crate::FieldView) instance with write access.
/// - `into_${field_name}`: Extract access. This destroys the `View` and returns a [FieldView](crate::FieldView) instance owning the storage. Mostly useful for slice fields when you want to return an owning slice.
#[macro_export]
macro_rules! define_layout {
    ($name: ident, $endianness: ident, {$($field_name: ident : $field_type: ty $(as $underlying_type: ty)?),* $(,)?}) => {
        $crate::internal::doc_comment!{
            concat!{"
            This module is autogenerated. It defines a layout using the [binary_layout] crate based on the following definition:
            ```ignore
            define_layout!(", stringify!($name), ", ", stringify!($endianness), ", {", $("
                ", stringify!($field_name), ": ", stringify!($field_type), $(" as ", stringify!($underlying_type), )? ",", )* "
            });
            ```
            "},
            #[allow(dead_code)]
            pub mod $name {
                #[allow(unused_imports)]
                use super::*;

                $crate::define_layout!(@impl_fields $crate::$endianness, Some(0), {$($field_name : $field_type $(as $underlying_type)?),*});

                $crate::internal::doc_comment!{
                    concat!{"
                    The [View] struct defines the [FieldView](crate::FieldView) API.
                    An instance of [View] wraps a storage (either borrowed or owned)
                    and allows accessors for the layout fields.

                    This view is based on the following layout definition:
                    ```ignore
                    define_layout!(", stringify!($name), ", ", stringify!($endianness), ", {", $("
                        ", stringify!($field_name), ": ", stringify!($field_type), $(" as ", stringify!($underlying_type), )? ",",)* "
                    });
                    ```
                    "},
                    pub struct View<S> {
                        storage: $crate::Data<S>,
                    }
                }
                impl <S: AsRef<[u8]>> View<S> {
                    /// You can create views over a storage by calling [View::new].
                    ///
                    /// `S` is the type of underlying storage. It can be
                    /// - Immutable borrowed storage: `&[u8]`
                    /// - Mutable borrowed storage: `&mut [u8]`
                    /// - Owning storage: impl `AsRef<u8>` (for example: `Vec<u8>`)
                    #[inline]
                    pub fn new(storage: S) -> Self {
                        Self {storage: storage.into()}
                    }

                    /// This destroys the view and returns the underlying storage back to you.
                    /// This is useful if you created an owning view (e.g. based on `Vec<u8>`)
                    /// and now need the underlying `Vec<u8>` back.
                    #[inline]
                    pub fn into_storage(self) -> $crate::Data<S> {
                        self.storage
                    }

                    $crate::define_layout!(@impl_view_into {$($field_name),*});
                }
                impl <S: AsRef<[u8]>> View<S> {
                    $crate::define_layout!(@impl_view_asref {$($field_name),*});
                }
                impl <S: AsMut<[u8]>> View<S> {
                    $crate::define_layout!(@impl_view_asmut {$($field_name),*});
                }
            }
        }
    };

    (@impl_fields $endianness: ty, $offset_accumulator: expr, {}) => {
        /// Total size of the layout in number of bytes.
        /// This can be None if the layout ends with an open ended field like a byte slice.
        pub const SIZE: Option<usize> = $offset_accumulator;
    };
    (@impl_fields $endianness: ty, $offset_accumulator: expr, {$name: ident : $type: ty as $underlying_type: ty $(, $($tail:tt)*)?}) => {
        $crate::internal::doc_comment!{
            concat!("Metadata and [Field](crate::Field) API accessors for the `", stringify!($name), "` field"),
            #[allow(non_camel_case_types)]
            pub type $name = $crate::WrappedField::<$underlying_type, $type, $crate::PrimitiveField::<$underlying_type, $endianness, {$crate::internal::unwrap_field_size($offset_accumulator)}>>;
        }
        $crate::define_layout!(@impl_fields $endianness, ($crate::internal::option_usize_add(<$name as $crate::Field>::OFFSET, <$name as $crate::Field>::SIZE)), {$($($tail)*)?});
    };
    (@impl_fields $endianness: ty, $offset_accumulator: expr, {$name: ident : $type: ty $(, $($tail:tt)*)?}) => {
        $crate::internal::doc_comment!{
            concat!("Metadata and [Field](crate::Field) API accessors for the `", stringify!($name), "` field"),
            #[allow(non_camel_case_types)]
            pub type $name = $crate::PrimitiveField::<$type, $endianness, {$crate::internal::unwrap_field_size($offset_accumulator)}>;
        }
        $crate::define_layout!(@impl_fields $endianness, ($crate::internal::option_usize_add(<$name as $crate::Field>::OFFSET, <$name as $crate::Field>::SIZE)), {$($($tail)*)?});
    };

    (@impl_view_asref {}) => {};
    (@impl_view_asref {$name: ident $(, $name_tail: ident)*}) => {
        $crate::internal::doc_comment!{
            concat!("Return a [FieldView](crate::FieldView) with read access to the `", stringify!($name), "` field"),
            #[inline]
            pub fn $name(&self) -> <$name as $crate::internal::StorageToFieldView<&[u8]>>::View {
                <$name as $crate::internal::StorageToFieldView<&[u8]>>::view(self.storage.as_ref())
            }
        }
        $crate::define_layout!(@impl_view_asref {$($name_tail),*});
    };

    (@impl_view_asmut {}) => {};
    (@impl_view_asmut {$name: ident $(, $name_tail: ident)*}) => {
        $crate::internal::paste!{
            $crate::internal::doc_comment!{
                concat!("Return a [FieldView](crate::FieldView) with write access to the `", stringify!($name), "` field"),
                #[inline]
                pub fn [<$name _mut>](&mut self) -> <$name as $crate::internal::StorageToFieldView<&mut [u8]>>::View {
                    <$name as $crate::internal::StorageToFieldView<&mut [u8]>>::view(self.storage.as_mut())
                }
            }
        }
        $crate::define_layout!(@impl_view_asmut {$($name_tail),*});
    };

    (@impl_view_into {}) => {};
    (@impl_view_into {$name: ident $(, $name_tail: ident)*}) => {
        $crate::internal::paste!{
            $crate::internal::doc_comment!{
                concat!("Destroy the [View] and return a field accessor to the `", stringify!($name), "` field owning the storage. This is mostly useful for [FieldView::extract](crate::FieldView::extract)"),
                #[inline]
                pub fn [<into_ $name>](self) -> <$name as $crate::internal::StorageIntoFieldView<S>>::View {
                    <$name as $crate::internal::StorageIntoFieldView<S>>::into_view(self.storage)
                }
            }
        }
        $crate::define_layout!(@impl_view_into {$($name_tail),*});
    };
}

// TODO This only exists because Option<usize>::unwrap() isn't const. Remove this once it is.
/// Internal function, don't use!
/// Unwraps an option<usize>
#[inline(always)]
pub const fn unwrap_field_size(opt: Option<usize>) -> usize {
    match opt {
        Some(x) => x,
        None => {
            #[allow(unconditional_panic)]
            #[allow(clippy::no_effect)]
            ["Error: Fields without a static size (e.g. open-ended byte arrays) can only be used at the end of a layout"][10];
            #[allow(clippy::empty_loop)]
            loop {}
        }
    }
}

/// Internal function, don't use!
#[inline(always)]
pub const fn option_usize_add(lhs: usize, rhs: Option<usize>) -> Option<usize> {
    match (lhs, rhs) {
        (lhs, Some(rhs)) => Some(lhs + rhs),
        (_, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use rand::{rngs::StdRng, RngCore, SeedableRng};

    fn data_region(size: usize, seed: u64) -> Vec<u8> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut res = vec![0; size];
        rng.fill_bytes(&mut res);
        res
    }

    define_layout!(module_level_layout, LittleEndian, {
        first: i8,
        second: i64,
        third: u16,
    });

    #[test]
    fn layouts_can_be_defined_at_module_level() {
        let storage: [u8; 1024] = [0; 1024];
        let view = module_level_layout::View::new(storage);
        assert_eq!(0, view.third().read());
    }

    #[test]
    fn layouts_can_be_defined_at_function_level() {
        define_layout!(function_level_layout, LittleEndian, {
            first: i8,
            second: i64,
            third: u16,
        });

        let storage: [u8; 1024] = [0; 1024];
        let view = function_level_layout::View::new(storage);
        assert_eq!(0, view.third().read());
    }

    #[test]
    fn can_be_created_with_and_without_trailing_comma() {
        define_layout!(first, LittleEndian, { field: u8 });
        define_layout!(second, LittleEndian, {
            field: u8,
            second: u16
        });
        define_layout!(third, LittleEndian, {
            field: u8,
        });
        define_layout!(fourth, LittleEndian, {
            field: u8,
            second: u16,
        });
    }

    #[test]
    fn there_can_be_multiple_views_if_readonly() {
        define_layout!(my_layout, BigEndian, {
            field1: u16,
            field2: i64,
        });

        let storage = data_region(1024, 0);
        let view1 = my_layout::View::new(&storage);
        let view2 = my_layout::View::new(&storage);
        view1.field1().read();
        view2.field1().read();
    }

    #[test]
    fn size_of_sized_layout() {
        define_layout!(my_layout, LittleEndian, {
            field1: u16,
            field2: i64,
        });
        assert_eq!(Some(10), my_layout::SIZE);
    }

    #[test]
    fn size_of_unsized_layout() {
        define_layout!(my_layout, LittleEndian, {
            field: u16,
            tail: [u8],
        });
        assert_eq!(None, my_layout::SIZE);
    }
}
