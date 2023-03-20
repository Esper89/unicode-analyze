use std::fmt::{self, Display};
use phf::{phf_map, phf_set};

// TODO Replace `ucd` and `unicode_names2` dependencies with custom tables.

pub trait UnicodeCharacter
{
    type Name: Display;

    fn name(&self) -> Option<Self::Name>;

    fn width(&self) -> usize;

    fn control_code(&self) -> Option<ControlCode>;

    fn diacritic(&self) -> Diacritic;

    fn direction(&self) -> Direction;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ControlCode { pub code: &'static str, pub name: &'static str }

#[derive(Debug, Clone)]
pub enum Diacritic { No, Single, Double }

#[derive(Debug, Clone)]
pub enum Direction { LeftToRight, RightToLeft, Neutral }

impl UnicodeCharacter for char
{
    type Name = CharName;

    fn name(&self) -> Option<CharName> { unicode_names2::name(*self).map(CharName) }

    fn width(&self) -> usize { unicode_width::UnicodeWidthChar::width(*self).unwrap_or(0) }

    fn control_code(&self) -> Option<ControlCode>
    {
        CONTROL_CODES.get(&self).map(|(code, name)| ControlCode { code, name })
    }

    fn diacritic(&self) -> Diacritic
    {
        use Diacritic::*;

        if !ucd::Codepoint::is_grapheme_extend(*self) { No }
        else if DOUBLE_WIDTH_DIACRITICS.contains(&self) { Double }
        else { Single }
    }

    fn direction(&self) -> Direction
    {
        use Direction::*;
        use ucd::BidiClass as Bidi;

        match ucd::Codepoint::bidi_class(*self)
        {
            Bidi::LeftToRight => LeftToRight,
            Bidi::RightToLeft => RightToLeft,
            Bidi::ArabicLetter => RightToLeft,
            Bidi::LeftToRightEmbedding => LeftToRight,
            Bidi::LeftToRightOverride => LeftToRight,
            Bidi::RightToLeftEmbedding => RightToLeft,
            Bidi::RightToLeftOverride => RightToLeft,
            Bidi::LeftToRightIsolate => LeftToRight,
            Bidi::RightToLeftIsolate => RightToLeft,
            _ => Neutral,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharName(unicode_names2::Name);

impl Display for CharName
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

// Characters that should be replaced with abbreviations.
const CONTROL_CODES: phf::Map<char, (&str, &str)> = phf_map!
{
    // Block: Basic Latin
    '\0' => ("NUL", "NULL"),
    '\x01' => ("SOH", "START OF HEADING"),
    '\x02' => ("STX", "START OF TEXT"),
    '\x03' => ("ETX", "END OF TEXT"),
    '\x04' => ("EOT", "END OF TRANSMISSION"),
    '\x05' => ("ENQ", "ENQUIRY"),
    '\x06' => ("ACK", "ACKNOWLEDGE"),
    '\x07' => ("BEL", "ALERT"),
    '\x08' => ("BS", "BACKSPACE"),
    '\t' => ("HT", "CHARACTER TABULATION"),
    '\n' => ("LF", "LINE FEED"),
    '\x0B' => ("VT", "LINE TABULATION"),
    '\x0C' => ("FF", "FORM FEED"),
    '\r' => ("CR", "CARRIAGE RETURN"),
    '\x0E' => ("SO", "SHIFT OUT"),
    '\x0F' => ("SI", "SHIFT IN"),
    '\x10' => ("DLE", "DATA LINK ESCAPE"),
    '\x11' => ("DC1", "DEVICE CONTROL 1"),
    '\x12' => ("DC2", "DEVICE CONTROL 2"),
    '\x13' => ("DC3", "DEVICE CONTROL 3"),
    '\x14' => ("DC4", "DEVICE CONTROL 4"),
    '\x15' => ("NAK", "NEGATIVE ACKNOWLEDGE"),
    '\x16' => ("SYN", "SYNCHRONOUS IDLE"),
    '\x17' => ("ETB", "END OF TRANSMISSION BLOCK"),
    '\x18' => ("CAN", "CANCEL"),
    '\x19' => ("EM", "END OF MEDIUM"),
    '\x1A' => ("SUB", "SUBSTITUTE"),
    '\x1B' => ("ESC", "ESCAPE"),
    '\x1C' => ("FS", "INFORMATION SEPARATOR FOUR"),
    '\x1D' => ("GS", "INFORMATION SEPARATOR THREE"),
    '\x1E' => ("RS", "INFORMATION SEPARATOR TWO"),
    '\x1F' => ("US", "INFORMATION SEPARATOR ONE"),
    '\x7F' => ("DEL", "DELETE"),

    // Block: Latin-1 Supplement
    '\u{0080}' => ("PAD", "PADDING CHARACTER"),
    '\u{0081}' => ("HOP", "HIGH OCTET PRESET"),
    '\u{0082}' => ("BPH", "BREAK PERMITTED HERE"),
    '\u{0083}' => ("NBH", "NO BREAK HERE"),
    '\u{0084}' => ("IND", "INDEX"),
    '\u{0085}' => ("NEL", "NEXT LINE"),
    '\u{0086}' => ("SSA", "START OF SELECTED AREA"),
    '\u{0087}' => ("ESA", "END OF SELECTED AREA"),
    '\u{0088}' => ("HTS", "CHARACTER TABULATION SET"),
    '\u{0089}' => ("HTJ", "CHARACTER TABULATION WITH JUSTIFICATION"),
    '\u{008A}' => ("VTS", "VERTICAL TABULATION SET"),
    '\u{008B}' => ("PLD", "PARTIAL LINE FORWARD"),
    '\u{008C}' => ("PLU", "PARTIAL LINE BACKWARD"),
    '\u{008D}' => ("RI", "REVERSE LINE FEED"),
    '\u{008E}' => ("SS2", "SINGLE SHIFT TWO"),
    '\u{008F}' => ("SS3", "SINGLE SHIFT THREE"),
    '\u{0090}' => ("DCS", "DEVICE CONTROL STRING"),
    '\u{0091}' => ("PU1", "PRIVATE USE ONE"),
    '\u{0092}' => ("PU2", "PRIVATE USE TWO"),
    '\u{0093}' => ("STS", "SET TRANSMIT STATE"),
    '\u{0094}' => ("CCH", "CANCEL CHARACTER"),
    '\u{0095}' => ("MW", "MESSAGE WAITING"),
    '\u{0096}' => ("SPA", "START OF GUARDED AREA"),
    '\u{0097}' => ("EPA", "END OF GUARDED AREA"),
    '\u{0098}' => ("SOS", "START OF STRING"),
    '\u{0099}' => ("SGC", "SINGLE GRAPHIC CHARACTER INTRODUCER"),
    '\u{009A}' => ("SCI", "SINGLE CHARACTER INTRODUCER"),
    '\u{009B}' => ("CSI", "CONTROL SEQUENCE INTRODUCER"),
    '\u{009C}' => ("ST", "STRING TERMINATOR"),
    '\u{009D}' => ("OSC", "OPERATING SYSTEM COMMAND"),
    '\u{009E}' => ("PM", "PRIVACY MESSAGE"),
    '\u{009F}' => ("APC", "APPLICATION PROGRAM COMMAND"),
    '\u{00AD}' => ("SHY", "SOFT HYPHEN"),

    // Block: Combining Diacritical Marks
    '\u{034F}' => ("CGJ", "COMBINING GRAPHEME JOINER"),

    // Block: Arabic
    '\u{061C}' => ("ALM", "ARABIC LETTER MARK"),

    // Block: Gemeral Punctuation
    '\u{200B}' => ("ZWSP", "ZERO WIDTH SPACE"),
    '\u{200C}' => ("ZWNJ", "ZERO WIDTH NON-JOINER"),
    '\u{200D}' => ("ZWJ", "ZERO WIDTH JOINER"),
    '\u{200E}' => ("LRM", "LEFT-TO-RIGHT MARK"),
    '\u{200F}' => ("RLM", "RIGHT-TO-LEFT MARK"),
    '\u{2028}' => ("LS", "LINE SEPARATOR"),
    '\u{2029}' => ("PS", "PARAGRAPH SEPARATOR"),
    '\u{202A}' => ("LRE", "LEFT-TO-RIGHT EMBEDDING"),
    '\u{202D}' => ("LRO", "LEFT-TO-RIGHT OVERRIDE"),
    '\u{202B}' => ("RLE", "RIGHT-TO-LEFT EMBEDDING"),
    '\u{202E}' => ("RLO", "RIGHT-TO-LEFT OVERRIDE"),
    '\u{202C}' => ("PDF", "POP DIRECTIONAL FORMATTING"),
    '\u{2060}' => ("WJ", "WORD JOINER"),
    '\u{2066}' => ("LRI", "LEFT-TO-RIGHT ISOLATE"),
    '\u{2067}' => ("RLI", "RIGHT-TO-LEFT ISOLATE"),
    '\u{2068}' => ("FSI", "FIRST STRONG ISOLATE"),
    '\u{2069}' => ("PDI", "POP DIRECTIONAL ISOLATE"),

    // Block: Variation Selectors
    '\u{FE00}' => ("VS1", "VARIATION SELECTOR-1"),
    '\u{FE01}' => ("VS2", "VARIATION SELECTOR-2"),
    '\u{FE02}' => ("VS3", "VARIATION SELECTOR-3"),
    '\u{FE03}' => ("VS4", "VARIATION SELECTOR-4"),
    '\u{FE04}' => ("VS5", "VARIATION SELECTOR-5"),
    '\u{FE05}' => ("VS6", "VARIATION SELECTOR-6"),
    '\u{FE06}' => ("VS7", "VARIATION SELECTOR-7"),
    '\u{FE07}' => ("VS8", "VARIATION SELECTOR-8"),
    '\u{FE08}' => ("VS9", "VARIATION SELECTOR-9"),
    '\u{FE09}' => ("VS10", "VARIATION SELECTOR-10"),
    '\u{FE0A}' => ("VS11", "VARIATION SELECTOR-11"),
    '\u{FE0B}' => ("VS12", "VARIATION SELECTOR-12"),
    '\u{FE0C}' => ("VS13", "VARIATION SELECTOR-13"),
    '\u{FE0D}' => ("VS14", "VARIATION SELECTOR-14"),
    '\u{FE0E}' => ("VS15", "VARIATION SELECTOR-15"),
    '\u{FE0F}' => ("VS16", "VARIATION SELECTOR-16"),

    // Block: Arabic Presentation Forms-B
    '\u{FEFF}' => ("BOM", "BYTE ORDER MARK"),

    // Block: Specials
    '\u{FFF9}' => ("IAA", "INTERLINEAR ANNOTATION ANCHOR"),
    '\u{FFFA}' => ("IAS", "INTERLINEAR ANNOTATION SEPARATOR"),
    '\u{FFFB}' => ("IAT", "INTERLINEAR ANNOTATION TERMINATOR"),
};

// Combining characters that need placeholder characters before and after.
const DOUBLE_WIDTH_DIACRITICS: phf::Set<char> = phf_set!
{
    // Block: Combining Diacritical Marks
    '\u{035C}', // COMBINING DOUBLE BREVE BELOW
    '\u{035D}', // COMBINING DOUBLE BREVE
    '\u{035E}', // COMBINING DOUBLE MACRON
    '\u{035F}', // COMBINING DOUBLE MACRON BELOW
    '\u{0360}', // COMBINING DOUBLE TILDE
    '\u{0361}', // COMBINING DOUBLE INVERTED BREVE
    '\u{0362}', // COMBINING DOUBLE RIGHTWARDS ARROW BELOW

    // Block: Combining Diacritical Marks Supplement
    '\u{1DCD}', // COMBINING DOUBLE CIRCUMFLEX ABOVE
    '\u{1DFC}', // COMBINING DOUBLE INVERTED BREVE BELOW
};
