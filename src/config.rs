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

// Rendering Settings
pub const MAX_LIGHTS: usize = 16; 
pub const DISPLACEMENT_SCALE: f32 = 0.05; 
pub const USE_POM: u32 = 0; // 1 = ON, 0 = OFF
pub const SHADOW_RESOLUTION: u32 = 2048; // Higher = Crisper shadows, but more VRAM


pub const AMBIENT_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
pub const AMBIENT_INTENSITY: f32 = 0.1;