use std::sync::Arc;

use bevy::prelude::{default, Deref};
use nom::bytes::complete::tag;
use nom::Parser;

use crate::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum CafManifestFile
{
    SelfRef,
    File(CafFilePath),
}

impl CafManifestFile
{
    pub fn write_to(&self, writer: &mut impl RawSerializer) -> Result<(), std::io::Error>
    {
        match self {
            Self::SelfRef => {
                writer.write_bytes("self".as_bytes())?;
            }
            Self::File(file) => {
                file.write_to(writer)?;
            }
        }
        Ok(())
    }
}

impl Default for CafManifestFile
{
    fn default() -> Self
    {
        Self::File(CafFilePath::default())
    }
}

/*
Parsing:
- Self: match 'self'
- File: file path parses
*/

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Deref)]
pub struct CafManifestKey(pub Arc<str>);

impl CafManifestKey
{
    pub fn write_to(&self, writer: &mut impl RawSerializer) -> Result<(), std::io::Error>
    {
        writer.write_bytes(self.as_bytes())?;
        Ok(())
    }
}

impl Default for CafManifestKey
{
    fn default() -> Self
    {
        Self(Arc::from(""))
    }
}

/*
Parsing: lowercase identifiers, can be a sequence separated by '.' and not ending or starting with '.'
*/

//-------------------------------------------------------------------------------------------------------------------

/// {file} as {key}
#[derive(Debug, Clone, PartialEq)]
pub struct CafManifestEntry
{
    pub entry_fill: CafFill,
    pub file: CafManifestFile,
    pub as_fill: CafFill,
    pub key_fill: CafFill,
    pub key: CafManifestKey,
}

impl CafManifestEntry
{
    pub fn write_to(&self, writer: &mut impl RawSerializer) -> Result<(), std::io::Error>
    {
        self.entry_fill.write_to_or_else(writer, "\n")?;
        self.file.write_to(writer)?;
        self.as_fill.write_to_or_else(writer, " ")?;
        writer.write_bytes("as".as_bytes())?;
        self.key_fill.write_to_or_else(writer, " ")?;
        self.key.write_to(writer)?;
        Ok(())
    }

    // Makes a new entry with default spacing.
    pub fn new(file: impl AsRef<str>, key: impl AsRef<str>) -> Self
    {
        Self {
            file: CafManifestFile::File(CafFilePath(Arc::from(file.as_ref()))),
            key: CafManifestKey(Arc::from(key.as_ref())),
            ..default()
        }
    }
}

impl Default for CafManifestEntry
{
    fn default() -> Self
    {
        Self {
            entry_fill: CafFill::new("\n"),
            file: Default::default(),
            as_fill: CafFill::new(" "),
            key_fill: CafFill::new(" "),
            key: CafManifestKey(Arc::from("")),
        }
    }
}

/*
Parsing:
- Must start with newline.
- Must be 'file as key'.
*/

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CafManifest
{
    pub start_fill: CafFill,
    pub entries: Vec<CafManifestEntry>,
}

impl CafManifest
{
    pub fn write_to(&self, first_section: bool, writer: &mut impl RawSerializer) -> Result<(), std::io::Error>
    {
        let space = if first_section { "" } else { "\n\n" };
        self.start_fill.write_to_or_else(writer, space)?;
        writer.write_bytes("#manifest".as_bytes())?;
        for entry in self.entries.iter() {
            entry.write_to(writer)?;
        }
        Ok(())
    }

    pub fn try_parse(
        content: Span,
        fill: CafFill,
    ) -> Result<(Option<Self>, CafFill, Span), nom::error::Error<Span>>
    {
        let Ok((remaining, _)) = tag::<_, _, ()>("#manifest").parse(content) else {
            return Ok((None, fill, content));
        };

        // TODO

        let manifest = CafManifest { start_fill: fill, entries: vec![] };
        Ok((Some(manifest), CafFill::default(), remaining))
    }
}

impl Default for CafManifest
{
    fn default() -> Self
    {
        Self { start_fill: CafFill::default(), entries: Vec::default() }
    }
}

//-------------------------------------------------------------------------------------------------------------------
