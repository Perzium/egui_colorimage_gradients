# egui_colorimage_gradients

EGUI ColorImage Gradients is a simple, lightweight crate to add to any of your projects.

It is made to improve the current ColorImage implementation, by providing extra features, by supporting gradient generation.

## Getting Started

Add the crate if you haven't yet:
```toml
egui_colorimage_gradients = "0.1.0"
```

Simply import the [`ColorImageGradient`] trait to extend your ColorImage functionality.

```rust
use egui_colorimage_gradients::ColorImageGradient;
```

The Usage tab goes more in depth about how to use this.

## Usage

There are two ways to create a [`gradient`]: 
- Manually create your gradient.
- Use one of my presets.

### Manually create your gradient:
Create a gradient by defining every field.

```rust
use egui::Color32;
use egui_colorimage_gradients::{Gradient, GradientType, GradientAngle};

let gradient = Gradient {
	kind: GradientType::Linear,
	colors: [Color32::BLACK],
	angle: GradientAngle::Radian(0.0)
};
```

and from there, you can create your ColorImage using the [`gradient`] fn:

```rust
use egui::ColorImage;
use egui_colorimage_gradients::ColorImageGradient;

let color_image = ColorImage::gradient([6, 9], gradient);
```

Note: The `size` determines the pixels of your ColorImage.

### Use one of my presets:

```toml
egui_colorimage_gradients = { version = "0.1.0", features = ["presets"]}
```

Note: You MUST to enable the feature flag for presets to work.

```rust
use egui::ColorImage;
use egui_colorimage_gradients::{
	ColorImageGradient,
	Gradient, GradientType, GradientAngle,
	presets::*
};

let color_image = ColorImage::gradient([4, 20], electric_neon_blue());
```

Note: Presets are in the experimental stages of sorts, I might add or remove existing ones, change the way they behave, etc.
Please look out for those changes.

## Gradient Types
I put together a small table, just to show how each type of gradient is like.

|    Type    |                   Description                    | Angle Support |
| :--------: | :----------------------------------------------: | :-----------: |
| **Linear** | A smooth transition along a straight line.       |   Full 360°   |
| **Radial** | Colors radiate outward from the center pixel.    |    Ignored    |
| **Conic**  | Colors go in an angular swirl around the center. |    Ignored    |

Note: Thank you for using my crate.

If you have any issues or queries, you can reach me at [Issues](https://github.com/Perzium/egui_colorimage_gradients/issues) or [Discussions](https://github.com/Perzium/egui_colorimage_gradients/discussions).