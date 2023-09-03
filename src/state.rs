use strum::{EnumCount, FromRepr};
use tui::{style::Color, widgets::ListState};

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

#[derive(Default, PartialEq, Copy, Clone, EnumCount, FromRepr)]
#[repr(u8)]
pub(super) enum Focus {
    #[default]
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
pub(super) struct State {
    pub countries: ListState,
    pub cities: ListState,
    pub focus: Focus,
}

impl State {
    pub(super) fn move_focus(&mut self, right: bool) {
        let mut focus = self.focus as u8;
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
    pub(super) fn focused(&mut self) -> &mut ListState {
        match self.focus {
            Focus::Cities => &mut self.cities,
            Focus::Countries => &mut self.countries,
        }
    }
}

impl State {
    pub(super) fn new() -> Self {
        let mut state = Self::default();
        state.countries.select(Some(0));
        state.cities.select(Some(0));

        state
    }
}
