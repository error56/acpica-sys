use core::ffi::{c_void, CStr, VaList};

use acpica_sys::*;

use crate::{ACPI_CPU_FLAGS, ACPI_THREAD_ID, ACPI_MUTEX, ACPI_SEMAPHORE, ACPI_SPINLOCK, format::CFmtConverter, OS_SERVICES_IMPLEMENTATION};

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsInitialize() -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().initialize()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsTerminate() -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().terminate()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsGetRootPointer() -> ACPI_PHYSICAL_ADDRESS {
    let mut root_pointer = 0;

    unsafe { AcpiFindRootPointer(&mut root_pointer); }
        
    root_pointer as ACPI_PHYSICAL_ADDRESS
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsPredefinedOverride(
    PredefinedObject: *mut ACPI_PREDEFINED_NAMES,
    NewValue: *mut ACPI_STRING,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .override_predefined(PredefinedObject, NewValue)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsTableOverride(
    ExistingTable: *mut ACPI_TABLE_HEADER,
    NewTable: *mut *mut ACPI_TABLE_HEADER,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .override_table(ExistingTable, NewTable)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsPhysicalTableOverride(
    ExistingTable: *mut ACPI_TABLE_HEADER,
    NewAddress: *mut ACPI_PHYSICAL_ADDRESS,
    NewTableLength: *mut u32,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .override_physical_table(ExistingTable, NewAddress, NewTableLength)
}

// -- Memory Management ---
// ------------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsMapMemory(
    PhysicalAddress: ACPI_PHYSICAL_ADDRESS,
    Length: ACPI_SIZE,
) -> *mut c_void {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .map(PhysicalAddress, Length)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsUnmapMemory(LogicalAddress: *mut c_void, Length: ACPI_SIZE) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .unmap(LogicalAddress, Length)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsGetPhysicalAddress(
    LogicalAddress: *mut c_void,
    PhysicalAddress: &mut ACPI_PHYSICAL_ADDRESS,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .get_physical_address(LogicalAddress, PhysicalAddress)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsAllocate(Size: ACPI_SIZE) -> *mut c_void {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().allocate(Size)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsFree(Memory: *mut c_void) {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().free(Memory)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReadable(Memory: *mut c_void, Length: ACPI_SIZE) -> bool {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .is_readable(Memory, Length)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWritable(Memory: *mut c_void, Length: ACPI_SIZE) -> bool {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .is_writable(Memory, Length)
}

// -- Thread Management --
// -----------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsGetThreadId() -> ACPI_THREAD_ID {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().get_thread_id()

    // 0 is special to ACPICA, so offset by one
    // - This is just used by ACPICA, so offsetting by one is safe
    //    0
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsExecute(
    Type: ACPI_EXECUTE_TYPE,
    Function: ACPI_OSD_EXEC_CALLBACK,
    Context: *mut c_void,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .execute(Type, Function, Context)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsSleep(Milliseconds: u64) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .sleep(Milliseconds)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsStall(Microseconds: u32) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .stall(Microseconds)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWaitEventsComplete() {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .wait_events_complete()
}

// --- Mutexes etc ---
// -------------------
#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsCreateMutex(OutHandle: *mut ACPI_MUTEX) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .create_mutex(OutHandle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsDeleteMutex(Handle: ACPI_MUTEX) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .delete_mutex(Handle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsAcquireMutex(Handle: ACPI_MUTEX, Timeout: u16) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .acquire_mutex(Handle, Timeout)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReleaseMutex(Handle: ACPI_MUTEX) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .release_mutex(Handle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsCreateSemaphore(
    MaxUnits: u32,
    InitialUnits: u32,
    OutHandle: *mut ACPI_SEMAPHORE,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .create_semaphore(MaxUnits, InitialUnits, OutHandle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsDeleteSemaphore(Handle: ACPI_SEMAPHORE) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .delete_semaphore(Handle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWaitSemaphore(Handle: ACPI_SEMAPHORE, Units: u32, Timeout: u16) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .wait_semaphore(Handle, Units, Timeout)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsSignalSemaphore(Handle: ACPI_SEMAPHORE, Units: u32) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .signal_semaphore(Handle, Units)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsCreateLock(OutHandle: *mut ACPI_SPINLOCK) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .create_lock(OutHandle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsDeleteLock(Handle: ACPI_SPINLOCK) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .delete_lock(Handle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsAcquireLock(Handle: ACPI_SPINLOCK) -> ACPI_CPU_FLAGS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .acquire_lock(Handle)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReleaseLock(Handle: ACPI_SPINLOCK, Flags: ACPI_CPU_FLAGS) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .release_lock(Handle, Flags)
}

// -- Interrupt handling --
// ------------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsInstallInterruptHandler(
    InterruptLevel: u32,
    Handler: ACPI_OSD_HANDLER,
    Context: *mut c_void,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .install_interrupt_handler(InterruptLevel, Handler, Context)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsRemoveInterruptHandler(
    InterruptLevel: u32,
    Handler: ACPI_OSD_HANDLER,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .remove_interrupt_handler(InterruptLevel, Handler)
}

// -- Memory Access --
// -------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReadMemory(
    Address: ACPI_PHYSICAL_ADDRESS,
    Value: *mut u64,
    Width: u32,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .read_memory(Address, Value, Width)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWriteMemory(
    Address: ACPI_PHYSICAL_ADDRESS,
    Value: u64,
    Width: u32,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .write_memory(Address, Value, Width)
}

// -- Port Input / Output --
// -------------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReadPort(Address: ACPI_IO_ADDRESS, Value: &mut u32, Width: u32) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .read_port(Address, Value, Width)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWritePort(Address: ACPI_IO_ADDRESS, Value: u32, Width: u32) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .write_port(Address, Value, Width)
}

// -- PCI Configuration Space Access --
// ------------------------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsReadPciConfiguration(
    PciId: *mut ACPI_PCI_ID,
    Register: u32,
    Value: *mut u64,
    Width: u32,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .read_pci_configuration(PciId, Register, Value, Width)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWritePciConfiguration(
    PciId: *mut ACPI_PCI_ID,
    Register: u32,
    Value: u64,
    Width: u32,
) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .write_pci_configuration(PciId, Register, Value, Width)
}

// -- Formatted Output --
// ----------------------

// NOTE: AcpiOsPrintf is handled by the acrust.h header

#[no_mangle]
#[linkage = "external"]
#[allow(dead_code)]
extern "C" fn AcpiOsVprintf(format: *mut i8, mut args: VaList) {
    let format = unsafe { CStr::from_ptr(format) };
    let format = format.to_str().unwrap();

    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .print(format_args!(
            "{}",
            CFmtConverter {
                format,
                args: args.as_va_list()
            }
        ))
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsRedirectOutput(_Destination: *mut c_void) {
    unimplemented!();
}

// -- Miscellaneous --
// -------------------

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsGetTimer() -> u64 {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().get_timer()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsSignal(Function: u32, Info: *mut c_void) -> ACPI_STATUS {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .signal(Function, Info)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsInitializeDebugger() {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .initialize_debugger()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsTerminateDebugger() {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .terminate_debugger()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsWaitCommandReady() {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .wait_command_ready()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsNotifyCommandComplete() {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .notify_command_complete()
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiOsEnterSleep(sleep_state: u32, registeravalue: u32, registerbvalue: u32) {
    OS_SERVICES_IMPLEMENTATION.get().unwrap().enter_sleep(
        sleep_state,
        registeravalue,
        registerbvalue,
    )
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiDmDisassemble(walk_state: u64, origin: u64, num_opcodes: u32) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .disassemble(walk_state, origin, num_opcodes)
}

#[no_mangle]
#[linkage = "external"]
extern "C" fn AcpiDmParseDeferredOps(root: u64) {
    OS_SERVICES_IMPLEMENTATION
        .get()
        .unwrap()
        .parse_deferred_operations(root)
}
