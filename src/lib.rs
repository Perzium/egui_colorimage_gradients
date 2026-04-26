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
//! egui_colorimage_gradients = "0.2.0"
//! ```
//! NOTE: Users below `egui 0.34.0` need to enable a feature flag:
//! ```toml
//! egui_colorimage_gradients = { version = "0.2.0", features = ["below_egui_0_34_0"] }
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
//! There are two ways to create a gradient:
//! - Manually create your gradient.
//! - Use one of my presets.
//!
//! ### Manually create your gradient:
//! Create a gradient by defining making a list of colors.
//!
//! ```rust
//! use egui::{Color32, ColorImage};
//! use egui_colorimage_gradients::*;
//!
//! // Note: `u8`s within `Some()` must add up to 100.
//! let colors = [
//! 	(Color32::from_rgb(0, 69, 83), Some(30)),
//! 	(Color32::from_rgb(177, 223, 239), Some(40)),
//! 	(Color32::from_rgb(0, 69, 83), Some(30)),
//! ];
//!
//! // From there, you can create your ColorImage using any one of the gradient functions:
//!
//! let color_image = ColorImage::linear_gradient(
//! 	[6, 9],
//! 	GradientAngle::Direction(Direction::TopDown),
//! 	&colors
//! );
//! ```
//!
//! Note: The `size` determines the pixels of your ColorImage.
//!
//! ### Use one of my presets:
//!
//! Note: You MUST to enable the feature flag for presets to work.
//! ```toml
//! egui_colorimage_gradients = { version = "0.2.0", features = ["presets"] }
//! ```
//!
//! ```rust
//! use egui::ColorImage;
//! use egui_colorimage_gradients::presets::*;
//! use egui_colorimage_gradients::*;
//!
//! let color_image = electric_neon_blue([100, 100]);
//! ```
//!
//! ### Blend function
//! The `blend()` function is still in early stages. It works, but ehh.
//!
//! ```rust
//! use egui::ColorImage;
//! use egui_colorimage_gradients::presets::*;
//! use egui_colorimage_gradients::*;
//! 
//! // Blends these two [`ColorImage`]s.
//! let mixed_gradient = ColorImage::blend([100, 100], &[
//! 	(electric_neon_blue([69, 420]), None),
//! 	(sunset_glow([911, 67]), None),
//! ]);
//! ```
//! 
//! And the results look somewhat like this:
//! |                  Electric Neon Blue                  |               Sunset Glow              |     If they had a child    |
//! | :--------------------------------------------------: | :------------------------------------: | :------------------------: |
//! | ![electric_neon_blue](assets/electric_neon_blue.png) | ![sunset_glow](assets/sunset_glow.png) | ![mixed](assets/mixed.png) |
//! 
//! Like I mentioned earlier, the blend function is still in early stages, and will get a massive refactor.
//!
//! Note: Presets are in the experimental stages of sorts, I might add or
//! remove existing ones, change the way they behave, etc.
//! Please look out for those changes.
//!
//! Note: Thank you for using my crate.
//! If you have any issues or queries, you can reach me at
//! [Issues](https://github.com/Perzium/egui_colorimage_gradients/issues) or
//! [Discussions](https://github.com/Perzium/egui_colorimage_gradients/discussions).

/// Pre-made presets for quick gradients.
///
/// I wouldn't recommend most of them.
#[cfg(feature = "presets")]
pub mod presets;

#[cfg(test)]
mod tests;

// Enums are included within this file; Or that's how it should seem?
mod enums;
pub use crate::enums::{GradientAngle, Position, Shape};

// Use whichever one based on features
#[cfg(feature = "below_egui_0_34_0")]
pub use crate::enums::Direction;
#[cfg(not(feature = "below_egui_0_34_0"))]
pub use egui::Direction;

// Use for simplification
use egui::{Color32, ColorImage};

// NO STANDARD: Just copied down the consts from f32.rs
// or atleast I tried to keep it no-std.

/// Archimedes' constant (π)
#[allow(clippy::approx_constant)]
const PI: f32 = 3.14159265358979323846264338327950288_f32;
/// π/2
#[allow(clippy::approx_constant)]
const FRAC_PI_2: f32 = 1.57079632679489661923132169163975144_f32;

/// ## ColorImageGradient
/// The entry point for gradient functions in `ColorImage`.
///
/// ### Usage
/// Import to your project and you can use `ColorImage::(some gradient function)()` right away.
///
/// ```rust
/// use egui_colorimage_gradients::ColorImageGradient;
/// ```
///
/// Comes with 4 functions so far
///
/// * Linear Gradient
/// * Radial Gradient
/// * Conic Gradient
/// * Blend (NEW!)
///
/// Details about each function are given in the function docs.
///
/// Note: While all the other gradient functions take percentages within `Option<u8>`,
/// the `blend()` function takes `Option<f32>` and it's more so as blend factor rather than percentages.
pub trait ColorImageGradient {
	fn linear_gradient(
		size: [usize; 2],
		direction: GradientAngle,
		colors: &[(Color32, Option<u8>)],
	) -> Self;

	fn radial_gradient(
		size: [usize; 2],
		shape: Option<Shape>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self;

	fn conic_gradient(
		size: [usize; 2],
		angle: Option<GradientAngle>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self;

	fn blend(size: [usize; 2], images: &[(ColorImage, Option<f32>)]) -> Self;
}

impl ColorImageGradient for ColorImage {
	#[allow(clippy::doc_lazy_continuation)]
	/// ## Linear Gradient
	/// Generates a [`ColorImage`] with a linear gradient
	///
	/// ## Usage
	/// The usage is similar to how you do it in CSS;
	/// ```rust
	/// use egui_colorimage_gradients::*;
	/// use egui::{Color32, ColorImage};
	///
	/// let size = [100, 100];
	///
	/// let colors = [
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// 	(Color32::from_rgb(177, 223, 239), None),
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// ];
	///
	/// // Usage here
	/// let image = ColorImage::linear_gradient(
	/// 	size,
	/// 	GradientAngle::Direction(Direction::TopDown),
	/// 	&colors
	/// );
	/// ```
	///
	/// ### Parameters
	/// * Size `[width, height]`: Determines the size of the ColorImage generated.
	/// * Gradient Angle `GradientAngle`: Determines the rotation of the gradient.
	/// * Colors `&[(Color32, Option<u8>)]`: A list of colors with the percent of the image they take up.
	/// 	Leave it at `None` for uniform distribution.
	fn linear_gradient(
		size: [usize; 2],
		gradient_angle: GradientAngle,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		// Fetch and store the size of image
		let [w, h] = size;
		let source_size = egui::vec2(w as f32, h as f32);

		// Return empty image if no colors were provided
		if colors.is_empty() {
			return Self { size, source_size, pixels: Vec::new() };
		}

		// Angle in which the gradient is rotated
		let theta = calculate_angle(gradient_angle);

		// Functions and stretches
		let (sin, cos) = theta.sin_cos();
		let stretch = (sin.abs() + cos.abs()) * 0.5;

		// Calculate resolved color stops
		let resolved = resolve_color_stops(colors);

		// Create a pixel buffer
		let mut pixels = Vec::with_capacity(w * h);

		// Loop through height
		for y in 0..h {
			let ny = if h > 1 { y as f32 / ((h - 1) as f32).max(1.0) } else { 0.0 };
			let dy = ny - 0.5;

			// Loop through width
			for x in 0..w {
				let nx = if w > 1 { x as f32 / (w - 1) as f32 } else { 0.0 };
				let dx = nx - 0.5;

				// Dot product projection
				let projection = dx * cos + dy * sin;

				// Find factor
				#[rustfmt::skip]
				let t =
				if stretch > 0.0 { ((projection / stretch) * 0.5 + 0.5).clamp(0.0, 1.0) }
				else { 0.5 };

				// Sample interpolated color and push
				let final_color = sample_resolved_gradient(t, &resolved);
				pixels.push(final_color);
			}
		}

		Self { size, source_size, pixels }
	}

	/// ## Radial Gradient
	/// Generates a [`ColorImage`] with a radial gradient
	///
	/// ## Usage
	/// The usage is similar to how you do it in CSS;
	/// ```rust
	/// use egui_colorimage_gradients::*;
	/// use egui::{Color32, ColorImage};
	///
	/// let size = [100, 100];
	///
	/// let colors = [
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// 	(Color32::from_rgb(177, 223, 239), None),
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// ];
	///
	/// // Usage here
	/// let image = ColorImage::radial_gradient(size, None, None, &colors);
	/// ```
	///
	/// ### Parameters
	/// * Size `[width, height]`: Determines the size of the ColorImage generated
	/// * Shape `Some(Shape)`: Determines the shape of the Radial gradient.
	/// 	Leave it at `None` for an ellipse.
	/// * Position `Some(Position)`: Determines where the Radial gradient starts.
	/// 	Leave it at `None` for center.
	/// * Colors `&[(Color32, Option<u8>)]`: A list of colors with the percent of the image they take up.
	/// 	Leave it at `None` for uniform distribution.
	#[allow(clippy::doc_lazy_continuation)]
	fn radial_gradient(
		size: [usize; 2],
		shape: Option<Shape>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		// Fetch and store the size of image
		let [w, h] = size;
		let source_size = egui::vec2(w as f32, h as f32);

		// Return empty image if no colors were provided
		if colors.is_empty() {
			return Self { size, source_size, pixels: Vec::new() };
		}

		// Resolved pixels and a pixel buffer
		let resolved = resolve_color_stops(colors);
		let mut pixels = Vec::with_capacity(w * h);

		// Resolve position to normalized coordinates (0 to 1)
		let (cx, cy) = calculate_position(position);

		let is_circle = matches!(shape, Some(Shape::Circle));
		let aspect_ratio = if is_circle && h > 0 { w as f32 / h as f32 } else { 1.0 };

		// Loop through height
		for y in 0..h {
			let ny = if h > 1 { y as f32 / (h - 1) as f32 } else { 0.0 };
			let dy = ny - cy;

			// Loop through width
			for x in 0..w {
				let nx = if w > 1 { x as f32 / (w - 1) as f32 } else { 0.0 };
				let mut dx = nx - cx;

				// Adjust aspect ratio for perfect circles
				if is_circle {
					dx *= aspect_ratio;
				}

				// Pythagorean theorem scaled by 2 to stretch across the bounds
				let t = ((dx * dx + dy * dy).sqrt() * 2.0).clamp(0.0, 1.0);
				pixels.push(sample_resolved_gradient(t, &resolved));
			}
		}

		Self { size, source_size, pixels }
	}

	#[allow(clippy::doc_lazy_continuation)]
	/// ## Conic Gradient
	/// Generates a [`ColorImage`] with a conic gradient
	///
	/// ## Usage
	/// The usage is similar to how you do it in CSS;
	/// ```rust
	/// use egui_colorimage_gradients::*;
	/// use egui::{Color32, ColorImage};
	///
	/// let size = [100, 100];
	///
	/// let colors = [
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// 	(Color32::from_rgb(177, 223, 239), None),
	/// 	(Color32::from_rgb(0, 69, 83), None),
	/// ];
	///
	/// // Usage here
	/// let image = ColorImage::conic_gradient(size, None, None, &colors);
	/// ```
	///
	/// ### Parameters
	/// * Size `[width, height]`: Determines the size of the ColorImage generated
	/// * Angle `Some(GradientAngle)`: Determines the angle of the Conic gradient.
	/// 	Leave it at `None` for an ellipse.
	/// * Position `Some(Position)`: Determines where the Conic gradient starts.
	/// 	Leave it at `None` for center.
	/// * Colors `&[(Color32, Option<u8>)]`: A list of colors with the percent of the image they take up.
	/// 	Leave it at `None` for uniform distribution.
	fn conic_gradient(
		size: [usize; 2],
		angle: Option<GradientAngle>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		// Fetch and store the size of image
		let [w, h] = size;
		let source_size = egui::vec2(w as f32, h as f32);

		// Return empty image if no colors were provided
		if colors.is_empty() {
			return Self { size, source_size, pixels: Vec::new() };
		}

		// Resolved pixels and a pixel buffer
		let resolved = resolve_color_stops(colors);
		let mut pixels = Vec::with_capacity(w * h);

		// Resolve position
		let (cx, cy) = calculate_position(position);

		// Resolve angle offset
		let angle_offset =
			if let Some(gradient_angle) = angle { calculate_angle(gradient_angle) } else { 0.0 }; // results to GradientAngle::Degree(0.0)

		// Loop through height
		for y in 0..h {
			let ny = if h > 1 { y as f32 / (h - 1) as f32 } else { 0.0 };
			let dy = ny - cy;

			// Loop through width
			for x in 0..w {
				let nx = if w > 1 { x as f32 / (w - 1) as f32 } else { 0.0 };
				let dx = nx - cx;

				// Calculate angle, apply starting offset, and normalize
				let pixel_angle = dy.atan2(dx);
				let mut t = (pixel_angle - angle_offset + PI) / (2.0 * PI);

				// Wrap around mapped value to keep it strictly between 0 and 1
				t = t.fract();
				if t < 0.0 {
					t += 1.0;
				}

				pixels.push(sample_resolved_gradient(t, &resolved));
			}
		}

		Self { size, source_size, pixels }
	}

	/// ## Blend
	/// Mixes a number of [`ColorImage`]s. Note that more than 100 would start looking slightly pale.
	///
	/// Note: I'm still experimenting with this function, and I cannot really guarantee anything.
	/// The usage is still kinda weird.
	///
	/// # Panics
	/// You cannot have inconsistent factors; Either all images get a factor, or none of them do.
	///
	/// ## Usage
	/// ```rust
	/// use egui_colorimage_gradients::*;
	/// use egui_colorimage_gradients::presets::*;
	/// use egui::{Color32, ColorImage};
	///
	/// let img = ColorImage::blend([100, 100], &[
	/// 	(sunset_glow([50, 100]), None),
	/// 	(electric_neon_blue([70, 100]), None)
	/// ]);
	/// ```
	///
	/// Those two images would be blended into one.
	///
	/// Note: Varied image sizes for parameters are allowed.
	fn blend(size: [usize; 2], images: &[(ColorImage, Option<f32>)]) -> Self {
		// Fetch and store the size of image
		let [w, h] = size;
		let source_size = egui::vec2(w as f32, h as f32);

		// Return empty image if no images were provided
		if images.is_empty() {
			return ColorImage::new(size, vec![Color32::TRANSPARENT]);
		}

		// Panic check
		let (all_some, all_none) = images
			.iter()
			.fold((true, true), |(some, none), &(_, f)| (some && f.is_some(), none && f.is_none()));
		if !(all_some || all_none) {
			panic!("[ERROR] Inconsistent factors: provide factors for all images or none.");
		}

		// Find total
		let total = images.iter().filter_map(|(_, opt)| *opt).sum();
		let total = if total > 0.0 { total } else { 1.0 };
		let uniform_factor = 1.0 / (images.len() as f32);

		// Create a Pixel Buffer
		let mut pixels = Vec::with_capacity(w * h);

		// Loop through height
		for y in 0..h {
			let ny = y as f32 / (h as f32).max(1.0);

			// Loop through width
			for x in 0..w {
				let nx = x as f32 / (w as f32).max(1.0);
				let mut blended_rgba = [0; 4];

				// Iterate through images
				for (image, factor) in images {
					// Calculate factor per image
					let factor = if all_some {
						factor.clone().get_or_insert(0.0).to_owned() / total
					} else {
						uniform_factor
					};

					// Normalized * Image res
					// Whatever the size of parameter image might be, it will be scaled
					let source_x = (nx * image.width() as f32) as usize;
					let source_y = (ny * image.height() as f32) as usize;

					let x = source_x.min(image.width().saturating_sub(1));
					let y = source_y.min(image.height().saturating_sub(1));

					// Push to temp buffer
					if let Some(&pixel) = image.pixels.get(y * image.width() + x) {
						blended_rgba[0] += (pixel[0] as f32 * factor) as u8;
						blended_rgba[1] += (pixel[1] as f32 * factor) as u8;
						blended_rgba[2] += (pixel[2] as f32 * factor) as u8;
						blended_rgba[3] += (pixel[3] as f32 * factor) as u8;
					}
				}

				// Push to pixel buffer
				pixels.push(Color32::from_array(blended_rgba));
			}
		}

		Self { size, source_size, pixels }
	}
}


/// ## FromArray
/// Note: This is just a helper trait, in order to convert an array to Color32 directly.
/// It removes the annoying conversion stuff
/// 
/// I MIGHT open a PR about this :yum:
#[doc(hidden)]
trait FromArray<T>: Sized {
	fn from_array(rgba: [u8; 4]) -> Self;
}

/// ### Helper
/// 
/// Implement `From<[u8; 4]>` for Color32
#[doc(hidden)]
impl FromArray<[u8; 4]> for Color32 {
	#[inline]
	fn from_array(rgba: [u8; 4]) -> Self {
		Color32::from_rgba_premultiplied(
			rgba[0],
			rgba[1],
			rgba[2],
			rgba[3],
		)
	}
}

/// ### Helper
///
/// Calculates the position of starting for `conic_gradient()` and `radial_gradient()` exclusively.
#[doc(hidden)]
#[inline]
fn calculate_position(position: Option<Position>) -> (f32, f32) {
	match position.unwrap_or(Position::Center) {
		Position::TopLeft => (0.0_f32, 0.0_f32),
		Position::Top => (0.5_f32, 0.0_f32),
		Position::TopRight => (1.0_f32, 0.0_f32),
		Position::Left => (0.0_f32, 0.5_f32),
		Position::Center => (0.5_f32, 0.5_f32),
		Position::Right => (1.0_f32, 0.5_f32),
		Position::BottomLeft => (0.0_f32, 1.0_f32),
		Position::Bottom => (0.5_f32, 1.0_f32),
		Position::BottomRight => (1.0_f32, 1.0_f32),
		Position::Percentage(x, y) => (x as f32 / 100.0, y as f32 / 100.0),
	}
}

/// ### Helper
///
/// Calculates the right angle (funny) for the gradient to start from
#[doc(hidden)]
#[inline]
fn calculate_angle(gradient_angle: GradientAngle) -> f32 {
	match gradient_angle {
		GradientAngle::Direction(direction) => match direction {
			Direction::TopDown => 0.0,                // 0	deg
			Direction::BottomUp => PI,                // 180	deg
			Direction::LeftToRight => FRAC_PI_2,      // 90	deg
			Direction::RightToLeft => PI + FRAC_PI_2, // 270	deg
		},
		GradientAngle::Degree(d) => d * (PI / 180.0), // Convert to radians
		GradientAngle::Radian(r) => {
			// Make sure radians are within 360 deg
			let rhs = PI * 2.0;
			let r = r % rhs;
			if r < 0.0 { r + rhs.abs() } else { r }
		}
	}
}

/// ### Helper
///
/// Resolves Option<u8> percentages into 0 to 1 bounds
#[doc(hidden)]
fn resolve_color_stops(colors: &[(Color32, Option<u8>)]) -> Vec<(Color32, f32)> {
	// No. of colors and init empty vec
	let len = colors.len();
	let mut resolved = vec![(Color32::TRANSPARENT, -1.0); len];

	// Set defined stops
	for (i, &(c, pct)) in colors.iter().enumerate() {
		resolved[i].0 = c;
		if let Some(p) = pct {
			resolved[i].1 = (p as f32 / 100.0).clamp(0.0, 1.0);
		} else if i == 0 {
			resolved[i].1 = 0.0;
		} else if i == len - 1 {
			resolved[i].1 = 1.0;
		} else {
			resolved[i].1 = 1.0 / colors.len() as f32;
		}
	}

	// Distribute undefined stops
	let mut i = 0;
	while i < len {
		if resolved[i].1 < 0.0 {
			let mut j = i;
			while j < len && resolved[j].1 < 0.0 {
				j += 1;
			}
			let start = resolved[i - 1].1;
			let end = resolved[j].1;
			let step = (end - start) / ((j - i + 1) as f32);
			resolved.iter_mut().enumerate().take(j).skip(i).for_each(|(idx, k)| {
				k.1 = start + step * ((idx - i + 1) as f32);
			});
			i = j;
		} else {
			i += 1;
		}
	}
	resolved
}

/// ### Helper
///
/// Sample pre resolved color stops
#[doc(hidden)]
#[inline]
fn sample_resolved_gradient(t: f32, stops: &[(Color32, f32)]) -> Color32 {
	if stops.len() == 1 || t <= stops[0].1 {
		return stops[0].0;
	}

	if t >= stops.last().unwrap().1 {
		return stops.last().unwrap().0;
	}

	for w_idx in 0..stops.len() - 1 {
		let (c1, t1) = stops[w_idx];
		let (c2, t2) = stops[w_idx + 1];
		if t >= t1 && t <= t2 {
			let range = t2 - t1;
			let factor = if range > 0.0 { (t - t1) / range } else { 0.0 };
			return lerp_color(c1, c2, factor);
		}
	}
	stops.last().unwrap().0
}

/// ### Helper
///
/// Linear Interpolate two colors
#[doc(hidden)]
#[inline]
fn lerp_color(c1: Color32, c2: Color32, t: f32) -> Color32 {
	#[inline(always)]
	fn lerp_c<F>(c1: Color32, c2: Color32, t: f32, c: F) -> u8
	where
		F: Fn(Color32) -> u8,
	{
		(c(c1) as f32 * (1.0 - t) + c(c2) as f32 * t) as u8
	}

	Color32::from_rgba_premultiplied(
		lerp_c(c1, c2, t, |c| c.r()),
		lerp_c(c1, c2, t, |c| c.g()),
		lerp_c(c1, c2, t, |c| c.b()),
		lerp_c(c1, c2, t, |c| c.a()),
	)
}
