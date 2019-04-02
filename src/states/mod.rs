mod game;
mod menu;

pub use self::game::GameRunning;
pub use self::menu::Menu;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    MainMenu,
    Gameplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction {
    OpenMenu,
    StartGame,
    Quit,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::MainMenu
    }
}

pub struct Game {
    pub user_action: Option<UserAction>,
    pub current_state: CurrentState,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            user_action: None,
            current_state: CurrentState::default(),
        }
    }
}
