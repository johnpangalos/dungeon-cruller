use bevy::{ecs::system::Insert, prelude::*};

pub struct StylesPlugin;

impl Plugin for StylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                button_cursor_system,
                interaction_style_system,
                interaction_text_style_system,
                interaction_background_color_system,
            ),
        );
    }
}

#[derive(Component)]
pub struct InteractionBackgroundColor {
    none: BackgroundColor,
    hover: BackgroundColor,
    pressed: BackgroundColor,
}

#[derive(Component, Clone)]
pub struct InteractionStyle {
    none: Style,
    hover: Style,
    pressed: Style,
}

#[derive(Component, Clone)]
pub struct InteractionTextStyle {
    none: TextStyle,
    hover: TextStyle,
    pressed: TextStyle,
}

fn button_cursor_system(
    mut windows: Query<&mut Window>,
    buttons: Query<&Interaction, With<Button>>,
) {
    for interaction in buttons.iter() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                for mut window in &mut windows {
                    window.cursor.icon = CursorIcon::Hand;
                }
                return;
            }
            _ => {}
        }
    }

    for mut window in &mut windows {
        window.cursor.icon = CursorIcon::Default;
    }
}

fn interaction_style_system(
    mut query: Query<(&Interaction, &InteractionStyle, &mut Style), Changed<Interaction>>,
) {
    for (interaction, state, mut style) in &mut query {
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
    for (interaction, state, mut style) in &mut query {
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
    for (interaction, state, mut style) in &mut query {
        for section in style.sections.iter_mut() {
            match interaction {
                Interaction::None => section.style = state.none.clone(),
                Interaction::Hovered => section.style = state.hover.clone(),
                Interaction::Pressed => section.style = state.pressed.clone(),
            }
        }
    }
}

pub trait DynElement {
    fn spawn(&self, parent: &mut ChildBuilder, slot: Element) -> Entity;
}

pub trait DynAs {
    fn spawn_as(&self, parent: &mut ChildBuilder) -> Entity;
}

pub trait IntoElement {
    fn as_el(self, element: Element) -> Element;
}

impl<T: DynElement + 'static> IntoElement for T {
    fn as_el(self, element: Element) -> Element {
        Element::DynAs(Box::new(self), Box::new(element))
    }
}

pub trait IntoMaterial<X: UiMaterial> {
    fn material(self) -> MaterialNodeBundle<X>;
}

pub enum Element {
    Dyn(Box<dyn DynElement>, Vec<Element>),
    DynAs(Box<dyn DynElement>, Box<Element>),
    Fragment(Vec<Element>),
    Div(
        (
            NodeBundle,
            Interaction,
            InteractionStyle,
            InteractionBackgroundColor,
        ),
        Vec<Element>,
    ),
    Button(
        (ButtonBundle, InteractionStyle, InteractionBackgroundColor),
        Vec<Element>,
    ),
    Image(
        (
            ImageBundle,
            Interaction,
            InteractionStyle,
            InteractionBackgroundColor,
        ),
    ),
    Text((TextBundle, Interaction, InteractionTextStyle)),
}

impl IntoIterator for Element {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
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

pub fn button(
    class: impl Fn(ButtonBundle, Interaction) -> ButtonBundle + 'static,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    let vec = children.into_iter().collect::<Vec<_>>();
    let bundle = create_interaction_bundle!(class, ButtonBundle);

    Element::Button(bundle, vec)
}

#[macro_export]
macro_rules! run_click_system {
    ($on_update:expr, $component:ty, ) => {
        fn run_click_system(
            input: Res<Input<MouseButton>>,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction)) {
                $on_update
            }

            for v in &query {
                match v.1 {
                    Interaction::Pressed | Interaction::Hovered => f()(&v),
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty) => {
        fn run_click_system(
            input: Res<Input<MouseButton>>,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a) {
                $on_click
            }

            for v in &query {
                match v.1 {
                    Interaction::Hovered => {
                        if (input.just_released(MouseButton::Left)) {
                            f()(&v, &mut a)
                        }
                    }
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty) => {
        fn run_click_system(
            input: Res<Input<MouseButton>>,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b) {
                $on_click
            }

            for v in &query {
                match v.1 {
                    Interaction::Hovered => {
                        if (input.just_released(MouseButton::Left)) {
                            f()(&v, &mut a, &mut b)
                        }
                    }
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty, $c:ty) => {
        fn run_click_system(
            input: Res<Input<MouseButton>>,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b, &mut $c) {
                $on_click
            }

            for v in &query {
                match v.1 {
                    Interaction::Hovered => {
                        if (input.just_released(MouseButton::Left)) {
                            f()(&v, &mut a, &mut b, &mut c)
                        }
                    }
                    _ => {}
                }
            }
        }
    };
    ($on_click:expr, $component:ty, $a:ty, $b:ty, $c:ty, $d:ty) => {
        fn run_click_system(
            input: Res<Input<MouseButton>>,
            query: Query<(Entity, &Interaction), (Changed<Interaction>, With<$component>)>,
            mut a: $a,
            mut b: $b,
            mut c: $c,
            mut d: $d,
        ) {
            fn f() -> impl Fn(&(Entity, &Interaction), &mut $a, &mut $b, &mut $c, &mut $d) {
                $on_click
            }

            for v in &query {
                match v.1 {
                    Interaction::Hovered => {
                        if (input.just_released(MouseButton::Left)) {
                            f()(&v, &mut a, &mut b, &mut c, &mut d)
                        }
                    }
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
            fn on_click() -> bevy::ecs::schedule::NodeConfigs<std::boxed::Box<dyn bevy::prelude::System<In = (), Out = ()>>>  {
                run_click_system!($function, $component, $($queries),*);

                run_click_system.run_if(bevy::input::common_conditions::input_just_released(MouseButton::Left))
            }
        }
    };
}

impl<T: Bundle + Clone> DynElement for T {
    fn spawn(&self, parent: &mut ChildBuilder, slot: Element) -> Entity {
        let e = spawn_element(parent, slot);

        parent.add_command(Insert {
            entity: e,
            bundle: self.clone(),
        });

        e
    }
}

pub fn spawn_element(parent: &mut ChildBuilder, element: Element) -> Entity {
    match element {
        Element::Fragment(children) => {
            for child in children {
                spawn_element(parent, child);
            }
            parent.parent_entity()
        }
        Element::Dyn(render, children) => render.spawn(parent, Element::Fragment(children)),
        Element::DynAs(render, element) => render.spawn(parent, *element),
        Element::Div(div, children) => parent
            .spawn(div)
            .with_children(|current| {
                for child in children {
                    spawn_element(current, child);
                }
            })
            .id(),
        Element::Button(button, children) => parent
            .spawn(button)
            .with_children(|current| {
                for child in children {
                    spawn_element(current, child);
                }
            })
            .id(),
        Element::Text(text) => parent.spawn(text).id(),

        Element::Image(image) => parent.spawn(image).id(),
    }
}

pub fn spawn_root_element<T: Component>(commands: &mut Commands, component: T, tree: Element) {
    let screen = (
        component,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(screen).with_children(|parent| {
        spawn_element(parent, tree);
    });
}

pub fn div(
    class: impl Fn(NodeBundle, Interaction) -> NodeBundle + 'static,
    children: impl IntoIterator<Item = Element>,
) -> Element {
    let vec = children.into_iter().collect::<Vec<_>>();
    let (a, b, c) = create_interaction_bundle!(class, NodeBundle);
    Element::Div((a, Interaction::None, b, c), vec)
}

pub fn root<T: Component>(component: T, element: Element) -> (T, Element) {
    (component, element)
}

pub fn img(
    class: impl Fn(ImageBundle, Interaction) -> ImageBundle + 'static,
    src: Handle<Image>,
) -> Element {
    let (mut a, b, c) = create_interaction_bundle!(class, ImageBundle);
    a.image.texture = src.clone();

    Element::Image((a, Interaction::None, b, c))
}

pub fn text(
    class: impl Fn(TextStyle, Interaction) -> TextStyle + 'static,
    text: impl ToString + 'static,
) -> Element {
    let interaction_style = InteractionTextStyle {
        none: class(TextStyle::default(), Interaction::None),
        hover: class(TextStyle::default(), Interaction::Hovered),
        pressed: class(TextStyle::default(), Interaction::Pressed),
    };

    let bundle = TextBundle::from_section(text.to_string(), interaction_style.none.clone());

    Element::Text((bundle, Interaction::None, interaction_style))
}

pub fn mat<T: UiMaterial>(
    class: impl Fn(MaterialNodeBundle<T>, Interaction) -> MaterialNodeBundle<T> + 'static,
    material: Handle<T>,
) -> Element {
    let base = MaterialNodeBundle::<T> {
        material,
        ..Default::default()
    };
    let bundle = class(base.clone(), Interaction::None);
    let bundle_none = class(base.clone(), Interaction::None);
    let bundle_hover = class(base.clone(), Interaction::Hovered);
    let bundle_pressed = class(base.clone(), Interaction::Pressed);

    let interaction_style = InteractionStyle {
        none: bundle_none.style,
        hover: bundle_hover.style,
        pressed: bundle_pressed.style,
    };

    Element::Dyn(
        Box::new(MaterialElement {
            bundle: (bundle, interaction_style),
        }),
        vec![],
    )
}

// Materials are dynamic because of the UiMaterial generic needed for them to work
struct MaterialElement<T: UiMaterial> {
    bundle: (MaterialNodeBundle<T>, InteractionStyle),
}

impl<T: UiMaterial> DynElement for MaterialElement<T> {
    fn spawn(&self, parent: &mut ChildBuilder, slot: Element) -> Entity {
        let (bundle, interaction_style) = &self.bundle;
        let entity = parent
            .spawn((bundle.clone(), interaction_style.clone()))
            .with_children(|current| {
                spawn_element(current, slot);
            })
            .id();

        entity
    }
}
