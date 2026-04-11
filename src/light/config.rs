// The angle of the sun in the sky.
// Azimuth is horizontal rotation (0 to 360 degrees)
// Elevation is vertical angle (0 is horizon, 90 is directly overhead)
pub const SUN_AZIMUTH: f32 = 45.0;
pub const SUN_ELEVATION: f32 = 45.0;

// RGB color of the sun (0.0 to 1.0)
// Currently set to a slightly warm, natural daylight tone
pub const SUN_COLOR: [f32; 3] = [1.0, 0.95, 0.9]; 
pub const SUN_INTENSITY: f32 = 5.0;

// Ambient light ensures shadows aren't 100% pitch black
pub const AMBIENT_COLOR: [f32; 3] = [1.0, 1.0, 1.0];
pub const AMBIENT_INTENSITY: f32 = 0.1;