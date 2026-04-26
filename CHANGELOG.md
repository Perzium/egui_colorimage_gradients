# 0.2.0
API Change, made it similar to CSS

Added:
- `blend()` function: Mixes any number of ColorImages into one.
- Backported `Direction` enum.
- `Shape` and `Position` enum.

Modified:
- Trait `ColorImageGradient` to have three new gradient functions instead of one.
- All presets and tests to match the new API.
- `GradientAngle` enum.

Removed:
- Struct `Gradient` and it's `impl`s.

# 0.1.0
Initial release

Added:
- Trait `ColorImageGradient` and the `gradient()` fn.
- Struct `Gradient` and implemented `Default` for it.
- Enums `GradientAngle` and `GradientType`.
- Tests, to ensure if the gradients work.
- Quick debug function to look at how the gradient would look like if rendered (commented out on tests.rs).
- 13 new presets.