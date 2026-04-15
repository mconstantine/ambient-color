use std::{
    f32::consts::PI,
    process::{Command, Stdio},
};

use core_logic::{
    data::Theme,
    palette::{IntoColor, Srgb},
};
use serde::Deserialize;
use tiny_skia::{Color, LineCap, Paint, PathBuilder, Pixmap, Stroke, Transform};

const CIRCLE_RADIUS: f32 = 128.0;
const TREE_ROOT_LENGTH: f32 = 42.0;
const TREE_ROOT_STROKE_WIDTH: f32 = 4.0;
const TREE_STEP_RATIO: f32 = 0.80;
const TREE_STEP_ANGLE: f32 = PI / 12.0; // 15° (30° between sibling branches)
const TREE_STEPS: i32 = 12;

#[derive(Deserialize, Debug)]
struct Monitor {
    name: String,
    width: u16,
    height: u16,
    scale: f32,
}

pub fn draw_wallpaper(theme: &Theme) -> () {
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

    let bg_srgb_f32: Srgb<f32> = theme.original_color.fg.into_color();
    let bg_srgb_u8: Srgb<u8> = bg_srgb_f32.into_format();
    let circle_srgb_f32: Srgb<f32> = theme.original_color.bg.into_color();
    let circle_srgb_u8: Srgb<u8> = circle_srgb_f32.into_format();

    let bg_color = Color::from_rgba8(bg_srgb_u8.red, bg_srgb_u8.green, bg_srgb_u8.blue, 255);

    let circle_color = Color::from_rgba8(
        circle_srgb_u8.red,
        circle_srgb_u8.green,
        circle_srgb_u8.blue,
        255,
    );

    pixmap.fill(bg_color);

    let mut paint = Paint::default();

    paint.set_color(circle_color);
    paint.anti_alias = true;

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = CIRCLE_RADIUS * monitor.scale;
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

    draw_tree(theme, &mut pixmap, monitor);

    pixmap
        .save_png(&path)
        .expect("Error: unable to save wallpaper");

    Command::new("hyprctl")
        .args(["hyprpaper", "preload", &path])
        .output()
        .expect("Error: unable to preload wallpaper");

    let apply_arg = format!("{},{}", monitor.name, path);

    Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &apply_arg])
        .output()
        .expect("Error: failed to set wallpaper");
}

#[derive(Copy, Clone)]
struct Branch {
    x: f32,
    y: f32,
    angle: f32,
}

fn draw_tree(theme: &Theme, pixmap: &mut Pixmap, monitor: &Monitor) -> () {
    let tree_srgb_f32: Srgb<f32> = theme.original_color.fg.into_color();
    let tree_srgb_u8: Srgb<u8> = tree_srgb_f32.into_format();

    let tree_color =
        Color::from_rgba8(tree_srgb_u8.red, tree_srgb_u8.green, tree_srgb_u8.blue, 255);

    let mut paint = Paint::default();

    paint.set_color(tree_color);
    paint.anti_alias = true;

    let mut stroke = Stroke::default();

    stroke.width = TREE_ROOT_STROKE_WIDTH * monitor.scale;
    stroke.line_cap = LineCap::Round;

    let center_x = monitor.width as f32 / 2.0;
    let center_y = monitor.height as f32 / 2.0 + CIRCLE_RADIUS * monitor.scale;
    let radius = TREE_ROOT_LENGTH * monitor.scale;

    let tip = Branch {
        x: center_x - radius * (0.0 as f32).sin(),
        y: center_y - radius * (0.0 as f32).cos(),
        angle: PI / 2.0,
    };

    let mut path_builder = PathBuilder::new();

    path_builder.move_to(center_x, center_y);
    path_builder.line_to(tip.x, tip.y);

    let path = path_builder.finish().expect("Error: unable to draw path");

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    let mut tips = vec![tip];

    for i in 1..TREE_STEPS {
        let mut next_tips = Vec::<Branch>::new();

        for tip in &tips {
            let (left, right) = draw_tree_step(i, pixmap, &monitor, &paint, tip);

            next_tips.push(left);
            next_tips.push(right);
        }

        tips = next_tips;
    }

    let circle_bottom_x = center_x;
    let circle_bottom_y = center_y;
    let screen_bottom_x = center_x;
    let screen_bottom_y = monitor.height as f32;

    let mut path_builder = PathBuilder::new();

    path_builder.move_to(circle_bottom_x, circle_bottom_y);
    path_builder.line_to(screen_bottom_x, screen_bottom_y);

    let path = path_builder
        .finish()
        .expect("Error: unable to draw last path");

    stroke.line_cap = LineCap::Butt;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}

fn draw_tree_step(
    step: i32,
    pixmap: &mut Pixmap,
    monitor: &Monitor,
    paint: &Paint,
    root: &Branch,
) -> (Branch, Branch) {
    let mut stroke = Stroke::default();
    let mut path_builder = PathBuilder::new();
    let radius = TREE_ROOT_LENGTH * monitor.scale * TREE_STEP_RATIO.powi(step);
    let stroke_width = TREE_ROOT_STROKE_WIDTH * monitor.scale * TREE_STEP_RATIO.powi(step);
    let left_angle = root.angle + TREE_STEP_ANGLE;
    let right_angle = root.angle - TREE_STEP_ANGLE;

    let left_tip = Branch {
        x: root.x - radius * left_angle.cos(),
        y: root.y - radius * left_angle.sin(),
        angle: left_angle,
    };

    let right_tip = Branch {
        x: root.x - radius * right_angle.cos(),
        y: root.y - radius * right_angle.sin(),
        angle: right_angle,
    };

    stroke.width = stroke_width;
    stroke.line_cap = LineCap::Round;

    path_builder.move_to(root.x, root.y);
    path_builder.line_to(left_tip.x, left_tip.y);
    path_builder.move_to(root.x, root.y);
    path_builder.line_to(right_tip.x, right_tip.y);

    let path = path_builder.finish().expect("Error: unable to draw path");

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

    (left_tip, right_tip)
}
