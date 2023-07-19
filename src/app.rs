use time::{Time, OffsetDateTime};
use egui::{Color32, Vec2, Sense, vec2, Stroke};
use std::f32::consts::TAU;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    time: Time,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            time: OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()).time(),
        }
    }
}

impl TemplateApp {
    
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { time } = self;
        let width = ctx.screen_rect().width();
        let height = ctx.screen_rect().height();

        use egui::FontFamily::Proportional;
        use egui::FontId;
        use egui::TextStyle::*;

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("DragValue".into()), FontId::new(50., Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
          ].into();
        style.drag_value_text_style = egui::TextStyle::Name("DragValue".into());
        //style.visuals.override_text_color = Some(egui::Color32::from_rgb(224, 242, 19));
        ctx.set_style(style);

        let mut hour = time.hour();
        let mut minute = time.minute();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("What time is it now?");
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                
                    let size = (width / 20., height / 10.);
                    ui.add_sized(size, egui::DragValue::new(&mut hour)
                        .speed(0.1)
                        .clamp_range(0..=24));
                    ui.label("  :  ");
                    ui.add_sized(size, egui::DragValue::new(&mut minute)
                        .speed(0.1)
                        .clamp_range(0..=60));
 
                });
                if minute == 60 {
                    //dbg!(minute);
                    hour += 1;
                    minute = 0;    
                }
                    //dbg!(minute);
                
                if hour == 24 {
                    hour = 0;
                }
             
                *time = Time::from_hms(hour, minute, 0).unwrap();
    
                if ui.button("time now").clicked() {
                    *time = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()).time();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //angle of hour arrow
            let h_angle = (TAU * (hour % 12) as f32 / 12.0 + TAU * minute as f32 / (12.0 * 60.)) - TAU / 4.;
            //angle for minute arrow
            let m_angle = TAU * minute as f32 / 60.0 - TAU / 4.;

            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
            let rect = response.rect;
            let c = rect.center();
            let r = rect.height() * 0.8 / 2. - 10.;
            painter.circle_stroke(c, r, (10., Color32::BLACK));
            painter.circle_filled(c, 10., Color32::BLACK);
            let stroke = Stroke::new(1., Color32::BLACK);
            for n in 0..60 {
                let r_end = c + r * Vec2::angled(TAU * n as f32 / 60.0);
                let r_start = if n % 5 == 0 {
                    let h_text_pos = c + r * 1.1 * Vec2::angled(TAU * n as f32 / 60.0);
                    let h = (n / 5 + 2) % 12 + 1;
                    painter.text(h_text_pos, egui::Align2::CENTER_CENTER, h, egui::FontId::proportional(30.), Color32::BLACK);
                    c + r * 0.9 * Vec2::angled(TAU * n as f32 / 60.0)
                } else {
                    c + r * 0.95 * Vec2::angled(TAU * n as f32 / 60.0)
                };
                painter.line_segment([r_start, r_end], stroke);
                let m = (n + 14) % 60 + 1;
                let m_text_pos = c + r * 0.87 * Vec2::angled(TAU * n as f32 / 60.0);
                painter.text(m_text_pos, egui::Align2::CENTER_CENTER, m, egui::FontId::proportional(14.), Color32::BLACK);
            }

            let h_arrow_stroke = Stroke::new(10., Color32::BLACK);
            let m_arrow_stroke = Stroke::new(5., Color32::BLACK);
            let h_rect = egui::Rect::from_center_size( c + r * 0.6 * Vec2::angled(h_angle), vec2(10., 10.));
            let m_rect = egui::Rect::from_center_size( c + r * 0.8 * Vec2::angled(m_angle), vec2(10., 10.));
            
            painter.line_segment([c, c + r * 0.6 * Vec2::angled(h_angle)], h_arrow_stroke);
            painter.line_segment([c, c + r * 0.8 * Vec2::angled(m_angle)], m_arrow_stroke);

            if response.hovered() {
                painter.rect_stroke(h_rect, 0., Stroke::new(5., Color32::BLUE));
                painter.rect_stroke(m_rect, 0., Stroke::new(5., Color32::BLUE));
            }           
        });

    }
}



