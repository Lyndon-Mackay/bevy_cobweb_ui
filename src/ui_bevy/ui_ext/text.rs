use bevy::prelude::*;
use bevy::text::LineBreak;
use bevy::ui::ContentSize;
use bevy_cobweb::prelude::*;

use crate::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

const TEXT_LINE_DEFAULT_TEXT: &str = "[[text line]]";

//-------------------------------------------------------------------------------------------------------------------

fn insert_text_line(
    In((entity, mut line)): In<(Entity, TextLine)>,
    mut commands: Commands,
    localizer: Res<TextLocalizer>,
    font_map: Res<FontMap>,
    color: Query<&TextLineColor>,
    mut localized: Query<&mut LocalizedText>,
)
{
    // Prep color.
    let color = color
        .get(entity)
        .map(|c| c.0)
        .unwrap_or_else(|_| TextLine::default_color());

    // Get font.
    let mut font = line.font.map(|f| font_map.get(&f)).unwrap_or_default();

    // Prep localization.
    // - We need to manually localize inserted text in case the text line is hot reloaded into an entity that
    //   already has Text (i.e. because auto-localization won't occur).
    // TODO: future localization rework should make this no longer necessary
    if line.text.as_str() != TEXT_LINE_DEFAULT_TEXT {
        if let Ok(mut localized) = localized.get_mut(entity) {
            localized.set_localization(line.text.as_str());
            //todo: what happens if line.font is None? it should use bevy's default font
            localized.localization_mut().set_font_backup(font.clone());
            localized.localize(&localizer, &font_map, &mut line.text, &mut font);
        }
    }

    // Add text to entity.
    let Some(mut ec) = commands.get_entity(entity) else { return };
    ec.try_insert((
        Text(line.text),
        TextLayout { justify: line.justify, linebreak: line.linebreak },
        TextFont { font, font_size: line.size, ..default() },
        TextColor(color),
    ));
}

//-------------------------------------------------------------------------------------------------------------------

/// Sets up an entity with a [`Text`] component and one text span.
///
/// The default font is "Fira Sans Medium" with size `25.0`.
#[derive(Reflect, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLine
{
    /// The starting text string.
    #[reflect(default = "TextLine::default_text")]
    pub text: String,
    /// The font handle.
    ///
    /// Defaults to the built-in "Fira Sans Medium" font.
    #[reflect(default = "TextLine::default_font")]
    pub font: Option<FontRequest>,
    /// The desired font size.
    ///
    /// Defaults to `25.0`.
    #[reflect(default = "TextLine::default_font_size")]
    pub size: f32,
    /// The line's [`LineBreak`] behavior.
    ///
    /// Defaults to [`LineBreak::NoWrap`].
    #[reflect(default = "TextLine::default_line_break")]
    pub linebreak: LineBreak,
    /// The line's [`JustifyText`] behavior.
    ///
    /// Defaults to [`JustifyText::Left`].
    #[reflect(default = "TextLine::default_justify_text")]
    pub justify: JustifyText,
}

impl TextLine
{
    pub fn from_text(text: impl Into<String>) -> Self
    {
        Self { text: text.into(), ..default() }
    }

    pub fn with_font(mut self, font: impl Into<FontRequest>) -> Self
    {
        self.font = Some(font.into());
        self
    }

    fn default_text() -> String
    {
        TEXT_LINE_DEFAULT_TEXT.into()
    }

    fn default_font() -> Option<FontRequest>
    {
        Some(FontRequest::new_static("Fira Sans").medium())
    }

    fn default_font_size() -> f32
    {
        25.
    }

    fn default_color() -> Color
    {
        Color::WHITE
    }

    fn default_line_break() -> LineBreak
    {
        LineBreak::NoWrap
    }

    fn default_justify_text() -> JustifyText
    {
        JustifyText::Left
    }
}

impl Instruction for TextLine
{
    fn apply(self, entity: Entity, world: &mut World)
    {
        world.syscall((entity, self), insert_text_line);
    }

    fn revert(entity: Entity, world: &mut World)
    {
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.remove_with_requires::<(Text, ContentSize)>();
        });
    }
}

impl Default for TextLine
{
    fn default() -> Self
    {
        Self {
            text: Self::default_text(),
            font: Self::default_font(),
            size: Self::default_font_size(),
            linebreak: Self::default_line_break(),
            justify: Self::default_justify_text(),
        }
    }
}

impl StaticAttribute for TextLine
{
    type Value = Self;
    fn construct(value: Self::Value) -> Self
    {
        value
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Instruction for setting the font size of a [`TextLine`] on an entity.
//todo: hook this up to TextLine or find a better abstraction
#[derive(Reflect, Component, Default, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct TextLineSize(pub f32);

impl Instruction for TextLineSize
{
    fn apply(self, entity: Entity, world: &mut World)
    {
        world.syscall(
            (entity, self.0),
            |In((id, size)): In<(Entity, f32)>, mut editor: TextEditor| {
                editor.set_font_size(id, size);
            },
        );
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.insert(self);
        });
    }

    fn revert(entity: Entity, world: &mut World)
    {
        Instruction::apply(Self(TextLine::default_font_size()), entity, world);
    }
}

impl StaticAttribute for TextLineSize
{
    type Value = f32;
    fn construct(value: Self::Value) -> Self
    {
        TextLineSize(value)
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// Instruction for setting the color of a [`TextLine`] on an entity.
#[derive(Reflect, Component, Default, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct TextLineColor(pub Color);

impl Instruction for TextLineColor
{
    fn apply(self, entity: Entity, world: &mut World)
    {
        world.syscall(
            (entity, self.0),
            |In((id, color)): In<(Entity, Color)>, mut editor: TextEditor| {
                let Some((_, _, text_color)) = editor.root(id) else {
                    tracing::warn!("failed setting TextLineColor({color:?}) on {id:?}; entity does not \
                        have Text");
                    return;
                };
                *text_color = color;
            },
        );
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.insert(self);
        });
    }

    fn revert(entity: Entity, world: &mut World)
    {
        world.syscall(entity, |In(id): In<Entity>, mut editor: TextEditor| {
            let Some((_, _, text_color)) = editor.root(id) else { return };
            *text_color = TextLine::default_color();
        });
        let _ = world.get_entity_mut(entity).map(|mut e| {
            e.remove::<Self>();
        });
    }
}

impl StaticAttribute for TextLineColor
{
    type Value = Color;
    fn construct(value: Self::Value) -> Self
    {
        TextLineColor(value)
    }
}

impl ResponsiveAttribute for TextLineColor {}
impl AnimatedAttribute for TextLineColor
{
    fn get_value(entity: Entity, world: &World) -> Option<Self::Value>
    {
        let color = world.get::<Self>(entity)?;
        Some(color.0)
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct UiTextExtPlugin;

impl Plugin for UiTextExtPlugin
{
    fn build(&self, app: &mut App)
    {
        app.register_static::<TextLine>()
            .register_static::<TextLineSize>()
            .register_animatable::<TextLineColor>();
    }
}

//-------------------------------------------------------------------------------------------------------------------
