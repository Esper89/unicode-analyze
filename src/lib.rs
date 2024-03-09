#![feature(utf8_chunks)]

use std::{cmp, ffi::OsStr, fmt::{self, Display}, hash, ops, str::Utf8Chunks};
use either::Either;
use smallvec::{smallvec, SmallVec};
use unicode_segmentation::UnicodeSegmentation;
use unicode::{CharName, Character, Diacritic, Direction};

mod unicode;

// TODO: Reduce the size of each `Codepoint` or switch to some kind of iteration.

// TODO: Add doc comments.

// TODO: Replace the `utf8_chunks` unstable feature with something else.

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash, Default)]
pub struct Text(Vec<Grapheme>);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash, Default)]
pub struct Grapheme(SmallVec<[Codepoint; 1]>);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct Codepoint(CodepointKind);

#[derive(Debug, Clone)]
enum CodepointKind {
    Character { c: char, width: u8, name: CharName, diacritic: Diacritic, direction: Direction },
    ControlCode { c: char, code: &'static str, name: &'static str },
    NonCharacter(char),
    PrivateUse(char),
    Unknown(char),
    Invalid(u8),
}

impl Text {
    pub fn parse_str(text: &str) -> Self {
        Text(text.graphemes(true).map(Grapheme::from_valid).collect())
    }

    pub fn parse_os_str(text: &OsStr) -> Self {
        Text::parse_bytes(text.as_encoded_bytes())
    }

    pub fn parse_bytes(text: &[u8]) -> Self {
        enum Utf8<'a> {
            Valid(&'a str),
            Invalid(&'a [u8]),
        }

        Text(Utf8Chunks::new(text)
            .flat_map(|chunk| {
                let (valid, invalid) = (chunk.valid(), chunk.invalid());
                let mut items = SmallVec::<[Utf8; 2]>::new();

                if !valid.is_empty() { items.push(Utf8::Valid(valid)) }
                if !invalid.is_empty() { items.push(Utf8::Invalid(invalid)) }

                items
            })
            .flat_map(|chunk| match chunk {
                Utf8::Valid(chunk) => Either::Left(chunk.graphemes(true).map(Grapheme::from_valid)),
                Utf8::Invalid(chunk) => Either::Right(
                    chunk.iter().copied().map(Grapheme::from_invalid)
                ),
            })
            .collect()
        )
    }

    pub fn graphemes(&self) -> impl Iterator<Item = &Grapheme> { self.0.iter() }
    pub fn into_graphemes(self) -> impl Iterator<Item = Grapheme> { self.0.into_iter() }

    pub fn codepoints(&self) -> impl Iterator<Item = &Codepoint> {
        self.graphemes().flat_map(Grapheme::codepoints)
    }

    pub fn into_codepoints(self) -> impl Iterator<Item = Codepoint> {
        self.into_graphemes().flat_map(Grapheme::into_codepoints)
    }
}

impl Grapheme {
    pub fn from_valid(s: &str) -> Self {
        Grapheme(s.chars().map(Codepoint::from_valid).collect())
    }

    pub fn from_invalid(byte: u8) -> Self {
        Grapheme(smallvec![Codepoint::from_invalid(byte)])
    }

    pub fn codepoints(&self) -> impl Iterator<Item = &Codepoint> { self.0.iter() }
    pub fn into_codepoints(self) -> impl Iterator<Item = Codepoint> { self.0.into_iter() }
}

impl Codepoint {
    pub fn from_valid(c: char) -> Self {
        use CodepointKind as Ck;

        Codepoint(match c {
            '\u{FDD0}'..='\u{FDEF}' => Ck::NonCharacter(c),
            c if c as u16 | 1 == 0xFFFF => Ck::NonCharacter(c),
            '\u{E000}'..='\u{F8FF}' | '\u{F0000}'.. => Ck::PrivateUse(c),
            c => if let Some(ctrl) = c.control_code() {
                Ck::ControlCode { c, code: ctrl.code, name: ctrl.name }
            } else if let Some(name) = c.name() {
                Ck::Character {
                    c,
                    width: c.width() as u8,
                    name,
                    diacritic: c.diacritic(),
                    direction: c.direction(),
                }
            } else { Ck::Unknown(c) },
        })
    }

    pub fn from_invalid(byte: u8) -> Self {
        Codepoint(CodepointKind::Invalid(byte))
    }

    pub fn value(&self) -> Result<char, u8> { self.0.value() }
    pub fn name(&self) -> String { self.display_name().to_string() }

    pub fn display_value(&self) -> impl Display + '_ {
        display_with(|f| match self.value() {
            Ok(c @ ..='\u{FFFF}') => write!(f, "U+{:04X}", c as u16),
            Ok(c @ '\u{10000}'..) => write!(f, "U+{:06X}", c as u32),
            Err(b) => write!(f, "0x{b:02X}"),
        })
    }

    pub fn display_character(&self) -> impl Display + '_ {
        use CodepointKind as Ck;
        use Diacritic as Dc;
        use Direction as Dir;

        display_with(|f| match self.0 {
            Ck::Character { c, diacritic: Dc::Single, direction: Dir::Rtl, .. }
                => write!(f, "'\u{200E}\u{200F}◌{c}\u{200F}\u{200E}'"),
            Ck::Character { c, diacritic: Dc::Double, direction: Dir::Rtl, .. }
                => write!(f, "'\u{200E}\u{200F}◌{c}◌\u{200F}\u{200E}'"),
            Ck::Character { c, diacritic: Dc::Single, .. } => write!(f, "'◌{c}'"),
            Ck::Character { c, diacritic: Dc::Double, .. } => write!(f, "'◌{c}◌'"),
            Ck::Character { width: 0, .. } => write!(f, "''"),
            Ck::Character { c, direction: Dir::Rtl, .. } => write!(f, "'\u{200E}{c}\u{200E}'"),
            Ck::Character { c, .. } => write!(f, "'{c}'"),
            Ck::ControlCode { code, .. } => write!(f, "{code}"),
            Ck::NonCharacter(_) => write!(f, "∅"),
            Ck::PrivateUse(_) => write!(f, "▨"),
            Ck::Unknown(_) => write!(f, "?"),
            Ck::Invalid(_) => write!(f, "�"),
        })
    }

    pub fn display_name(&self) -> impl Display + '_ {
        use CodepointKind as Ck;

        display_with(|f| match &self.0 {
            Ck::Character { name, .. } => name.fmt(f),
            Ck::ControlCode { name, .. } => name.fmt(f),
            Ck::NonCharacter(_) => f.write_str("NOT A CHARACTER"),
            Ck::PrivateUse(_) => f.write_str("RESERVED FOR PRIVATE USE"),
            Ck::Unknown(_) => f.write_str("UNKNOWN CHARACTER"),
            Ck::Invalid(_) => f.write_str("INVALID UTF-8"),
        })
    }
}

fn display_with(f: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl Display {
    struct Displayer<F>(F);
    impl<F> Display for Displayer<F> where F: Fn(&mut fmt::Formatter) -> fmt::Result {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0(f) }
    }

    Displayer(f)
}

impl Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut graphemes = self.graphemes().filter(|g| g.len() > 0);
        f.write_str("[")?;

        if let Some(first) = graphemes.next() { first.fmt(f)?; }
        for grapheme in graphemes {
            f.write_str(", ")?;
            grapheme.fmt(f)?;
        }

        f.write_str("]")
    }
}

impl Display for Grapheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let [c] = &**self { c.fmt(f) } else {
            let mut codepoints = self.codepoints();
            f.write_str("[")?;

            if let Some(first) = codepoints.next() { first.fmt(f)?; }
            for codepoint in codepoints {
                f.write_str(" + ")?;
                codepoint.fmt(f)?;
            }

            f.write_str("]")
        }
    }
}

impl Display for Codepoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CodepointKind as Ck;

        match self.0 {
            Ck::Character { .. } | Ck::ControlCode { .. } => self.display_character().fmt(f),
            Ck::NonCharacter(_) | Ck::PrivateUse(_) | Ck::Unknown(_) | Ck::Invalid(_)
                => self.display_value().fmt(f),
        }
    }
}

impl CodepointKind {
    fn value(&self) -> Result<char, u8> {
        use CodepointKind as Ck;

        match &self {
            Ck::Character { c, .. } |
            Ck::NonCharacter(c) |
            Ck::ControlCode { c, .. } |
            Ck::PrivateUse(c) |
            Ck::Unknown(c) => Ok(*c),
            Ck::Invalid(b) => Err(*b),
        }
    }
}

impl cmp::Eq for CodepointKind { }

impl cmp::PartialEq for CodepointKind {
    fn eq(&self, other: &Self) -> bool { self.value() == other.value() }
}

impl cmp::PartialOrd for CodepointKind {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl hash::Hash for CodepointKind {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value().hash(state)
    }
}

impl ops::Deref for Text {
    type Target = [Grapheme];
    fn deref(&self) -> &[Grapheme] { &self.0[..] }
}

impl ops::DerefMut for Text {
    fn deref_mut(&mut self) -> &mut [Grapheme] { &mut self.0[..] }
}

impl ops::Deref for Grapheme {
    type Target = [Codepoint];
    fn deref(&self) -> &[Codepoint] { &self.0[..] }
}

impl ops::DerefMut for Grapheme {
    fn deref_mut(&mut self) -> &mut [Codepoint] { &mut self.0[..] }
}
