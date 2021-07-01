use std::path::PathBuf;

use eframe::egui;
use eframe::epi;
use rfd::FileDialog;

#[derive(Default)]
pub struct App {
    uexp_path: Option<PathBuf>,
    uasset_path: Option<PathBuf>,
    injected_path: Option<PathBuf>,
    last_output: String,
}

impl epi::App for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Open UEXP").clicked() {
                        self.uexp_path =
                            FileDialog::new().add_filter("UEXP files", &["uexp"]).pick_file();
                    }

                    ui.label(format!(
                        "UEXP Path: {}",
                        self.uexp_path
                            .as_ref()
                            .map_or("not selected", |p| p.to_str().unwrap())
                    ));
                });
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Open UASSET").clicked() {
                        self.uasset_path = FileDialog::new()
                            .add_filter("UASSET files", &["uasset"])
                            .pick_file();
                    }

                    ui.label(format!(
                        "UASSET Path: {}",
                        self.uasset_path
                            .as_ref()
                            .map_or("not selected", |p| p.to_str().unwrap())
                    ));
                });

                ui.horizontal_wrapped(|ui| {
                    if ui.button("Select file to inject").clicked() {
                        self.injected_path = FileDialog::new().pick_file();
                    }

                    ui.label(format!(
                        "Injected Path: {}",
                        self.injected_path
                            .as_ref()
                            .map_or("not selected", |p| p.to_str().unwrap())
                    ));
                });
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    if ui.button("Extract").clicked() {
                        if let Some(path) = self.uexp_path.as_ref() {
                            if let Some(output) = FileDialog::new().save_file() {
                                if let Err(e) = crate::extract_file(path.clone(), output, true) {
                                    self.last_output = e.to_string()
                                } else {
                                    self.last_output = "Success".into()
                                }
                            } else {
                                self.last_output = "No output file selected".into()
                            }
                        } else {
                            self.last_output = "No UEXP selected".into()
                        }
                    }

                    ui.separator();

                    if ui.button("Inject").clicked() {
                        if let Some(uexp) = self.uexp_path.as_ref() {
                            if let Some(uasset) = self.uasset_path.as_ref() {
                                if let Some(inject) = self.injected_path.as_ref() {
                                    if let Err(e) = crate::inject_file(inject.clone(), uexp.clone(), uasset.clone(), true) {
                                        self.last_output = e.to_string();
                                    } else {
                                        self.last_output = "Success".into();
                                    }
                                } else {
                                    self.last_output = "No file to inject selected".into()
                                }
                            } else {
                                self.last_output = "No UASSET selected".into()
                            }
                        } else {
                            self.last_output = "No UEXP selected".into()
                        }
                    }
                });

                ui.label(format!("Result: {}", self.last_output.as_str()));
            });
        });
    }

    fn name(&self) -> &str {
        "GGST BBScript Unpacker"
    }
}
