#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use eframe::egui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Title of the message box
    #[arg(short = 't', long = "title")]
    title: String,

    /// Message of the message box
    #[arg(short = 'm', long = "message")]
    message: String
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 240.0]),
        ..Default::default()
    };

    let args = Args::parse();
    eframe::run_simple_native(&format!("Metamorph - {}", args.title), options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let args = Args::parse();
            ctx.set_pixels_per_point(2.0);
            ui.heading(args.title);
            ui.label(args.message);        
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                if ui.add(egui::Button::new("Close")).clicked() {
                    let _ = ctx.end_frame();
                }
            });
        });
    }).expect("OK");
}