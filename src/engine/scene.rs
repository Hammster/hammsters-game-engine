use std::collections::HashMap;
use engine::gameobject::{GameObjectManager, GameObject};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Running,
    Paused,
    Stoped,
    None,
}

#[derive(Debug)]
pub struct Scene {
    pub gameobjectmanager: GameObjectManager,
    pub state: State,
}

#[derive(Debug)]
pub struct SceneManager {
    pub scenes: HashMap<&'static str, Scene>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            gameobjectmanager: GameObjectManager::new(),
            state: State::Stoped,
        }
    }

    pub fn start(&mut self) {
        self.state = State::Running;
    }

    pub fn stop(&mut self) {
        self.state = State::Stoped;
    }

    pub fn pause(&mut self) {
        self.state = State::Paused;
    }

    pub fn debug(&mut self) -> &mut HashMap<&'static str, Box<GameObject>>{
        let ret : &mut HashMap<&'static str, Box<GameObject>> = &mut self.gameobjectmanager.objects;
        ret
    }
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager { scenes: HashMap::new() }
    }

    pub fn create(&mut self, id: &'static str) {
        self.scenes.insert(id, Scene::new());
    }

    pub fn insert(&mut self, id: &'static str, mut scene: Scene) {
        self.scenes.insert(id, scene);
    }

    pub fn get(&mut self, id: &'static str) -> Option<&mut Scene> {
        self.scenes.get_mut(id)
    }

    pub fn get_state(&mut self, id: &'static str) -> State {
        match self.scenes.get(id) {
            Some(scene) => scene.state,
            None => State::None,
        }
    }

    /*
        pub fn get_objectmanager(&mut self, id: &'static str) -> &mut HashMap<&'static str, Box<GameObject>> {
            self.scenes.get(id).unwrap().debug()
        }
    */

    pub fn is_running(&mut self, id: &'static str) -> bool {
        self.get_state(id) == State::Running
    }

    pub fn is_paused(&mut self, id: &'static str) -> bool {
        (self.get_state(id) == State::Paused)
    }

    pub fn is_stoped(&mut self, id: &'static str) -> bool {
        (self.get_state(id) == State::Stoped)
    }
}
