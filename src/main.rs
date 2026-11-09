#![cfg_attr(not(test), windows_subsystem = "windows")]
mod model;

use eframe::egui::{self, Color32, RichText, Stroke};
use egui_extras::{Column, TableBuilder};
use model::{calc, parse_num, totals, Item};
use std::path::PathBuf;

const BG: Color32 = Color32::from_rgb(0xf1, 0xf5, 0xf9);
const BLUE: Color32 = Color32::from_rgb(0x1e, 0x40, 0xaf);
const GREEN: Color32 = Color32::from_rgb(0x05, 0x96, 0x69);
const RED: Color32 = Color32::from_rgb(0xef, 0x44, 0x44);
const GRAY: Color32 = Color32::from_rgb(0x64, 0x74, 0x8b);

struct App {
    path: PathBuf,
    items: Vec<Item>,
    name: String,
    price: String,
    purchase: String,
    premium: String,
    sale: u32,
    withdraw: u32,
    selected: Option<usize>,
    error: Option<String>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_look(&cc.egui_ctx);
        let path = model::data_path();
        Self {
            items: model::load(&path),
            path,
            name: String::new(),
            price: "0".into(),
            purchase: "0".into(),
            premium: "0".into(),
            sale: 10,
            withdraw: 6,
            selected: None,
            error: None,
        }
    }

    fn save(&mut self) {
        if let Err(e) = model::save(&self.path, &self.items) {
            self.error = Some(format!("Не удалось сохранить {}: {e}", self.path.display()));
        }
    }

    fn add(&mut self) {
        let name = self.name.trim().to_string();
        if name.is_empty() {
            self.error = Some("Введите название товара".into());
            return;
        }
        let (Some(price), Some(purchase), Some(premium)) =
            (parse_num(&self.price), parse_num(&self.purchase), parse_num(&self.premium))
        else {
            self.error = Some("В числовых полях только неотрицательные числа".into());
            return;
        };
        let (on_card, profit) = calc(price, purchase, premium, self.sale, self.withdraw);
        self.items.push(Item {
            name, price, sale_comm: self.sale, withdraw_comm: self.withdraw,
            purchase, premium, on_card, profit,
        });
        self.save();
        self.error = None;
        self.name.clear();
        self.price = "0".into();
        self.purchase = "0".into();
        self.premium = "0".into();
        self.selected = None;
    }

    fn delete(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.items.remove(idx);
            self.selected = None;
            self.save();
        }
    }
}

fn setup_look(ctx: &egui::Context) {
    // Segoe UI на Windows — запасной шрифт, чтобы точно были кириллица и ₽
    if let Ok(bytes) = std::fs::read(r"C:\Windows\Fonts\segoeui.ttf") {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("segoe".into(), egui::FontData::from_owned(bytes).into());
        for fam in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(fam).or_default().insert(0, "segoe".into());
        }
        ctx.set_fonts(fonts);
    }
    ctx.set_theme(egui::Theme::Light);
    ctx.global_style_mut(|s| {
        s.visuals.panel_fill = BG;
        s.visuals.selection.bg_fill = Color32::from_rgb(0xdb, 0xea, 0xfe);
        s.visuals.selection.stroke = Stroke::new(1.0, BLUE);
        s.spacing.item_spacing = egui::vec2(10.0, 8.0);
        s.spacing.button_padding = egui::vec2(12.0, 6.0);
    });
}

fn card(ui: &mut egui::Ui, title: &str, value: String, color: Color32) {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.0, Color32::from_rgb(0xe2, 0xe8, 0xf0)))
        .corner_radius(16)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(title).size(11.0).strong().color(Color32::from_rgb(0x94, 0xa3, 0xb8)));
                ui.label(RichText::new(value).size(26.0).strong().color(color));
            });
        });
}

fn rub(x: f64) -> String {
    format!("{} ₽", (x * 100.0).round() / 100.0)
}

fn money(x: f64) -> String {
    // 12 345.6 — пробел как разделитель тысяч
    let s = format!("{:.1}", x.abs());
    let (int, frac) = s.split_once('.').unwrap();
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(' ');
        }
        out.push(c);
    }
    format!("{}{out}.{frac} ₽", if x < 0.0 { "-" } else { "" })
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let (rev, profit) = totals(&self.items);
        let mut add = false;
        let mut del = None;

        egui::Frame::new().inner_margin(24).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("PlayerokUNIVERSITY").size(30.0).strong().color(BLUE));
                ui.label(RichText::new("Система управления доходами ⚡").italics().color(GRAY));
            });
            ui.add_space(12.0);

            ui.columns(3, |c| {
                card(&mut c[0], "ОБЩАЯ ВЫРУЧКА", money(rev), BLUE);
                card(&mut c[1], "ЧИСТЫЙ ПРОФИТ", money(profit), if profit < 0.0 { RED } else { GREEN });
                card(&mut c[2], "ВСЕГО СДЕЛОК", self.items.len().to_string(), BLUE);
            });
            ui.add_space(12.0);

            egui::Frame::new()
                .fill(Color32::WHITE)
                .stroke(Stroke::new(1.0, Color32::from_rgb(0xe2, 0xe8, 0xf0)))
                .corner_radius(16)
                .inner_margin(16)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        let field = |ui: &mut egui::Ui, label: &str, v: &mut String, w: f32| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(label).small().strong());
                                ui.add(egui::TextEdit::singleline(v).desired_width(w));
                            });
                        };
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Название товара").small().strong());
                            let r = ui.add(egui::TextEdit::singleline(&mut self.name).desired_width(220.0));
                            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                add = true;
                            }
                        });
                        field(ui, "Цена сайта", &mut self.price, 90.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Выручка %").small().strong());
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.sale, 10, "10%");
                                ui.selectable_value(&mut self.sale, 20, "20%");
                            });
                        });
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Вывод %").small().strong());
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.withdraw, 6, "6%");
                                ui.selectable_value(&mut self.withdraw, 10, "10%");
                            });
                        });
                        field(ui, "Закуп", &mut self.purchase, 90.0);
                        field(ui, "Премиум", &mut self.premium, 90.0);
                        if ui.add(egui::Button::new(RichText::new("ДОБАВИТЬ").strong().color(Color32::WHITE)).fill(BLUE).min_size(egui::vec2(110.0, 40.0))).clicked() {
                            add = true;
                        }
                    });
                    if let Some(e) = &self.error {
                        ui.colored_label(RED, e);
                    }
                });
            ui.add_space(12.0);

            egui::Frame::new()
                .fill(Color32::WHITE)
                .stroke(Stroke::new(1.0, Color32::from_rgb(0xe2, 0xe8, 0xf0)))
                .corner_radius(16)
                .inner_margin(12)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let h = (ui.available_height() - 52.0).max(120.0);
                    TableBuilder::new(ui)
                        .striped(true)
                        .sense(egui::Sense::click())
                        .max_scroll_height(h)
                        .min_scrolled_height(h)
                        .column(Column::remainder().at_least(160.0))
                        .columns(Column::auto().at_least(80.0), 7)
                        .header(30.0, |mut row| {
                            for t in ["ТОВАР", "ЦЕНА", "ВЫРУЧКА %", "ВЫВОД %", "ЗАКУП", "ПРЕМИУМ", "НА КАРТУ", "ПРОФИТ"] {
                                row.col(|ui| { ui.label(RichText::new(t).small().strong().color(GRAY)); });
                            }
                        })
                        .body(|body| {
                            let n = self.items.len();
                            body.rows(34.0, n, |mut row| {
                                let i = n - 1 - row.index(); // новые сверху
                                let it = &self.items[i];
                                row.set_selected(self.selected == Some(i));
                                row.col(|ui| { ui.label(&it.name); });
                                row.col(|ui| { ui.label(rub(it.price)); });
                                row.col(|ui| { ui.label(format!("{}%", it.sale_comm)); });
                                row.col(|ui| { ui.label(format!("{}%", it.withdraw_comm)); });
                                row.col(|ui| { ui.label(rub(it.purchase)); });
                                row.col(|ui| { ui.label(rub(it.premium)); });
                                row.col(|ui| { ui.label(rub(it.on_card)); });
                                row.col(|ui| {
                                    ui.label(RichText::new(rub(it.profit)).strong().color(if it.profit < 0.0 { RED } else { GREEN }));
                                });
                                if row.response().clicked() {
                                    self.selected = Some(i);
                                }
                                row.response().context_menu(|ui| {
                                    if ui.button("Удалить запись").clicked() {
                                        del = Some(i);
                                        ui.close();
                                    }
                                });
                            });
                        });
                    ui.add_space(6.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let b = egui::Button::new(RichText::new("УДАЛИТЬ ВЫБРАННЫЙ ТОВАР").strong().color(Color32::WHITE)).fill(RED);
                        if ui.add_enabled(self.selected.is_some(), b).clicked() {
                            del = self.selected;
                        }
                    });
                });
        });

        if add { self.add(); }
        if let Some(i) = del { self.delete(i); }
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("PlayerokUniversity")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("PlayerokUniversity", opts, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}
