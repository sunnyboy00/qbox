use crate::app::widget::{Orientation, RadioGroup};
use crate::app::State;
use druid::kurbo::{Affine, BezPath};
use druid::widget::{Flex, Painter, WidgetExt};
use druid::{Color, Data, FontDescriptor, FontFamily, RenderContext, TextLayout, Widget};
use qbox_core::indicators::{Normalize, Scale, EMA, MACD};

pub struct IndicatorView;

#[derive(Debug, Clone, Data, Copy, PartialEq)]
pub enum MyRadio {
    SMA,
    EMA,
    MACD,
    BLJJ,
    KDJ,
    BOLL,
}

impl Default for MyRadio {
    fn default() -> Self {
        MyRadio::MACD
    }
}

impl MyRadio {
    pub fn name(&self) -> String {
        match self {
            &MyRadio::SMA => "SMA",
            &MyRadio::MACD => "MACD",
            &MyRadio::BLJJ => "BLJJ",
            &MyRadio::EMA => "EMA",
            &MyRadio::KDJ => "KDJ",
            &MyRadio::BOLL => "BOLL",
        }
        .into()
    }
}

impl IndicatorView {
    pub fn view() -> impl Widget<State> {
        Flex::column()
            .with_child(
                RadioGroup::new(
                    vec![
                        ("SMA", MyRadio::SMA),
                        ("EMA", MyRadio::EMA),
                        ("MACD", MyRadio::MACD),
                        ("BLJJ", MyRadio::BLJJ),
                        ("KDJ", MyRadio::KDJ),
                        ("BOLL", MyRadio::BOLL),
                    ],
                    Orientation::Horizontal,
                )
                .lens(State::radio)
                .align_left(),
            )
            .with_default_spacer()
            .with_flex_child(
                Painter::new(|ctx, state: &State, env| {
                    let size = ctx.size();
                    let rect = size.to_rect();
                    ctx.fill(rect, &Color::BLACK);
                    ctx.with_save(|ctx| {
                        let text = state.radio.name();
                        let mut layout = TextLayout::<String>::from_text(text);
                        layout.set_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0));
                        layout.set_text_color(Color::RED);
                        layout.rebuild_if_needed(ctx.text(), env);
                        layout.draw(ctx, (0. + 10., 0. + 10.));
                        if state.radio.same(&MyRadio::MACD) {
                            let mut layout = TextLayout::<String>::from_text("EMA-3");
                            layout.set_font(
                                FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0),
                            );
                            layout.set_text_color(Color::YELLOW);
                            layout.rebuild_if_needed(ctx.text(), env);
                            layout.draw(ctx, (0. + 80., 0. + 10.));
                            let mut layout = TextLayout::<String>::from_text("EMA-6");
                            layout.set_font(
                                FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0),
                            );
                            layout.set_text_color(Color::GREEN);
                            layout.rebuild_if_needed(ctx.text(), env);
                            layout.draw(ctx, (0. + 160., 0. + 10.));
                            let mut layout = TextLayout::<String>::from_text("EMA-4");
                            layout.set_font(
                                FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0),
                            );
                            layout.set_text_color(Color::RED);
                            layout.rebuild_if_needed(ctx.text(), env);
                            layout.draw(ctx, (0. + 240., 0. + 10.));
                        }
                    });
                    ctx.with_save(|ctx| {
                        let data = state.data.as_ref();
                        match state.radio {
                            MyRadio::SMA => {
                                let (blue, purple, red, green, yellow, cyan) = (
                                    Color::BLUE,
                                    Color::PURPLE,
                                    Color::RED,
                                    Color::GREEN,
                                    Color::YELLOW,
                                    Color::rgb8(0, 255, 255),
                                );
                            }
                            MyRadio::EMA => {
                                let ndata = data.ema(12).normalize().scale(size.height * 0.8);
                                if ndata.len() != 0 {
                                    let step = size.width / ndata.len() as f64;
                                    let mut idx = rect.x0;
                                    let mut path = BezPath::new();
                                    for (i, elem) in ndata.iter().enumerate() {
                                        let volum = *elem;
                                        if i == 0 {
                                            path.move_to((idx, volum));
                                        } else {
                                            path.line_to((idx, volum));
                                        }
                                        idx += step;
                                    }
                                    ctx.transform(
                                        Affine::translate((rect.x0, rect.y1)) * Affine::FLIP_Y,
                                    ); //变换坐标、缩放、Y方向翻转
                                    ctx.stroke(path, &Color::YELLOW, 1.0);
                                }
                            }
                            MyRadio::MACD => {
                                let ndata =
                                    data.macd(12, 26, 9).normalize().scale(size.height * 0.6);

                                if ndata.len() != 0 {
                                    let step = size.width / ndata.len() as f64;
                                    let mut idx = rect.x0;
                                    let mut fase_path = BezPath::new();
                                    let mut slow_path = BezPath::new();
                                    let mut signal_path = BezPath::new();
                                    for (i, (fast_volum, slow_volum, signal_volum)) in
                                        ndata.iter().enumerate()
                                    {
                                        if i == 0 {
                                            fase_path.move_to((idx, *fast_volum));
                                            slow_path.move_to((idx, *slow_volum));
                                            signal_path.move_to((idx, *signal_volum));
                                        } else {
                                            fase_path.line_to((idx, *fast_volum));
                                            slow_path.line_to((idx, *slow_volum));
                                            signal_path.line_to((idx, *signal_volum));
                                        }
                                        idx += step;
                                    }
                                    ctx.transform(
                                        Affine::translate((rect.x0, rect.y1 - 30.))
                                            * Affine::FLIP_Y,
                                    ); //Y方向翻转
                                    ctx.stroke(fase_path, &Color::YELLOW, 1.0);
                                    ctx.stroke(slow_path, &Color::GREEN, 1.0);
                                }
                            }
                            MyRadio::BLJJ => {
                                let (blue, purple, red, green, yellow, cyan) = (
                                    Color::BLUE,
                                    Color::PURPLE,
                                    Color::RED,
                                    Color::GREEN,
                                    Color::YELLOW,
                                    Color::rgb8(0, 255, 255),
                                );
                            }
                            MyRadio::KDJ => {}
                            MyRadio::BOLL => {}
                        }
                    });
                }),
                4.,
            )
    }
}
