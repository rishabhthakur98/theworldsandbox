// Starting Coordinates (X, Y, Z)
// Starting at Y=390.0 puts you roughly at the top of a 390m radius planet
pub const START_POS: [f32; 3] = [0.0, 390.0, 1000.0]; 

// Starting Orientation (in degrees)
pub const START_YAW: f32 = -90.0;
pub const START_PITCH: f32 = 0.0;

// Movement & Feel
pub const CAMERA_SPEED: f32 = 100.0; 
pub const MOUSE_SENSITIVITY: f32 = 0.002;
pub const PITCH_LIMIT: f32 = 89.0;