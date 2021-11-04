use crate::app::view::MainView;
use crate::app::widget::Button;
use crate::app::State;
use druid::widget::Flex;
use druid::{Widget, WidgetExt};

pub struct MainMenu;

impl MainMenu {
    pub fn view() -> impl Widget<State> {
        // Scroll::new(
        Flex::column()
            .with_child(
                Button::new("配置")
                    //.padding(10.0)
                    .expand_width()
                    .height(50.0)
                    // .background(Color::rgb(0.5, 0.5, 0.5))
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::Config;
                    })
                    .lens(State::current_view),
            )
            .with_spacer(1.0)
            .with_child(
                Button::new("行情")
                    .expand_width()
                    .height(50.0)
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::Quotes;
                    })
                    .lens(State::current_view),
            )
            .with_spacer(1.0)
            .with_child(
                Button::new("交易")
                    .expand_width()
                    .height(50.0)
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::Trade;
                    })
                    .lens(State::current_view),
            )
            .with_spacer(1.0)
            .with_child(
                Button::new("策略")
                    .expand_width()
                    .height(50.0)
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::Strategy;
                    })
                    .lens(State::current_view),
            )
            .with_spacer(1.0)
            .with_child(
                Button::new("账户")
                    //.padding(10.0)
                    .expand_width()
                    .height(50.0)
                    // .background(Color::rgb(0.5, 0.5, 0.5))
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::Account;
                    })
                    .lens(State::current_view),
            )
            .with_flex_spacer(1.0)
            .with_child(
                Button::new("系统")
                    //.padding(10.0)
                    .expand_width()
                    .height(50.0)
                    // .background(Color::rgb(0.5, 0.5, 0.5))
                    .on_click(|_ctx, data: &mut MainView, _env| {
                        *data = MainView::System;
                    })
                    .lens(State::current_view),
                // Button::new("系统")
                //     //.padding(10.0)
                //     .expand_width()
                //     .height(50.0)
                //     // .background(Color::rgb(0.5, 0.5, 0.5))
                //     .on_click(|ctx, state: &mut State, env| {
                //         let size = ctx.size();
                //         let origin = ctx.window_origin();
                //         let position = origin + (size.width + 1.0, -400. + size.height);
                //         #[cfg(target_os = "macos")]
                //         let position =
                //             ctx.to_screen(Point::new(size.width + 1.0, -400. + size.height));
                //         let wid = ctx.new_sub_window(
                //             WindowConfig::default()
                //                 .show_titlebar(false)
                //                 .resizable(false)
                //                 .window_size(Size::new(200., 400.))
                //                 .transparent(true)
                //                 .set_position(position)
                //                 .set_level(WindowLevel::Tooltip(ctx.window().clone())),
                //             SysMenu::view().background(theme::BACKGROUND_DARK),
                //             state.clone(),
                //             env.clone(),
                //         );

                //         state.windid = Some(Arc::new(wid));
                //         ctx.set_handled();
                //         ctx.set_active(true);
                //     }),
            )
            .with_spacer(5.)
    }
}
