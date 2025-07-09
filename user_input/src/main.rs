use iocraft::prelude::*;

#[derive(Default, Props)]
struct NameInputProps {
    value: Option<State<String>>,
    has_focus: bool
}

#[component]
fn NameInput(props: &NameInputProps) -> impl Into<AnyElement<'static>> {
    let Some(mut value) = props.value else {
        panic!("Value is required");
    };

    element! {
        View (
            border_style: if props.has_focus { BorderStyle::Round} else {BorderStyle::None},
            border_color: Color::Blue,
            padding: if props.has_focus { 0 } else { 1 }, 
        ) {
            View (
                background_color: Color::DarkGrey,
                width: 30,
                height: 1
            ) {
                TextInput (
                    has_focus: props.has_focus,
                    value: value.to_string(),
                    on_change: move |new_value| value.set(new_value),
                )
            }
        }
    }
}

#[component]
fn NameForm(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let name = hooks.use_state(|| "".to_string());
    let mut has_focus = hooks.use_state(|| true);
    let mut should_submit = hooks.use_state(|| false);

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Enter => should_submit.set(true),
                _ => {}
            }
        }
        _ => {}
    });

    if should_submit.get() {
        system.exit();
        element! (View)
    }
    else {
        element! {
            View (
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: 2
            ) {
                View (margin_bottom : 1) {
                    Text(content: "Enter your Name : ", color: Color::White)
                }
                NameInput(value: name, has_focus: has_focus.get())
                View(margin_top: 1) {
                    Text(content: "Press Enter to submit", color: Color::Grey)
                }
            }
        }
    }
}

fn main() {
    smol::block_on(
        element! {
            NameForm
        }
        .render_loop(),
    )
        .unwrap();
}
