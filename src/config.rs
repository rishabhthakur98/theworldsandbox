// Window & Screen
pub const WINDOW_TITLE: &str = "TheWorldSandbox";
pub const WINDOW_WIDTH: u32 = 1366;
pub const WINDOW_HEIGHT: u32 = 768;
pub const IS_FULLSCREEN: bool = true;
pub const CLEAR_COLOR: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

// Camera Settings
pub const START_POS: [f32; 3] = [0.0, 2.0, 50.0]; 
pub const START_YAW: f32 = -90.0; 
pub const START_PITCH: f32 = -5.0; 
pub const CAMERA_SPEED: f32 = 20.0; 
pub const MOUSE_SENSITIVITY: f32 = 0.002;
pub const PITCH_LIMIT: f32 = 89.0;

// Lighting Settings
pub const SUN_AZIMUTH: f32 = 45.0;
pub const SUN_ELEVATION: f32 = 45.0;
pub const SUN_COLOR: [f32; 3] = [1.0, 0.95, 0.9]; 
pub const SUN_INTENSITY: f32 = 1.0;
pub const AMBIENT_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
pub const AMBIENT_INTENSITY: f32 = 0.1;