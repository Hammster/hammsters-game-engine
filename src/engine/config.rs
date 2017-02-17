use std::path::Path;
use yaml_rust::yaml::{Yaml, YamlLoader};
use engine::util::file;

// define config values
#[derive(Debug, Clone)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub title: String,
}

// path to the config file
pub const CONFIG_FILE_PATH: &'static str = "resources/config.yaml";

impl Config {
    pub fn load() -> Self {
        // load the default config set and make the mutable
        let mut config: Config = Default::default();
        // if a config file exists load it and replace then inthe
        if Path::new(&CONFIG_FILE_PATH).exists() {
            // get yaml file content
            let yaml_file = file::read_file(&CONFIG_FILE_PATH);
            // parse file content as `Yaml`
            let yaml_data = YamlLoader::load_from_str(&yaml_file).unwrap().pop().unwrap();

            // override the default settings, each readout requires
            // you to define the output type, and the default fallback
            config.fullscreen = yaml_data["fullscreen"]
                .as_bool()
                .unwrap_or(config.fullscreen);

            config.fullscreen = yaml_data["vsync"]
                .as_bool()
                .unwrap_or(config.vsync);

            config.window_width = yaml_data["window_width"]
                .as_i64()
                .unwrap_or(config.window_width as i64) as u32;

            config.window_height = yaml_data["window_height"]
                .as_i64()
                .unwrap_or(config.window_height as i64) as u32;

            config.title = yaml_data["title"]
                .as_str()
                .unwrap_or(config.title.as_str())
                .to_string();
        }

        // return `Config`
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_height: 600,
            window_width: 800,
            fullscreen: false,
            vsync: false,
            title: String::new(),
        }
    }
}
