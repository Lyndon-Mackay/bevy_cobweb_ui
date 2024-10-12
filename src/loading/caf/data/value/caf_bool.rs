use crate::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct CafBool
{
    pub fill: CafFill,
    pub value: bool,
}

impl CafBool
{
    pub fn write_to(&self, writer: &mut impl std::io::Write) -> Result<(), std::io::Error>
    {
        self.write_to_with_space(writer, "")
    }

    pub fn write_to_with_space(&self, writer: &mut impl std::io::Write, space: &str)
        -> Result<(), std::io::Error>
    {
        self.fill.write_to_or_else(writer, space)?;
        let string = match self.value {
            true => "true",
            false => "false",
        };
        writer.write(string.as_bytes())?;
        Ok(())
    }

    pub fn recover_fill(&mut self, other: &Self)
    {
        self.fill.recover(&other.fill);
    }
}

impl From<bool> for CafBool
{
    fn from(value: bool) -> Self
    {
        Self { fill: CafFill::default(), value }
    }
}

/*
Parsing:
- parse as string
*/

//-------------------------------------------------------------------------------------------------------------------
