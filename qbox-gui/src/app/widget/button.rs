use druid::widget::{Label, LabelText, Painter, Widget, WidgetExt};
use druid::{theme, Color, Data, RenderContext};

pub struct Button;

impl Button {
    pub fn new<T: Data>(name: &str) -> impl Widget<T> {
        let painter = Painter::new(|ctx, _, env| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));
            if ctx.is_hot() {
                ctx.fill(bounds, &Color::rgb8(0x51, 0x51, 0x51));
            }
            if ctx.is_active() {
                ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
            }
        });
        Label::new(name)
            .with_text_size(16.)
            .center()
            .background(painter)
    }

    pub fn dynamic<T: Data>(text: impl Into<LabelText<T>>) -> impl Widget<T> {
        let painter = Painter::new(|ctx, _, env| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

            if ctx.is_hot() {
                ctx.fill(bounds, &Color::rgb8(0x51, 0x51, 0x51));
            }
            if ctx.is_active() {
                ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
            }
        });
        Label::new(text)
            .with_text_size(16.)
            .center()
            .background(painter)
    }
}
