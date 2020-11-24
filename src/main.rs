use cursive::views::{Dialog, SelectView, TextView};
use cursive::{align::HAlign, Cursive};

mod groups;
use groups::group_manager::GroupManager;

struct Controller {
    groupManager: GroupManager,
}

impl Controller {
    fn new() -> Self {
        return Controller {
            groupManager: GroupManager::new(),
        };
    }
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let mut group_select = SelectView::<String>::new().h_align(HAlign::Center);
    for group in groups {
        group_select.add_item_str(group);
    }

    group_select.set_on_submit(move |s: &mut Cursive, selected_group: &String| {
        s.pop_layer();
        let mut challenge_select = SelectView::new().h_align(HAlign::Center);
        for challenge in manager.get_group_challenge_names(&*selected_group).unwrap() {
            challenge_select.add_item_str(challenge);
        }
        s.add_layer(Dialog::around(challenge_select).button("Quit", |s| s.quit()));

        challenge_select.set_on_submit(|s: &mut Cursive, selected_challenge: &String| {
            s.pop_layer();
            s.add_layer(
                Dialog::around(TextView::new(selected_challenge)).button("Quit", |s| s.quit()),
            );
        });
    });

    siv.add_layer(Dialog::around(group_select).title("Challenge Group"));

    // Starts the event loop.
    siv.run();
}
