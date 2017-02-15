use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub isRunning: bool,
}

#[derive(Debug)]
pub struct StateManager {
    pub states: HashMap<&'static str, State>,
}

impl State {
    pub fn new() -> Self {
        let mut a: bool = false;
        State { isRunning: a }
    }

    pub fn toggle(&mut self) {
        self.isRunning = !self.isRunning;
    }

    pub fn set(&mut self, isRunning: bool) {
        self.isRunning = isRunning;
    }
}

impl StateManager {
    pub fn new() -> Self {
        StateManager { states: HashMap::new() }
    }

    // wrapper for inserts
    pub fn insert(&mut self, id: &'static str, mut state: State) {
        self.states.insert(id, state);
    }

    pub fn get(&mut self, id: &'static str) -> Option<&mut State> {
        self.states.get_mut(id)
    }

    pub fn is_running(&mut self, id: &'static str) -> bool {
        match self.states.get(id) {
            Some(state) => state.isRunning,
            None => false,
        }
    }
}
