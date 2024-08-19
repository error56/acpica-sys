#define ACPI_USE_DO_WHILE_0
#define ACPI_USE_LOCAL_CACHE

typedef char CHAR;
typedef unsigned char UCHAR, UINT8;
typedef signed char INT8;
typedef short SHORT;
typedef signed short INT16;
typedef unsigned short USHORT, UINT16;
typedef int INT;
typedef unsigned int UINT32;
typedef signed int INT32;
typedef long LONG;
typedef unsigned long ULONG;
typedef long long LONGLONG, LONG64;
typedef signed long long INT64;
typedef unsigned long long ULONGLONG, DWORDLONG, ULONG64, DWORD64, UINT64;

typedef INT64 ACPI_NATIVE_INT;
typedef UINT64 ACPI_SIZE;
typedef UINT64 ACPI_IO_ADDRESS;
typedef UINT64 ACPI_PHYSICAL_ADDRESS;

#define ACPI_MACHINE_WIDTH 64

#define ACPI_INLINE __inline__

#define ACPI_INIT_FUNCTION

#define ACPI_EXTERNAL_RETURN_STATUS(Prototype) \
    static ACPI_INLINE Prototype { return (AE_NOT_CONFIGURED); }
#define ACPI_EXTERNAL_RETURN_OK(Prototype) \
    static ACPI_INLINE Prototype { return (AE_OK); }
#define ACPI_EXTERNAL_RETURN_VOID(Prototype) \
    static ACPI_INLINE Prototype { return; }
#define ACPI_EXTERNAL_RETURN_UINT32(Prototype) \
    static ACPI_INLINE Prototype { return (0); }
#define ACPI_EXTERNAL_RETURN_PTR(Prototype) \
    static ACPI_INLINE Prototype { return (NULL); }

#define ACPI_SYSTEM_XFACE __cdecl
#define ACPI_INTERNAL_VAR_XFACE __cdecl

typedef __builtin_va_list va_list;

#include "../acpica/source/include/platform/acenv.h"

#include "../acpica/source/include/actypes.h"
#include "../acpica/source/include/acexcep.h"
#include "../acpica/source/include/acrestyp.h"
#include "../acpica/source/include/acpixf.h"
#include "../acpica/source/include/acpiosxf.h"
