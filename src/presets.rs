use crate::{ColorImageGradient, Direction, GradientAngle};
use egui::{Color32, ColorImage};

/// Think '90s retro consoles vibe, or the music
///
/// It's more TRON actually.
pub fn electric_neon_blue(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 3] = &[
		(Color32::from_rgb(0, 69, 83), None),
		(Color32::from_rgb(177, 223, 239), None),
		(Color32::from_rgb(0, 69, 83), None),
	];

	ColorImage::linear_gradient(size, GradientAngle::Direction(Direction::TopDown), colors)
}

/// An amazing sunset feel
///
/// Try golden, comfortable ish
pub fn sunset_glow(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(255, 155, 159), None), (Color32::from_rgb(250, 208, 196), None)];

	ColorImage::linear_gradient(size, GradientAngle::Degree(45.0), colors)
}

/// Think deep blue, vibrant as it goes down
pub fn ocean_2(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(36, 57, 73), None), (Color32::from_rgb(81, 127, 164), None)];

	ColorImage::linear_gradient(size, GradientAngle::Direction(Direction::TopDown), colors)
}

/// Dark green to slightly less darker green
/// looks good, i vouch
pub fn forest(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 3] = &[
		(Color32::from_rgb(15, 39, 32), None),
		(Color32::from_rgb(32, 67, 58), None),
		(Color32::from_rgb(44, 100, 83), None),
	];

	ColorImage::linear_gradient(size, GradientAngle::Direction(Direction::TopDown), colors)
}

/// Not really golden, more so as orange juice
pub fn golden_hour(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(255, 140, 0), None), (Color32::from_rgb(255, 0, 128), None)];

	ColorImage::linear_gradient(size, GradientAngle::Direction(Direction::LeftToRight), colors)
}

/// Weird ish colour, I wouldn't use.
pub fn slate_misty(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(189, 195, 199), None), (Color32::from_rgb(44, 62, 80), None)];

	ColorImage::linear_gradient(size, GradientAngle::Direction(Direction::BottomUp), colors)
}

/// Very soft white ish blue
/// looks cold, feels cold
pub fn soft_frost(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(225, 225, 255), None), (Color32::from_rgb(175, 175, 215), None)];

	ColorImage::radial_gradient(size, None, None, colors)
}

/// Orange dot. ORANGE.
///
/// I'd use a custom style if i were you.
pub fn burning_fffire(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(255, 100, 0), None), (Color32::from_rgb(60, 40, 0), None)];

	ColorImage::radial_gradient(size, None, None, colors)
}

/// Not good looking, trust me.
///
/// It's pink and purple, but radial never looks good.
pub fn royal(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 2] =
		&[(Color32::from_rgb(194, 21, 227), None), (Color32::from_rgb(66, 0, 100), None)];

	ColorImage::radial_gradient(size, None, None, colors)
}

/// pride
pub fn rainbow(size: [usize; 2]) -> ColorImage {
	let colors: &[(Color32, Option<u8>); 7] = &[
		(Color32::from_rgb(255, 0, 0), None),   // Red
		(Color32::from_rgb(255, 255, 0), None), // Yellow
		(Color32::from_rgb(0, 255, 0), None),   // Green
		(Color32::from_rgb(0, 255, 255), None), // Cyan
		(Color32::from_rgb(0, 0, 255), None),   // Blue
		(Color32::from_rgb(255, 0, 255), None), // Magenta
		(Color32::from_rgb(255, 0, 0), None),   // Back to Red
	];

	ColorImage::conic_gradient(size, None, None, colors)
}

/// Matrix green and black ish, but looks bad
pub fn matrix(size: [usize; 2]) -> ColorImage {
	let colors = &[
		(Color32::from_rgb(0, 255, 0), None),
		(Color32::from_rgb(0, 50, 0), None),
		(Color32::from_rgb(0, 255, 0), None),
	];

	ColorImage::conic_gradient(
		size,
		Some(GradientAngle::Direction(Direction::RightToLeft)),
		None,
		colors,
	)
}

/// One of the good looking gradients.
/// Looks exactly as it says.
pub fn copper(size: [usize; 2]) -> ColorImage {
	let colors = &[
		(Color32::from_rgb(184, 115, 51), None),
		(Color32::from_rgb(255, 215, 180), None),
		(Color32::from_rgb(139, 69, 19), None),
		(Color32::from_rgb(184, 115, 51), None),
	];

	ColorImage::conic_gradient(size, None, None, colors)
}

/// Looks similar to a comically large lollipop.
///
/// Similar to those they have at the shop counters.
pub fn candy(size: [usize; 2]) -> ColorImage {
	let colors = &[
		(Color32::from_rgb(255, 255, 255), None),
		(Color32::from_rgb(255, 0, 100), None),
		(Color32::from_rgb(255, 255, 255), None),
		(Color32::from_rgb(255, 0, 100), None),
		(Color32::from_rgb(255, 255, 255), None),
	];

	ColorImage::conic_gradient(size, Some(GradientAngle::Radian(0.785)), None, colors)
}
