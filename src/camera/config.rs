// Starting Coordinates (X, Y, Z)
// Edge of a 100x100m floor (Z=50), at eye-level (Y=2.0)
pub const START_POS: [f32; 3] = [0.0, 2.0, 50.0]; 

// Starting Orientation (in degrees)
pub const START_YAW: f32 = -90.0; // Looking towards the center (down -Z)
pub const START_PITCH: f32 = -5.0; // Looking slightly down

// Movement & Feel
pub const CAMERA_SPEED: f32 = 20.0; // Reduced from 100.0 for smaller scale
pub const MOUSE_SENSITIVITY: f32 = 0.002;
pub const PITCH_LIMIT: f32 = 89.0;