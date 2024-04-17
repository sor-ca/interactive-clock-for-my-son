use egui::{vec2, Color32, Painter, Pos2, Sense, Stroke, Vec2};
use std::f32::consts::TAU;
use std::ops::RangeInclusive;
use time::{OffsetDateTime, Time};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    time: Time,
    hour_arrow_pos: Option<f32>,
    //minute_arrow_pos: Option<Pos2>,
    minute_arrow_pos: Option<f32>,
    change_hour: Hour,
    prev_raw_minute: Option<i32>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            time: OffsetDateTime::now_local()
                .unwrap_or(OffsetDateTime::now_utc())
                .time(),
            hour_arrow_pos: None,
            minute_arrow_pos: None,
            change_hour: Hour::Same,
            prev_raw_minute: None,
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

    fn draw_clock_face(painter: &Painter, c: Pos2, r: f32) {
        painter.circle_stroke(c, r, (10., Color32::BLACK));
        painter.circle_filled(c, 10., Color32::BLACK);

        let stroke = Stroke::new(1., Color32::BLACK);
        for n in 0..60 {
            let r_end = c + r * Vec2::angled(TAU * n as f32 / 60.0);
            let r_start = if n % 5 == 0 {
                // TODO: так тоже не надо делать - эти вычисления не касаются иниуциализации переменной r_start
                let h_text_pos = c + r * 1.1 * Vec2::angled(TAU * n as f32 / 60.0);
                let h = (n / 5 + 2) % 12 + 1;

                // TODO: вот вообще плохо так делать - ты рисуешь текст в блоке присвоения значения переменной
                painter.text(
                    h_text_pos,
                    egui::Align2::CENTER_CENTER,
                    h,
                    egui::FontId::proportional(30.),
                    Color32::BLACK,
                );
                c + r * 0.9 * Vec2::angled(TAU * n as f32 / 60.0)
            } else {
                c + r * 0.95 * Vec2::angled(TAU * n as f32 / 60.0)
            };

            painter.line_segment([r_start, r_end], stroke);

            let m = (n + 14) % 60 + 1;
            let m_text_pos = c + r * 0.87 * Vec2::angled(TAU * n as f32 / 60.0);
            painter.text(
                m_text_pos,
                egui::Align2::CENTER_CENTER,
                m,
                egui::FontId::proportional(14.),
                Color32::BLACK,
            );
        }
    }

    /*fn draw_minute_arrow(
        &mut self,
        ui: &mut Ui,
        painter: &Painter,
        c: Pos2,
        r: f32,
        mut minute: i8) -> (Hour, i8) {

        let mut m_angle = TAU * minute as f32 / 60.0 - TAU / 4.;
        let m_rect = egui::Rect::from_center_size( c + r * 0.8 * Vec2::angled(m_angle), vec2(10., 10.));
        let mut change_hour = Hour::Same;

        if let Some(angle) = self.minute_arrow_pos {
            m_angle = angle;
            let mut new_minute = ((m_angle + TAU / 4.) * 60. / TAU).floor() as i8;
            dbg!(new_minute);
            if new_minute == 60 {
                new_minute = 0;
            }
            if new_minute == 0 {
                if minute <= 59 && minute >= 55 {
                    change_hour = Hour::Next;
                } else if minute >= 1 && minute <= 5 {
                    change_hour = Hour::Previous;
                }
            }
            minute = new_minute as i8;
        }

        let m_arrow_stroke = Stroke::new(5., Color32::BLACK);
        painter.line_segment([c, c + r * 0.8 * Vec2::angled(m_angle)], m_arrow_stroke);

        let m_arrow_resp = ui.allocate_rect(m_rect, Sense::drag());

        if m_arrow_resp.hovered() {
            painter.rect_stroke(m_rect, 0., Stroke::new(5., Color32::BLUE));
        }

        if m_arrow_resp.dragged() {
            let pos = m_rect.center() + m_arrow_resp.drag_delta();
            let mut angle = (pos - c).angle();
            if angle < - TAU / 4. {
                angle = TAU + angle;
            }
            self.minute_arrow_pos = Some(angle);
        } else {
            self.minute_arrow_pos = None;
        }
        (change_hour, minute)
    }*/
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum Hour {
    Same,
    Next,
    Previous,
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let Self { time, hour_arrow_pos, minute_arrow_pos } = self;
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
            (Button, FontId::new(18.0, Proportional)),
        ]
        .into();
        style.drag_value_text_style = egui::TextStyle::Name("DragValue".into());
        ctx.set_style(style);

        let mut raw_hour = self.time.hour() as i32;
        let mut raw_minute = self.time.minute() as i32;
        let mut set_local_time = false;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("What time is it?");
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let size = (width / 20., height / 10.);

                    ui.add_sized(
                        size,
                        egui::DragValue::new(&mut raw_hour)
                            .speed(0.1)
                            //.clamp_range(0..=24)
                            .custom_formatter(|h, _| format!("{h:02}")),
                    );

                    ui.label("  :  ");

                    ui.add_sized(
                        size,
                        egui::DragValue::new(&mut raw_minute)
                            .speed(0.1)
                            //.clamp_range(-1..=60)
                            .custom_formatter(|m, _| format!("{m:02}")),
                    );
                });

                if ui.button("time now").clicked() {
                    set_local_time = true;
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        // TODO: попробовать перенести вверх или вниз
        let mut norm_hour = raw_hour.rem_euclid(24);
        let mut norm_minute = raw_minute.rem_euclid(60);

        //let prev_raw_minute = raw_minute;
        let prev_raw_minute = match self.prev_raw_minute {
            None => raw_minute,
            Some(value) => value,
        };

        let mut prev_norm_minute = prev_raw_minute.rem_euclid(60);
        if prev_norm_minute != norm_minute {
            //dbg!(prev_norm_minute != norm_minute);
            //dbg!(prev_norm_minute);
            //dbg!(norm_minute);
            // dbg!(prev_raw_minute);
            // dbg!(raw_minute);
            // dbg!(prev_raw_minute - raw_minute);
            self.prev_raw_minute = Some(raw_minute);
            // инициализированы переменные предыдущего значения времени
            // и значение тянущейся переменной минут
            if raw_minute.abs_diff(prev_raw_minute) < 30 {
                // изменение времени за тик больше 30 секунд
                if raw_minute < prev_raw_minute && norm_minute > prev_norm_minute {
                    norm_hour = (norm_hour - 1).rem_euclid(24);
                    //dbg!("inc hour (slider)");
                }
                if raw_minute > prev_raw_minute && norm_minute < prev_norm_minute {
                    norm_hour = (norm_hour + 1).rem_euclid(24);
                    //dbg!("dec hour (slider)");
                }
            }
        }

        norm_hour = norm_hour.rem_euclid(24);

        egui::CentralPanel::default().show(ctx, |ui| {
            //angle of hour arrow
            let h_angle = TAU * (norm_hour.rem_euclid(12) as f32) / 12.0
                + TAU * (norm_minute as f32) / 60.0 / 12.0;

            //angle for minute arrow
            let mut m_angle = TAU * (norm_minute as f32) / 60.0;

            let size = ui.available_size();
            let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
            let rect = response.rect;
            let center = rect.center();
            let radius = rect.height() * 0.8 / 2. - 10.;

            Self::draw_clock_face(&painter, center, radius);

            let h_arrow_stroke = Stroke::new(10., Color32::BLACK);
            painter.line_segment(
                [
                    center,
                    center + radius * 0.6 * Vec2::angled(h_angle - TAU / 4.),
                ],
                h_arrow_stroke,
            );

            if let Some(angle) = self.minute_arrow_pos {
                m_angle = angle;
                norm_minute = (m_angle * 60. / TAU).floor() as i32;

                raw_minute = norm_minute;
            }

            let m_arrow_stroke = Stroke::new(5., Color32::BLACK);
            painter.line_segment(
                [
                    center,
                    center + radius * 0.8 * Vec2::angled(m_angle - TAU / 4.),
                ],
                m_arrow_stroke,
            );

            let m_rect = egui::Rect::from_center_size(
                center + radius * 0.8 * Vec2::angled(m_angle - TAU / 4.),
                vec2(10., 10.),
            );
            let m_arrow_resp = ui.allocate_rect(m_rect, Sense::drag());

            if m_arrow_resp.hovered() {
                painter.rect_stroke(m_rect, 0., Stroke::new(5., Color32::BLUE));
            }

            if m_arrow_resp.dragged() {
                let pos = m_rect.center() + m_arrow_resp.drag_delta();
                let mut angle = (pos - center).angle() + TAU / 4.;
                if angle < 0. {
                    angle = TAU + angle;
                }
                self.minute_arrow_pos = Some(angle);

                // Calculate inc|dec hour
                prev_norm_minute = (m_angle * 60. / TAU).floor() as i32;
                norm_minute = (angle * 60. / TAU).floor() as i32;
                if prev_norm_minute != norm_minute {
                    if norm_minute.abs_diff(prev_norm_minute) > 30 {
                        // изменение времени за тик больше 30 секунд
                        if norm_minute > prev_norm_minute {
                            norm_hour = (norm_hour - 1).rem_euclid(24);
                            //dbg!("dec hour (dragged)");
                        }
                        if norm_minute < prev_norm_minute {
                            norm_hour = (norm_hour + 1).rem_euclid(24);
                            //dbg!("inc hour (dragged)");
                        }
                    }
                }
            } else {
                self.minute_arrow_pos = None;
            }
        });

        norm_hour = norm_hour.rem_euclid(24);

        self.prev_raw_minute = Some(raw_minute);
        self.time = if set_local_time {
            OffsetDateTime::now_local()
                .unwrap_or(OffsetDateTime::now_utc())
                .time()
        } else {
            Time::from_hms(norm_hour as u8, norm_minute as u8, 0).unwrap()
        };
    }
}
