#![deny(clippy::all, missing_docs)]
#![allow(clippy::tabs_in_doc_comments, clippy::doc_lazy_continuation)]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
//! egui_colorimage_gradients = "0.2.2"
//! ```
//! NOTE: Users below `egui 0.34.0` need to enable a feature flag:
//! ```toml
//! egui_colorimage_gradients = { version = "0.2.2", features = ["below_egui_0_34_0"] }
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

#[cfg(feature = "presets")]
/// Pre-made presets for quick gradients.
///
/// I wouldn't recommend most of them.
pub mod presets;

#[cfg(test)]
/// ## Tests
/// Just a few tests to check if it runs.
mod tests;

// Use whichever one based on features
#[cfg(not(feature = "below_egui_0_34_0"))]
pub use egui::Direction;
#[cfg(feature = "below_egui_0_34_0")]
pub use egui_gradients_core::Direction;
pub use egui_gradients_core::{GradientAngle, Position, Shape};

// Use for simplification
use egui::{Color32, ColorImage};
use egui_gradients_core::{conic_gradient, FromArray, linear_gradient, radial_gradient};

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
		direction: GradientAngle,
		colors: &[(Color32, Option<u8>)],
	) -> Self;

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
	fn radial_gradient(
		size: [usize; 2],
		shape: Option<Shape>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self;

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
	) -> Self;

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
	fn blend(size: [usize; 2], images: &[(ColorImage, Option<f32>)]) -> Self;
}

impl ColorImageGradient for ColorImage {
	fn linear_gradient(
		size: [usize; 2],
		gradient_angle: GradientAngle,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		linear_gradient(size, gradient_angle, colors)
	}

	fn radial_gradient(
		size: [usize; 2],
		shape: Option<Shape>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		radial_gradient(size, shape, position, colors)
	}

	fn conic_gradient(
		size: [usize; 2],
		angle: Option<GradientAngle>,
		position: Option<Position>,
		colors: &[(Color32, Option<u8>)],
	) -> Self {
		conic_gradient(size, angle, position, colors)
	}

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
