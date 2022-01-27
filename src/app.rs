use eframe::{egui, epi};
use qrcode::QrCode;
use std::process::exit;
use std::iter;


pub struct DemoApp {
    input: String,
    status: String,
    output: String,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            input: "".to_owned(),
            status: "".to_owned(),
            output: "".to_string(),
        }
    }
}

impl epi::App for DemoApp {
    fn name(&self) -> &str {
        "QR Code Generator"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trigger = self.ui_tcp(ui);

            if trigger {
                let code = match QrCode::new(&self.input[..]) {
                    Ok(code) => {
                        self.status = "Valid input".to_string();
                        code
                    }
                    Err(_error) => {
                        self.status = "Can't generate QR code".to_string();
                        exit(1)
                    }
                };
            
                let string = code.render()
                    .light_color(' ')
                    .dark_color('#')
                    .build();
            
                let mut empty_str: String;
                let mut line_buffer = String::new();
                let mut lines = string.lines().into_iter();
            
                while let Some(line_top) = lines.next() {
                    let line_bottom = match lines.next() {
                        Some(l) => l,
                        None => {
                            empty_str = iter::repeat(' ').take(line_top.len()).collect();
                            empty_str.as_str()
                        }
                    };
            
                    for (top, bottom) in line_top.chars().zip(line_bottom.chars()) {
                        let block = match (top, bottom) {
                            ('#', '#') => '█',
                            (' ', '#') => '▄',
                            ('#', ' ') => '▀',
                            _ => ' ',
                        };
                        line_buffer.push(block);
                    }
                    self.output += &format!("{}\n", line_buffer);
                    line_buffer.clear();
                }
            }

            ui.separator();
        });
    }
}

impl DemoApp {
    fn ui_tcp(&mut self, ui: &mut egui::Ui) -> bool {
        let mut trigger = false;

        egui::Grid::new("request_parameters")
            .spacing(egui::Vec2::splat(4.0))
            .min_col_width(70.0)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Input:");
                ui.text_edit_singleline(&mut self.input).lost_focus();
                ui.end_row();
            });

        ui.end_row();

        trigger |= ui.button("generate ▶").clicked();

        self.response_ui(ui);

        trigger
    }
    fn response_ui(&mut self, ui: &mut egui::Ui) {
        ui.monospace(format!("Status: {}", self.status));
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            selectable_text(ui, &self.output);
        });
    }
}

fn selectable_text(ui: &mut egui::Ui, mut text: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_width(f32::INFINITY)
            .text_style(egui::TextStyle::Monospace),
    );
}
