use cursive::align::HAlign;
use cursive::views::{Dialog, SelectView, TextView};

mod groups;
use groups::group_manager::GroupManager;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let manager = GroupManager::new();
    // Read groups
    let groups = manager.get_group_names();

    let mut time_select = SelectView::new().h_align(HAlign::Center);
    for group in groups {
        time_select.add_item(group, 1);
    }

    time_select.set_on_submit(|s, time| {
        s.pop_layer();
        let text = format!("You will wait for {} minutes...", time);
        s.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
    });

    siv.add_layer(Dialog::around(time_select).title("Challenge"));

    // Starts the event loop.
    siv.run();
}
