use iced::{canvas::{Stroke, Fill, FillRule, LineCap, LineJoin}, Color};

pub static DARK_GREY: Color = Color{r: 0.125, g: 0.125, b: 0.125, a: 1.0};
pub static LIGHT_GREY: Color = Color {r:0.5, g:0.5, b:0.5, a:1.0};
pub static BAR_COLOR: Color = Color{r: 0.375, g: 0.375, b: 0.375, a: 1.0};
pub static HIGHLIGHT_BAR_COLOR: Color = Color {r:0.8, g:0.8, b:0.8, a:1.0};
pub static LABEL_COLOR: Color = Color {r:1.0, g:1.0, b:1.0, a:1.0};

pub static PERCENTILE_STROKE: Stroke = Stroke {
    width: 0.5,
    color: LIGHT_GREY,
    line_cap: LineCap::Butt,
    line_join: LineJoin::Miter
};

pub static BAR_FILL: Fill = Fill{color: BAR_COLOR, rule: FillRule::NonZero};
pub static H_BAR_FILL: Fill = Fill{color: HIGHLIGHT_BAR_COLOR, rule: FillRule::NonZero};

pub static BAR_STROKE: Stroke = Stroke {
    width: 0.5,
    color: DARK_GREY,
    line_cap: LineCap::Butt,
    line_join: LineJoin::Miter
};
pub static FRAME_BG_FILL: Fill = Fill{color:DARK_GREY, rule: FillRule::NonZero};
