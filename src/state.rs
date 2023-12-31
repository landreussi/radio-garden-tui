use api::Place;
use ratatui::{style::Color, widgets::ListState};
use strum::{EnumCount, FromRepr};

pub(super) fn r#move(state: &mut ListState, up: bool, mut max_value: usize) {
    let mut selected = state.selected();
    {
        let selected = selected.get_or_insert(0);
        if up {
            if selected > &mut 0 {
                *selected -= 1;
            }
        } else if selected < &mut max_value {
            *selected += 1;
        }
    }
    state.select(selected);
}

#[derive(Default, PartialEq, Clone, EnumCount, FromRepr)]
#[repr(u8)]
pub enum Focus {
    #[default]
    Search = 0,
    Countries = 1,
    Cities = 2,
}

impl Focus {
    pub fn color_from(&self, focus: &Self) -> Color {
        if self == focus {
            Color::LightGreen
        } else {
            Color::White
        }
    }
}

#[derive(Default)]
pub struct State {
    pub countries: ListState,
    pub cities: ListState,
    pub search: String,
    pub focus: Focus,
    pub places: Vec<Place>,
}

impl State {
    pub(super) fn new() -> Self {
        let mut state = Self::default();
        state.countries.select(Some(0));
        state.cities.select(Some(0));

        state
    }
    fn move_focus(&mut self, right: bool) {
        let mut focus = self.focus.clone() as u8;
        if right {
            if focus == Focus::COUNT as u8 {
                focus = 1;
            } else {
                focus += 1;
            }
        } else if focus == 1 {
            focus = Focus::COUNT as u8;
        } else {
            focus -= 1;
        }

        self.focus = Focus::from_repr(focus).unwrap();
    }
    fn focused(&mut self) -> &mut ListState {
        match self.focus {
            Focus::Cities => &mut self.cities,
            Focus::Countries | Focus::Search => &mut self.countries,
        }
    }
    pub(super) fn move_up(&mut self) {
        let num_places = self.places.len();
        let focused = self.focused();
        r#move(focused, true, num_places)
    }
    pub(super) fn move_down(&mut self) {
        if self.focus == Focus::Search {
            self.focus = Focus::Countries;
        } else {
            let num_places = self.places.len();
            let focused = self.focused();
            r#move(focused, false, num_places)
        }
    }
    pub(super) fn move_right(&mut self) {
        self.move_focus(true)
    }
    pub(super) fn move_left(&mut self) {
        self.move_focus(false)
    }
    pub(super) fn move_to_search(&mut self) {
        self.focus = Focus::from_repr(0).unwrap();
    }
}
