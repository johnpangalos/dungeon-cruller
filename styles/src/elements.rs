use bevy::{ecs::system::Insert, prelude::*};

pub struct StylesPlugin;

impl Plugin for StylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interaction_style_system,
                interaction_text_style_system,
                interaction_background_color_system,
            ),
        );
    }
}

#[derive(Component)]
struct InteractionBackgroundColor {
    none: BackgroundColor,
    hover: BackgroundColor,
    pressed: BackgroundColor,
}

#[derive(Component)]
struct InteractionStyle {
    none: Style,
    hover: Style,
    pressed: Style,
}

#[derive(Component)]
struct InteractionTextStyle {
    none: TextStyle,
    hover: TextStyle,
    pressed: TextStyle,
}

fn interaction_style_system(
    mut query: Query<(&Interaction, &InteractionStyle, &mut Style), Changed<Interaction>>,
) {
    for (interaction, state, mut style) in query.iter_mut() {
        match interaction {
            Interaction::None => *style = state.none.clone(),
            Interaction::Hovered => *style = state.hover.clone(),
            Interaction::Pressed => *style = state.pressed.clone(),
        }
    }
}

fn interaction_background_color_system(
    mut query: Query<(
        &Interaction,
        &InteractionBackgroundColor,
        &mut BackgroundColor,
    )>,
) {
    for (interaction, state, mut style) in query.iter_mut() {
        match interaction {
            Interaction::None => *style = state.none.clone(),
            Interaction::Hovered => *style = state.hover.clone(),
            Interaction::Pressed => *style = state.pressed.clone(),
        }
    }
}

fn interaction_text_style_system(
    mut query: Query<(&Interaction, &InteractionTextStyle, &mut Text), Changed<Interaction>>,
) {
    for (interaction, state, mut style) in query.iter_mut() {
        for section in style.sections.iter_mut() {
            match interaction {
                Interaction::None => section.style = state.none.clone(),
                Interaction::Hovered => section.style = state.hover.clone(),
                Interaction::Pressed => section.style = state.pressed.clone(),
            }
        }
    }
}

pub struct Element(pub Box<dyn FnOnce(&mut ChildBuilder) -> Entity>);

impl IntoIterator for Element {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl Element {
    fn new(node: impl FnOnce(&mut ChildBuilder) -> Entity + 'static) -> Self {
        Self(Box::new(node))
    }

    pub fn with<T: Bundle>(self, bundle: T) -> Self {
        Element::new(move |parent| {
            let entity = self.0(parent);
            parent.add_command(Insert { entity, bundle });
            entity
        })
    }
}

pub fn fragment(children: impl IntoIterator<Item = Element>) -> Element {
    let vec = children.into_iter().collect::<Vec<_>>();
    Element::new(move |parent| {
        for child in vec.into_iter() {
            child.0(parent);
        }

        parent.parent_entity()
    })
}

#[macro_export]
macro_rules! create_interaction_bundle {
    ($class:expr, $bundle:ident) => {{
        let bundle = $class($bundle::default(), Interaction::None);
        let bundle_none = $class($bundle::default(), Interaction::None);
        let bundle_hover = $class($bundle::default(), Interaction::Hovered);
        let bundle_pressed = $class($bundle::default(), Interaction::Pressed);

        let interaction_style = InteractionStyle {
            none: bundle_none.style,
            hover: bundle_hover.style,
            pressed: bundle_pressed.style,
        };

        let interaction_background_color = InteractionBackgroundColor {
            none: bundle_none.background_color,
            hover: bundle_hover.background_color,
            pressed: bundle_pressed.background_color,
        };

        (bundle, interaction_style, interaction_background_color)
    }};
}

#[derive(Component)]
struct ElementButton;

pub fn button(
    class: impl Fn(ButtonBundle, Interaction) -> ButtonBundle + 'static,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    let vec = children.into_iter().collect::<Vec<_>>();

    Element::new(move |parent| {
        let bundle = create_interaction_bundle!(class, ButtonBundle);

        parent
            .spawn((ElementButton, bundle))
            .with_children(|current| {
                for child in vec.into_iter() {
                    child.0(current);
                }
            })
            .id()
    })
}

#[macro_export]
macro_rules! on_update_system {
    ($on_update:expr, $component:ty, ) => {
        fn on_update(mut commands: Commands, mut query: Query<&mut $component>) {
            for mut v in query.iter_mut() {
                $on_update(&mut commands, &mut v);
            }
        }
    };
    ($on_update:expr, $component:ty, $a:ty) => {
        fn on_update(mut commands: Commands, mut query: Query<&mut $component>, mut a: $a) {
            for mut v in query.iter_mut() {
                $on_update(&mut commands, &mut v, &mut a);
            }
        }
    };
    ($on_update:expr, $component:ty, $a:ty, $b:ty) => {
        fn on_update(
            mut commands: Commands,
            mut query: Query<&mut $component>,
            mut a: $a,
            mut b: $b,
        ) {
            for mut v in query.iter_mut() {
                $on_update(&mut commands, &mut v, &mut a, &mut b);
            }
        }
    };
    ($on_update:expr, $component:ty, $a:ty, $b:ty, $c:ty) => {
        fn on_update(
            mut commands: Commands,
            mut query: Query<&mut $component>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
        ) {
            for mut v in query.iter_mut() {
                $on_update(&mut commands, &mut v, &mut a, &mut b, &mut c);
            }
        }
    };
    ($on_update:expr, $component:ty, $a:ty, $b:ty, $c:ty, $d:ty) => {
        fn on_update(
            mut commands: Commands,
            mut query: Query<&mut $component>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
            mut d: $d,
        ) {
            for mut v in query.iter_mut() {
                $on_update(&mut commands, &v, &mut a, &mut b, &mut c, &mut d);
            }
        }
    };
}

#[macro_export]
macro_rules! on_click_system {
    ($on_update:expr, $component:ty, ) => {
        fn on_click(
            mut commands: Commands,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction)) {
                $on_update
            }

            for v in query.iter() {
                match v.1 {
                    Interaction::Pressed => f()(&v),
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty) => {
        fn on_click(
            mut commands: Commands,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a) {
                $on_click
            }

            for v in query.iter() {
                match v.1 {
                    Interaction::Pressed => f()(&v, &mut a),
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty) => {
        fn on_click(
            mut commands: Commands,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b) {
                $on_click
            }

            for v in query.iter() {
                match v.1 {
                    Interaction::Pressed => f()(&v, &mut a, &mut b),
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty, $c:ty) => {
        fn on_click(
            mut commands: Commands,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b, &mut $c) {
                $on_click
            }

            for v in query.iter() {
                match v.1 {
                    Interaction::Pressed => f()(&v, &mut a, &mut b, &mut c),
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty, $c:ty, $d:ty) => {
        fn on_click(
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
            mut d: $d,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b, &mut $c, &mut $d) {
                $on_click
            }

            for v in query.iter() {
                match v.1 {
                    Interaction::Pressed => f()(&v, &mut a, &mut b, &mut c, &mut d),
                    _ => {}
                }
            }
        }
    };
}

#[macro_export]
macro_rules! on_click {
    ($component:ty, ($($queries:ty),*), $function:expr) => {
        impl $component {
            on_click_system!($function, $component, $($queries),*);
        }
    };
}

#[macro_export]
macro_rules! element {
    ($component:ty, $children:expr) => {
        impl $component {
            #[allow(dead_code)]
            fn element() -> Element {
                $children(fragment([])).with(Self::default())
            }

            #[allow(dead_code)]
            fn slot(children: impl IntoIterator<Item = Element>) -> Element {
                $children(fragment(children)).with(Self::default())
            }
        }
    };
}

#[derive(Component)]
struct ElementDiv;

pub fn div(
    class: impl Fn(NodeBundle, Interaction) -> NodeBundle + 'static,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    let vec = children.into_iter().collect::<Vec<_>>();

    Element::new(move |parent| {
        let bundle = create_interaction_bundle!(class, NodeBundle);

        parent
            .spawn((ElementDiv, Interaction::None, bundle))
            .with_children(|current| {
                for child in vec.into_iter() {
                    child.0(current);
                }
            })
            .id()
    })
}

pub fn root(
    component: impl Component,
    class: impl Fn(NodeBundle, Interaction) -> NodeBundle,
    el: Element,
) -> impl FnOnce(&mut Commands) {
    let bundle = class(NodeBundle::default(), Interaction::None);

    move |commands| {
        commands
            .spawn((component, ElementDiv, Interaction::None, bundle))
            .with_children(|parent| {
                el.0(parent);
            });
    }
}

#[derive(Component)]
struct ElementImg;
pub fn img(
    class: impl Fn(ImageBundle, Interaction) -> ImageBundle + 'static,
    src: Handle<Image>,
) -> Element {
    Element::new(move |parent| {
        let mut bundle = create_interaction_bundle!(class, ImageBundle);
        bundle.0.image.texture = src.clone();
        parent.spawn((ElementImg, Interaction::None, bundle)).id()
    })
}

#[derive(Component)]
struct ElementText;

pub fn text(
    class: impl Fn(TextStyle, Interaction) -> TextStyle + 'static,
    text: impl ToString + 'static,
) -> Element {
    Element::new(move |parent| {
        let interaction_style = InteractionTextStyle {
            none: class(TextStyle::default(), Interaction::None),
            hover: class(TextStyle::default(), Interaction::Hovered),
            pressed: class(TextStyle::default(), Interaction::Pressed),
        };

        let bundle = TextBundle::from_section(text.to_string(), interaction_style.none.clone());

        parent
            .spawn((ElementText, Interaction::None, interaction_style, bundle))
            .id()
    })
}
