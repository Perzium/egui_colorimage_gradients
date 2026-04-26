# egui_colorimage_gradients

EGUI ColorImage Gradients is a simple, lightweight crate to add to any of your projects.

It is made to improve the current ColorImage implementation, by providing extra features,
by supporting gradient generation.


## Getting Started

Add the crate if you haven't yet:
```toml
egui_colorimage_gradients = "0.2.0"
```
NOTE: Users below `egui 0.34.0` need to enable a feature flag:
```toml
egui_colorimage_gradients = { version = "0.2.0", features = ["below_egui_0_34_0"] }
```

Simply import the [`ColorImageGradient`] trait to extend your ColorImage functionality.

```rust
use egui_colorimage_gradients::ColorImageGradient;
```

The Usage tab goes more in depth about how to use this.

## Usage

There are two ways to create a gradient:
- Manually create your gradient.
- Use one of my presets.

### Manually create your gradient:
Create a gradient by defining making a list of colors.

```rust
use egui::{Color32, ColorImage};
use egui_colorimage_gradients::*;

// Note: `u8`s within `Some()` must add up to 100.
let colors = [
	(Color32::from_rgb(0, 69, 83), Some(30)),
	(Color32::from_rgb(177, 223, 239), Some(40)),
	(Color32::from_rgb(0, 69, 83), Some(30)),
];

// From there, you can create your ColorImage using any one of the gradient functions:

let color_image = ColorImage::linear_gradient(
	[6, 9],
	GradientAngle::Direction(Direction::TopDown),
	&colors
);
```

Note: The `size` determines the pixels of your ColorImage.

### Use one of my presets:

Note: You MUST to enable the feature flag for presets to work.
```toml
egui_colorimage_gradients = { version = "0.2.0", features = ["presets"] }
```

```rust
use egui::ColorImage;
use egui_colorimage_gradients::presets::*;
use egui_colorimage_gradients::*;

let color_image = electric_neon_blue([100, 100]);
```

### Blend function
The `blend()` function is still in early stages. It works, but ehh.

```rust
use egui::ColorImage;
use egui_colorimage_gradients::presets::*;
use egui_colorimage_gradients::*;

// Blends these two [`ColorImage`]s.
let mixed_gradient = ColorImage::blend([100, 100], [
	(electric_neon_blue(69, 420), None),
	(sunset_glow(911, 67), None),
]);
```

And the results look somewhat like this:
|                  Electric Neon Blue                  |               Sunset Glow              |     If they had a child    |
| :--------------------------------------------------: | :------------------------------------: | :------------------------: |
| ![electric_neon_blue](assets/electric_neon_blue.png) | ![sunset_glow](assets/sunset_glow.png) | ![mixed](assets/mixed.png) |

Like I mentioned earlier, the blend function is still in early stages, and will get a massive refactor.

Note: Presets are in the experimental stages of sorts, I might add or
remove existing ones, change the way they behave, etc.
Please look out for those changes.

Note: Thank you for using my crate.
If you have any issues or queries, you can reach me at
[Issues](https://github.com/Perzium/egui_colorimage_gradients/issues) or
[Discussions](https://github.com/Perzium/egui_colorimage_gradients/discussions).