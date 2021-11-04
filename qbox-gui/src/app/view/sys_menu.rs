use crate::app::widget::Button;
use crate::app::State;
use druid::widget::{Controller, Flex, LineBreaking, RawLabel, SizedBox};
use druid::{Application, Env, Event, EventCtx, TextAlignment, Widget, WidgetExt};

struct CancelClose;

impl<W: Widget<State>> Controller<State, W> for CancelClose {
    fn event(
        &mut self,
        w: &mut W,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut State,
        env: &Env,
    ) {
        println!("{:?}", event);
        match event {
            // Event::WindowCloseRequested => {
            //     println!("CancelClonse {:?}", event);
            //     ctx.window().close();
            // }
            // Event::WindowDisconnected => {
            //     println!("CancelClonse {:?}", event);
            //     ctx.window().close();
            //     // ctx.set_handled();
            // }
            _ => w.event(ctx, event, data, env),
        }
    }
}

pub struct SysMenu;

impl SysMenu {
    pub fn view() -> impl Widget<State> {
        Flex::column()
            // .with_child(
            //     RawLabel::new()
            //         .with_line_break_mode(LineBreaking::Clip)
            //         .with_text_alignment(TextAlignment::Start)
            //         .center()
            //         .padding(2.0)
            //         .expand_width()
            //         .lens(State::name2),
            // )
            // .with_spacer(1.0)
            .with_flex_child(SizedBox::empty().expand(), 1.0)
            .with_spacer(1.0)
            .with_child(Button::new("退出").expand_width().height(30.).on_click(
                |_ctx, _state: &mut State, _env| {
                    Application::global().quit();
                },
            ))
            .padding(5.0)

        // .fix_height(30.)
        // .controller(CancelClose {})
    }
}
