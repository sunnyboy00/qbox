//证券代码视图
use crate::app::command::QBOX_INSTRUMENT_SELECTED;
use crate::trade::Order;

use crate::app::State;

use druid::widget::{prelude::*, SizedBox};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, List, Painter, Scroll, Tabs, TabsEdge, TabsTransition,
};
use druid::{Color, WidgetExt};

const TABLE_HEADER: &[(&str, f64)] = &[
    ("订单号", 100.0),
    ("时间", 100.0),
    ("交易所", 60.0),
    ("标的", 100.0),
    ("类型", 80.0),
    ("方向", 50.0),
    ("价格", 100.0),
    ("成交均价", 100.0),
    ("委托量", 100.0),
    ("已成交", 100.0),
    ("状态", 60.0),
    ("账户", 60.0),
    ("策略", 60.0),
    ("操作", 60.0),
    // ("证券类别", 80.0),
    // ("基础货币", 60.0),
    // ("计价货币", 60.0),
    // ("报单精度", 120.0),
    // ("杠杆", 60.0),
    // ("保证金费率", 80.0),
    // ("报单费率", 80.0),
    // ("Taker费率", 80.0),
    // ("Maker费率", 80.0),
    // ("操作", 80.0),
];

pub struct ManualTradeView;

impl ManualTradeView {
    pub fn view() -> impl Widget<State> {
        Tabs::new()
            .with_tab(
                "全部委托",
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Flex::row()
                            .with_child(Label::new(TABLE_HEADER[0].0).fix_width(TABLE_HEADER[0].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[1].0).fix_width(TABLE_HEADER[1].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[2].0).fix_width(TABLE_HEADER[2].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[3].0).fix_width(TABLE_HEADER[3].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[4].0).fix_width(TABLE_HEADER[4].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[5].0).fix_width(TABLE_HEADER[5].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[6].0).fix_width(TABLE_HEADER[6].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[7].0).fix_width(TABLE_HEADER[7].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[8].0).fix_width(TABLE_HEADER[8].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[9].0).fix_width(TABLE_HEADER[9].1))
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[10].0).fix_width(TABLE_HEADER[10].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[11].0).fix_width(TABLE_HEADER[11].1),
                            ),
                    )
                    .with_default_spacer()
                    .with_flex_child(
                        Scroll::new(List::new(make_list_item).lens(State::orders)).vertical(),
                        1.0,
                    ),
            )
            .with_tab(
                "未完成委托",
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(
                        Flex::row()
                            .with_child(Label::new(TABLE_HEADER[0].0).fix_width(TABLE_HEADER[0].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[1].0).fix_width(TABLE_HEADER[1].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[2].0).fix_width(TABLE_HEADER[2].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[3].0).fix_width(TABLE_HEADER[3].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[4].0).fix_width(TABLE_HEADER[4].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[5].0).fix_width(TABLE_HEADER[5].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[6].0).fix_width(TABLE_HEADER[6].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[7].0).fix_width(TABLE_HEADER[7].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[8].0).fix_width(TABLE_HEADER[8].1))
                            .with_flex_spacer(1.0)
                            .with_child(Label::new(TABLE_HEADER[9].0).fix_width(TABLE_HEADER[9].1))
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[10].0).fix_width(TABLE_HEADER[10].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[11].0).fix_width(TABLE_HEADER[11].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[12].0).fix_width(TABLE_HEADER[12].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[12].0).fix_width(TABLE_HEADER[12].1),
                            )
                            .with_flex_spacer(1.0)
                            .with_child(
                                Label::new(TABLE_HEADER[13].0).fix_width(TABLE_HEADER[13].1),
                            ),
                    )
                    .with_default_spacer()
                    .with_flex_child(
                        Scroll::new(List::new(make_list_item).lens(State::orders)).vertical(),
                        1.0,
                    ),
            )
            .with_tab("手动交易", SizedBox::empty())
            .with_transition(TabsTransition::Instant)
            .with_edge(TabsEdge::Leading)
            .padding(5.0)
    }
}

fn make_list_item() -> impl Widget<Order> {
    let brush = Painter::new(|ctx, d: &Order, env| {
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
        .with_child(Label::new(|d: &Order, _env: &Env| "".to_string()).fix_width(TABLE_HEADER[0].1))
        .with_flex_spacer(1.0)
        .with_child(Label::new(|d: &Order, _env: &Env| "".to_string()).fix_width(TABLE_HEADER[1].1))
        .with_flex_spacer(1.0)
        .with_child(Label::new(|d: &Order, _env: &Env| "".to_string()).fix_width(TABLE_HEADER[2].1))
        .background(brush)
        .on_click(|ctx, b, c| {})
}
