# Offset macro
This crate defines multiple macros that make specifying structs with fields at specific offsets easy.

### Macrotypes
This crate currently contains two different macros.
- offset!
- offset_debug!

#### offset!
offset! just defines a struct with members at specific offsets and with a given type, name, and, visibility.
#### offset_debug!
Same as offset! except that Debug is also automatically implemented, this can also be done by adding a derive however this also prints the padding fields.
offset_debug's Debug implementation behaves like derive Debug except it ommits the generated padding fields.

### Features
This crate has a feature named "checked", which inserts compile time assertions that all fields are placed at the correct offsets this feature is only available on nightly compilers, and, with the offset_of feature enabled.

### Examples
#### DRIVER_OBJECT as seen in windows drivers.
```rust
offset_debug!(
    pub struct DRIVER_OBJECT {
        0x0  pub type_: u16,
        0x2  pub size: u16,
        0x8  pub device_object: *mut DEVICE_OBJECT,
        0x10 pub flags: u32,
        0x18 pub driver_start: usize,
        0x20 pub driver_size: u32,
        0x28 pub driver_section: usize,
        0x30 pub driver_extension: usize,
        0x38 pub driver_name: UNICODE_STRING,
        0x48 pub hardware_database: *mut UNICODE_STRING,
        0x50 pub fast_io_dispatch: usize,
        0x58 pub driver_init: usize,
        0x60 pub driver_start_io: usize,
        0x68 pub driver_unload: usize,
        0x70 pub major_function: [usize; 28],
    }
);
```

#### UWorld using optional sizing argument
```rust
offset_debug!(
    pub struct UWorld[0x08E8] {
        0x30 pub PersistentLevel: Pointer64<ULevel>,
    }
);
```

