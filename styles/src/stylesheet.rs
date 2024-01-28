#![allow(non_camel_case_types)]
use bevy::prelude::*;

#[macro_export]
macro_rules! cn {
    ($($rest:expr),+) => {
        {
            move |mut x, i| {
                match i {
                    Interaction::None => {
                        cn!(expand style, x, $($rest),*);
                    }
                    Interaction::Hovered => {
                        cn!(expand style, x, $($rest),*);
                        cn!(expand style_hover, x, $($rest),*);
                    }
                    Interaction::Pressed => {
                        cn!(expand style, x, $($rest),*);
                        cn!(expand style_hover, x, $($rest),*);
                        cn!(expand style_pressed, x, $($rest),*);
                    }
                }

                x
            }
        }

    };
    (expand $interaction:ident, $inner:expr, $function:expr, $($rest:expr),*) => {
        $function.$interaction(&mut $inner);
        cn!(expand $interaction, $inner, $($rest),*)
    };
    (expand $interaction:ident, $inner:expr, $function:expr) => {
        $function.$interaction(&mut $inner);
    };
}

pub trait ApplyStyle<T> {
    fn style(&self, _bundle: &mut T) {}
    fn style_hover(&self, _bundle: &mut T) {}
    fn style_pressed(&self, _bundle: &mut T) {}
}

#[macro_export]
macro_rules! apply_style {
    ($t:ty, $name:ty, $function:expr) => {
        impl ApplyStyle<$t> for $name {
            fn style(&self, mut bundle: &mut $t) {
                fn f() -> impl Fn(&$name, &mut $t) {
                    $function
                }
                f()(self, &mut bundle);
            }
        }
    };
}

#[macro_export]
macro_rules! apply_bundle_style {
    ($component:ty, $name:ty, $function:expr, $property:ident, $($t:ty),*) => {
        $(
            impl ApplyStyle<$t> for $name {
                fn style(&self, bundle: &mut $t) {
                    fn f() -> impl Fn(&$name, &mut $component) {
                        $function
                    }
                    f()(self, &mut bundle.$property);
                }
            }
        )*

    };
}

#[macro_export]
macro_rules! apply_material_style {
    ($component:ty, $name:ty, $function:expr, $property:ident) => {
        impl<T: UiMaterial> ApplyStyle<MaterialNodeBundle<T>> for $name {
            fn style(&self, bundle: &mut MaterialNodeBundle<T>) {
                fn f() -> impl Fn(&$name, &mut $component) {
                    $function
                }
                f()(self, &mut bundle.$property);
            }
        }
    };
}

#[derive(Clone)]
pub struct hover_<T>(pub T);

impl<T: ApplyStyle<Y>, Y> ApplyStyle<Y> for hover_<T> {
    fn style_hover(&self, mut bundle: &mut Y) {
        self.0.style(&mut bundle);
    }
}

#[derive(Clone)]
pub struct pressed_<T>(pub T);

impl<T: ApplyStyle<Y>, Y> ApplyStyle<Y> for pressed_<T> {
    fn style_pressed(&self, mut bundle: &mut Y) {
        self.0.style(&mut bundle);
    }
}

macro_rules! node_style {
    ($name:ty, $function:expr) => {
        apply_style!(Style, $name, $function);
        apply_bundle_style!(
            Style,
            $name,
            $function,
            style,
            NodeBundle,
            ImageBundle,
            ButtonBundle,
            TextBundle
        );
        apply_material_style!(Style, $name, $function, style);
    };
}

macro_rules! node_background_color {
    ($name:ty, $function:expr) => {
        apply_style!(BackgroundColor, $name, $function);
        apply_bundle_style!(
            BackgroundColor,
            $name,
            $function,
            background_color,
            NodeBundle,
            ButtonBundle
        );
    };
}

macro_rules! visibility {
    ($name:ty, $function:expr) => {
        apply_style!(Visibility, $name, $function);
        apply_bundle_style!(
            Visibility,
            $name,
            $function,
            visibility,
            NodeBundle,
            ButtonBundle,
            TextBundle
        );
        apply_material_style!(Visibility, $name, $function, visibility);
    };
}

macro_rules! text_style {
    ($name:ty, $function:expr) => {
        apply_style!(TextStyle, $name, $function);
    };
}

// FONT FAMILY

#[derive(Clone)]
pub struct font_family(pub Handle<Font>);
text_style!(font_family, |font_family(font), bundle| {
    bundle.font = font.clone();
});

// DISPLAY

#[derive(Clone)]
pub struct flex;
node_style!(flex, |_, style| {
    style.display = Display::Flex;
});

// FLEX DIRECTION

#[derive(Clone)]
pub struct flex_col;
node_style!(flex_col, |_, style| {
    style.flex_direction = FlexDirection::Column;
});

#[derive(Clone)]
pub struct flex_row;
node_style!(flex_row, |_, style| {
    style.flex_direction = FlexDirection::Row;
});

// WIDTH

#[derive(Clone)]
pub struct w_full;
node_style!(w_full, |_, style| {
    style.width = Val::Percent(100.0);
});

#[derive(Clone)]
pub struct w_16;
node_style!(w_16, |_, style| {
    style.width = Val::Px(64.0);
});

#[derive(Clone)]
pub struct w_64;
node_style!(w_64, |_, style| {
    style.width = Val::Px(256.0);
});

// HEIGHT

#[derive(Clone)]
pub struct h_full;
node_style!(h_full, |_, style| {
    style.height = Val::Percent(100.0);
});

#[derive(Clone)]
pub struct h_16;
node_style!(h_16, |_, style| {
    style.height = Val::Px(64.0);
});

#[derive(Clone)]
pub struct h_64;
node_style!(h_64, |_, style| {
    style.height = Val::Px(256.0);
});

// ALIGN ITEMS

#[derive(Clone)]
pub struct items_center;
node_style!(items_center, |_, style| {
    style.align_items = AlignItems::Center;
});
#[derive(Clone)]
pub struct items_start;
node_style!(items_start, |_, style| {
    style.align_items = AlignItems::FlexStart;
});

#[derive(Clone)]
pub struct items_end;
node_style!(items_end, |_, style| {
    style.align_items = AlignItems::FlexEnd;
});

// JUSTIFY CONTENT

#[derive(Clone)]
pub struct justify_center;
node_style!(justify_center, |_, style| {
    style.justify_content = JustifyContent::Center;
});

#[derive(Clone)]
pub struct justify_between;
node_style!(justify_between, |_, style| {
    style.justify_content = JustifyContent::SpaceBetween;
});

#[derive(Clone)]
pub struct justify_around;
node_style!(justify_around, |_, style| {
    style.justify_content = JustifyContent::SpaceAround;
});

#[derive(Clone)]
pub struct justify_evenly;
node_style!(justify_evenly, |_, style| {
    style.justify_content = JustifyContent::SpaceEvenly;
});

#[derive(Clone)]
pub struct justify_start;
node_style!(justify_start, |_, style| {
    style.justify_content = JustifyContent::FlexStart;
});

#[derive(Clone)]
pub struct justify_end;
node_style!(justify_end, |_, style| {
    style.justify_content = JustifyContent::FlexEnd;
});

// TEXT COLORS

#[derive(Clone)]
pub struct text_white;
text_style!(text_white, |_, bundle| {
    bundle.color = Color::WHITE;
});

#[derive(Clone)]
pub struct text_black;
text_style!(text_black, |_, bundle| {
    bundle.color = Color::BLACK;
});

// BACKGROUND COLORS

#[derive(Clone)]
pub struct bg_white;
node_background_color!(bg_white, |_, background_color| {
    background_color.0 = Color::WHITE;
});

#[derive(Clone)]
pub struct bg_black;
node_background_color!(bg_black, |_, background_color| {
    background_color.0 = Color::BLACK;
});

#[derive(Clone)]
pub struct bg_red_50;
node_background_color!(bg_red_50, |_, background_color| {
    background_color.0 = Color::rgb_u8(254, 242, 242);
});

#[derive(Clone)]
pub struct bg_red_100;
node_background_color!(bg_red_100, |_, background_color| {
    background_color.0 = Color::rgb_u8(254, 226, 226);
});

#[derive(Clone)]
pub struct bg_red_200;
node_background_color!(bg_red_200, |_, background_color| {
    background_color.0 = Color::rgb_u8(254, 202, 202);
});

#[derive(Clone)]
pub struct bg_red_300;
node_background_color!(bg_red_300, |_, background_color| {
    background_color.0 = Color::rgb_u8(252, 165, 165);
});

#[derive(Clone)]
pub struct bg_red_400;
node_background_color!(bg_red_400, |_, background_color| {
    background_color.0 = Color::rgb_u8(248, 113, 113);
});

#[derive(Clone)]
pub struct bg_red_500;
node_background_color!(bg_red_500, |_, background_color| {
    background_color.0 = Color::rgb_u8(239, 68, 68);
});

#[derive(Clone)]
pub struct bg_red_600;
node_background_color!(bg_red_600, |_, background_color| {
    background_color.0 = Color::rgb_u8(220, 38, 38);
});

#[derive(Clone)]
pub struct bg_red_700;
node_background_color!(bg_red_700, |_, background_color| {
    background_color.0 = Color::rgb_u8(185, 28, 28);
});

#[derive(Clone)]
pub struct bg_red_800;
node_background_color!(bg_red_800, |_, background_color| {
    background_color.0 = Color::rgb_u8(153, 27, 27);
});

#[derive(Clone)]
pub struct bg_red_900;
node_background_color!(bg_red_900, |_, background_color| {
    background_color.0 = Color::rgb_u8(127, 29, 29);
});

#[derive(Clone)]
pub struct bg_red_950;
node_background_color!(bg_red_950, |_, background_color| {
    background_color.0 = Color::rgb_u8(69, 10, 10);
});

// TEXT SIZES

#[derive(Clone)]
pub struct text_xs;
text_style!(text_xs, |_, bundle| {
    bundle.font_size = 12.0;
});
#[derive(Clone)]
pub struct text_sm;
text_style!(text_sm, |_, bundle| {
    bundle.font_size = 14.0;
});

#[derive(Clone)]
pub struct text_base;
text_style!(text_base, |_, bundle| {
    bundle.font_size = 16.0;
});

#[derive(Clone)]
pub struct text_lg;
text_style!(text_lg, |_, bundle| {
    bundle.font_size = 18.0;
});

#[derive(Clone)]
pub struct text_xl;
text_style!(text_xl, |_, bundle| {
    bundle.font_size = 20.0;
});

#[derive(Clone)]
pub struct text_2xl;
text_style!(text_2xl, |_, bundle| {
    bundle.font_size = 24.0;
});

#[derive(Clone)]
pub struct text_3xl;
text_style!(text_3xl, |_, bundle| {
    bundle.font_size = 30.0;
});

#[derive(Clone)]
pub struct text_4xl;
text_style!(text_4xl, |_, bundle| {
    bundle.font_size = 36.0;
});

#[derive(Clone)]
pub struct text_5xl;
text_style!(text_5xl, |_, bundle| {
    bundle.font_size = 48.0;
});

#[derive(Clone)]
pub struct text_6xl;
text_style!(text_6xl, |_, bundle| {
    bundle.font_size = 60.0;
});

#[derive(Clone)]
pub struct text_7xl;
text_style!(text_7xl, |_, bundle| {
    bundle.font_size = 72.0;
});
#[derive(Clone)]
pub struct text_8xl;
text_style!(text_8xl, |_, bundle| {
    bundle.font_size = 96.0;
});

#[derive(Clone)]
pub struct text_9xl;
text_style!(text_9xl, |_, bundle| {
    bundle.font_size = 128.0;
});

// Visibility

#[derive(Clone)]
pub struct visible;
visibility!(visible, |_, visibility| {
    *visibility = Visibility::Inherited
});

#[derive(Clone)]
pub struct invisible;
visibility!(invisible, |_, visibility| {
    *visibility = Visibility::Hidden
});

#[derive(Clone)]
pub struct always_visible;
visibility!(always_visible, |_, visibility| {
    *visibility = Visibility::Visible
});
