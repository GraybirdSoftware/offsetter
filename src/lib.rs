#![no_std]
pub extern crate paste;

#[macro_export]
/// Creates a struct with fields placed at specific memory offsets.
///
/// This macro allows you to define a struct with precise memory layout by specifying
/// the byte offset of each field. The macro automatically inserts padding between fields
/// to ensure proper alignment.
///
/// # Optional Total Size
///
/// You can optionally specify the total size of the struct by adding a size value in
/// square brackets after the struct name. This will ensure the struct has exactly that
/// size by adding padding at the end if necessary.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// offset!(
///     pub struct Example {
///         0x0 pub field1: u32,
///         0x4 pub field2: u16,
///         0x8 pub field3: u64
///     }
/// );
/// ```
///
/// With explicit total size:
///
/// ```rust
/// offset!(
///     pub struct ExampleWithSize[0x20] {
///         0x0 pub field1: u32,
///         0x4 pub field2: u16,
///         0x8 pub field3: u64
///         // The struct will be padded to exactly 0x20 bytes
///     }
/// );
/// ```
///
/// For platform compatibility (e.g., Windows driver structures):
///
/// ```rust
/// offset!(
///     pub struct DEVICE_OBJECT[0x150] {
///         0x0 pub type_: u16,
///         0x2 pub size: u16,
///         0x8 pub next: *mut DEVICE_OBJECT,
///         // More fields...
///     }
/// );
/// ```
macro_rules! offset {
    (@guard_with_size ($current_offset:expr, $struct_size:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name {
                $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty,)*
                _remaining_padding: [u8; $struct_size - $current_offset]
            }
        }
    };

    (@guard ($current_offset:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name { $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty),* }
        }
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>()) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard_with_size ($current_offset:expr, $struct_size:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset!(@guard_with_size ($offset + core::mem::size_of::<$ty>(), $struct_size) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard_with_size ($current_offset:expr, $struct_size:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset!(@guard_with_size ($offset + core::mem::size_of::<$ty>(), $struct_size, $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };


    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident {$($input:tt)*}) => {
        offset!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };

    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident[$struct_size:expr] {$($input:tt)*}) => {
        offset!(@guard_with_size (0, $struct_size, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };
}

#[macro_export]
/// Creates a struct with fields at specific offsets and a custom Debug implementation.
///
/// This macro works the same as the `offset!` macro but also implements the Debug trait
/// in a way that hides padding fields. This gives you cleaner debug output that shows
/// only the actual data fields, not the padding.
///
/// # Optional Total Size
///
/// You can optionally specify the total size of the struct by adding a size value in
/// square brackets after the struct name. This will ensure the struct has exactly that
/// size by adding padding at the end.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// offset_debug!(
///     pub struct Example {
///         0x0 pub field1: u32,
///         0x4 pub field2: u16,
///         0x8 pub field3: u64
///     }
/// );
///
/// // Debug output will be something like:
/// // Example { field1: 1, field2: 2, field3: 3 }
/// // (no padding fields shown)
/// ```
///
/// With explicit total size:
///
/// ```rust
/// offset_debug!(
///     pub struct KernelStructure[0x100] {
///         0x00 pub header: u32,
///         0x08 pub pointer: *mut u8,
///         0x10 pub flags: u32
///         // The struct will be padded to exactly 0x100 bytes
///         // Debug output will not show the padding
///     }
/// );
/// ```
///
/// Real-world example for Windows kernel structures:
///
/// ```rust
/// offset_debug!(
///     pub struct DRIVER_OBJECT[0x150] {
///         0x0  pub type_: u16,
///         0x2  pub size: u16,
///         0x8  pub device_object: *mut DEVICE_OBJECT,
///         0x10 pub flags: u32,
///         0x18 pub driver_start: usize,
///         // More fields...
///     }
/// );
/// ```
macro_rules! offset_debug {

    (@guard_with_size ($current_offset:expr, $struct_size:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name {
                $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty,)*
                _remaining_padding: [u8; $struct_size - $current_offset]
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                 $(.field(stringify!($id), unsafe { &core::ptr::read_unaligned(core::ptr::addr_of!(self.$id))}))*
                 .finish()
            }
        }
    };


    (@guard ($current_offset:expr) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($amount:expr, $vis_field:vis $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])* $vis struct $name { $([<_pad $id>]: [u8;$amount], $vis_field $id: $ty),* }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                 $(.field(stringify!($id), unsafe { &core::ptr::read_unaligned(core::ptr::addr_of!(self.$id))}))*
                 .finish()
            }
        }
    };


    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset_debug!(@guard ($offset + core::mem::size_of::<$ty>()) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset_debug!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };


    (@guard_with_size ($current_offset:expr, $struct_size:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty $(,)?) -> {$($output:tt)*}) => {
        offset_debug!(@guard_with_size ($offset + core::mem::size_of::<$ty>(), $struct_size) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };

    (@guard_with_size ($current_offset:expr, $struct_size:expr, $offset:literal $vis_field:vis $id:ident: $ty:ty, $($next:tt)+) -> {$($output:tt)*}) => {
        offset_debug!(@guard_with_size ($offset + core::mem::size_of::<$ty>(), $struct_size, $($next)+) -> {$($output)* ($offset - ($current_offset), $vis_field $id: $ty)});
    };


    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident {$($input:tt)*}) => {
        offset_debug!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };


    ($(#[$attr:meta])* $vis:vis struct $struct_name:ident[$struct_size:expr] {$($input:tt)*}) => {
        offset_debug!(@guard_with_size (0, $struct_size, $($input)*) -> {$(#[$attr])* $vis struct $struct_name});
        $crate::offset_checker!($struct_name {$($input)*});
    };
}

#[cfg(feature = "checked")]
#[macro_export]
macro_rules! offset_checker {
    ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {
        $(const _: () = assert!(core::mem::offset_of!($struct_name, $id) == $offset);)*
    };
}

#[cfg(not(feature = "checked"))]
#[macro_export]
macro_rules! offset_checker {
    ($struct_name:ident {$($offset:literal $vis_field:vis $id:ident: $ty:ty),* $(,)?}) => {};
}
