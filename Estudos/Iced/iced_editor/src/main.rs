use iced::{
    Settings,
    Sandbox,
    Element,
    widget::text_editor
};

#[derive(Debug, Clone)]
enum Message {
    Edit(
        text_editor::Action
    )
}

struct Editor {
    content: text_editor::Content
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content:: text_editor::Content::new()
        }
    }

    fn title(
        &self
    ) -> String {
        String::from(
            "A cool editor!"
        )
    }

    fn update(
        &mut self,
        message: Message
    ) {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
        }
    }

    fn view(
        &self
    ) -> Element<'_, Message> {
        text_editor(
            &self.content
        ).on_edit(
            Message::Edit
        ).into()
    }
}

fn main() -> iced::Result {
    Editor::run(
        Settings::default()
    )
}