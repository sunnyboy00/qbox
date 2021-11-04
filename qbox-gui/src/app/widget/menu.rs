use super::Button;
use druid::widget::{Flex, Label, Painter, Widget, WidgetExt};
use druid::{
    theme, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, RenderContext, Size, UpdateCtx, WidgetPod,
};

pub struct Menu<T> {
    button: WidgetPod<T, Box<dyn Widget<T>>>,
    child: Option<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data> Menu<T> {
    pub fn new(name: &str) -> Self {
        Self {
            button: WidgetPod::new(Button::new(name).on_click(|ctx, data, env| {
                //TODO:
            }))
            .boxed(),
            child: None,
        }
    }
    pub fn with_menu<W: Widget<T> + 'static>(mut self, w: W) -> Self {
        self.child = Some(WidgetPod::new(w).boxed());
        self
    }
}

// impl<T> Either<T> {
//     /// Create a new widget that switches between two views.
//     ///
//     /// The given closure is evaluated on data change. If its value is `true`, then
//     /// the `true_branch` widget is shown, otherwise `false_branch`.
//     pub fn new(
//         closure: impl Fn(&T, &Env) -> bool + 'static,
//         true_branch: impl Widget<T> + 'static,
//         false_branch: impl Widget<T> + 'static,
//     ) -> Either<T> {
//         Either {
//             closure: Box::new(closure),
//             true_branch: WidgetPod::new(true_branch).boxed(),
//             false_branch: WidgetPod::new(false_branch).boxed(),
//             current: false,
//         }
//     }
// }

impl<T: Data> Widget<T> for Menu<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.button.event(ctx, event, data, env);
        if let Some(child) = &mut self.child {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.button.lifecycle(ctx, event, data, env);
        if let Some(child) = &mut self.child {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.button.update(ctx, data, env);
        if let Some(child) = &mut self.child {
            child.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // let mut flex = Flex::column().with_child(self.button);
        // flex.add_child(child)
        self.button.paint(ctx, data, env);
    }
}
