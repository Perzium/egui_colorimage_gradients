//	## Internal tests
//	Not necessary for you to run it, but if you wanna, go ahead.
//
//	If you do, and need to check how the gradient might look, dump to ppm
//	using a function I used during development.
//
//	```rust
//	use std::{fs::File, io::{Write, Result}};
//
//	fn dump_to_ppm(img: &egui::ColorImage, filename: &str) -> Result<()> {
//		let mut file = File::create(filename)?;
//
//		writeln!(file, "P3")?;
//		writeln!(file, "{} {}", img.size[0], img.size[1])?;
//		writeln!(file, "255")?;
//
//		for pixel in &img.pixels {
//			writeln!(file, "{} {} {}", pixel.r(), pixel.g(), pixel.b())?;
//		}
//
//		Ok(())
//	}
//	```
//
//	Note: This function needed standard, but I tried my best to keep the crate
//	no standard.
//	Therefore, I will not be adding it myself, but if you wanted to try it,
//	then go for it.
//
//
//	Run it as a test, it works like a script for some reason:
//	```rust
//	#[test]
//	fn trial() {
//		use crate::ColorImageGradient;
//
//		let gradient = candy();
//		let img = egui::ColorImage::gradient([100, 100], gradient);
//
//		let _ = dump_to_ppm(&img, "trial.ppm");
//	}
//	```
//
//	Note: I prevented using the `image` crate, as it is QUITE HEAVY.
//	Just use GIMP to open a `.ppm` file, or find another application which does.
use crate::*;
use egui::{Color32, ColorImage};

/// Tests the Linear gradient and rotation
///
/// Test Details:
/// - Kind: Linear
/// - Colors: 2; Black & White
/// - Angle: 45deg. Always rotates anti-clockwise.
#[test]
fn test_linear_rotated_weights() {
	// Verifies 45 degree stretch and weight distribution
	let size = [10, 10];
	let colors = &[(Color32::BLACK, None), (Color32::WHITE, None)];

	let img = ColorImage::linear_gradient(size, GradientAngle::Degree(45.0), colors);

	// Math for [`ColorImage`] under the hood is such that
	// `pixels: Vec<Color32>` of `w * h` size
	// to reach a pixel, we do `pixels[y * w + x]`

	// Corner (top left) must be pure first color
	assert_eq!(img.pixels[0], Color32::BLACK);
	// Corner (bottom right) must be pure last color
	assert_eq!(img.pixels[99], Color32::WHITE);
	// Corner top right and bottom left grey
	assert_eq!(img.pixels[9].b(), 127);
	assert_eq!(img.pixels[90].b(), 127);
}

/// Tests the radial gradient
///
/// Test Details:
/// - Kind: Radial
/// - Colors: 3; Red, Green & Blue
/// - Angle: Ignored; Anything will do.
#[test]
fn test_radial_multi_color() {
	// Verifies distance based sampling and multi color boundaries
	let size = [500, 500];
	let colors = &[(Color32::RED, None), (Color32::GREEN, None), (Color32::BLUE, None)];

	let img = ColorImage::radial_gradient(size, None, None, colors);

	// Exact center must be the first color
	// Center: 500 x 500 img -> 250, 250 mid -> 250 * 500 + 250 = 125250
	// Approx color, as code cannot reach 100% with radial
	assert!(img.pixels[125250].r() >= 250);
	// Corners are furthest points (t=1.0), must be the last color
	assert_eq!(img.pixels[0], Color32::BLUE);
}

/// Tests the conic gradient
///
/// Test Details:
/// - Kind: Conic
/// - Colors: 2; Black & White
/// - Angle: Not ignored; Determines where the gradient starts.
#[test]
fn test_conic_wrap() {
	let size = [100, 100];
	let colors = &[(Color32::BLACK, None), (Color32::WHITE, None)];

	let img = ColorImage::conic_gradient(size, None, None, colors);

	// Wrap area
	// Image: 100 * 100
	// -> Start: 49 * 100 = 4900
	// -> End: 50 * 100 = 5000
	assert_eq!(img.pixels[4900].g(), 0);
	assert!(img.pixels[5000].g() >= 250);
}

// Note: I did test v0.2.0 with ppm mapping instead of writing test functions
// it gets boring innit
