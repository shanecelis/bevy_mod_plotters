# bevy_mod_plotters
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_mod_plotters/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_mod_plotters/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_mod_plotters.svg)](https://crates.io/crates/bevy_mod_plotters)
  [![api-docs](https://docs.rs/bevy_mod_plotters/badge.svg)](https://docs.rs/bevy_mod_plotters)

A [bevy game engine](https://bevyengine.org) material to display the [plotters](https://github.com/plotters-rs/plotters) BGRX texture format.

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

## Create an Image

``` rust,no_run
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn setup() {
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0x00],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));
    
    
    let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        &mut image.data,
        (WIDTH, HEIGHT),
    )
    .unwrap()
    .into_drawing_area();
}
```

## Plot to Image Data

``` rust

fn plot<DB: DrawingBackend>(
    root: &DrawingArea<DB, Shift>,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {

}
```

## Add Material to Object

```rust,compile
use bevy::{
    prelude::*,
    color::palettes::basic,
    pbr::ExtendedMaterial,
};
use bevy_mod_plotters::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlaneCutMaterial>>) {

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: basic::RED.into(),
                ..default()
            },
            extension: PlaneCutExt {
                plane: Vec4::new(-1.0, 1.0, -2.0, 0.0),
                color: Color::linear_rgb(0.0, 0.0, 0.7),
                shaded: true,
                space: Space::World,
            },
        }),
        ..default()
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

Consider adding a backend specifically for the BGRA8 formats that will not
overwrite the alpha channel.

# Compatibility

| bevy_mod_plotters | bevy |
|-------------------|------|
| 0.1               | 0.14 |

# License

This crate is licensed under the MIT License or the Apache License 2.0. The
examples are licensed under the CC0 license.

# Acknowlegments

* Thanks to [Claire V. Hammond](https://github.com/cvhammond) for the [bevy_plotters_test](https://github.com/cvhammond/bevy_plotters_test) repository.
