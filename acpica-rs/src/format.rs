use core::ffi::VaListImpl;
use core::ffi::CStr;
use core::iter::Peekable;
use core::fmt::Display;
use core::ffi::VaList;
use core::fmt::Write;

fn read_format_parameter(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl<'_>,
) -> Option<(usize, bool)> {
    match chars.peek() {
        Some('*') => {
            // SAFETY: '*' in a format parameter means to read the parameter as an int from the arguments
            let param = unsafe { args.arg::<core::ffi::c_int>() as isize };

            chars.next();

            Some((param.unsigned_abs(), param < 0))
        }
        Some('0'..='9') => {
            let mut number = 0;

            loop {
                let Some(&c) = chars.peek() else {
                    break;
                };
                if !c.is_ascii_digit() {
                    break;
                }

                chars.next();

                number *= 10;
                number += c as usize - '0' as usize;
            }

            Some((number, false))
        }

        _ => None,
    }
}

fn read_min_width(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl,
    justify_left: &mut bool,
) -> Option<usize> {
    match read_format_parameter(chars, args) {
        Some((result, output_was_negative)) => {
            if output_was_negative {
                *justify_left = true;
            }
            Some(result)
        }
        None => None,
    }
}

fn read_precision(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl,
) -> Option<usize> {
    match read_format_parameter(chars, args) {
        Some((result, false)) => Some(result),
        Some((_, true)) => panic!("Invalid printf precision specifier"),
        None => None,
    }
}

pub struct CFmtConverter<'a, 'b> {
    pub format: &'a str,
    pub args: VaList<'b, 'a>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Copy)]
struct FormatParameters {
    justify_left: bool,
    always_show_sign: bool,
    prepend_space: bool,
    alternative: bool,
    leading_zeroes: bool,

    minimum_width: Option<usize>,
    precision: Option<usize>,
}

impl<'a, 'b> Display for CFmtConverter<'a, 'b> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut args = self.args.clone();

        let mut chars = self.format.chars().peekable();

        loop {
            let Some(c) = chars.next() else {
                break;
            };

            match c {
                '%' => (),
                c => {
                    f.write_fmt(format_args!("{c}"))?;
                    continue;
                }
            }

            // '%%' in a format string means write a literal % sign
            if let Some('%') = chars.peek() {
                chars.next();
                f.write_str("%")?;
                continue;
            }

            let mut params = FormatParameters::default();

            loop {
                match chars.peek() {
                    Some('-') => params.justify_left = true,
                    Some('+') => params.always_show_sign = true,
                    Some(' ') => params.prepend_space = true,
                    Some('#') => params.alternative = true,
                    Some('0') => params.leading_zeroes = true,
                    _ => break,
                }
                chars.next();
            }

            params.minimum_width = read_min_width(&mut chars, &mut args, &mut params.justify_left);

            params.precision = 'p: {
                let Some('.') = chars.peek() else {
                    break 'p None;
                };
                chars.next();

                read_precision(&mut chars, &mut args)
            };

            match_formatter(&mut chars, f, &mut args, params)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Base {
    Octal,
    Decimal,
    Hex,
}

impl Base {
    const fn as_number(self) -> usize {
        match self {
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::Hex => 16,
        }
    }

    const fn prefix(self, capital: bool) -> &'static str {
        match (self, capital) {
            (Base::Octal, _) => "0",
            (Base::Decimal, false) => "0d",
            (Base::Decimal, true) => "0D",
            (Base::Hex, false) => "0x",
            (Base::Hex, true) => "0X",
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy)]
struct UnsignedIntFormatParams {
    pad_char: char,
    base: Base,
    justify_left: bool,
    print_prefix: bool,
    capital: bool,
    precision: usize,
    min_length: usize,
}

fn format_int_unsigned(
    f: &mut core::fmt::Formatter<'_>,
    value: usize,
    mut params: UnsignedIntFormatParams,
) -> Result<(), core::fmt::Error> {
    let mut number_length = 0;

    if params.print_prefix {
        match params.base {
            Base::Octal => params.precision = params.precision.saturating_sub(1),
            Base::Decimal | Base::Hex => number_length += 2,
        }
    }

    number_length += (TryInto::<u32>::try_into(params.precision).unwrap())
        .max(value.checked_ilog(params.base.as_number()).unwrap_or(0) + 1);

    let pad_length = params.min_length.saturating_sub(number_length as _);

    if !params.justify_left {
        for _ in 0..pad_length {
            f.write_char(params.pad_char)?;
        }
    }

    if params.print_prefix {
        f.write_str(params.base.prefix(params.capital))?;
    }

    // Makes format strings nicer
    let precision = params.precision;

    match (params.base, params.capital) {
        (Base::Octal, true) => panic!("Uppercase octal formatting should not be reachable"),
        (Base::Octal, false) => {
            if !(value == 0 && precision == 0) {
                f.write_fmt(format_args!("{value:0>precision$o}"))?;
            }
        }
        (Base::Decimal, true) => panic!("Uppercase decimal formatting should not be reachable"),
        (Base::Decimal, false) => f.write_fmt(format_args!("{value:0>precision$}"))?,
        (Base::Hex, true) => f.write_fmt(format_args!("{value:0>precision$X}"))?,
        (Base::Hex, false) => f.write_fmt(format_args!("{value:0>precision$x}"))?,
    }

    if params.justify_left {
        for _ in 0..pad_length {
            f.write_char(params.pad_char)?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct SignedIntFormatParams {
    pad_char: char,
    justify_left: bool,
    precision: usize,
    min_length: usize,
    always_print_sign: bool,
    prepend_space: bool,
}

fn format_int_signed(
    f: &mut core::fmt::Formatter<'_>,
    value: isize,
    params: SignedIntFormatParams,
) -> Result<(), core::fmt::Error> {
    let mut number_length = 0;

    number_length += (TryInto::<u32>::try_into(params.precision).unwrap())
        .max(value.unsigned_abs().checked_ilog10().unwrap_or(0) + 1);

    if value < 0 || params.always_print_sign {
        number_length += 1;
    }

    let pad_length = params.min_length.saturating_sub(number_length as _);

    if !params.justify_left {
        for _ in 0..pad_length {
            f.write_char(params.pad_char)?;
        }
    }

    if value < 0 {
        f.write_char('-')?;
    } else if params.always_print_sign {
        f.write_char('+')?;
    } else if params.prepend_space {
        f.write_char(' ')?;
    }

    f.write_fmt(format_args!(
        "{:0>precision$}",
        value.unsigned_abs(),
        precision = params.precision
    ))?;

    if params.justify_left {
        for _ in 0..pad_length {
            f.write_char(params.pad_char)?;
        }
    }

    Ok(())
}

fn match_formatter(
    chars: &mut Peekable<core::str::Chars<'_>>,
    f: &mut core::fmt::Formatter<'_>,
    args: &mut VaListImpl<'_>,
    params: FormatParameters,
) -> Result<(), core::fmt::Error> {
    let pad_char = if params.leading_zeroes { '0' } else { ' ' };

    match chars.next() {
        None => panic!("Printf format string ended after %"),
        // Char
        // SAFETY: The '%c' specifier means the char data type
        Some('c') => f.write_fmt(format_args!("{}", unsafe {
            let c: u8 = args.arg::<core::ffi::c_char>().try_into().unwrap();
            c as char
        }))?,
        // Signed int
        Some('d' | 'i') => {
            let precision = params.precision.unwrap_or(1);
            let min_length = params.minimum_width.unwrap_or(0);

            // SAFETY: '%d' and '%i' both mean the int data type
            let value = unsafe { args.arg::<core::ffi::c_int>() };

            format_int_signed(
                f,
                value as _,
                SignedIntFormatParams {
                    pad_char,
                    justify_left: params.justify_left,
                    precision,
                    min_length,
                    always_print_sign: params.always_show_sign,
                    prepend_space: params.prepend_space,
                },
            )?;
        }
        // Unsigned int
        Some(c @ ('u' | 'o' | 'x' | 'X')) => {
            let (base, capital) = match c {
                'u' => (Base::Decimal, false),
                'o' => (Base::Octal, false),
                'x' => (Base::Hex, false),
                'X' => (Base::Hex, true),
                _ => unreachable!(),
            };

            let precision = params.precision.unwrap_or(1);
            let min_length = params.minimum_width.unwrap_or(0);

            // SAFETY: '%u', '%o', '%x', and '%X' all mean the unsigned int data type
            let value = unsafe { args.arg::<core::ffi::c_uint>() };

            format_int_unsigned(
                f,
                value as _,
                UnsignedIntFormatParams {
                    pad_char,
                    base,
                    justify_left: params.justify_left,
                    print_prefix: params.alternative,
                    capital,
                    precision,
                    min_length,
                },
            )?;
        }

        // String
        // SAFETY: '%s' means a string so the next argument is a C string.
        Some('s') => unsafe { print_string(args, params, f, pad_char)? },

        // Pointer
        // SAFETY: '%p' means a void* data type
        Some('p') => f.write_fmt(format_args!("{:p}", unsafe {
            args.arg::<*const core::ffi::c_void>()
        }))?,

        // No hurry on implementing this because it doesn't look like it's used in ACPICA
        Some('n') => todo!("'%n' formatter which writes the current number of bytes to a pointer"),
        Some(s @ ('h' | 'l' | 'j' | 'z' | 't' | 'L')) => todo!("Format modifier '{s}'"),
        Some(s @ ('f' | 'F' | 'e' | 'E' | 'a' | 'A' | 'g' | 'G')) => panic!(
            "Formatter '{s}' is not supported because floating point numbers are not VaArgSafe"
        ),
        Some(s) => panic!("Unknown printf format specifier '{s}'"),
    }

    Ok(())
}

/// Prints a string from the provided `args` to the provided formatter `f`.
///
/// # Safety
/// The next argument in `args` must be a pointer to a C string.
unsafe fn print_string(
    args: &mut VaListImpl<'_>,
    params: FormatParameters,
    f: &mut core::fmt::Formatter<'_>,
    pad_char: char,
) -> Result<(), core::fmt::Error> {
    // SAFETY: The next argument is a C string
    let ptr = unsafe { args.arg::<*const u8>() };

    // If max length is specified, string may not be null-terminated
    let bytes = match params.precision {
        Some(precision) => {
            let string_length = (0..precision)
                .take(precision)
                // SAFETY: This read is before any null terminator and before `precision` bytes, so it is part of the string passed to printf
                // Therefore a read of this byte is safe
                .take_while(|i| unsafe { core::ptr::read(ptr.add(*i)) } != 0)
                .count();

            // SAFETY: The calculated byte length is memory in the passed C string and is only used here, so this reference is valid
            unsafe { core::slice::from_raw_parts(ptr, string_length) }
        }
        // SAFETY: If `precision` is not specified, the pointer just points to a null-terminated string.
        None => unsafe { CStr::from_ptr(ptr.cast()).to_bytes() },
    };

    let string =
        core::str::from_utf8(bytes).expect("Shortened string should have been valid utf-8");

    if let Some(minimum_width) = params.minimum_width {
        let pad_width = minimum_width.saturating_sub(string.len());

        if !params.justify_left {
            for _ in 0..pad_width {
                f.write_char(pad_char)?;
            }
        }

        f.write_str(string)?;

        if params.justify_left {
            for _ in 0..pad_width {
                f.write_char(pad_char)?;
            }
        }
    } else {
        f.write_str(string)?;
    }

    Ok(())
}
