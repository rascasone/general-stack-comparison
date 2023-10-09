use leptos::RwSignal;

use crate::models::QueryableUser;

#[derive(Clone)]
pub enum UserPageView {
    List,
    Add,
    Edit(usize),
}

#[derive(Clone)]
pub struct UserPageState {
    pub view: RwSignal<UserPageView>,
    pub users: RwSignal<Option<Vec<QueryableUser>>>,
}

impl UserPageState {
    pub fn new() -> Self {
        Self {
            view: RwSignal::new(UserPageView::List),
            users: RwSignal::new(None),
        }
    }
}

impl Default for UserPageState {
    fn default() -> Self {
        UserPageState::new()
    }
}
