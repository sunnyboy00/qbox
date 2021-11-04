pub mod chart_header;
pub mod indicator;
pub mod instrument;
pub mod kline;
pub mod main_menu;
pub mod manual_trade;
pub mod sys_menu;
pub mod trades;

use self::instrument::InstrumentView;

use super::widget::Button;
use super::State;
use crate::app::command::{
    QBOX_INSTRUMENT_CLICKED, QBOX_INSTRUMENT_SELECTED, QBOX_INSTRUMENT_UNSELECTED,
    QBOX_KLINE_LOSE_HOT,
};
use chart_header::ChartHeader;
use druid::widget::{Controller, Flex, SizedBox, Split, ViewSwitcher, Widget, WidgetExt};
use druid::{commands, theme, Application, Data, Env, Event, EventCtx};
use kline::KLine;
use main_menu::MainMenu;
use manual_trade::ManualTradeView;
use std::sync::Arc;
use sys_menu::SysMenu;

#[derive(Debug, Clone, Data, Copy, PartialEq)]
pub enum MainView {
    Config,
    Quotes,
    Trade,
    Strategy,
    Account,
    System,
}

impl Default for MainView {
    fn default() -> Self {
        MainView::Quotes
    }
}

impl MainView {
    pub fn view() -> impl Widget<State> {
        let view_switcher = ViewSwitcher::new(
            |data: &State, _env| data.current_view,
            |selector, _data, _env| match selector {
                MainView::Config => Box::new(
                    Split::columns(InstrumentView::view(), SizedBox::empty())
                        .draggable(true)
                        .solid_bar(false)
                        .min_size(60.0, 60.0)
                        .split_point(0.25)
                        .bar_size(2.0)
                        .min_bar_area(5.0)
                        .padding(5.0),
                ),
                MainView::Quotes => Box::new(
                    Split::columns(
                        Split::rows(KLine::view(), ManualTradeView::view())
                            .split_point(0.6)
                            .draggable(true)
                            .solid_bar(false)
                            .bar_size(2.0)
                            .min_bar_area(10.0)
                            .min_size(200.0, 200.0),
                        Split::rows(
                            InstrumentView::view(),
                            InstrumentView::detail().lens(State::current_instrument),
                        )
                        .split_point(0.4)
                        .draggable(true)
                        .solid_bar(false)
                        .bar_size(2.0)
                        .min_bar_area(5.0)
                        .min_size(200.0, 200.0), //.border(Color::rgb8(0x71, 0x71, 0x71), 1.0),
                    )
                    .draggable(true)
                    .solid_bar(false)
                    .min_size(520.0, 280.0)
                    .split_point(0.84)
                    .bar_size(2.0)
                    .min_bar_area(5.0),
                ),
                MainView::Trade => Box::new(SizedBox::empty()),
                MainView::Strategy => Box::new(SizedBox::empty()),
                MainView::Account => Box::new(SizedBox::empty()),
                MainView::System => Box::new(SysMenu::view().background(theme::BACKGROUND_DARK)),
            },
        );

        Flex::column()
            .with_child(
                ChartHeader::view().fix_height(50.).align_left().center(), //.background(Color::rgb8(0, 0x77, 0x88)),
            )
            .with_flex_child(
                Flex::row()
                    .with_child(MainMenu::view().fix_width(50.).padding(5.0).align_left())
                    .with_flex_child(
                        view_switcher,
                        //.padding(5.0),
                        1.0,
                    ),
                1.0,
            )
            .background(theme::BACKGROUND_DARK)
            .controller(MainController::default())
    }
}

fn symbol_ui() -> impl Widget<State> {
    // Scroll::new(
    Flex::column()
        .with_flex_child(
            Button::new("GME").expand_width().height(50.0).on_click(
                |_ctx, state: &mut State, _env| {
                    state.flush_symbol("GME");
                },
            ),
            1.0,
        )
        .with_flex_child(
            Button::new("DLPN").expand_width().height(50.0).on_click(
                |_ctx, state: &mut State, _env| {
                    state.flush_symbol("dlpn");
                },
            ),
            1.0,
        )
        .with_flex_child(
            Button::new("OIL")
                //.padding(10.0)
                .expand_width()
                .height(50.0)
                // .background(Color::rgb(0.5, 0.5, 0.5))
                .on_click(|_ctx, state: &mut State, _env| {
                    state.flush_symbol("oil");
                }),
            1.0,
        )
}

#[derive(Default)]
struct MainController;

impl<W: Widget<State>> Controller<State, W> for MainController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        state: &mut State,
        env: &Env,
    ) {
        match event {
            Event::WindowCloseRequested => Application::global().quit(),
            Event::MouseDown(_me) => {
                if let Some(wid) = state.clone().windid {
                    println!("close window {:?}", wid);
                    ctx.submit_command(commands::CLOSE_WINDOW.to(*wid));
                    state.windid = None;
                }
            }
            Event::Command(cmd) if cmd.is(QBOX_KLINE_LOSE_HOT) => {
                state.coord = None;
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(QBOX_INSTRUMENT_CLICKED) => {
                println!("{:?}", cmd);
                state.current_instrument = cmd.get(QBOX_INSTRUMENT_CLICKED).unwrap().clone();
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(QBOX_INSTRUMENT_SELECTED) => {
                println!("{:?}", cmd);
                let data = Arc::make_mut(&mut state.self_instruments);
                data.push(cmd.get(QBOX_INSTRUMENT_SELECTED).unwrap().clone());
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(QBOX_INSTRUMENT_UNSELECTED) => {
                println!("{:?}", cmd);
                let instr = cmd.get(QBOX_INSTRUMENT_UNSELECTED).unwrap().clone();
                if let Some(idx) = state.self_instruments.iter().position(|x| {
                    x.exchange == instr.exchange && x.security_id == instr.security_id
                }) {
                    let data = Arc::make_mut(&mut state.self_instruments);
                    data.remove(idx);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
        if !ctx.is_handled() {
            child.event(ctx, event, state, env);
        }
    }
}
