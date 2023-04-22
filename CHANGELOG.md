# Change Log

## [0.0.7] - 2023-04-22

## Added
- Rays, and ray-object intersections.
- First primitive shape: the sphere!

## [0.0.6] - 2023-04-14

## Added
- Translate, scale, rotate,and shear matrix transformations.

## Changed
- Programme now outputs a clock instead of a trajectory plot.

## [0.0.5] - 2023-04-13

## Added
- Extra matrix functions (cofactors, determinants, inverse).

## Changed
- Refactored code yet again. Completely removed generics from matrices, and gone back to the original plan of using a flat vector. Use traits to handle size specifics rather than struct aliases and a size generic.

## [0.0.4] - 2023-04-08

### Changed
- Refactored code. Type generic removed to greatly simplify (since all calculations will likey be in f64 anyway). As matrices are always square, reduced size generic down to one.

## [0.0.3] - 2023-04-08

### Added
- Basic matrix operations

### Changed
- `Metric` struct renamed to`Tuple`, `math.rs` file renamed to suit, affected files modified.

## [0.0.2] - 2023-04-03

### Added
- Colour and canvas modules completed. We can now draw things!
- Integrated ability to export in formats bmp, gif, ico, jpg, jpeg, pam, png, ppm, tiff, and tga using the `image` crate.
- Export format autodetected from path.

### Changed
- Program now outputs a hardcoded graph.