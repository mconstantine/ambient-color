use std::process::{Command, Stdio};

use core_logic::{
    data::Theme,
    palette::{IntoColor, Srgb},
};
use serde::Deserialize;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Transform};

#[derive(Deserialize, Debug)]
struct Monitor {
    name: String,
    width: u16,
    height: u16,
}

pub fn draw_wallpaper(theme: &Theme) -> () {
    // todo: check that Hyprland is running
    // maybe this should be called by `theme.sh`
    let monitors_output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .stdout(Stdio::piped())
        .output()
        .expect("Error: unable to read monitors");

    let monitors_string = String::from_utf8(monitors_output.stdout)
        .expect("Error: unable to turn monitors output into string");

    let monitors: Vec<Monitor> = serde_json::from_str(&monitors_string)
        .expect("Error: unable to parse monitors output to JSON");

    for monitor in monitors {
        draw_monitor_wallpaper(&monitor, theme);
    }

    Command::new("hyprctl")
        .args(["hyprpaper", "unload", "all"])
        .output()
        .expect("Error: failed to unload old wallpapers");
}

fn draw_monitor_wallpaper(monitor: &Monitor, theme: &Theme) {
    let width = monitor.width as u32;
    let height = monitor.height as u32;
    let path = format!("/tmp/ambient_color_wallpaper_{}.png", monitor.name);

    let mut pixmap = Pixmap::new(width, height).expect("Error: unable to create canvas");

    let bg_srgb_f32: Srgb<f32> = theme.primary_palette.w950.bg.into_color();
    let bg_srgb_u8: Srgb<u8> = bg_srgb_f32.into_format();
    let circle_srgb_f32: Srgb<f32> = theme.original_color.bg.into_color();
    let circle_srgb_u8: Srgb<u8> = circle_srgb_f32.into_format();
    // let tree_srgb_f32: Srgb<f32> = theme.original_color.fg.into_color();
    // let tree_srgb_u8: Srgb<u8> = tree_srgb_f32.into_format();

    let bg_color = Color::from_rgba8(bg_srgb_u8.red, bg_srgb_u8.green, bg_srgb_u8.blue, 255);

    let circle_color = Color::from_rgba8(
        circle_srgb_u8.red,
        circle_srgb_u8.green,
        circle_srgb_u8.blue,
        255,
    );

    // let tree_color =
    //     Color::from_rgba8(tree_srgb_u8.red, tree_srgb_u8.green, tree_srgb_u8.blue, 255);

    pixmap.fill(bg_color);

    let mut paint = Paint::default();

    paint.set_color(circle_color);
    paint.anti_alias = true;

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = 256.0;
    let mut path_builder = PathBuilder::new();

    path_builder.push_circle(center_x, center_y, radius);

    let circle_path = path_builder.finish().expect("Error: unable to draw circle");

    pixmap.fill_path(
        &circle_path,
        &paint,
        Default::default(),
        Transform::identity(),
        None,
    );

    // paint.set_color(tree_color);
    //
    // let mut stroke = Stroke::default();
    //
    // stroke.width = 8.0;
    // stroke.line_cap = LineCap::Round;
    //
    // let hex_radius = radius / 2.0;
    // let mut path_builder = PathBuilder::new();
    //
    // path_builder.move_to(
    //     center_x + hex_radius * (PI / 3.0 * 0.0).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 0.0).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 1.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 1.0 as f32).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 2.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 2.0 as f32).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 3.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 3.0 as f32).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 4.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 4.0 as f32).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 5.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 5.0 as f32).cos(),
    // );
    //
    // path_builder.line_to(
    //     center_x + hex_radius * (PI / 3.0 * 6.0 as f32).sin(),
    //     center_y + hex_radius * (PI / 3.0 * 6.0 as f32).cos(),
    // );
    //
    // let hex_path = path_builder
    //     .finish()
    //     .expect("Error: unable to draw hexagon");
    //
    // pixmap.stroke_path(&hex_path, &paint, &stroke, Transform::identity(), None);

    pixmap
        .save_png(&path)
        .expect("Error: unable to save wallpaper");

    Command::new("hyprctl")
        .args(["hyprpaper", "preload", &path])
        .output()
        .expect("Error: unable to preload wallpaper");

    Command::new("hyprctl")
        .args([
            "hyprpaper",
            "wallpaper",
            &format!("{},{}", monitor.name, path),
        ])
        .output()
        .expect("Error: failed to set wallpaper");
}
