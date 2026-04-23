#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::tabs_in_doc_comments)]

//! # egui_colorimage_gradients
//! 
//! EGUI ColorImage Gradients is a simple, lightweight crate to add to any of your projects.
//! 
//! It is made to improve the current ColorImage implementation, by providing extra features,
//! by supporting gradient generation.
//! 
//! 
//! ## Getting Started
//! 
//! Add the crate if you haven't yet:
//! ```toml
//! egui_colorimage_gradients = "0.1.0"
//! ```
//! 
//! Simply import the [`ColorImageGradient`] trait to extend your ColorImage functionality.
//! 
//! ```rust
//! use egui_colorimage_gradients::ColorImageGradient;
//! ```
//! 
//! The Usage tab goes more in depth about how to use this.
//! 
//! ## Usage
//! 
//! There are two ways to create a [`gradient`]: 
//! - Manually create your gradient.
//! - Use one of my presets.
//! 
//! ### Manually create your gradient:
//! Create a gradient by defining every field.
//! 
//! ```rust
//! use egui::Color32;
//! use egui_colorimage_gradients::{Gradient, GradientType, GradientAngle};
//! 
//! // Within the <> brackets, you have to enter the number of colors
//! let gradient = Gradient<1> {
//! 	kind: GradientType::Linear,
//! 	colors: [Color32::BLACK],
//! 	angle: GradientAngle::Radian(0.0)
//! };
//! ```
//! 
//! and from there, you can create your ColorImage using the [`gradient`] fn:
//! 
//! ```rust
//! use egui::ColorImage;
//! use egui_colorimage_gradients::ColorImageGradient;
//! 
//! let color_image = ColorImage::gradient([6, 9], gradient);
//! ```
//! 
//! Note: The `size` determines the pixels of your ColorImage.
//! 
//! ### Use one of my presets:
//! 
//! ```toml
//! egui_colorimage_gradients = { version = "0.1.0", features = ["presets"]}
//! ```
//! 
//! Note: You MUST to enable the feature flag for presets to work.
//! 
//! ```rust
//! use egui::ColorImage;
//! use egui_colorimage_gradients::{
//! 	ColorImageGradient,
//! 	Gradient, GradientType, GradientAngle,
//! 	presets::*
//! };
//! 
//! let color_image = ColorImage::gradient([4, 20], electric_neon_blue());
//! ```
//! 
//! Note: Presets are in the experimental stages of sorts, I might add or 
//! remove existing ones, change the way they behave, etc.
//! Please look out for those changes.
//! 
//! ## Gradient Types
//! I put together a small table, just to show how each type of gradient
//! is like.
//! 
//! |    Type    |                   Description                    | Angle Support |
//! | :--------: | :----------------------------------------------: | :-----------: |
//! | **Linear** | A smooth transition along a straight line.       |   Full 360°   |
//! | **Radial** | Colors radiate outward from the center pixel.    |    Ignored    |
//! | **Conic**  | Colors go in an angular swirl around the center. |    Ignored    |
//! 
//! Note: Thank you for using my crate.
//! If you have any issues or queries, you can reach me at 
//! [Issues](https://github.com/Perzium/egui_colorimage_gradients/issues) or 
//! [Discussions](https://github.com/Perzium/egui_colorimage_gradients/discussions).

#[cfg(feature = "presets")]
pub mod presets;
#[cfg(test)]
mod tests;

use egui::{Color32, ColorImage};

// NO STANDARD: Just copied down the consts from f32.rs
// or atleast I tried to keep it no-std.

/// Archimedes' constant (π)
#[allow(clippy::approx_constant)]
const PI: f32 = 3.14159265358979323846264338327950288_f32;
/// π/2
#[allow(clippy::approx_constant)]
const FRAC_PI_2: f32 = 1.57079632679489661923132169163975144_f32;


/// There are 4 options so far;
/// 
/// ### Vertical:
/// Top to Bottom; `y` axis.
/// 
/// ### Horizontal:
/// Left to Right; `x` axis.
/// 
/// ### Degree:
/// Angle of gradient in degree; Will be clamped within -360 to 360.
/// 
/// ### Radian:
/// Angle of gradient in radian; Will be clamped within -π to π.
/// 
/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
/// If you prefer to see clockwise rotations, just provide the negative of your value.
pub enum GradientAngle {
	/// ### Vertical:
	/// Top to Bottom; `y` axis.
	Vertical,
	/// ### Horizontal:
	/// Left to Right; `x` axis.
	Horizontal,
	/// ### Degree:
	/// Angle of gradient in degree; Will be clamped within -360 to 360.
	/// 
	/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
	/// If you prefer to see clockwise rotations, just provide the negative of your value.
	Degree(f32), // Clamp within -360.0 to 360.0
	/// ### Radian:
	/// Angle of gradient in radian; Will be clamped within -π to π.
	/// 
	/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
	/// If you prefer to see clockwise rotations, just provide the negative of your value.
	Radian(f32), // Clamp within -π to π
}

/// The type of [`Gradient`] you want to create.
/// 
/// Currently, there are 3 types of gradients:
/// 
/// ### Linear:
/// Line Gradient; Generates a smooth line-ish gradient from point A to point B
/// 
/// ### Radial:
/// Colors go from inward to outward.
/// 
/// ### Conic:
/// Colors go from left and swirls toward the right.
/// Starts at the left edge.
pub enum GradientType {
	/// ### Linear:
	/// Line Gradient; Generates a smooth line-ish gradient from point A to point B
	Linear,
	/// ### Radial:
	/// Colors go from inward to outward.
	Radial,
	/// ### Conic:
	/// Colors go from left and swirls toward the right.
	/// Starts at the left edge.
	Conic,
}

/// Gradient struct; Allows you to define custom gradients
/// 
/// # Usage
/// Define the type of gradients you'd like, by doing this:
/// 
/// ```rust
/// use egui::Color32;
/// use egui_colorimage_gradients::{Gradient, GradientType, GradientAngle};
/// 
/// let gradient = Gradient<1> {
/// 	kind: GradientType::Linear,
/// 	colors: [Color32::BLACK],
/// 	angle: GradientAngle::Radian(0.0)
/// };
/// ```
pub struct Gradient<const NUM: usize> {
	pub kind: GradientType,
	pub colors: [Color32; NUM],
	pub angle: GradientAngle,
}

impl Default for Gradient<2> {
	/// Just a linear gradient as default
	/// to show that it works, kinda.
	fn default() -> Self {
		Self {
			kind: GradientType::Linear,
			colors: [Color32::BLACK, Color32::WHITE],
			angle: GradientAngle::Vertical,
		}
	}
}

pub trait ColorImageGradient {
	#[allow(clippy::doc_lazy_continuation)]
	/// Generates a ColorImage with a gradient
	/// 
	/// This function maps a [`Gradient`] into an egui::ColorImage of the specified `size`.
	/// 
	/// # Arguments 
	/// - `size` - The width and height of the resulting image in pixels.
	/// - `gradient` - The gradient configuration. Includes type of gradient,
	/// 	colors and their angles.
	/// 
	/// # Usage
	/// It's kind of similar to CSS Gradients, you create a gradient:
	/// 
	/// ```rust
	/// use egui::{Color32, ColorImage};
	/// use egui_colorimage_gradients::{ColorImageGradient, Gradient, GradientType, GradientAngle};
	/// 
	/// let gradient = Gradient<2> {
	/// 	kind: GradientType::Linear,
	/// 	colors: [Color32::BLACK, Color32::WHITE],
	/// 	angle: GradientAngle::Vertical
	/// };
	/// 
	/// let image = ColorImage::gradient([100, 100], gradient);
	/// ```
	fn gradient<const NUM: usize>(size: [usize; 2], gradient: Gradient<NUM>) -> Self;
	
	// TODO: Make a fn which takes a preset fn as a template
}

impl ColorImageGradient for ColorImage {
	#[allow(clippy::doc_lazy_continuation)]
	/// Generates a ColorImage with a gradient
	/// 
	/// This function maps a [`Gradient`] into an egui::ColorImage of the specified `size`.
	/// 
	/// # Arguments 
	/// - `size` - The width and height of the resulting image in pixels.
	/// - `gradient` - The gradient configuration. Includes type of gradient,
	/// 	colors and their angles.
	/// 
	/// # Usage
	/// It's kind of similar to CSS Gradients, you create a gradient:
	/// 
	/// ```rust
	/// use egui::{Color32, ColorImage};
	/// use egui_colorimage_gradients::{ColorImageGradient, Gradient, GradientType, GradientAngle};
	/// 
	/// let gradient = Gradient<2> {
	/// 	kind: GradientType::Linear,
	/// 	colors: [Color32::BLACK, Color32::WHITE],
	/// 	angle: GradientAngle::Vertical
	/// };
	/// 
	/// let image = ColorImage::gradient([100, 100], gradient);
	/// ```
	fn gradient<const NUM: usize>(size: [usize; 2], gradient: Gradient<NUM>) -> Self {
		// Fetch and store the size of image
		let [w, h] = size;
		// Create a pixel buffer
		let mut pixels = Vec::with_capacity(w * h);

		// Angle in which the gradient is rotated
		let theta = match gradient.angle {
			GradientAngle::Horizontal => 0.0,
			GradientAngle::Vertical => FRAC_PI_2,
			GradientAngle::Degree(d) => d * (PI / 180.0),
			GradientAngle::Radian(r) => {
				let rhs = PI * 2.0;
				let r = r % rhs;
				if r < 0.0 { r + rhs.abs() } else { r }
			}
		};

		// Functions and stretches
		let (sin, cos) = theta.sin_cos();
		let stretch = (sin.abs() + cos.abs()) * 0.5;

		// Loop through height
		for y in 0..h {
			// Store normalized y and dy
			let ny = if h > 1 { y as f32 / ((h - 1) as f32).max(1.0) } else { 0.0 };
			let dy = ny - 0.5;

			// Loop through width
			for x in 0..w {
				// Store normalized x and dx
				let nx = if w > 1 { x as f32 / (w - 1) as f32 } else { 0.0 };
				let dx = nx - 0.5;

				// Match t to type of gradient and clamp
				let t = match gradient.kind {
					GradientType::Linear => {
						// Dot product math...
						// Takes me back to school
						let projection = dx * cos + dy * sin;

						#[rustfmt::skip]
						let t = if stretch > 0.0 {
							((projection / stretch) * 0.5 + 0.5).clamp(0.0, 1.0)
						} else { 0.5 };

						t
					}

					// PYTHAGOREAN THEOREM
					GradientType::Radial => (dx * dx + dy * dy).sqrt() * 2.0,

					GradientType::Conic => {
						let angle = dy.atan2(dx);

						(angle + PI) / (2.0 * PI)
					}
				}
				.clamp(0.0, 1.0);

				// Find color stops and interpolate color
				pixels.push(sample_gradient(t, &gradient.colors));
			}
		}

		return Self { size, pixels };

		fn sample_gradient<const NUM: usize>(t: f32, colors: &[Color32; NUM]) -> Color32 {
			
			#[rustfmt::skip] {
				if NUM == 0 { return Color32::TRANSPARENT; }
				if NUM == 1 { return colors[0]; }

				// 100% Saturation at ends
				// f32 precision can diminish this sometimes
				if t <= 0.0 { return colors[0]; }
				if t >= 1.0 { return colors[NUM - 1]; }
			};

			// Scale t to range
			let scaled_t = t * (NUM - 1) as f32;
			let index = (scaled_t.floor() as usize).min(NUM - 2);

			// Progress between this nad next color
			let local_t = scaled_t - index as f32;

			// Return color
			lerp_color(colors[index], colors[index + 1], local_t)
		}

		/// Calculates the mixed value of color A and color B.
		/// `t` is the factor in which the colors get mixed.
		fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
			Color32::from_rgba_premultiplied(
				(a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
				(a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
				(a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
				(a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
			)
		}
	}
}