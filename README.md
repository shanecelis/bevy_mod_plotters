# bevy_mod_plotters
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_mod_plotters/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_mod_plotters/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_mod_plotters.svg)](https://crates.io/crates/bevy_mod_plotters)
  [![api-docs](https://docs.rs/bevy_mod_plotters/badge.svg)](https://docs.rs/bevy_mod_plotters)

A [bevy game engine](https://bevyengine.org) material to display the [plotters](https://github.com/plotters-rs/plotters) BGRX texture format.

![Example anim_graph of sine wave.](https://github.com/user-attachments/assets/4c953a1b-a95a-4b72-8d18-efdeab4b79b9)

# Motivation

The BGRX texture used by plotters is like a BGRA texture, but the plotters crate
does not does not abstain from overwriting the 'X' byte, which for BGRA is the
alpha channel, so one cannot rely on setting alpha once in the texture data but
must assume it was changed after any render from plotters. This crate exists
principally to avoid that alpha resetting problem and provide convenience
methods.

# Install

Install the crate.

```sh
cargo add bevy_mod_plotters
```

# Usage

## Add Plugin to App

```rust,no_run
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy_mod_plotters::PlottersPlugin)
        .run();
}
```

## Create Image and Plot

``` rust,no_run
# use bevy::{
#     prelude::*,
#     render::{
#         render_asset::RenderAssetUsages,
#         render_resource::{Extent3d, TextureDimension, TextureFormat},
#         texture::Image,
#     },
# };
# use plotters::coord::Shift;
use bevy_mod_plotters::prelude::*;
use plotters::prelude::*;
fn setup() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: 500,
            height: 500,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0x00, 0x00, 0x00, 0x00],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    
    {
        let root = try_as_backend(&mut image)
            .unwrap()
            .into_drawing_area();
        plot(&root)
            .unwrap();
    }
    image
}

fn plot<DB: DrawingBackend>(
    root: &DrawingArea<DB, Shift>,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {

    let mut chart = ChartBuilder::on(&root)
        .caption("Hello, World!", ("sans-serif", 40).into_font())
        .build_cartesian_2d(-1.0..1.0, -1.0..1.0)?;

    let points = vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)];
    chart.draw_series(LineSeries::new(points, &RED))?;
    root.present()
}
```

## Add Image as a `PlotUiMaterial`

```rust,compile
# use bevy::{
#     prelude::*,
#     color::palettes::basic,
#     pbr::ExtendedMaterial,
# };
# use bevy_mod_plotters::*;
fn add_image(
    In(image): In<Image>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut ui_materials: ResMut<Assets<PlotUiMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(MaterialNodeBundle {
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(500.0),
                    border: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                material: ui_materials.add(PlotUiMaterial {
                    color: LinearRgba::WHITE,
                    texture: images.add(image),
                }),
                ..default()
            });
        });
}
```

# Examples

Run the "draw_graph" example like so:

```sh
cargo run --example draw_graph
```

This will show a red sphere with a light rotating around it and blue plane cut.

* `draw_graph` - Draw a plot of lines, hit 'Space' to update.
* `anim_graph` - Animate a sinusoidal wave.

# TODO

Consider adding a plotters backend specifically for the BGRA8 formats that will
not overwrite the alpha channel. A backend like that might be slightly slower for writing, but it would allow one to produce plots with a transparent background.

# Compatibility

| bevy_mod_plotters | bevy |
|-------------------|------|
| 0.1               | 0.14 |

# License

This crate is licensed under the MIT License or the Apache License 2.0. The
examples are licensed under the CC0 license.

# Acknowlegments

* Thanks to [Claire V. Hammond](https://github.com/cvhammond) for the [bevy_plotters_test](https://github.com/cvhammond/bevy_plotters_test) repository.
