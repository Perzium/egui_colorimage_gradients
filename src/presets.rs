// This file contains all the presets;

use egui::Color32;
use crate::{Gradient, GradientType, GradientAngle};

/// Think '90s retro consoles vibe, or the music
/// 
/// It's more TRON actually.
pub fn electric_neon_blue() -> Gradient<3> {
	Gradient {
		kind: GradientType::Linear,
		colors: [
			Color32::from_rgb(0, 69, 83),
			Color32::from_rgb(177, 223, 239),
			Color32::from_rgb(0, 69, 83)
			],
		angle: GradientAngle::Vertical,
	}
}

/// An amazing sunset feel
/// 
/// Try golden, comfortable ish
pub fn sunset_glow() -> Gradient<2> {
	Gradient {
		kind: GradientType::Linear,
		colors: [
			Color32::from_rgb(255, 155, 159),
			Color32::from_rgb(250, 208, 196),
		],
		angle: GradientAngle::Degree(45.0)
	}
}



/// Think deep blue, vibrant as it goes down 
pub fn ocean_2() -> Gradient<2> {
	Gradient {
		kind: GradientType::Linear,
		colors: [
			Color32::from_rgb(36, 57, 73),
			Color32::from_rgb(81, 127, 164),
		],
		angle: GradientAngle::Vertical
	}
}

/// Dark green to slightly less darker green
/// looks good, i vouch
pub fn forest() -> Gradient<3> {
	Gradient {
		kind: GradientType::Linear, 
		colors: [
			Color32::from_rgb(15, 39, 32),
			Color32::from_rgb(32, 67, 58),
			Color32::from_rgb(44, 100, 83)
		], 
		angle: GradientAngle::Vertical
	}
}

/// Not really golden, more so as orange juice
pub fn golden_hour() -> Gradient<2> {
	Gradient {
		kind: GradientType::Linear, 
		colors: [
			Color32::from_rgb(255, 140, 0),
			Color32::from_rgb(255, 0, 128)
		], 
		angle: GradientAngle::Horizontal
	}
}

/// Weird ish colour, I wouldn't use.
pub fn slate_misty() -> Gradient<2> {
	Gradient {
		kind: GradientType::Linear, 
		colors: [
			Color32::from_rgb(189, 195, 199),
			Color32::from_rgb(44, 62, 80)
		], 
		angle: GradientAngle::Degree(90.0)
	}
}

/// Very soft white ish blue
/// looks cold, feels cold
pub fn soft_frost() -> Gradient<2> {
	Gradient {
		kind: GradientType::Radial,
		colors: [
			Color32::from_rgb(225, 225, 255),
			Color32::from_rgb(175, 175, 215),
		],
		angle: GradientAngle::Horizontal
	}
}

/// Orange dot. ORANGE.
/// 
/// I'd use a custom style if i were you.
pub fn burning_fffire() -> Gradient<2> {
	Gradient {
		kind: GradientType::Radial, 
		colors: [
			Color32::from_rgb(255, 100, 0),
			Color32::from_rgb(60, 40, 0)
		], 
		angle: GradientAngle::Vertical
	}
}

/// Not good looking, trust me.
/// 
/// It's pink and purple, but radial never looks good.
pub fn royal() -> Gradient<2> {
	Gradient {
		kind: GradientType::Radial, 
		colors: [
			Color32::from_rgb(194, 21, 227),
			Color32::from_rgb(66, 0, 100)
		], 
		angle: GradientAngle::Horizontal
	}
}

/// pride
pub fn rainbow() -> Gradient<7> {
	Gradient {
		kind: GradientType::Conic,
		colors: [
			Color32::from_rgb(255, 0, 0),    // Red
			Color32::from_rgb(255, 255, 0),  // Yellow
			Color32::from_rgb(0, 255, 0),    // Green
			Color32::from_rgb(0, 255, 255),  // Cyan
			Color32::from_rgb(0, 0, 255),    // Blue
			Color32::from_rgb(255, 0, 255),  // Magenta
			Color32::from_rgb(255, 0, 0),    // Back to Red
		],
		angle: GradientAngle::Vertical
	}
}

/// Matrix green and black ish, but looks bad
pub fn matrix() -> Gradient<3> {
	Gradient {
		kind: GradientType::Conic, 
		colors: [
			Color32::from_rgb(0, 255, 0),
			Color32::from_rgb(0, 50, 0),
			Color32::from_rgb(0, 255, 0)
		], 
		angle: GradientAngle::Degree(270.0)
	}
}

/// One of the good looking gradients.
/// Looks exactly as it says.
pub fn copper() -> Gradient<4> {
	Gradient {
		kind: GradientType::Conic, 
		colors: [
			Color32::from_rgb(184, 115, 51),
			Color32::from_rgb(255, 215, 180),
			Color32::from_rgb(139, 69, 19),
			Color32::from_rgb(184, 115, 51)
		], 
		angle: GradientAngle::Degree(0.0)
	}
}

/// Looks similar to a comically large lollipop.
/// 
/// Similar to those they have at the shop counters.
pub fn candy() -> Gradient<5> {
	Gradient {
		kind: GradientType::Conic, 
		colors: [
			Color32::from_rgb(255, 255, 255),
			Color32::from_rgb(255, 0, 100),
			Color32::from_rgb(255, 255, 255),
			Color32::from_rgb(255, 0, 100),
			Color32::from_rgb(255, 255, 255),
		], 
		angle: GradientAngle::Radian(0.785)
	}
}