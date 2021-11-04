use crate::app::command::{QBOX_INSTRUMENT_SELECTED, QBOX_KLINE_LOSE_HOT};
use crate::app::widget::Button;
use crate::app::State;
use druid::kurbo::{Affine, BezPath, Line, Rect};
use druid::widget::{Controller, Flex, Painter};
use druid::{
    Color, Env, Event, EventCtx, FontDescriptor, FontFamily, LifeCycle, LifeCycleCtx,
    RenderContext, TextLayout, Widget, WidgetExt,
};
use qbox_core::indicators::{Normalize, Scale, SMA};

pub struct KLine;

impl KLine {
    pub fn view() -> impl Widget<State> {
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(
                        Button::new("m")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("图形")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("指标")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .fix_height(30.)
                    .align_left()
                    .padding(5.0),
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(KLine::candle(), 5.0)
                    .with_flex_child(KLine::volum(), 2.0),
                1.0,
            )
            .with_spacer(1.0)
            .with_child(
                Flex::row()
                    .with_child(
                        Button::new("1D")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("5D")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("5D")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("15D")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("1M")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("3M")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("6M")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("YTD")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("1Y")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("5Y")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .with_spacer(1.0)
                    .with_child(
                        Button::new("ALL")
                            .fix_width(50.)
                            .expand_height()
                            .on_click(|_ctx, state: &mut State, env| {}),
                    )
                    .fix_height(30.)
                    .align_left(),
            )
    }
    fn candle() -> impl Widget<State> {
        Painter::new(|ctx, state: &State, env| {
            let size = ctx.size();
            let rect = size.to_rect();
            ctx.fill(rect, &Color::BLACK);
            ctx.with_save(|ctx| {
                let mut layout = TextLayout::<String>::from_text("K线");
                layout.set_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0));
                layout.set_text_color(Color::RED);
                layout.rebuild_if_needed(ctx.text(), env);
                layout.draw(ctx, (0. + 10., 0. + 10.));
            });
            if state.data.len() > 0 {
                if let Some(p) = state.coord {
                    ctx.paint_with_z_index(1, move |ctx| {
                        let mut path = BezPath::new();
                        path.move_to((
                            0.0,
                            if p.y > rect.y1 {
                                rect.y1
                            } else if p.y < 0. {
                                1.0
                            } else {
                                p.y
                            },
                        ));
                        path.line_to((
                            rect.x1,
                            if p.y > rect.y1 {
                                rect.y1
                            } else if p.y < 0. {
                                1.0
                            } else {
                                p.y
                            },
                        ));
                        ctx.stroke(path, &Color::rgba(1., 1., 1., 0.5), 1.0);
                        let mut path = BezPath::new();
                        path.move_to((
                            if p.x > rect.x1 {
                                rect.x1
                            } else if p.x < 0. {
                                1.0
                            } else {
                                p.x
                            },
                            0.0,
                        ));
                        path.line_to((
                            if p.x > rect.x1 {
                                rect.x1
                            } else if p.x < 0. {
                                1.0
                            } else {
                                p.x
                            },
                            rect.y1,
                        ));
                        ctx.stroke(path, &Color::rgba(1., 1., 1., 0.5), 1.0);
                    });
                }
            }

            let data = state.data.normalize().scale(size.height * 0.8);
            if data.len() != 0 {
                ctx.with_save(|ctx| {
                    ctx.transform(
                        Affine::translate((rect.x0, rect.y1))
                            * Affine::FLIP_Y
                            * Affine::scale(state.scale),
                    ); //Y方向翻转
                    let step = size.width / data.len() as f64;
                    let mut idx = rect.x0 + step;
                    for elem in data {
                        let open = elem.open;
                        let high = elem.high;
                        let low = elem.low;
                        let close = elem.close;
                        let col = if close > open {
                            Color::RED //阳线
                        } else {
                            Color::GREEN //阴线
                        };
                        ctx.stroke(Line::new((idx, high), (idx, low)), &Color::WHITE, 1.0);
                        ctx.fill(
                            Rect::from_origin_size((idx - 2., open), (4., (close - open).abs())),
                            &col,
                        );
                        idx += step;
                    }
                });
                ctx.with_save(|ctx| {
                    let ndata = state.data.ma(5).normalize().scale(size.height * 0.8);
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
                        Affine::translate((rect.x0, rect.y1))
                            * Affine::FLIP_Y
                            * Affine::scale(state.scale),
                    ); //Y方向翻转
                    ctx.stroke(path, &Color::WHITE, 1.0);
                });

                ctx.with_save(|ctx| {
                    let ndata = state.data.ma(20).normalize().scale(size.height * 0.8);
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
                        Affine::translate((rect.x0, rect.y1))
                            * Affine::FLIP_Y
                            * Affine::scale(state.scale),
                    ); //Y方向翻转
                    ctx.stroke(path, &Color::YELLOW, 1.0);
                });
            }
        })
        .controller(CoordinateController::default())
    }

    fn volum() -> impl Widget<State> {
        Painter::new(|ctx, state: &State, _env| {
            let size = ctx.size();
            let rect = size.to_rect();
            ctx.fill(rect, &Color::BLACK);

            ctx.with_save(|ctx| {
                ctx.transform(
                    Affine::translate((rect.x0, rect.y1))
                        * Affine::FLIP_Y
                        * Affine::scale(state.scale),
                ); //Y方向翻转
                let data = state.data.normalize().scale(size.height * 0.8);
                if data.len() != 0 {
                    let step = size.width / data.len() as f64;
                    let mut idx = rect.x0 + step;
                    for elem in data {
                        let col = if elem.close > elem.open {
                            Color::RED //阳线
                        } else {
                            Color::GREEN //阴线
                        };
                        let volum = elem.volume;
                        ctx.fill(Rect::from_origin_size((idx - 2., 0.), (4., volum)), &col);
                        idx += step;
                    }
                }
            });
            let data: Vec<f64> = state.data.iter().map(|bar| bar.volume).collect();
            ctx.with_save(|ctx| {
                let ndata = data.ma(5).normalize().scale(size.height * 0.8);
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
                    Affine::translate((rect.x0, rect.y1))
                        * Affine::FLIP_Y
                        * Affine::scale(state.scale),
                ); //Y方向翻转
                ctx.stroke(path, &Color::WHITE, 1.0);
            });
            ctx.with_save(|ctx| {
                let ndata = data.ma(60).normalize().scale(size.height * 0.8);
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
                    Affine::translate((rect.x0, rect.y1))
                        * Affine::FLIP_Y
                        * Affine::scale(state.scale),
                ); //Y方向翻转
                ctx.stroke(path, &Color::YELLOW, 1.0);
            });
        })
    }
}

#[derive(Default)]
struct CoordinateController;

impl<W: Widget<State>> Controller<State, W> for CoordinateController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        state: &mut State,
        env: &Env,
    ) {
        match event {
            Event::MouseUp(ev) | Event::MouseDown(ev) | Event::MouseMove(ev) => {
                state.coord = Some(ev.pos);
                ctx.request_paint();
            }
            Event::Wheel(ev) => {
                if ev.wheel_delta.y > 0. {
                    state.scale -= 0.3;
                } else if ev.wheel_delta.y < 0. {
                    state.scale += 0.3;
                }
                ctx.request_paint_rect(ctx.size().to_rect());
                // ctx.request_paint();
            }
            // Event::Command(cmd) if cmd.is(QBOX_KLINE_LOSE_HOT) => {
            //     state.coord = None;
            //     ctx.request_paint();
            // }
            // Event::Command(cmd) if cmd.is(QBOX_INSTRUMENT_SELECTED) => {
            //     println!("{:?}", cmd);
            //     state.current_instrument = cmd.get(QBOX_INSTRUMENT_SELECTED).unwrap().clone();
            //     ctx.request_paint();
            // }
            _ => {}
        }

        if !ctx.is_handled() {
            child.event(ctx, event, state, env);
        }
    }

    fn lifecycle(
        &mut self,
        _child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &State,
        _env: &Env,
    ) {
        if let LifeCycle::HotChanged(hot) = event {
            if !hot {
                ctx.submit_command(QBOX_KLINE_LOSE_HOT);
            }
        }
    }
}
