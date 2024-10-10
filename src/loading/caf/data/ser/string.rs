//! This file was adapted from the `serde_json` crate.

//-------------------------------------------------------------------------------------------------------------------

// Characters for use in literal escape sequences.
const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const UU: u8 = b'u'; // \x00...\x1F except the ones above
const __: u8 = 0;

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

//-------------------------------------------------------------------------------------------------------------------

/// Represents a character escape code in a type-safe manner.
enum CharEscape
{
    /// An escaped quote `"`
    Quote,
    /// An escaped reverse solidus `\`
    ReverseSolidus,
    // /// An escaped solidus `/`
    // Solidus,
    /// An escaped backspace character (usually escaped as `\b`)
    Backspace,
    /// An escaped form feed character (usually escaped as `\f`)
    FormFeed,
    /// An escaped line feed character (usually escaped as `\n`)
    LineFeed,
    /// An escaped carriage return character (usually escaped as `\r`)
    CarriageReturn,
    /// An escaped tab character (usually escaped as `\t`)
    Tab,
    /// An escaped ASCII plane control character (usually escaped as
    /// `\u00XX` where `XX` are two hex characters)
    AsciiControl(u8),
}

impl CharEscape
{
    #[inline]
    fn from_escape_table(escape: u8, byte: u8) -> CharEscape
    {
        match escape {
            BB => CharEscape::Backspace,
            TT => CharEscape::Tab,
            NN => CharEscape::LineFeed,
            FF => CharEscape::FormFeed,
            RR => CharEscape::CarriageReturn,
            QU => CharEscape::Quote,
            BS => CharEscape::ReverseSolidus,
            UU => CharEscape::AsciiControl(byte),
            _ => unreachable!(),
        }
    }

    fn write_to(self, writer: &mut impl std::io::Write) -> std::io::Result<()>
    {
        let s = match self {
            Self::Quote => b"\\\"",
            Self::ReverseSolidus => b"\\\\",
            // Self::Solidus => b"\\/",
            Self::Backspace => b"\\b",
            Self::FormFeed => b"\\f",
            Self::LineFeed => b"\\n",
            Self::CarriageReturn => b"\\r",
            Self::Tab => b"\\t",
            Self::AsciiControl(byte) => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                let bytes = &[
                    b'\\',
                    b'u',
                    b'{',
                    HEX_DIGITS[(byte >> 4) as usize],
                    HEX_DIGITS[(byte & 0xF) as usize],
                    b'}',
                ];
                return writer.write_all(bytes);
            }
        };

        writer.write_all(s)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Converts a string containing special characters into a JSON-serialized string with escape sequences.
// TODO: rework this so ASCII-escaped are escaped properly, as well as unicode-escaped; see
// char::escape_debug/escape_unicode
pub(crate) fn format_escaped_str_contents(writer: &mut impl std::io::Write, value: &str) -> std::io::Result<()>
{
    let bytes = value.as_bytes();

    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            writer.write_all(&value[start..i].as_bytes())?;
        }

        let char_escape = CharEscape::from_escape_table(escape, byte);
        char_escape.write_to(writer)?;

        start = i + 1;
    }

    if start == bytes.len() {
        return Ok(());
    }

    writer.write_all(&value[start..].as_bytes())
}

//-------------------------------------------------------------------------------------------------------------------
