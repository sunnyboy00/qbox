use crate::app::widget::Button;
use crate::app::State;
use druid::widget::{Controller, Flex, LineBreaking, RawLabel};
use druid::{Env, Event, EventCtx, Point, TextAlignment, Widget, WidgetExt, WindowState};

pub struct ChartHeader;

impl ChartHeader {
    pub fn view() -> impl Widget<State> {
        Flex::row()
            .with_child(Button::new("三").fix_width(50.).expand_height().on_click(
                |_ctx, state: &mut State, env| {
                    state.left_menu = !state.left_menu;
                },
            ))
            .with_spacer(2.0)
            .with_child(
                RawLabel::new()
                    .with_line_break_mode(LineBreaking::Clip)
                    .with_text_alignment(TextAlignment::Start)
                    .padding(2.0)
                    .center()
                    .lens(State::name),
            )
            //.with_spacer(2.0)
            // .with_child(
            //     Button::new("BTCUSDT")
            //         .fix_width(120.)
            //         .expand_height()
            //         .on_click(|_ctx, state: &mut State, _env| {
            //             //state.flush_symbol("GME");
            //         }),
            // )
            // .with_spacer(1.0)
            // .with_child(Button::new("D").fix_width(40.).expand_height().on_click(
            //     |_ctx, state: &mut State, _env| {
            //         //state.flush_symbol("GME");
            //     },
            // ))
            // .with_spacer(1.0)
            // .with_child(Button::new("图形").fix_width(50.).expand_height().on_click(
            //     |_ctx, state: &mut State, _env| {
            //         //state.flush_symbol("GME");
            //     },
            // ))
            // .with_spacer(1.0)
            // .with_child(Button::new("对比").fix_width(50.).expand_height().on_click(
            //     |_ctx, state: &mut State, _env| {
            //         //state.flush_symbol("GME");
            //     },
            // ))
            // .with_spacer(1.0)
            // .with_child(
            //     Button::new("技术指标")
            //         .fix_width(100.)
            //         .expand_height()
            //         .on_click(|_ctx, state: &mut State, _env| {
            //             //state.flush_symbol("GME");
            //         }),
            // )
            // .with_spacer(1.0)
            // .with_child(
            //     Button::new("财务指标")
            //         .fix_width(100.)
            //         .expand_height()
            //         .on_click(|_ctx, state: &mut State, _env| {
            //             //state.flush_symbol("GME");
            //         }),
            // )
            // .with_spacer(1.0)
            // .with_child(Button::new("模板").fix_width(50.).expand_height().on_click(
            //     |_ctx, state: &mut State, _env| {
            //         //state.flush_symbol("GME");
            //     },
            // ))
            // .with_spacer(1.0)
            // .with_child(Button::new("报警").fix_width(50.).expand_height().on_click(
            //     |_ctx, state: &mut State, _env| {
            //         //state.flush_symbol("GME");
            //     },
            // ))
            .with_flex_spacer(1.0)
            .with_child(Button::new("╋").fix_width(50.).expand_height().on_click(
                |_ctx, state: &mut State, _env| {
                    //state.flush_symbol("GME");
                },
            ))
            .with_spacer(1.0)
            .with_child(Button::new("_").fix_width(40.).expand_height().on_click(
                |ctx, _state: &mut State, _env| {
                    ctx.window()
                        .clone()
                        .set_window_state(WindowState::Minimized);
                },
            ))
            .with_spacer(1.0)
            .with_child(Button::new("□").fix_width(40.).expand_height().on_click(
                |ctx, _state: &mut State, _env| {
                    if ctx.window().get_window_state() == WindowState::Maximized {
                        ctx.window().clone().set_window_state(WindowState::Restored);
                    } else if ctx.window().get_window_state() == WindowState::Restored {
                        ctx.window()
                            .clone()
                            .set_window_state(WindowState::Maximized);
                    }
                },
            ))
            .padding(5.0)
            .fix_height(50.0)
            .controller(DragController::default())
    }
}

#[derive(Default)]
struct DragController {
    init_pos: Option<Point>,
}

impl<W: Widget<State>> Controller<State, W> for DragController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut State,
        env: &Env,
    ) {
        match event {
            Event::MouseDown(me) => {
                if me.buttons.has_left() {
                    ctx.set_active(true);
                    self.init_pos = Some(me.window_pos)
                }
            }
            Event::MouseMove(me) if ctx.is_active() && me.buttons.has_left() => {
                if ctx.window().get_window_state() != WindowState::Maximized {
                    if let Some(init_pos) = self.init_pos {
                        let within_window_change = me.window_pos.to_vec2() - init_pos.to_vec2();
                        let old_pos = ctx.window().get_position();
                        let new_pos = old_pos + within_window_change;
                        // let scale = ctx.window().get_scale().unwrap();
                        ctx.window().set_position(new_pos)
                    }
                }
            }
            Event::MouseUp(_me) if ctx.is_active() => {
                self.init_pos = None;
                ctx.set_active(false)
            }

            _ => (),
        }
        if !ctx.is_handled() {
            child.event(ctx, event, data, env);
        }
    }
}
