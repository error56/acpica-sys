#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

use core::ffi::c_void;
use core::include;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    pub fn AcpiFindRootPointer(RsdpAddress: *mut ACPI_PHYSICAL_ADDRESS) -> ACPI_STATUS;
    pub fn AcpiInitializeTables(InitialStorage: *mut ACPI_TABLE_DESC, InitialTableCount: UINT32, AllowResize: BOOLEAN) -> ACPI_STATUS;
    pub fn AcpiInitializeSubsystem() -> ACPI_STATUS;
    pub fn AcpiEnableSubsystem(Flags: UINT32) -> ACPI_STATUS;
    pub fn AcpiInitializeObjects(Flags: UINT32) -> ACPI_STATUS;
    pub fn AcpiLoadTables() -> ACPI_STATUS;
    pub fn AcpiWalkNamespace(Type: ACPI_OBJECT_TYPE, StartObject: ACPI_HANDLE, MaxDepth: UINT32, DescendingCallback: ACPI_WALK_CALLBACK, AscendingCallback: ACPI_WALK_CALLBACK, Context: *mut c_void, ReturnValue: *mut *mut c_void) -> ACPI_STATUS;
    pub fn AcpiGetCurrentResources(Device: ACPI_HANDLE, RetBuffer: *mut ACPI_BUFFER) -> ACPI_STATUS;
    pub fn AcpiGetObjectInfo(Object: ACPI_HANDLE, ReturnBuffer: *mut *mut ACPI_DEVICE_INFO) -> ACPI_STATUS;
}

