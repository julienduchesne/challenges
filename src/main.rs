#[macro_use]
extern crate lazy_static;

use cursive::traits::{Nameable, Resizable, View};
use cursive::view::{IntoBoxedView, SizeConstraint};
use cursive::views::{LinearLayout, PaddedView, Panel, ScrollView, SelectView, TextView};
use cursive::{align::HAlign, Cursive};

mod groups;
use groups::challenge_config::ChallengeConfig;
use groups::group_manager::GroupManager;

fn pad<V>(v: V) -> PaddedView<V> {
    return PaddedView::lrtb(
        1, 1, 1, 1, // Left, Right, Top, Bottom
        v,
    );
}

struct UserData {
    group_manager: GroupManager,
    // selected_group: &str,
    // selected_challenge: &str,
}

impl UserData {
    fn get_group_manager(&self) -> &GroupManager {
        return &self.group_manager;
    }
}

fn create_group_select(group_names: Vec<String>) -> Box<dyn View> {
    let mut group_select = SelectView::<String>::new()
        .h_align(HAlign::Center)
        .on_select(|s: &mut Cursive, item: &String| {
            let user_data: &mut UserData = s.user_data::<UserData>().unwrap();
            let challenges = user_data
                .get_group_manager()
                .get_group_challenge_names(item)
                .unwrap();
            s.call_on_name("challenge_select", |view: &mut SelectView| {
                view.clear();
                for challenge in challenges {
                    view.add_item_str(challenge);
                }
            });
        });
    for group in group_names {
        group_select.add_item_str(group);
    }
    let mut panel = Panel::new(ScrollView::new(
        group_select
            .with_name("group_select")
            .resized(SizeConstraint::AtLeast(25), SizeConstraint::Free),
    ));
    panel.set_title("Groups");
    return pad(panel).as_boxed_view();
}

fn create_challenge_select(initial_challenges: Vec<String>) -> Box<dyn View> {
    let mut challenge_select = SelectView::<String>::new()
        .h_align(HAlign::Center)
        .on_select(|s: &mut Cursive, item: &String| {
            let user_data: &mut UserData = s.user_data::<UserData>().unwrap();
            s.call_on_name("content", |view: &mut TextView| view.set_content("test"));
        });
    for challenge in initial_challenges {
        challenge_select.add_item_str(challenge);
    }
    let mut panel = Panel::new(ScrollView::new(
        challenge_select
            .with_name("challenge_select")
            .resized(SizeConstraint::AtLeast(25), SizeConstraint::Free),
    ));
    panel.set_title("Challenges");
    return pad(panel).as_boxed_view();
}

fn create_challenge_display(initial_challenge: &Box<dyn ChallengeConfig>) -> Box<dyn View> {
    let mut panel = Panel::new(TextView::new(initial_challenge.description()).with_name("content"));
    panel.set_title(initial_challenge.title());
    return pad(panel
        .with_name("panel")
        .resized(SizeConstraint::Full, SizeConstraint::Full))
    .as_boxed_view();
}

// fn update_challenge_display()

fn main() {
    let mut siv = cursive::default();
    let group_manager = GroupManager::new();
    let group_names = group_manager.get_group_names();
    let first_group = group_names[0].as_str();
    let initial_challenges = group_manager
        .get_group_challenge_names(first_group)
        .unwrap();
    let first_challenge = group_manager
        .get_challenge(first_group, initial_challenges[0].as_str())
        .unwrap();
    siv.set_user_data(UserData {
        group_manager: GroupManager::new(),
    });

    let linear_layout = LinearLayout::horizontal()
        .child(create_group_select(group_names))
        .child(create_challenge_select(initial_challenges))
        .child(create_challenge_display(first_challenge))
        .full_screen();
    siv.add_fullscreen_layer(linear_layout);

    siv.run();
}
