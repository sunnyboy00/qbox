//证券代码视图
use crate::app::command::{
    QBOX_INSTRUMENT_CLICKED, QBOX_INSTRUMENT_SELECTED, QBOX_INSTRUMENT_UNSELECTED,
};

use crate::app::State;
use crate::trade::Instrument;
use druid::widget::{prelude::*, Checkbox};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, Painter, Scroll, Tabs, TabsTransition};
use druid::{Color, WidgetExt};

const TABLE_HEADER: &[(&str, f64)] = &[
    ("自选", 60.0),
    ("标的", 100.0),
    ("最新价", 100.0),
    ("均价", 100.0),
];

pub struct InstrumentView;

impl InstrumentView {
    pub fn view() -> impl Widget<State> {
        Tabs::new()
            .with_tab(
                "自选",
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Flex::row()
                            .with_child(Label::new(TABLE_HEADER[1].0).fix_width(TABLE_HEADER[1].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[2].0).fix_width(TABLE_HEADER[2].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[3].0).fix_width(TABLE_HEADER[3].1)),
                    )
                    .with_default_spacer()
                    .with_flex_child(
                        Scroll::new(List::new(make_list_item2).lens(State::self_instruments))
                            .vertical(),
                        1.0,
                    ),
            )
            .with_tab(
                "全部",
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Flex::row()
                            .with_child(
                                Label::new(TABLE_HEADER[0].0)
                                    .center()
                                    .fix_width(TABLE_HEADER[0].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[1].0).fix_width(TABLE_HEADER[1].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[2].0).fix_width(TABLE_HEADER[2].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[3].0).fix_width(TABLE_HEADER[3].1)),
                    )
                    .with_default_spacer()
                    .with_flex_child(
                        Scroll::new(List::new(make_list_item).lens(State::instruments)).vertical(),
                        1.0,
                    ),
            )
            .with_transition(TabsTransition::Instant)
            .padding(5.0)
    }

    pub fn detail() -> impl Widget<Instrument> {
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Label::new(|instr: &Instrument, _: &Env| {
                format!("{}({})", instr.symbol.as_str(), instr.security_id.as_str())
            }))
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("种类"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.kind.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("均价"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.average_price.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("开盘价"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.open_price.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("最新结算价"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.last_settlement_price.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("昨结算价"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.pre_settlement_price.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("杠杆"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.margin_level.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("保证金率"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.fee_margin.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("申报费率"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.fee_offer.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("成交费率(主)"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.fee_take.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("成交费率(被)"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        instr.fee_make.to_string()
                    })),
            )
            .with_default_spacer()
            .with_child(
                Flex::row()
                    .with_child(Label::new("报单精度"))
                    .with_default_spacer()
                    .with_child(Label::new(|instr: &Instrument, _: &Env| {
                        format!("{:?}", instr.tick_precision)
                    })),
            )
            .scroll()
    }
}

fn make_list_item() -> impl Widget<Instrument> {
    let brush = Painter::new(|ctx, d: &Instrument, env| {
        let bounds = ctx.size().to_rect();
        if ctx.is_hot() {
            ctx.fill(bounds, &Color::rgb8(0x00, 0x51, 0x51));
        }
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x00, 0x71, 0x71));
        }
    });
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Checkbox::new("")
                .lens(Instrument::myself)
                .center()
                .fix_width(TABLE_HEADER[0].1)
                .on_click(|ctx, b, _c| {
                    if !b.myself {
                        ctx.submit_command(QBOX_INSTRUMENT_SELECTED.with(b.clone()));
                    } else {
                        ctx.submit_command(QBOX_INSTRUMENT_UNSELECTED.with(b.clone()));
                    }
                }),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.symbol.to_string())
                .fix_width(TABLE_HEADER[1].1),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.last_settlement_price.to_string())
                .fix_width(TABLE_HEADER[2].1),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.average_price.to_string())
                .fix_width(TABLE_HEADER[3].1),
        )
        .background(brush)
        .on_click(|ctx, b, c| {
            let bb = b.clone();
            ctx.submit_command(QBOX_INSTRUMENT_CLICKED.with(bb));
        })
}

fn make_list_item2() -> impl Widget<Instrument> {
    let brush = Painter::new(|ctx, d: &Instrument, env| {
        let bounds = ctx.size().to_rect();
        if ctx.is_hot() {
            ctx.fill(bounds, &Color::rgb8(0x00, 0x51, 0x51));
        }
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x00, 0x71, 0x71));
        }
    });
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.symbol.to_string())
                .fix_width(TABLE_HEADER[0].1),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.last_settlement_price.to_string())
                .fix_width(TABLE_HEADER[1].1),
        )
        .with_flex_spacer(1.0)
        .with_child(
            Label::new(|d: &Instrument, _env: &Env| d.average_price.to_string())
                .fix_width(TABLE_HEADER[2].1),
        )
        .background(brush)
        .on_click(|ctx, b, c| {
            let bb = b.clone();
            ctx.submit_command(QBOX_INSTRUMENT_CLICKED.with(bb));
        })
}
