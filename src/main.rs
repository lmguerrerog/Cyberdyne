#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::aplicacion_egui_rfleiva::AplicacionDatosUsuario;
use eframe::{HardwareAcceleration, Theme};
use egui::{Pos2, Vec2};

mod aplicacion_egui_rfleiva;
mod gestor_usuarios_pablo;
mod menu_amartinc;

fn main() {
    // menu amartinc
    // menu_amartinc::menu_aplicacion();

    // When compiling natively:
    #[cfg(not(target_arch = "wasm32"))]
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Some(Pos2::new(300.0, 300.0)),
        initial_window_size: Option::from(Vec2::new(100_f32, 110_f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: false,
        default_theme: Theme::Light,
        run_and_return: false,
        event_loop_builder: None,
        shader_version: None,
        centered: true,
    };

    // let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gestión datos usuario",
        options,
        Box::new(|cc| Box::new(AplicacionDatosUsuario::new(cc))),
    );
}
