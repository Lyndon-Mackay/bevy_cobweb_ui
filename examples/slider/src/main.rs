//! Example demonstrating the slider widget.

use bevy::prelude::*;
use bevy::window::WindowTheme;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::builtin::widgets::slider::{SliderValue, SliderWidgetExt};
use bevy_cobweb_ui::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn build_ui(mut c: Commands, mut s: ResMut<SceneLoader>)
{
    let scene = ("main.cob", "scene");
    c.ui_root().load_scene_and_edit(scene, &mut s, |l| {
        // Basic vertical slider.
        l.edit("basic::slider", |l| {
            let basic_text = l.get_entity_from_root("basic::text").unwrap();

            l.on_slider(move |id: UpdateId, mut e: TextEditor, sliders: Reactive<SliderValue>| {
                let val = sliders.get(*id)?.single().result()?;
                let val = val * 100.;
                write_text!(e, basic_text, "{}", val as usize);
                OK
            });
        });

        // Vertical slider with reversed axis.
        l.edit("reverse::slider", |l| {
            let reverse_text = l.get_entity_from_root("reverse::text").unwrap();

            l.on_slider(move |id: UpdateId, mut e: TextEditor, sliders: Reactive<SliderValue>| {
                let val = sliders.get(*id)?.single().result()?;
                let val = val * 100.;
                write_text!(e, reverse_text, "{}", val as usize);
                OK
            });
        });

        // Fancy slider with slider visuals 'lifted' off the core slider/slider handle entities so the handle can
        // overlap with the end of the slider.
        l.edit("fancy::slider", |l| {
            let fancy_text = l.get_entity_from_root("fancy::text").unwrap();

            l.on_slider(move |id: UpdateId, mut e: TextEditor, sliders: Reactive<SliderValue>| {
                let val = sliders.get(*id)?.single().result()?;
                let val = val * 100.;
                write_text!(e, fancy_text, "{}", val as usize);
                OK
            });
        });

        // Planar slider.
        l.edit("planar::slider", |l| {
            let planar_text = l.get_entity_from_root("planar::text").unwrap();

            l.on_slider(move |id: UpdateId, mut e: TextEditor, sliders: Reactive<SliderValue>| {
                let val = sliders.get(*id)?.planar().result()?;
                let val = val * 100.;
                write_text!(e, planar_text, "({}, {})", val.x as usize, val.y as usize);
                OK
            });
        });
    });
}

//-------------------------------------------------------------------------------------------------------------------

fn setup(mut c: Commands)
{
    c.spawn(Camera2d);
}

//-------------------------------------------------------------------------------------------------------------------

fn main()
{
    App::new()
        .add_plugins(bevy::DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { window_theme: Some(WindowTheme::Dark), ..default() }),
            ..default()
        }))
        .add_plugins(CobwebUiPlugin)
        .load("main.cob")
        .add_systems(PreStartup, setup)
        .add_systems(OnEnter(LoadState::Done), build_ui)
        .run();
}

//-------------------------------------------------------------------------------------------------------------------
