use std::sync::{Arc, Mutex};

use adw::prelude::*;
use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use adw::gtk::{Box, ListBox, Orientation, SelectionMode};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    //let mut count = 0;
    let count = Arc::new(Mutex::new(0));

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let countRow = ActionRow::builder()
            .activatable(false)
            .title(format!("Count: {:?}", count))
            .build();
        let row = ActionRow::builder()
            .activatable(true)
            .title("Increase")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
            let mut count = count.lock().unwrap();
            *count += 1;
        });
        let row2 = ActionRow::builder()
            .activatable(true)
            .title("Decrease")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&countRow);
        list.append(&row);
        list.append(&row2);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}