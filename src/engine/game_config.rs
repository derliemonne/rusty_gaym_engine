use ini::Ini;

#[derive(PartialEq, Debug)]
pub struct GameConfig {
    pub screen_width: usize,
    pub screen_height: usize,
    pub target_fps: usize,
    pub camera_fov: f32,
    pub camera_draw_distance: f32,
}

impl GameConfig {
    pub fn default() -> GameConfig {
        GameConfig { 
            screen_width: 300,
            screen_height: 200,
            target_fps: 30,
            camera_fov: 75.0,
            camera_draw_distance: 100.0,
        }
    }

    /// Loads configs from file.
    pub fn load_from_file(filepath: &str) -> Result<GameConfig, Box<dyn std::error::Error>> {
        let ini = Ini::load_from_file(filepath)?;
        let section = ini.general_section();
        let screen_width = section.get("screen_width").ok_or("Missing screen_width")?.parse()?;
        let screen_height = section.get("screen_height").ok_or("Missing screen_height")?.parse()?;
        let target_fps = section.get("target_fps").ok_or("Missing target_fps")?.parse()?;
        let camera_fov = section.get("camera_fov").ok_or("Missing camera_fov")?.parse()?;
        let camera_draw_distance = section.get("camera_draw_distance").ok_or("Missing camera_draw_distance")?.parse()?;

        Ok(GameConfig {
            screen_width,
            screen_height,
            target_fps,
            camera_fov,
            camera_draw_distance,
        })
    }

    /// Saves configs to file.
    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut ini = Ini::new();
        ini.with_general_section()
            .set("screen_width", &self.screen_width.to_string())
            .set("screen_height", &self.screen_height.to_string())
            .set("target_fps", &self.target_fps.to_string())
            .set("camera_fov", &self.camera_fov.to_string())
            .set("camera_draw_distance", &self.camera_draw_distance.to_string());
        ini.write_to_file(filepath).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod game_config_tests {
    use std::fs;
    use super::*;

    fn remove_file(filename: &str) {
        fs::remove_file(filename)
            .expect("Could not delete test file `{}`. Try to remove it.");
    }

    #[test]
    fn save_and_load_file() {
        let filename = "src/config.ini"; 

        let save = GameConfig::default();
        save.save_to_file(filename)
            .expect("Could not save to file.");

        let load = GameConfig::load_from_file(filename)
            .expect("Could not read from file.");

        assert_eq!(save, load);

        remove_file(filename);
    }

    #[test]
    fn save_and_read_different() {
        let mut a = GameConfig::default();
        a.camera_draw_distance = 100.0;
        
        let mut b = GameConfig::default();
        b.camera_draw_distance = 90.0;

        assert_ne!(a, b);

        a.save_to_file("a.ini").expect("Could not save to file a.ini");
        b.save_to_file("b.ini").expect("Could not save to file b.ini");

        let a_load = GameConfig::load_from_file("a.ini").expect("Could not read from a.ini");
        let b_load = GameConfig::load_from_file("b.ini").expect("Could not read from b.ini");

        assert_ne!(a_load, b_load);

        remove_file("a.ini");
        remove_file("b.ini");
    }
}
