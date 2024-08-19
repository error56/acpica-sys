# ACPICA Rust Bindings Library

## Overview

The library exposes an interface that allows you to implement OS-specific services required by ACPICA and call various ACPICA functions from Rust code.

## Features

- **OS Agnostic:** Implement the `AcpicaOsServices` trait to provide the necessary services for ACPICA in your OS environment.
- **No Standard Library:** This library is designed for `no_std` environments, making it lightweight and flexible.

## Getting Started

### 1. Add the Library to Your Project

Include the library in your `Cargo.toml`:

```toml
[dependencies]
acpica-rs = { git = "https://github.com/moose-os/acpica-rs" }
```

### 2. Implement the `AcpicaOsServices` Trait

You need to implement the `AcpicaOsServices` trait for your operating system. This trait defines various methods required by ACPICA to interact with the OS, such as memory management, synchronization, and I/O operations.

```rust
use acpica_rs::{
    AcpicaOsServices, 
    sys::{ACPI_STATUS, ACPI_PHYSICAL_ADDRESS, ACPI_SIZE}
};
use core::ffi::c_void;

struct MyAcpicaOsServices;

impl AcpicaOsServices for MyAcpicaOsServices {
    fn initialize(&self) -> ACPI_STATUS {
        // Implement initialization logic here
    }

    fn terminate(&self) -> ACPI_STATUS {
        // Implement termination logic here
    }

    fn map(&self, physical_address: ACPI_PHYSICAL_ADDRESS, length: ACPI_SIZE) -> *mut c_void {
        // Implement memory mapping logic here
    }

    // Implement the rest of the trait methods...
}
```

### 3. Set the OS Services Implementation

Once you have your `AcpicaOsServices` implementation, set it globally before calling any ACPICA code:

```rust
let os_services = Box::new(MyAcpicaOsServices);
set_os_services_implementation(os_services);

// Now you can use the ACPI subsystem
```

### 4. Use the ACPI Subsystem

After setting up your OS services, you can interact with the ACPI subsystem.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

