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

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlottersPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            update_image.run_if(input_just_pressed(KeyCode::Space)),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut ui_materials: ResMut<Assets<PlotUiMaterial>>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0x00, 0x00, 0x00, 0x00],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    {
        let root = try_as_backend(&mut image).unwrap().into_drawing_area();
        let points = vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)];
        hello_plot(&root, points).unwrap();
    }
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
                    texture: images.add(image),
                }),
                ..default()
            });
        });
}

/// Adapated from plotters' README for "Our first chart" code sample.
fn hello_plot<DB: DrawingBackend>(
    root: &DrawingArea<DB, Shift>,
    points: Vec<(f32, f32)>,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    root.fill(&WHITE)?;
    let root = root.margin(30, 40, 30, 40);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Hello, Plot!", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

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
    chart.draw_series(LineSeries::new(points.clone(), &RED))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        points,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("({:.1}, {:.1})", c.0, c.1), (10, 0), ("sans-serif", 16).into_font());
        },
    ))?;

    root.present()?;
    Ok(())
}

fn update_image(
    query: Query<&Handle<PlotUiMaterial>>,
    mut ui_materials: ResMut<Assets<PlotUiMaterial>>,
    mut images: ResMut<Assets<Image>>,
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
            let mut rng = rand::thread_rng();
            let mut points = vec![];
            for _ in 0..3 {
                let x = rng.gen_range(0.0..10.0);
                let y = rng.gen_range(0.0..10.0);
                points.push((x, y));
            }
            hello_plot(&root, points).unwrap();
        }
    }
}
