#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// There are 3 options;
///
/// ### Direction:
/// Specify one of four cardinal directions.
///
/// ### Degree:
/// Angle of gradient in degree; Will be clamped within -360 to 360.
///
/// ### Radian:
/// Angle of gradient in radian; Will be clamped within -π to π.
///
/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
/// If you prefer to see clockwise rotations, just provide the negative of your value.
///
/// Note: `GradientAngle` does not implement `Eq` like other enums in this crate,
/// as `Eq` cannot be implemented for `f32`s as of now.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum GradientAngle {
	/// ### Direction:
	/// Angle of gradient restricted to the four cardinal directions.
	Direction(crate::Direction),

	/// ### Degree:
	/// Angle of gradient in degree; Will be clamped within -360 to 360.
	///
	/// Starts from Left
	///
	/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
	/// If you prefer to see clockwise rotations, just provide the negative of your value.
	Degree(f32), // Clamp within -360.0 to 360.0

	/// ### Radian:
	/// Angle of gradient in radian; Will be clamped within -π to π.
	///
	/// Starts from Left
	///
	/// Note: Rotations work in **ANTI-CLOCKWISE**, so be aware when you give inputs.
	/// If you prefer to see clockwise rotations, just provide the negative of your value.
	Radian(f32), // Clamp within -π to π
}

/// ## Shape
/// You can select one of two shapes, similar to CSS.
///
/// Note: Defaults to Ellipse.
///
/// ### Ellipse:
/// Gradient's radius changes based on the width and height of the image.
///
/// ### Circle:
/// Gradient's radius is fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Shape {
	/// ### Ellipse:
	/// Gradient's radius changes based on the width and height of the image.
	Ellipse,

	/// ### Circle:
	/// Gradient's radius is fixed.
	Circle,
}

/// ## Position
/// You can select from Center, or use a custom position by Percentage.
///
/// Note: Position implements `From` for `f32`s within 0 and 1, aswell as `u8`s within 0 and 255.
///
/// ### Center:
/// The gradient starts from the middle.
///
/// Equivalent to `(at 50% 50%, ..)` in CSS.
///
/// ### Percentage:
/// The gradient starts from anywhere within 0% to 100%.
///
/// Note: The percentages **WILL** get clamped within 0 to 100.
/// Please make sure your values are within 0 to 100.
///
/// `(u8, u8)` maps to x% from left, y% from top.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Position {
	/// ### TopLeft:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 0% 0%, ..)` in CSS.
	TopLeft,

	/// ### Top:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 50% 0%, ..)` in CSS.
	Top,

	/// ### TopRight:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 100% 0%, ..)` in CSS.
	TopRight,

	/// ### Left:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 0% 50%, ..)` in CSS.
	Left,

	/// ### Center:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 50% 50%, ..)` in CSS.
	Center,

	/// ### Right:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 100% 50%, ..)` in CSS.
	Right,

	/// ### BottomLeft:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 0% 100%, ..)` in CSS.
	BottomLeft,

	/// ### Bottom:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 50% 100%, ..)` in CSS.
	Bottom,

	/// ### BottomRight:
	/// The gradient starts from the middle.
	///
	/// Equivalent to `(at 100% 100%, ..)` in CSS.
	BottomRight,

	/// ### Percentage:
	/// The gradient starts from anywhere within 0% to 100%.
	///
	/// Note: The percentages **WILL** get clamped within 0 to 100.
	/// Please make sure your values are within 0 to 100.
	///
	/// `(u8, u8)` maps to x% from left, y% from top.
	Percentage(u8, u8),
}

impl From<f32> for Position {
	/// From a `f32` factor to `Position`.
	fn from(factor: f32) -> Self {
		Position::Percentage(
			(100.0 * factor).clamp(0.0, 100.0) as u8,
			(100.0 * factor).clamp(0.0, 100.0) as u8,
		)
	}
}

impl From<(f32, f32)> for Position {
	/// From `f32` factors to `Position`.
	fn from(factors: (f32, f32)) -> Self {
		Position::Percentage(
			(100.0 * factors.0).clamp(0.0, 100.0) as u8,
			(100.0 * factors.1).clamp(0.0, 100.0) as u8,
		)
	}
}

impl From<u8> for Position {
	/// From a `u8` factor to `Position`.
	fn from(factor: u8) -> Self {
		Position::Percentage(
			(100.0 * (factor as f32 / 255.0)) as u8,
			(100.0 * (factor as f32 / 255.0)) as u8,
		)
	}
}

impl From<(u8, u8)> for Position {
	/// From `u8` factors to `Position`.
	fn from(factors: (u8, u8)) -> Self {
		Position::Percentage(
			(100.0 * (factors.0 as f32 / 255.0)) as u8,
			(100.0 * (factors.1 as f32 / 255.0)) as u8,
		)
	}
}

/// ## Direction
/// One of the four cardinal directions.
///
/// Note: This does not exist in versions below egui 0.34.0.
/// I have ported them back for those versions only.
///
/// So, please enable the feature on cargo.
#[cfg(feature = "below_egui_0_34_0")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Direction {
	/// ### LeftToRight
	/// Create a gradient from left to right.
	LeftToRight,

	/// ### RightToLeft
	/// Create a gradient from right to left.
	RightToLeft,

	/// ### TopDown
	/// Create a gradient from top to bottom.
	TopDown,

	/// ### BottomUp
	/// Create a gradient from bottom to top.
	BottomUp,
}

#[cfg(feature = "below_egui_0_34_0")]
impl Direction {
	/// Checks if a direction is horizontal.
	#[inline(always)]
	pub fn is_horizontal(self) -> bool {
		match self {
			Self::LeftToRight | Self::RightToLeft => true,
			Self::TopDown | Self::BottomUp => false,
		}
	}

	/// Checks if a direction is vertical.
	#[inline(always)]
	pub fn is_vertical(self) -> bool {
		match self {
			Self::LeftToRight | Self::RightToLeft => false,
			Self::TopDown | Self::BottomUp => true,
		}
	}
}