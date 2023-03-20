#![feature(utf8_chunks)]

use std::{cmp, ffi::OsStr, fmt::{self, Display}, hash, ops, str::Utf8Chunks};
use either::Either;
use os_str_bytes::OsStrBytes;
use smallvec::{smallvec, SmallVec};
use unicode_segmentation::UnicodeSegmentation;
use unicode::{CharName, UnicodeCharacter, Diacritic, Direction};

mod unicode;

// TODO Reduce the size of each `Codepoint`.
// TODO Replace the `utf8_chunks` unstable feature with something else.
// TODO Add doc comments.

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash, Default)]
pub struct Text(Vec<Grapheme>);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash, Default)]
pub struct Grapheme(SmallVec<[Codepoint; 1]>);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct Codepoint(CodepointKind);

#[derive(Debug, Clone)]
enum CodepointKind
{
    Character { c: char, width: u8, name: CharName, diacritic: Diacritic, direction: Direction },
    ControlCode { c: char, code: &'static str, name: &'static str },
    NonCharacter(char),
    PrivateUse(char),
    Unknown(char),
    Invalid(u8),
}

impl Text
{
    pub fn parse_str(text: &str) -> Self
    {
        Text(text.graphemes(true).map(Grapheme::from_valid).collect())
    }

    pub fn parse_os_str(text: &OsStr) -> Self { Text::parse_bytes(&text.to_raw_bytes()) }

    pub fn parse_bytes(text: &[u8]) -> Self
    {
        enum Utf8<'a> { Valid(&'a str), Invalid(&'a [u8]) }

        Text(Utf8Chunks::new(text)
            .flat_map(|chunk|
            {
                let (valid, invalid) = (chunk.valid(), chunk.invalid());
                let mut items = SmallVec::<[Utf8<'_>; 2]>::new();

                if !valid.is_empty() { items.push(Utf8::Valid(valid)) }
                if !invalid.is_empty() { items.push(Utf8::Invalid(invalid)) }

                items
            })
            .flat_map(|chunk| match chunk
            {
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

    pub fn codepoints(&self) -> impl Iterator<Item = &Codepoint>
    {
        self.graphemes().flat_map(Grapheme::codepoints)
    }

    pub fn into_codepoints(self) -> impl Iterator<Item = Codepoint>
    {
        self.into_graphemes().flat_map(Grapheme::into_codepoints)
    }
}

impl Grapheme
{
    pub fn from_valid(s: &str) -> Self { Grapheme(s.chars().map(Codepoint::from_valid).collect()) }

    pub fn from_invalid(byte: u8) -> Self { Grapheme(smallvec![Codepoint::from_invalid(byte)]) }

    pub fn codepoints(&self) -> impl Iterator<Item = &Codepoint> { self.0.iter() }
    pub fn into_codepoints(self) -> impl Iterator<Item = Codepoint> { self.0.into_iter() }
}

impl Codepoint
{
    pub fn from_valid(c: char) -> Self
    {
        use CodepointKind::*;

        Codepoint(match c
        {
            '\u{FDD0}'..='\u{FDEF}' => NonCharacter(c),
            c if c as u16 | 1 == 0xFFFF => NonCharacter(c),
            '\u{E000}'..='\u{F8FF}' | '\u{F0000}'.. => PrivateUse(c),
            c => if let Some(ctrl) = c.control_code()
            {
                ControlCode { c, code: ctrl.code, name: ctrl.name }
            }
            else if let Some(name) = c.name()
            {
                Character
                {
                    c,
                    width: c.width() as u8,
                    name,
                    diacritic: c.diacritic(),
                    direction: c.direction(),
                }
            }
            else { Unknown(c) },
        })
    }

    pub fn from_invalid(byte: u8) -> Self { Codepoint(CodepointKind::Invalid(byte)) }

    pub fn value(&self) -> Result<char, u8> { self.0.value() }
    pub fn name(&self) -> String { self.display_name().to_string() }

    pub fn display_value(&self) -> impl Display + '_
    {
        display_with(|f| match self.value()
        {
            Ok(c @ ..='\u{FFFF}') => write!(f, "U+{:04X}", c as u16),
            Ok(c @ '\u{10000}'..) => write!(f, "U+{:06X}", c as u32),
            Err(b) => write!(f, "0x{b:02X}"),
        })
    }

    pub fn display_character(&self) -> impl Display + '_
    {
        use CodepointKind::*;
        use Diacritic::*;
        use Direction::*;

        display_with(|f| match self.0
        {
            Character { c, diacritic: Single, direction: RightToLeft, .. }
                => write!(f, "'\u{200E}\u{200F}◌{c}\u{200F}\u{200E}'"),
            Character { c, diacritic: Double, direction: RightToLeft, .. }
                => write!(f, "'\u{200E}\u{200F}◌{c}◌\u{200F}\u{200E}'"),
            Character { c, diacritic: Single, .. } => write!(f, "'◌{c}'"),
            Character { c, diacritic: Double, .. } => write!(f, "'◌{c}◌'"),
            Character { width: 0, .. } => write!(f, "''"),
            Character { c, direction: RightToLeft, .. } => write!(f, "'\u{200E}{c}\u{200E}'"),
            Character { c, .. } => write!(f, "'{c}'"),
            ControlCode { code, .. } => write!(f, "{code}"),
            NonCharacter(_) => write!(f, "∅"),
            PrivateUse(_) => write!(f, "▨"),
            Unknown(_) => write!(f, "?"),
            Invalid(_) => write!(f, "�"),
        })
    }

    pub fn display_name(&self) -> impl Display + '_
    {
        use CodepointKind::*;

        display_with(|f| match &self.0
        {
            Character { name, .. } => name.fmt(f),
            ControlCode { name, .. } => name.fmt(f),
            NonCharacter(_) => f.write_str("NOT A CHARACTER"),
            PrivateUse(_) => f.write_str("RESERVED FOR PRIVATE USE"),
            Unknown(_) => f.write_str("UNKNOWN CHARACTER"),
            Invalid(_) => f.write_str("INVALID UTF-8"),
        })
    }
}

fn display_with(f: impl Fn(&mut fmt::Formatter<'_>) -> fmt::Result) -> impl Display
{
    struct Displayer<F>(F);

    impl<F> Display for Displayer<F> where F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0(f) }
    }

    Displayer(f)
}

impl Display for Text
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut graphemes = self.graphemes().filter(|g| g.len() > 0);
        f.write_str("[")?;

        if let Some(first) = graphemes.next() { first.fmt(f)?; }
        for grapheme in graphemes
        {
            f.write_str(", ")?;
            grapheme.fmt(f)?;
        }

        f.write_str("]")
    }
}

impl Display for Grapheme
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &**self
        {
            [c] => c.fmt(f),
            _ =>
            {
                let mut codepoints = self.codepoints();
                f.write_str("[")?;

                if let Some(first) = codepoints.next() { first.fmt(f)?; }
                for codepoint in codepoints
                {
                    f.write_str(" + ")?;
                    codepoint.fmt(f)?;
                }

                f.write_str("]")
            },
        }
    }
}

impl Display for Codepoint
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        use CodepointKind::*;

        match self.0
        {
            Character { .. } | ControlCode { .. } => self.display_character().fmt(f),
            NonCharacter(_) | PrivateUse(_) | Unknown(_) | Invalid(_)
                => self.display_value().fmt(f),
        }
    }
}

impl CodepointKind
{
    fn value(&self) -> Result<char, u8>
    {
        use CodepointKind::*;

        match &self
        {
            Character { c, .. } |
            NonCharacter(c) |
            ControlCode { c, .. } |
            PrivateUse(c) |
            Unknown(c) => Ok(*c),
            Invalid(b) => Err(*b),
        }
    }
}

impl cmp::Eq for CodepointKind { }

impl cmp::PartialEq for CodepointKind
{
    fn eq(&self, other: &Self) -> bool { self.value() == other.value() }
}

impl cmp::PartialOrd for CodepointKind
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        self.value().partial_cmp(&other.value())
    }
}

impl hash::Hash for CodepointKind
{
    fn hash<H: hash::Hasher>(&self, state: &mut H)
    {
        self.value().hash(state)
    }
}

impl ops::Deref for Text
{
    type Target = [Grapheme];
    fn deref(&self) -> &[Grapheme] { &self.0[..] }
}

impl ops::DerefMut for Text
{
    fn deref_mut(&mut self) -> &mut [Grapheme] { &mut self.0[..] }
}

impl ops::Deref for Grapheme
{
    type Target = [Codepoint];
    fn deref(&self) -> &[Codepoint] { &self.0[..] }
}

impl ops::DerefMut for Grapheme
{
    fn deref_mut(&mut self) -> &mut [Codepoint] { &mut self.0[..] }
}
