use std::{cell::RefCell, rc::Rc};

use crate::caldav::client::{CalDAVClient, Calendar};




pub enum CurrentScreen {
    Home(HomeState),
    List(ListType),
    View(Rc<RefCell<Calendar>>, SelectedTodo),
    Edit(Rc<RefCell<Calendar>>, SelectedTodo),
    New()
}

pub struct HomeState {
    selection: usize
}

pub enum ListType {
    Today,
    All,
    Past,
    Calendar(String)
}

pub struct SelectedTodo {

}

pub struct App {
    client: CalDAVClient,
    screen: CurrentScreen
}

impl App {
    pub fn new(client: CalDAVClient) -> App {
        let state = HomeState { selection: 0 };
        App {
            client, screen: CurrentScreen::Home(state)
        }
    }
}
