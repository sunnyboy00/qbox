use druid::widget::{CrossAxisAlignment, Flex, LabelText, Radio, Widget};
use druid::Data;

pub enum Orientation {
    Horizontal,
    Vertical,
}
pub struct RadioGroup;

impl RadioGroup {
    pub fn new<T: Data + PartialEq>(
        variants: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
        orientation: Orientation,
    ) -> impl Widget<T> {
        let mut col = match orientation {
            Orientation::Horizontal => Flex::row().cross_axis_alignment(CrossAxisAlignment::Start),
            Orientation::Vertical => Flex::column().cross_axis_alignment(CrossAxisAlignment::Start),
        };
        let mut is_first = true;
        for (label, variant) in variants.into_iter() {
            if !is_first {
                col.add_default_spacer();
            }
            let radio = Radio::new(label, variant);
            col.add_child(radio);
            is_first = false;
        }
        col
    }
}
