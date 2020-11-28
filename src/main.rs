use cursive::traits::Resizable;
use cursive::view::SizeConstraint;
use cursive::views::{Button, Dialog, LinearLayout, ScrollView, SelectView, TextArea, TextView};
use cursive::{align::HAlign, Cursive};
use cursive::{traits::Boxable, views::PaddedView};

mod groups;
use groups::group_manager::GroupManager;

struct Controller {
    group_manager: GroupManager,
}

impl Controller {
    fn new() -> Self {
        return Controller {
            group_manager: GroupManager::new(),
        };
    }

    fn run(&self) {
        let mut siv = cursive::default();
        let mut group_select = SelectView::<String>::new().h_align(HAlign::Center);
        for group in self.group_manager.get_group_names() {
            for i in 0..100 {
                group_select.add_item_str(i.to_string());
            }
        }
        let mut group_select2 = SelectView::<String>::new().h_align(HAlign::Center);
        for group in self.group_manager.get_group_names() {
            group_select2.add_item_str(group);
        }
        let linear_layout = LinearLayout::horizontal()
            .child(PaddedView::lrtb(
                2,
                2,
                0,
                0, // Left, Right, Top, Bottom
                ScrollView::new(group_select).resized(SizeConstraint::Free, SizeConstraint::Full),
            ))
            .child(PaddedView::lrtb(
                2,
                2,
                0,
                0, // Left, Right, Top, Bottom
                ScrollView::new(group_select2).resized(SizeConstraint::Free, SizeConstraint::Full),
            ))
            .child(Button::new("Ok", |s| s.quit()));
        siv.add_fullscreen_layer(linear_layout);

        // let mut challenge_select = SelectView::new().h_align(HAlign::Center);
        // for challenge in self
        //     .group_manager
        //     .get_group_challenge_names("Project Euler")
        //     .unwrap()
        // {
        //     challenge_select.add_item_str(challenge);
        // }
        // siv.add_layer(ScrollView::new(challenge_select));

        siv.run();
    }
}

fn main() {
    let controller = Controller::new();
    controller.run();

    // Starts the event loop.
}
