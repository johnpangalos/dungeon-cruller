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
pub struct w_12;
node_style!(w_12, |_, style| {
    style.width = Val::Px(48.0);
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
pub struct h_12;
node_style!(h_12, |_, style| {
    style.height = Val::Px(48.0);
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

// PADDING

#[derive(Clone)]
pub struct p_0;
node_style!(p_0, |_, style| {
    style.padding = UiRect::all(Val::Px(0.0));
});

#[derive(Clone)]
pub struct p_1;
node_style!(p_1, |_, style| {
    style.padding = UiRect::all(Val::Px(4.0));
});

#[derive(Clone)]
pub struct p_1p5;
node_style!(p_1p5, |_, style| {
    style.padding = UiRect::all(Val::Px(6.0));
});

#[derive(Clone)]
pub struct p_2;
node_style!(p_2, |_, style| {
    style.padding = UiRect::all(Val::Px(8.0));
});

#[derive(Clone)]
pub struct p_2p5;
node_style!(p_2p5, |_, style| {
    style.padding = UiRect::all(Val::Px(10.0));
});

#[derive(Clone)]
pub struct p_3;
node_style!(p_3, |_, style| {
    style.padding = UiRect::all(Val::Px(12.0));
});

#[derive(Clone)]
pub struct p_3p5;
node_style!(p_3p5, |_, style| {
    style.padding = UiRect::all(Val::Px(14.0));
});

#[derive(Clone)]
pub struct p_4;
node_style!(p_4, |_, style| {
    style.padding = UiRect::all(Val::Px(16.0));
});

#[derive(Clone)]

pub struct p_5;
node_style!(p_5, |_, style| {
    style.padding = UiRect::all(Val::Px(20.0));
});

#[derive(Clone)]
pub struct p_6;
node_style!(p_6, |_, style| {
    style.padding = UiRect::all(Val::Px(24.0));
});

#[derive(Clone)]
pub struct p_7;
node_style!(p_7, |_, style| {
    style.padding = UiRect::all(Val::Px(28.0));
});

#[derive(Clone)]
pub struct p_8;
node_style!(p_8, |_, style| {
    style.padding = UiRect::all(Val::Px(32.0));
});

#[derive(Clone)]
pub struct p_9;
node_style!(p_9, |_, style| {
    style.padding = UiRect::all(Val::Px(36.0));
});

#[derive(Clone)]
pub struct p_10;
node_style!(p_10, |_, style| {
    style.padding = UiRect::all(Val::Px(40.0));
});

#[derive(Clone)]
pub struct p_11;
node_style!(p_11, |_, style| {
    style.padding = UiRect::all(Val::Px(44.0));
});

#[derive(Clone)]
pub struct p_12;
node_style!(p_12, |_, style| {
    style.padding = UiRect::all(Val::Px(48.0));
});

#[derive(Clone)]
pub struct p_14;
node_style!(p_14, |_, style| {
    style.padding = UiRect::all(Val::Px(56.0));
});

#[derive(Clone)]
pub struct p_16;
node_style!(p_16, |_, style| {
    style.padding = UiRect::all(Val::Px(64.0));
});

#[derive(Clone)]
pub struct p_20;
node_style!(p_20, |_, style| {
    style.padding = UiRect::all(Val::Px(80.0));
});

#[derive(Clone)]
pub struct p_24;
node_style!(p_24, |_, style| {
    style.padding = UiRect::all(Val::Px(96.0));
});

#[derive(Clone)]
pub struct p_28;
node_style!(p_28, |_, style| {
    style.padding = UiRect::all(Val::Px(112.0));
});

#[derive(Clone)]
pub struct p_32;
node_style!(p_32, |_, style| {
    style.padding = UiRect::all(Val::Px(128.0));
});

#[derive(Clone)]
pub struct p_36;
node_style!(p_36, |_, style| {
    style.padding = UiRect::all(Val::Px(144.0));
});

#[derive(Clone)]
pub struct p_40;
node_style!(p_40, |_, style| {
    style.padding = UiRect::all(Val::Px(160.0));
});

#[derive(Clone)]
pub struct p_44;
node_style!(p_44, |_, style| {
    style.padding = UiRect::all(Val::Px(176.0));
});

#[derive(Clone)]
pub struct p_48;
node_style!(p_48, |_, style| {
    style.padding = UiRect::all(Val::Px(192.0));
});

#[derive(Clone)]
pub struct p_52;
node_style!(p_52, |_, style| {
    style.padding = UiRect::all(Val::Px(208.0));
});

#[derive(Clone)]
pub struct p_56;
node_style!(p_56, |_, style| {
    style.padding = UiRect::all(Val::Px(224.0));
});

#[derive(Clone)]
pub struct p_60;
node_style!(p_60, |_, style| {
    style.padding = UiRect::all(Val::Px(240.0));
});

#[derive(Clone)]
pub struct p_64;
node_style!(p_64, |_, style| {
    style.padding = UiRect::all(Val::Px(256.0));
});

#[derive(Clone)]
pub struct p_72;
node_style!(p_72, |_, style| {
    style.padding = UiRect::all(Val::Px(288.0));
});

#[derive(Clone)]
pub struct p_80;
node_style!(p_80, |_, style| {
    style.padding = UiRect::all(Val::Px(320.0));
});

#[derive(Clone)]
pub struct p_96;
node_style!(p_96, |_, style| {
    style.padding = UiRect::all(Val::Px(384.0));
});

// PADDING LEFT

#[derive(Clone)]
pub struct pl_0;
node_style!(pl_0, |_, style| {
    style.padding.left = Val::Px(0.0);
});

#[derive(Clone)]
pub struct pl_1;
node_style!(pl_1, |_, style| {
    style.padding.left = Val::Px(4.0);
});

#[derive(Clone)]
pub struct pl_1p5;
node_style!(pl_1p5, |_, style| {
    style.padding.left = Val::Px(6.0);
});

#[derive(Clone)]
pub struct pl_2;
node_style!(pl_2, |_, style| {
    style.padding.left = Val::Px(8.0);
});

#[derive(Clone)]
pub struct pl_2p5;
node_style!(pl_2p5, |_, style| {
    style.padding.left = Val::Px(10.0);
});

#[derive(Clone)]
pub struct pl_3;
node_style!(pl_3, |_, style| {
    style.padding.left = Val::Px(12.0);
});

#[derive(Clone)]
pub struct pl_3p5;
node_style!(pl_3p5, |_, style| {
    style.padding.left = Val::Px(14.0);
});

#[derive(Clone)]
pub struct pl_4;
node_style!(pl_4, |_, style| {
    style.padding.left = Val::Px(16.0);
});

#[derive(Clone)]

pub struct pl_5;
node_style!(pl_5, |_, style| {
    style.padding.left = Val::Px(20.0);
});

#[derive(Clone)]
pub struct pl_6;
node_style!(pl_6, |_, style| {
    style.padding.left = Val::Px(24.0);
});

#[derive(Clone)]
pub struct pl_7;
node_style!(pl_7, |_, style| {
    style.padding.left = Val::Px(28.0);
});

#[derive(Clone)]
pub struct pl_8;
node_style!(pl_8, |_, style| {
    style.padding.left = Val::Px(32.0);
});

#[derive(Clone)]
pub struct pl_9;
node_style!(pl_9, |_, style| {
    style.padding.left = Val::Px(36.0);
});

#[derive(Clone)]
pub struct pl_10;
node_style!(pl_10, |_, style| {
    style.padding.left = Val::Px(40.0);
});

#[derive(Clone)]
pub struct pl_11;
node_style!(pl_11, |_, style| {
    style.padding.left = Val::Px(44.0);
});

#[derive(Clone)]
pub struct pl_12;
node_style!(pl_12, |_, style| {
    style.padding.left = Val::Px(48.0);
});

#[derive(Clone)]
pub struct pl_14;
node_style!(pl_14, |_, style| {
    style.padding.left = Val::Px(56.0);
});

#[derive(Clone)]
pub struct pl_16;
node_style!(pl_16, |_, style| {
    style.padding.left = Val::Px(64.0);
});

#[derive(Clone)]
pub struct pl_20;
node_style!(pl_20, |_, style| {
    style.padding.left = Val::Px(80.0);
});

#[derive(Clone)]
pub struct pl_24;
node_style!(pl_24, |_, style| {
    style.padding.left = Val::Px(96.0);
});

#[derive(Clone)]
pub struct pl_28;
node_style!(pl_28, |_, style| {
    style.padding.left = Val::Px(112.0);
});

#[derive(Clone)]
pub struct pl_32;
node_style!(pl_32, |_, style| {
    style.padding.left = Val::Px(128.0);
});

#[derive(Clone)]
pub struct pl_36;
node_style!(pl_36, |_, style| {
    style.padding.left = Val::Px(144.0);
});

#[derive(Clone)]
pub struct pl_40;
node_style!(pl_40, |_, style| {
    style.padding.left = Val::Px(160.0);
});

#[derive(Clone)]
pub struct pl_44;
node_style!(pl_44, |_, style| {
    style.padding.left = Val::Px(176.0);
});

#[derive(Clone)]
pub struct pl_48;
node_style!(pl_48, |_, style| {
    style.padding.left = Val::Px(192.0);
});

#[derive(Clone)]
pub struct pl_52;
node_style!(pl_52, |_, style| {
    style.padding.left = Val::Px(208.0);
});

#[derive(Clone)]
pub struct pl_56;
node_style!(pl_56, |_, style| {
    style.padding.left = Val::Px(224.0);
});

#[derive(Clone)]
pub struct pl_60;
node_style!(pl_60, |_, style| {
    style.padding.left = Val::Px(240.0);
});

#[derive(Clone)]
pub struct pl_64;
node_style!(pl_64, |_, style| {
    style.padding.left = Val::Px(256.0);
});

#[derive(Clone)]
pub struct pl_72;
node_style!(pl_72, |_, style| {
    style.padding.left = Val::Px(288.0);
});

#[derive(Clone)]
pub struct pl_80;
node_style!(pl_80, |_, style| {
    style.padding.left = Val::Px(320.0);
});

#[derive(Clone)]
pub struct pl_96;
node_style!(pl_96, |_, style| {
    style.padding.left = Val::Px(384.0);
});

// PADDING RIGHT

#[derive(Clone)]
pub struct pr_0;
node_style!(pr_0, |_, style| {
    style.padding.right = Val::Px(0.0);
});

#[derive(Clone)]
pub struct pr_1;
node_style!(pr_1, |_, style| {
    style.padding.right = Val::Px(4.0);
});

#[derive(Clone)]
pub struct pr_1p5;
node_style!(pr_1p5, |_, style| {
    style.padding.right = Val::Px(6.0);
});

#[derive(Clone)]
pub struct pr_2;
node_style!(pr_2, |_, style| {
    style.padding.right = Val::Px(8.0);
});

#[derive(Clone)]
pub struct pr_2p5;
node_style!(pr_2p5, |_, style| {
    style.padding.right = Val::Px(10.0);
});

#[derive(Clone)]
pub struct pr_3;
node_style!(pr_3, |_, style| {
    style.padding.right = Val::Px(12.0);
});

#[derive(Clone)]
pub struct pr_3p5;
node_style!(pr_3p5, |_, style| {
    style.padding.right = Val::Px(14.0);
});

#[derive(Clone)]
pub struct pr_4;
node_style!(pr_4, |_, style| {
    style.padding.right = Val::Px(16.0);
});

#[derive(Clone)]

pub struct pr_5;
node_style!(pr_5, |_, style| {
    style.padding.right = Val::Px(20.0);
});

#[derive(Clone)]
pub struct pr_6;
node_style!(pr_6, |_, style| {
    style.padding.right = Val::Px(24.0);
});

#[derive(Clone)]
pub struct pr_7;
node_style!(pr_7, |_, style| {
    style.padding.right = Val::Px(28.0);
});

#[derive(Clone)]
pub struct pr_8;
node_style!(pr_8, |_, style| {
    style.padding.right = Val::Px(32.0);
});

#[derive(Clone)]
pub struct pr_9;
node_style!(pr_9, |_, style| {
    style.padding.right = Val::Px(36.0);
});

#[derive(Clone)]
pub struct pr_10;
node_style!(pr_10, |_, style| {
    style.padding.right = Val::Px(40.0);
});

#[derive(Clone)]
pub struct pr_11;
node_style!(pr_11, |_, style| {
    style.padding.right = Val::Px(44.0);
});

#[derive(Clone)]
pub struct pr_12;
node_style!(pr_12, |_, style| {
    style.padding.right = Val::Px(48.0);
});

#[derive(Clone)]
pub struct pr_14;
node_style!(pr_14, |_, style| {
    style.padding.right = Val::Px(56.0);
});

#[derive(Clone)]
pub struct pr_16;
node_style!(pr_16, |_, style| {
    style.padding.right = Val::Px(64.0);
});

#[derive(Clone)]
pub struct pr_20;
node_style!(pr_20, |_, style| {
    style.padding.right = Val::Px(80.0);
});

#[derive(Clone)]
pub struct pr_24;
node_style!(pr_24, |_, style| {
    style.padding.right = Val::Px(96.0);
});

#[derive(Clone)]
pub struct pr_28;
node_style!(pr_28, |_, style| {
    style.padding.right = Val::Px(112.0);
});

#[derive(Clone)]
pub struct pr_32;
node_style!(pr_32, |_, style| {
    style.padding.right = Val::Px(128.0);
});

#[derive(Clone)]
pub struct pr_36;
node_style!(pr_36, |_, style| {
    style.padding.right = Val::Px(144.0);
});

#[derive(Clone)]
pub struct pr_40;
node_style!(pr_40, |_, style| {
    style.padding.right = Val::Px(160.0);
});

#[derive(Clone)]
pub struct pr_44;
node_style!(pr_44, |_, style| {
    style.padding.right = Val::Px(176.0);
});

#[derive(Clone)]
pub struct pr_48;
node_style!(pr_48, |_, style| {
    style.padding.right = Val::Px(192.0);
});

#[derive(Clone)]
pub struct pr_52;
node_style!(pr_52, |_, style| {
    style.padding.right = Val::Px(208.0);
});

#[derive(Clone)]
pub struct pr_56;
node_style!(pr_56, |_, style| {
    style.padding.right = Val::Px(224.0);
});

#[derive(Clone)]
pub struct pr_60;
node_style!(pr_60, |_, style| {
    style.padding.right = Val::Px(240.0);
});

#[derive(Clone)]
pub struct pr_64;
node_style!(pr_64, |_, style| {
    style.padding.right = Val::Px(256.0);
});

#[derive(Clone)]
pub struct pr_72;
node_style!(pr_72, |_, style| {
    style.padding.right = Val::Px(288.0);
});

#[derive(Clone)]
pub struct pr_80;
node_style!(pr_80, |_, style| {
    style.padding.right = Val::Px(320.0);
});

#[derive(Clone)]
pub struct pr_96;
node_style!(pr_96, |_, style| {
    style.padding.right = Val::Px(384.0);
});

// PADDING TOP

#[derive(Clone)]
pub struct pt_0;
node_style!(pt_0, |_, style| {
    style.padding.top = Val::Px(0.0);
});

#[derive(Clone)]
pub struct pt_1;
node_style!(pt_1, |_, style| {
    style.padding.top = Val::Px(4.0);
});

#[derive(Clone)]
pub struct pt_1p5;
node_style!(pt_1p5, |_, style| {
    style.padding.top = Val::Px(6.0);
});

#[derive(Clone)]
pub struct pt_2;
node_style!(pt_2, |_, style| {
    style.padding.top = Val::Px(8.0);
});

#[derive(Clone)]
pub struct pt_2p5;
node_style!(pt_2p5, |_, style| {
    style.padding.top = Val::Px(10.0);
});

#[derive(Clone)]
pub struct pt_3;
node_style!(pt_3, |_, style| {
    style.padding.top = Val::Px(12.0);
});

#[derive(Clone)]
pub struct pt_3p5;
node_style!(pt_3p5, |_, style| {
    style.padding.top = Val::Px(14.0);
});

#[derive(Clone)]
pub struct pt_4;
node_style!(pt_4, |_, style| {
    style.padding.top = Val::Px(16.0);
});

#[derive(Clone)]

pub struct pt_5;
node_style!(pt_5, |_, style| {
    style.padding.top = Val::Px(20.0);
});

#[derive(Clone)]
pub struct pt_6;
node_style!(pt_6, |_, style| {
    style.padding.top = Val::Px(24.0);
});

#[derive(Clone)]
pub struct pt_7;
node_style!(pt_7, |_, style| {
    style.padding.top = Val::Px(28.0);
});

#[derive(Clone)]
pub struct pt_8;
node_style!(pt_8, |_, style| {
    style.padding.top = Val::Px(32.0);
});

#[derive(Clone)]
pub struct pt_9;
node_style!(pt_9, |_, style| {
    style.padding.top = Val::Px(36.0);
});

#[derive(Clone)]
pub struct pt_10;
node_style!(pt_10, |_, style| {
    style.padding.top = Val::Px(40.0);
});

#[derive(Clone)]
pub struct pt_11;
node_style!(pt_11, |_, style| {
    style.padding.top = Val::Px(44.0);
});

#[derive(Clone)]
pub struct pt_12;
node_style!(pt_12, |_, style| {
    style.padding.top = Val::Px(48.0);
});

#[derive(Clone)]
pub struct pt_14;
node_style!(pt_14, |_, style| {
    style.padding.top = Val::Px(56.0);
});

#[derive(Clone)]
pub struct pt_16;
node_style!(pt_16, |_, style| {
    style.padding.top = Val::Px(64.0);
});

#[derive(Clone)]
pub struct pt_20;
node_style!(pt_20, |_, style| {
    style.padding.top = Val::Px(80.0);
});

#[derive(Clone)]
pub struct pt_24;
node_style!(pt_24, |_, style| {
    style.padding.top = Val::Px(96.0);
});

#[derive(Clone)]
pub struct pt_28;
node_style!(pt_28, |_, style| {
    style.padding.top = Val::Px(112.0);
});

#[derive(Clone)]
pub struct pt_32;
node_style!(pt_32, |_, style| {
    style.padding.top = Val::Px(128.0);
});

#[derive(Clone)]
pub struct pt_36;
node_style!(pt_36, |_, style| {
    style.padding.top = Val::Px(144.0);
});

#[derive(Clone)]
pub struct pt_40;
node_style!(pt_40, |_, style| {
    style.padding.top = Val::Px(160.0);
});

#[derive(Clone)]
pub struct pt_44;
node_style!(pt_44, |_, style| {
    style.padding.top = Val::Px(176.0);
});

#[derive(Clone)]
pub struct pt_48;
node_style!(pt_48, |_, style| {
    style.padding.top = Val::Px(192.0);
});

#[derive(Clone)]
pub struct pt_52;
node_style!(pt_52, |_, style| {
    style.padding.top = Val::Px(208.0);
});

#[derive(Clone)]
pub struct pt_56;
node_style!(pt_56, |_, style| {
    style.padding.top = Val::Px(224.0);
});

#[derive(Clone)]
pub struct pt_60;
node_style!(pt_60, |_, style| {
    style.padding.top = Val::Px(240.0);
});

#[derive(Clone)]
pub struct pt_64;
node_style!(pt_64, |_, style| {
    style.padding.top = Val::Px(256.0);
});

#[derive(Clone)]
pub struct pt_72;
node_style!(pt_72, |_, style| {
    style.padding.top = Val::Px(288.0);
});

#[derive(Clone)]
pub struct pt_80;
node_style!(pt_80, |_, style| {
    style.padding.top = Val::Px(320.0);
});

#[derive(Clone)]
pub struct pt_96;
node_style!(pt_96, |_, style| {
    style.padding.top = Val::Px(384.0);
});

// PADDING BOTTOM

#[derive(Clone)]
pub struct pb_0;
node_style!(pb_0, |_, style| {
    style.padding.bottom = Val::Px(0.0);
});

#[derive(Clone)]
pub struct pb_1;
node_style!(pb_1, |_, style| {
    style.padding.bottom = Val::Px(4.0);
});

#[derive(Clone)]
pub struct pb_1p5;
node_style!(pb_1p5, |_, style| {
    style.padding.bottom = Val::Px(6.0);
});

#[derive(Clone)]
pub struct pb_2;
node_style!(pb_2, |_, style| {
    style.padding.bottom = Val::Px(8.0);
});

#[derive(Clone)]
pub struct pb_2p5;
node_style!(pb_2p5, |_, style| {
    style.padding.bottom = Val::Px(10.0);
});

#[derive(Clone)]
pub struct pb_3;
node_style!(pb_3, |_, style| {
    style.padding.bottom = Val::Px(12.0);
});

#[derive(Clone)]
pub struct pb_3p5;
node_style!(pb_3p5, |_, style| {
    style.padding.bottom = Val::Px(14.0);
});

#[derive(Clone)]
pub struct pb_4;
node_style!(pb_4, |_, style| {
    style.padding.bottom = Val::Px(16.0);
});

#[derive(Clone)]

pub struct pb_5;
node_style!(pb_5, |_, style| {
    style.padding.bottom = Val::Px(20.0);
});

#[derive(Clone)]
pub struct pb_6;
node_style!(pb_6, |_, style| {
    style.padding.bottom = Val::Px(24.0);
});

#[derive(Clone)]
pub struct pb_7;
node_style!(pb_7, |_, style| {
    style.padding.bottom = Val::Px(28.0);
});

#[derive(Clone)]
pub struct pb_8;
node_style!(pb_8, |_, style| {
    style.padding.bottom = Val::Px(32.0);
});

#[derive(Clone)]
pub struct pb_9;
node_style!(pb_9, |_, style| {
    style.padding.bottom = Val::Px(36.0);
});

#[derive(Clone)]
pub struct pb_10;
node_style!(pb_10, |_, style| {
    style.padding.bottom = Val::Px(40.0);
});

#[derive(Clone)]
pub struct pb_11;
node_style!(pb_11, |_, style| {
    style.padding.bottom = Val::Px(44.0);
});

#[derive(Clone)]
pub struct pb_12;
node_style!(pb_12, |_, style| {
    style.padding.bottom = Val::Px(48.0);
});

#[derive(Clone)]
pub struct pb_14;
node_style!(pb_14, |_, style| {
    style.padding.bottom = Val::Px(56.0);
});

#[derive(Clone)]
pub struct pb_16;
node_style!(pb_16, |_, style| {
    style.padding.bottom = Val::Px(64.0);
});

#[derive(Clone)]
pub struct pb_20;
node_style!(pb_20, |_, style| {
    style.padding.bottom = Val::Px(80.0);
});

#[derive(Clone)]
pub struct pb_24;
node_style!(pb_24, |_, style| {
    style.padding.bottom = Val::Px(96.0);
});

#[derive(Clone)]
pub struct pb_28;
node_style!(pb_28, |_, style| {
    style.padding.bottom = Val::Px(112.0);
});

#[derive(Clone)]
pub struct pb_32;
node_style!(pb_32, |_, style| {
    style.padding.bottom = Val::Px(128.0);
});

#[derive(Clone)]
pub struct pb_36;
node_style!(pb_36, |_, style| {
    style.padding.bottom = Val::Px(144.0);
});

#[derive(Clone)]
pub struct pb_40;
node_style!(pb_40, |_, style| {
    style.padding.bottom = Val::Px(160.0);
});

#[derive(Clone)]
pub struct pb_44;
node_style!(pb_44, |_, style| {
    style.padding.bottom = Val::Px(176.0);
});

#[derive(Clone)]
pub struct pb_48;
node_style!(pb_48, |_, style| {
    style.padding.bottom = Val::Px(192.0);
});

#[derive(Clone)]
pub struct pb_52;
node_style!(pb_52, |_, style| {
    style.padding.bottom = Val::Px(208.0);
});

#[derive(Clone)]
pub struct pb_56;
node_style!(pb_56, |_, style| {
    style.padding.bottom = Val::Px(224.0);
});

#[derive(Clone)]
pub struct pb_60;
node_style!(pb_60, |_, style| {
    style.padding.bottom = Val::Px(240.0);
});

#[derive(Clone)]
pub struct pb_64;
node_style!(pb_64, |_, style| {
    style.padding.bottom = Val::Px(256.0);
});

#[derive(Clone)]
pub struct pb_72;
node_style!(pb_72, |_, style| {
    style.padding.bottom = Val::Px(288.0);
});

#[derive(Clone)]
pub struct pb_80;
node_style!(pb_80, |_, style| {
    style.padding.bottom = Val::Px(320.0);
});

#[derive(Clone)]
pub struct pb_96;
node_style!(pb_96, |_, style| {
    style.padding.bottom = Val::Px(384.0);
});

// PADDING VERTICAL

#[derive(Clone)]
pub struct py_0;
node_style!(py_0, |_, style| {
    style.padding.top = Val::Px(0.0);
    style.padding.bottom = Val::Px(0.0);
});

#[derive(Clone)]
pub struct py_1;
node_style!(py_1, |_, style| {
    style.padding.top = Val::Px(4.0);
    style.padding.bottom = Val::Px(4.0);
});

#[derive(Clone)]
pub struct py_1p5;
node_style!(py_1p5, |_, style| {
    style.padding.top = Val::Px(6.0);
    style.padding.bottom = Val::Px(6.0);
});

#[derive(Clone)]
pub struct py_2;
node_style!(py_2, |_, style| {
    style.padding.top = Val::Px(8.0);
    style.padding.bottom = Val::Px(8.0);
});

#[derive(Clone)]
pub struct py_2p5;
node_style!(py_2p5, |_, style| {
    style.padding.top = Val::Px(10.0);
    style.padding.bottom = Val::Px(10.0);
});

#[derive(Clone)]
pub struct py_3;
node_style!(py_3, |_, style| {
    style.padding.top = Val::Px(12.0);
    style.padding.bottom = Val::Px(12.0);
});

#[derive(Clone)]
pub struct py_3p5;
node_style!(py_3p5, |_, style| {
    style.padding.top = Val::Px(14.0);
    style.padding.bottom = Val::Px(14.0);
});

#[derive(Clone)]
pub struct py_4;
node_style!(py_4, |_, style| {
    style.padding.top = Val::Px(16.0);
    style.padding.bottom = Val::Px(16.0);
});

#[derive(Clone)]

pub struct py_5;
node_style!(py_5, |_, style| {
    style.padding.top = Val::Px(20.0);
    style.padding.bottom = Val::Px(20.0);
});

#[derive(Clone)]
pub struct py_6;
node_style!(py_6, |_, style| {
    style.padding.top = Val::Px(24.0);
    style.padding.bottom = Val::Px(24.0);
});

#[derive(Clone)]
pub struct py_7;
node_style!(py_7, |_, style| {
    style.padding.top = Val::Px(28.0);
    style.padding.bottom = Val::Px(28.0);
});

#[derive(Clone)]
pub struct py_8;
node_style!(py_8, |_, style| {
    style.padding.top = Val::Px(32.0);
    style.padding.bottom = Val::Px(32.0);
});

#[derive(Clone)]
pub struct py_9;
node_style!(py_9, |_, style| {
    style.padding.top = Val::Px(36.0);
    style.padding.bottom = Val::Px(36.0);
});

#[derive(Clone)]
pub struct py_10;
node_style!(py_10, |_, style| {
    style.padding.top = Val::Px(40.0);
    style.padding.bottom = Val::Px(40.0);
});

#[derive(Clone)]
pub struct py_11;
node_style!(py_11, |_, style| {
    style.padding.top = Val::Px(44.0);
    style.padding.bottom = Val::Px(44.0);
});

#[derive(Clone)]
pub struct py_12;
node_style!(py_12, |_, style| {
    style.padding.top = Val::Px(48.0);
    style.padding.bottom = Val::Px(48.0);
});

#[derive(Clone)]
pub struct py_14;
node_style!(py_14, |_, style| {
    style.padding.top = Val::Px(56.0);
    style.padding.bottom = Val::Px(56.0);
});

#[derive(Clone)]
pub struct py_16;
node_style!(py_16, |_, style| {
    style.padding.top = Val::Px(64.0);
    style.padding.bottom = Val::Px(64.0);
});

#[derive(Clone)]
pub struct py_20;
node_style!(py_20, |_, style| {
    style.padding.top = Val::Px(80.0);
    style.padding.bottom = Val::Px(80.0);
});

#[derive(Clone)]
pub struct py_24;
node_style!(py_24, |_, style| {
    style.padding.top = Val::Px(96.0);
    style.padding.bottom = Val::Px(96.0);
});

#[derive(Clone)]
pub struct py_28;
node_style!(py_28, |_, style| {
    style.padding.top = Val::Px(112.0);
    style.padding.bottom = Val::Px(112.0);
});

#[derive(Clone)]
pub struct py_32;
node_style!(py_32, |_, style| {
    style.padding.top = Val::Px(128.0);
    style.padding.bottom = Val::Px(128.0);
});

#[derive(Clone)]
pub struct py_36;
node_style!(py_36, |_, style| {
    style.padding.top = Val::Px(144.0);
    style.padding.bottom = Val::Px(144.0);
});

#[derive(Clone)]
pub struct py_40;
node_style!(py_40, |_, style| {
    style.padding.top = Val::Px(160.0);
    style.padding.bottom = Val::Px(160.0);
});

#[derive(Clone)]
pub struct py_44;
node_style!(py_44, |_, style| {
    style.padding.top = Val::Px(176.0);
    style.padding.bottom = Val::Px(176.0);
});

#[derive(Clone)]
pub struct py_48;
node_style!(py_48, |_, style| {
    style.padding.top = Val::Px(192.0);
    style.padding.bottom = Val::Px(192.0);
});

#[derive(Clone)]
pub struct py_52;
node_style!(py_52, |_, style| {
    style.padding.top = Val::Px(208.0);
    style.padding.bottom = Val::Px(208.0);
});

#[derive(Clone)]
pub struct py_56;
node_style!(py_56, |_, style| {
    style.padding.top = Val::Px(224.0);
    style.padding.bottom = Val::Px(224.0);
});

#[derive(Clone)]
pub struct py_60;
node_style!(py_60, |_, style| {
    style.padding.top = Val::Px(240.0);
    style.padding.bottom = Val::Px(240.0);
});

#[derive(Clone)]
pub struct py_64;
node_style!(py_64, |_, style| {
    style.padding.top = Val::Px(256.0);
    style.padding.bottom = Val::Px(256.0);
});

#[derive(Clone)]
pub struct py_72;
node_style!(py_72, |_, style| {
    style.padding.top = Val::Px(288.0);
    style.padding.bottom = Val::Px(288.0);
});

#[derive(Clone)]
pub struct py_80;
node_style!(py_80, |_, style| {
    style.padding.top = Val::Px(320.0);
    style.padding.bottom = Val::Px(320.0);
});

#[derive(Clone)]
pub struct py_96;
node_style!(py_96, |_, style| {
    style.padding.top = Val::Px(384.0);
    style.padding.bottom = Val::Px(384.0);
});

// PADDING HORIZONTAL

#[derive(Clone)]
pub struct px_0;
node_style!(px_0, |_, style| {
    style.padding.left = Val::Px(0.0);
    style.padding.right = Val::Px(0.0);
});

#[derive(Clone)]
pub struct px_1;
node_style!(px_1, |_, style| {
    style.padding.left = Val::Px(4.0);
    style.padding.right = Val::Px(4.0);
});

#[derive(Clone)]
pub struct px_1p5;
node_style!(px_1p5, |_, style| {
    style.padding.left = Val::Px(6.0);
    style.padding.right = Val::Px(6.0);
});

#[derive(Clone)]
pub struct px_2;
node_style!(px_2, |_, style| {
    style.padding.left = Val::Px(8.0);
    style.padding.right = Val::Px(8.0);
});

#[derive(Clone)]
pub struct px_2p5;
node_style!(px_2p5, |_, style| {
    style.padding.left = Val::Px(10.0);
    style.padding.right = Val::Px(10.0);
});

#[derive(Clone)]
pub struct px_3;
node_style!(px_3, |_, style| {
    style.padding.left = Val::Px(12.0);
    style.padding.right = Val::Px(12.0);
});

#[derive(Clone)]
pub struct px_3p5;
node_style!(px_3p5, |_, style| {
    style.padding.left = Val::Px(14.0);
    style.padding.right = Val::Px(14.0);
});

#[derive(Clone)]
pub struct px_4;
node_style!(px_4, |_, style| {
    style.padding.left = Val::Px(16.0);
    style.padding.right = Val::Px(16.0);
});

#[derive(Clone)]

pub struct px_5;
node_style!(px_5, |_, style| {
    style.padding.left = Val::Px(20.0);
    style.padding.right = Val::Px(20.0);
});

#[derive(Clone)]
pub struct px_6;
node_style!(px_6, |_, style| {
    style.padding.left = Val::Px(24.0);
    style.padding.right = Val::Px(24.0);
});

#[derive(Clone)]
pub struct px_7;
node_style!(px_7, |_, style| {
    style.padding.left = Val::Px(28.0);
    style.padding.right = Val::Px(28.0);
});

#[derive(Clone)]
pub struct px_8;
node_style!(px_8, |_, style| {
    style.padding.left = Val::Px(32.0);
    style.padding.right = Val::Px(32.0);
});

#[derive(Clone)]
pub struct px_9;
node_style!(px_9, |_, style| {
    style.padding.left = Val::Px(36.0);
    style.padding.right = Val::Px(36.0);
});

#[derive(Clone)]
pub struct px_10;
node_style!(px_10, |_, style| {
    style.padding.left = Val::Px(40.0);
    style.padding.right = Val::Px(40.0);
});

#[derive(Clone)]
pub struct px_11;
node_style!(px_11, |_, style| {
    style.padding.left = Val::Px(44.0);
    style.padding.right = Val::Px(44.0);
});

#[derive(Clone)]
pub struct px_12;
node_style!(px_12, |_, style| {
    style.padding.left = Val::Px(48.0);
    style.padding.right = Val::Px(48.0);
});

#[derive(Clone)]
pub struct px_14;
node_style!(px_14, |_, style| {
    style.padding.left = Val::Px(56.0);
    style.padding.right = Val::Px(56.0);
});

#[derive(Clone)]
pub struct px_16;
node_style!(px_16, |_, style| {
    style.padding.left = Val::Px(64.0);
    style.padding.right = Val::Px(64.0);
});

#[derive(Clone)]
pub struct px_20;
node_style!(px_20, |_, style| {
    style.padding.left = Val::Px(80.0);
    style.padding.right = Val::Px(80.0);
});

#[derive(Clone)]
pub struct px_24;
node_style!(px_24, |_, style| {
    style.padding.left = Val::Px(96.0);
    style.padding.right = Val::Px(96.0);
});

#[derive(Clone)]
pub struct px_28;
node_style!(px_28, |_, style| {
    style.padding.left = Val::Px(112.0);
    style.padding.right = Val::Px(112.0);
});

#[derive(Clone)]
pub struct px_32;
node_style!(px_32, |_, style| {
    style.padding.left = Val::Px(128.0);
    style.padding.right = Val::Px(128.0);
});

#[derive(Clone)]
pub struct px_36;
node_style!(px_36, |_, style| {
    style.padding.left = Val::Px(144.0);
    style.padding.right = Val::Px(144.0);
});

#[derive(Clone)]
pub struct px_40;
node_style!(px_40, |_, style| {
    style.padding.left = Val::Px(160.0);
    style.padding.right = Val::Px(160.0);
});

#[derive(Clone)]
pub struct px_44;
node_style!(px_44, |_, style| {
    style.padding.left = Val::Px(176.0);
    style.padding.right = Val::Px(176.0);
});

#[derive(Clone)]
pub struct px_48;
node_style!(px_48, |_, style| {
    style.padding.left = Val::Px(192.0);
    style.padding.right = Val::Px(192.0);
});

#[derive(Clone)]
pub struct px_52;
node_style!(px_52, |_, style| {
    style.padding.left = Val::Px(208.0);
    style.padding.right = Val::Px(208.0);
});

#[derive(Clone)]
pub struct px_56;
node_style!(px_56, |_, style| {
    style.padding.left = Val::Px(224.0);
    style.padding.right = Val::Px(224.0);
});

#[derive(Clone)]
pub struct px_60;
node_style!(px_60, |_, style| {
    style.padding.left = Val::Px(240.0);
    style.padding.right = Val::Px(240.0);
});

#[derive(Clone)]
pub struct px_64;
node_style!(px_64, |_, style| {
    style.padding.left = Val::Px(256.0);
    style.padding.right = Val::Px(256.0);
});

#[derive(Clone)]
pub struct px_72;
node_style!(px_72, |_, style| {
    style.padding.left = Val::Px(288.0);
    style.padding.right = Val::Px(288.0);
});

#[derive(Clone)]
pub struct px_80;
node_style!(px_80, |_, style| {
    style.padding.left = Val::Px(320.0);
    style.padding.right = Val::Px(320.0);
});

#[derive(Clone)]
pub struct px_96;
node_style!(px_96, |_, style| {
    style.padding.left = Val::Px(384.0);
    style.padding.right = Val::Px(384.0);
});
