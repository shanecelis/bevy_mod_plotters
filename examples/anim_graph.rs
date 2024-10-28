use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::Image,
    },
};
use bevy_mod_plotters::prelude::*;
use plotters::prelude::*;
use plotters::prelude::{Circle, Text};
use rand::Rng;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const AMP: f32 = 1.0;
const FREQ: f32 = 3.0;
const X_RANGE: std::ops::Range<f32> = -2f32..2f32;
const Y_RANGE: std::ops::Range<f32> = -2f32..2f32;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlottersPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update_image)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut ui_materials: ResMut<Assets<PlotUiMaterial>>,
    time: Res<Time>,
) {
    let mut bytes: Vec<u8> = vec![0x0; (WIDTH * HEIGHT * 4) as usize];
    {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut bytes, (WIDTH, HEIGHT))
            .unwrap()
            .into_drawing_area();
        let points = vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)];
        sin_plot(&root, AMP, FREQ, time.elapsed_seconds()).unwrap();
    }
    let image = images.add(Image::new(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));

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
                    width: Val::Px(WIDTH as f32),
                    height: Val::Px(HEIGHT as f32),
                    border: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                material: ui_materials.add(PlotUiMaterial {
                    color: LinearRgba::WHITE,
                    texture: image,
                }),
                ..default()
            });
        });
}

/// Adapated from plotters' README for "Our first chart" code sample.
fn sin_plot<DB: DrawingBackend>(
    root: &DrawingArea<DB, Shift>,
    amp: f32,
    freq: f32,
    phase: f32,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    root.fill(&WHITE)?;
    let root = root.margin(30, 40, 30, 40);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Hello, Sine!", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(X_RANGE, Y_RANGE)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .x_label_style(("sans-serif", 16))
        .y_label_style(("sans-serif", 16))
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.2}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 25.0).map(|x| (x,amp * ((x + phase) * freq).sin())),
                &RED,
            ))?;

    root.present()?;
    Ok(())
}

fn update_image(
    query: Query<&Handle<PlotUiMaterial>>,
    mut ui_materials: ResMut<Assets<PlotUiMaterial>>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
) {
    let handle = query.single();
    // Must use get_mut() material, otherwise it won't shows changes to image.
    if let Some(material) = ui_materials.get_mut(handle) {
        if let Some(image) = images.get_mut(&material.texture) {
            let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                &mut image.data,
                (WIDTH, HEIGHT),
            )
            .unwrap()
            .into_drawing_area();

            sin_plot(&root, AMP, FREQ, time.elapsed_seconds()).unwrap();
        }
    }
}
