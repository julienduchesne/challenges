#[macro_use]
extern crate lazy_static;

use cursive::traits::{Nameable, Resizable, View};
use cursive::view::{IntoBoxedView, SizeConstraint};
use cursive::views::{LinearLayout, PaddedView, Panel, ScrollView, SelectView, TextView};
use cursive::{align::HAlign, Cursive};

mod groups;
use groups::group_manager::GroupManager;

fn pad<V>(v: V) -> PaddedView<V> {
    return PaddedView::lrtb(
        1, 1, 1, 1, // Left, Right, Top, Bottom
        v,
    );
}

struct UserData {
    group_manager: GroupManager,
    selected_group: String,
    selected_challenge: Option<String>,
}

impl UserData {
    fn get_group_manager(&self) -> &GroupManager {
        return &self.group_manager;
    }
}

fn update_view(s: &mut Cursive) {
    let user_data: &mut UserData = s.user_data::<UserData>().unwrap();
    let challenges = user_data
        .get_group_manager()
        .get_group_challenge_names(user_data.selected_group.as_str())
        .unwrap();

    if user_data.selected_challenge.is_none() {
        user_data.selected_challenge = Some(challenges[0].clone());
    }
    let selected_challenge_name = user_data.selected_challenge.clone().unwrap();
    let selected_challenge = user_data
        .group_manager
        .get_challenge(
            user_data.selected_group.as_str(),
            selected_challenge_name.as_str(),
        )
        .unwrap();
    let title = selected_challenge.title().to_owned();
    let description = selected_challenge.description().to_owned();
    let content = format!("{}", description);

    s.call_on_name("challenge_select", |view: &mut SelectView| {
        view.clear();
        for (i, challenge) in challenges.iter().enumerate() {
            view.add_item_str(challenge);
            if *challenge == selected_challenge_name {
                view.set_selection(i);
            }
        }
    });

    s.call_on_name("panel", |view: &mut Panel<TextView>| {
        view.set_title(title);
    });

    s.call_on_name("content", |view: &mut TextView| {
        view.set_content(content);
    });
}

fn create_group_select(group_names: Vec<String>) -> Box<dyn View> {
    let mut group_select = SelectView::<String>::new()
        .h_align(HAlign::Center)
        .on_select(|s: &mut Cursive, item: &String| {
            let user_data: &mut UserData = s.user_data::<UserData>().unwrap();
            user_data.selected_group = (*item).clone();
            user_data.selected_challenge = None;
            update_view(s);
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

fn create_challenge_select() -> Box<dyn View> {
    let challenge_select = SelectView::<String>::new()
        .h_align(HAlign::Center)
        .on_select(|s: &mut Cursive, item: &String| {
            let user_data: &mut UserData = s.user_data::<UserData>().unwrap();
            user_data.selected_challenge = Some((*item).clone());
            update_view(s);
        });
    let mut panel = Panel::new(ScrollView::new(
        challenge_select
            .with_name("challenge_select")
            .resized(SizeConstraint::AtLeast(25), SizeConstraint::Free),
    ));
    panel.set_title("Challenges");
    return pad(panel).as_boxed_view();
}

fn create_challenge_display() -> Box<dyn View> {
    let panel = Panel::new(TextView::new("").with_name("content"));
    return pad(panel
        .with_name("panel")
        .resized(SizeConstraint::Full, SizeConstraint::Full))
    .as_boxed_view();
}

fn main() {
    let mut siv = cursive::default();
    let group_manager = GroupManager::new();
    let group_names = group_manager.get_group_names();
    let first_group = group_names[0].as_str();
    siv.set_user_data(UserData {
        group_manager: GroupManager::new(),
        selected_group: first_group.to_owned(),
        selected_challenge: None,
    });

    let linear_layout = LinearLayout::horizontal()
        .child(create_group_select(group_names))
        .child(create_challenge_select())
        .child(create_challenge_display())
        .full_screen();
    siv.add_fullscreen_layer(linear_layout);
    update_view(&mut siv);

    siv.run();
}
