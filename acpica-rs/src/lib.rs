//! ACPICA Rust bindings library
//! 
//! This library provides Rust bindings for the ACPICA (ACPI Component Architecture) library, 
//! enabling Rust programs to interact with the ACPI (Advanced Configuration and Power Interface) 
//! subsystem. 
//!
//! ## Overview
//!
//! The `AcpicaOsServices` trait defines the required interface that must be implemented to 
//! allow ACPICA to function in a specific operating system environment. Implementations of 
//! this trait are responsible for providing OS-specific services such as memory management, 
//! thread synchronization, and I/O operations. 
//!
//! The global OS services implementation is set once using the `set_os_services_implementation` 
//! function, which ensures that the ACPICA subsystem has access to the necessary system resources 
//! to operate correctly. Once set, this implementation cannot be changed for the lifetime of the 
//! system.
//!
//! ## Features
//! 
//! - **Thread Safety:** The OS services implementation is stored in a thread-safe manner using 
//!   the `Once` primitive, ensuring that it is initialized only once.
//! - **Customizable OS Services:** Developers can provide custom implementations of the 
//!   `AcpicaOsServices` trait to tailor the ACPI interactions to their specific OS environment.
//!
//! ## Usage
//!
//! To use this library, you need to implement the `AcpicaOsServices` trait for your target 
//! operating system. Then, set your implementation using `set_os_services_implementation` 
//! before performing any ACPI-related operations.
//!
//! Example:
//! 
//! ```rust
//! let my_os_services = Box::new(MyAcpicaOsServicesImplementation::new());
//! set_os_services_implementation(my_os_services);
//! ```
//! 
//! In the example above, `MyAcpicaOsServicesImplementation` is a custom struct that implements 
//! the `AcpicaOsServices` trait. The ACPI subsystem will use this implementation to interface 
//! with the underlying OS.
//!
//! # Notes
//! This library is `no_std` and is intended to be used in environments where the standard 
//! library is not available. The crate depends on the `alloc` crate for dynamic memory 
//! allocation.
#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(prelude_2024)]
#![no_std]

extern crate alloc;

mod delegates;
mod format;
pub mod sys;

use core::ffi::c_void;

use acpica_sys::*;
use alloc::boxed::Box;
use spin::Once;

pub type ACPI_SPINLOCK = u64;
pub type ACPI_SEMAPHORE = u64;
pub type ACPI_MUTEX = u64;
pub type ACPI_THREAD_ID = u64;
pub type ACPI_CPU_FLAGS = u64;
pub const AE_OK: ACPI_STATUS = 0;
pub const AE_BAD_PARAMETER: ACPI_STATUS = 0x1001;

static OS_SERVICES_IMPLEMENTATION: Once<Box<dyn AcpicaOsServices>> = Once::new();

/// Sets the global implementation of the `AcpicaOsServices` trait.
///
/// This function allows you to provide a custom implementation of the `AcpicaOsServices` trait,
/// which the ACPI subsystem will use to interact with the underlying operating system.
/// The implementation is stored globally and is only set once during the lifetime of the application.
///
/// # Arguments
///
/// * `implementation` - An `Arc<dyn AcpicaOsServices>` representing the implementation of
///   the `AcpicaOsServices` trait. This ensures that the implementation is thread-safe and
///   can be shared across multiple threads.
///
/// # Panics
///
/// This function will panic if called more than once, as the implementation can only be set once
/// using the `call_once` method from the `Once` primitive.
///
/// # Example
///
/// ```rust
/// let my_os_services = Arc::new(MyAcpicaOsServicesImplementation::new());
/// set_os_services_implementation(my_os_services);
/// ```
///
/// In the example above, `MyAcpicaOsServicesImplementation` is a custom implementation of the
/// `AcpicaOsServices` trait. The `set_os_services_implementation` function is called to register
/// this implementation, allowing the ACPI subsystem to use it for its operations.
pub fn set_os_services_implementation(implementation: Box<dyn AcpicaOsServices>) {
    OS_SERVICES_IMPLEMENTATION.call_once(|| implementation);
}

pub fn current_os_services_implementation() -> Option<&'static Box<dyn AcpicaOsServices>> {
    OS_SERVICES_IMPLEMENTATION.get()
}

/// The `AcpicaOsServices` trait defines the interface required by the ACPI (Advanced Configuration and Power Interface)
/// subsystem to interact with the underlying operating system. Implementations of this trait must provide various
/// methods for memory management, synchronization primitives, I/O operations, and more, enabling the ACPI
/// subsystem to function correctly in different OS environments.
pub trait AcpicaOsServices: Send + Sync {
    /// Initializes the ACPI subsystem.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn initialize(&self) -> ACPI_STATUS;

    /// Terminates the ACPI subsystem.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn terminate(&self) -> ACPI_STATUS;

    /// Maps a physical address to a logical address space.
    ///
    /// * `physical_address` - The physical address to be mapped.
    /// * `length` - The length of the region to map.
    ///
    /// Returns a pointer to the mapped logical address.
    fn map(&self, physical_address: ACPI_PHYSICAL_ADDRESS, length: ACPI_SIZE) -> *mut c_void;

    /// Unmaps a previously mapped logical address.
    ///
    /// * `logical_address` - The logical address to be unmapped.
    /// * `length` - The length of the region to unmap.
    fn unmap(&self, logical_address: *mut c_void, length: ACPI_SIZE);

    /// Translates a logical address to its corresponding physical address.
    ///
    /// * `logical_address` - The logical address to translate.
    /// * `physical_address` - The output parameter to store the resulting physical address.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn get_physical_address(
        &self,
        logical_address: *mut c_void,
        physical_address: &mut ACPI_PHYSICAL_ADDRESS,
    ) -> ACPI_STATUS;

    /// Allocates a block of memory.
    ///
    /// * `size` - The size of the memory block to allocate.
    ///
    /// Returns a pointer to the allocated memory.
    fn allocate(&self, size: ACPI_SIZE) -> *mut c_void;

    /// Frees a previously allocated block of memory.
    ///
    /// * `address` - The pointer to the memory block to be freed.
    fn free(&self, address: *mut c_void);

    /// Checks if a memory region is readable.
    ///
    /// * `address` - The starting address of the memory region.
    /// * `length` - The length of the memory region.
    ///
    /// Returns `true` if the memory region is readable, `false` otherwise.
    fn is_readable(&self, address: *mut c_void, length: ACPI_SIZE) -> bool;

    /// Checks if a memory region is writable.
    ///
    /// * `address` - The starting address of the memory region.
    /// * `length` - The length of the memory region.
    ///
    /// Returns `true` if the memory region is writable, `false` otherwise.
    fn is_writable(&self, address: *mut c_void, length: ACPI_SIZE) -> bool;

    /// Retrieves the current thread ID.
    ///
    /// Returns the current thread ID.
    fn get_thread_id(&self) -> ACPI_THREAD_ID;

    /// Suspends execution for a specified number of milliseconds.
    ///
    /// * `milliseconds` - The number of milliseconds to sleep.
    fn sleep(&self, milliseconds: u64);

    /// Stalls execution for a specified number of microseconds.
    ///
    /// * `microseconds` - The number of microseconds to stall.
    fn stall(&self, microseconds: u32);

    /// Waits until all pending events are completed.
    fn wait_events_complete(&self);

    /// Creates a mutex.
    ///
    /// * `handle` - The output parameter to store the handle to the created mutex.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn create_mutex(&self, handle: *mut ACPI_MUTEX) -> ACPI_STATUS;

    /// Deletes a mutex.
    ///
    /// * `handle` - The handle to the mutex to delete.
    fn delete_mutex(&self, handle: ACPI_MUTEX);

    /// Acquires a mutex, blocking if necessary.
    ///
    /// * `handle` - The handle to the mutex to acquire.
    /// * `timeout` - The timeout value in milliseconds.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn acquire_mutex(&self, handle: ACPI_MUTEX, timeout: u16) -> ACPI_STATUS;

    /// Releases a previously acquired mutex.
    ///
    /// * `handle` - The handle to the mutex to release.
    fn release_mutex(&self, handle: ACPI_MUTEX);

    /// Creates a semaphore.
    ///
    /// * `max_units` - The maximum number of units for the semaphore.
    /// * `initial_units` - The initial number of units for the semaphore.
    /// * `handle` - The output parameter to store the handle to the created semaphore.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn create_semaphore(
        &self,
        max_units: u32,
        initial_units: u32,
        handle: *mut ACPI_SEMAPHORE,
    ) -> ACPI_STATUS;

    /// Deletes a semaphore.
    ///
    /// * `handle` - The handle to the semaphore to delete.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn delete_semaphore(&self, handle: ACPI_SEMAPHORE) -> ACPI_STATUS;

    /// Waits on a semaphore, blocking if necessary.
    ///
    /// * `handle` - The handle to the semaphore to wait on.
    /// * `units` - The number of units to wait for.
    /// * `timeout` - The timeout value in milliseconds.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn wait_semaphore(&self, handle: ACPI_SEMAPHORE, units: u32, timeout: u16) -> ACPI_STATUS;

    /// Signals a semaphore, incrementing its count.
    ///
    /// * `handle` - The handle to the semaphore to signal.
    /// * `units` - The number of units to signal.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn signal_semaphore(&self, handle: ACPI_SEMAPHORE, units: u32) -> ACPI_STATUS;

    /// Creates a spinlock.
    ///
    /// * `handle` - The output parameter to store the handle to the created spinlock.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn create_lock(&self, handle: *mut ACPI_SPINLOCK) -> ACPI_STATUS;

    /// Deletes a spinlock.
    ///
    /// * `handle` - The handle to the spinlock to delete.
    fn delete_lock(&self, handle: ACPI_SPINLOCK);

    /// Acquires a spinlock.
    ///
    /// * `handle` - The handle to the spinlock to acquire.
    ///
    /// Returns the CPU flags prior to acquiring the spinlock.
    fn acquire_lock(&self, handle: ACPI_SPINLOCK) -> ACPI_CPU_FLAGS;

    /// Releases a spinlock.
    ///
    /// * `handle` - The handle to the spinlock to release.
    /// * `flags` - The CPU flags to restore after releasing the spinlock.
    fn release_lock(&self, handle: ACPI_SPINLOCK, flags: ACPI_CPU_FLAGS);

    /// Installs an interrupt handler.
    ///
    /// * `interrupt_level` - The interrupt level for the handler.
    /// * `handler` - The function pointer to the interrupt handler.
    /// * `context` - A pointer to the context to pass to the handler.
    fn install_interrupt_handler(
        &self,
        interrupt_level: u32,
        handler: ACPI_OSD_HANDLER,
        context: *mut c_void,
    ) -> ACPI_STATUS;

    /// Removes an interrupt handler.
    ///
    /// * `interrupt_level` - The interrupt level for the handler.
    /// * `handler` - The function pointer to the interrupt handler.
    fn remove_interrupt_handler(&self, interrupt_level: u32, handler: ACPI_OSD_HANDLER) -> ACPI_STATUS;

    /// Reads a value from a physical memory address.
    ///
    /// * `address` - The physical address to read from.
    /// * `value` - The output parameter to store the read value.
    /// * `width` - The width of the value to read, in bits.
    fn read_memory(&self, address: ACPI_PHYSICAL_ADDRESS, value: *mut u64, width: u32) -> ACPI_STATUS;

    /// Writes a value to a physical memory address.
    ///
    /// * `address` - The physical address to write to.
    /// * `value` - The value to write.
    /// * `width` - The width of the value to write, in bits.
    fn write_memory(&self, address: ACPI_PHYSICAL_ADDRESS, value: u64, width: u32) -> ACPI_STATUS;

    /// Reads a value from an I/O port.
    ///
    /// * `address` - The I/O port address to read from.
    /// * `value` - The output parameter to store the read value.
    /// * `width` - The width of the value to read, in bits.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn read_port(&self, address: ACPI_IO_ADDRESS, value: &mut u32, width: u32) -> ACPI_STATUS;

    /// Writes a value to an I/O port.
    ///
    /// * `address` - The I/O port address to write to.
    /// * `value` - The value to write.
    /// * `width` - The width of the value to write, in bits.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn write_port(&self, address: ACPI_IO_ADDRESS, value: u32, width: u32) -> ACPI_STATUS;

    /// Reads a value from PCI configuration space.
    ///
    /// * `pci_id` - The PCI device ID.
    /// * `register` - The configuration register to read from.
    /// * `value` - The output parameter to store the read value.
    /// * `width` - The width of the value to read, in bits.
    fn read_pci_configuration(
        &self,
        pci_id: *mut ACPI_PCI_ID,
        register: u32,
        value: *mut u64,
        width: u32,
    ) -> ACPI_STATUS;

    /// Writes a value to PCI configuration space.
    ///
    /// * `pci_id` - The PCI device ID.
    /// * `register` - The configuration register to write to.
    /// * `value` - The value to write.
    /// * `width` - The width of the value to write, in bits.
    fn write_pci_configuration(&self, pci_id: *mut ACPI_PCI_ID, register: u32, value: u64, width: u32) -> ACPI_STATUS;

    /// Overrides a predefined ACPI object.
    ///
    /// * `predefined_object` - A pointer to the predefined object to override.
    /// * `new_value` - The new value to override with.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn override_predefined(
        &self,
        predefined_object: *mut ACPI_PREDEFINED_NAMES,
        new_value: *mut ACPI_STRING,
    ) -> ACPI_STATUS;

    /// Overrides an ACPI table.
    ///
    /// * `existing_table` - A pointer to the existing table to override.
    /// * `new_table` - The output parameter to store the pointer to the new table.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn override_table(
        &self,
        existing_table: *mut ACPI_TABLE_HEADER,
        new_table: *mut *mut ACPI_TABLE_HEADER,
    ) -> ACPI_STATUS;

    /// Overrides a physical ACPI table.
    ///
    /// * `existing_table` - A pointer to the existing table to override.
    /// * `new_address` - The output parameter to store the new physical address.
    /// * `new_table_length` - The output parameter to store the length of the new table.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn override_physical_table(
        &self,
        existing_table: *mut ACPI_TABLE_HEADER,
        new_address: *mut ACPI_PHYSICAL_ADDRESS,
        new_table_length: *mut u32,
    ) -> ACPI_STATUS;

    /// Executes an ACPI-defined function.
    ///
    /// * `type_` - The type of execution to perform.
    /// * `function` - The function to execute.
    /// * `context` - A pointer to the context to pass to the function.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn execute(
        &self,
        type_: ACPI_EXECUTE_TYPE,
        function: ACPI_OSD_EXEC_CALLBACK,
        context: *mut c_void,
    ) -> ACPI_STATUS;

    /// Retrieves the current timer value.
    ///
    /// Returns the current timer value.
    fn get_timer(&self) -> u64;

    /// Signals an ACPI event.
    ///
    /// * `function` - The function to signal.
    /// * `info` - A pointer to additional information to pass with the signal.
    ///
    /// Returns an `ACPI_STATUS` indicating success or failure.
    fn signal(&self, function: u32, info: *mut c_void) -> ACPI_STATUS;

    /// Initializes the ACPI debugger.
    fn initialize_debugger(&self);

    /// Terminates the ACPI debugger.
    fn terminate_debugger(&self);

    /// Waits until the ACPI command is ready.
    fn wait_command_ready(&self);

    /// Notifies that the ACPI command is complete.
    fn notify_command_complete(&self);

    /// Enters a specific sleep state.
    ///
    /// * `sleep_state` - The sleep state to enter.
    /// * `register_a_value` - The value to write to the first register.
    /// * `register_b_value` - The value to write to the second register.
    fn enter_sleep(&self, sleep_state: u32, register_a_value: u32, register_b_value: u32);

    /// Disassembles an ACPI bytecode stream.
    ///
    /// * `walk_state` - The current walk state.
    /// * `origin` - The starting point of the disassembly.
    /// * `num_opcodes` - The number of opcodes to disassemble.
    fn disassemble(&self, walk_state: u64, origin: u64, num_opcodes: u32);

    /// Parses deferred ACPI operations.
    ///
    /// * `root` - The root of the operations to parse.
    fn parse_deferred_operations(&self, root: u64);

    /// Prints formatted text to the output.
    //
    /// * `text` - A `core::fmt::Arguments` instance containing the text and formatting information.
    fn print(&self, text: core::fmt::Arguments);
}
