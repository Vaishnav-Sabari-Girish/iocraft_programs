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
    let name = hooks.use_state(|| "".to_string());
    let age = hooks.use_state(|| "".to_string());
    let mut current_field = hooks.use_state(|| 0);   //0 = name, 1 = age
    let mut should_submit = hooks.use_state(|| false);
    let mut submitted_name = hooks.use_state(|| "".to_string());
    let mut submitted_age = hooks.use_state(|| 0);


    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Tab => {
                    current_field.set((current_field.get() + 1) % 2);
                },
                KeyCode::Enter => {
                    if current_field.get() == 0 {
                        current_field.set(1);
                    }
                    else {
                        should_submit.set(true);
                    }
                },
                _ => {}
            }
        }
        _ => {}
    });

    if should_submit.get() {
        if submitted_name.to_string().is_empty() {
            submitted_name.set(name.to_string());
        }
        if submitted_age.get() == 0 {
            submitted_age.set(age.to_string().parse().expect("Not a valid number"));
        }

        element! {
            View (
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: 2
            ) {
                Text(content: format!("Hello, {}", submitted_name.to_string()), color: Color::Green)
                Text(content: format!("You are {} years old", submitted_age), color: Color::Blue)
            }
        }
    }
    else {
        element! {
            View (
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: 2
            ) {
                View (
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin_bottom: 1
                ) {
                    Text(content: "Enter your Name : ", color: Color::White)
                    NameInput(value: name, has_focus: current_field.get() == 0)
                }

                View (
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin_bottom: 1,
                ) {
                    Text(content: "Enter you age: ", color: Color::White)
                    NameInput(value: age, has_focus: current_field.get() == 1)
                }

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
