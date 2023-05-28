# Change Log

## [0.0.14] - 2023-05-29

## Added
- Materials: basic radial pattern.
- Reflections.
- Basic refraction.
- UV manifold for spheres. Just be reminded that the resulting uv map should not be rotated - the object itself should.

## Changed
- Experimenting with jitter. Only some success on the gradient pattern.
- Added borrows in places to make debugging a bit easier. Maybe will slightly improve performance.
- Precomputed data now chains off Intersections instead of Intersection.
- Objects can be toggled whether or not to cast a shadow (shadows cast by default).

## [0.0.13] - 2023-05-06

## Added
- Materials: basic patterns - checkers, stripes, rings, gradient, solid.

## Changed
- Refactored materials to remove colour since a solid pattern can do the same thing. Controlling colour via patterns allows for more advanced features down the line like nested and blended patterns.

## Fixed
- The preset for the colour blue had the blue channel set to 0.1. It has now been corrected to 1.0.

## [0.0.12] - 2023-05-05

## Added
- Primitive: planes.

## Changed
- Reorganised file structure due to increasing number of files.

## Fixed
- Having planes in the scene caused the renderer to crash due to a weird issue with the colour_at() method. The function tested if intersections were greater than 0, and intersections is a vector of hits, so logically if intersections is greater than 0, then there must be a hit. Apparently with planes there can be intersections without hits. How that makes sense I have no idea.

## [0.0.11] - 2023-05-02

## Added
- Cast shadows. This will be upgraded to ambient occlusion in time.

## [0.0.10] - 2023-04-30

## Added
- Cameras and views.
- World construct and scenes.
- Precomputed colour data.
- Extra operator for colours, and greyscale preset.
- Transform builder, which makes setting up complex transforms easier.

## Changed
- Intersections made publicly accessible.

## Fixed
- Reintroduced Tuple methods to_point() and to_vector() to resolve 64-bit floating point issues that caused the w-component of Tuples to be off by 1.0e-12 or something crazy like that, which ultimately breaks the renderer.

## [0.0.9] - 2023-04-26

## Added
- Surface normals for spheres.
- Rounding methods for tuple and colour testing.
- Reflections.
- Point lights.
- Basic materials.
- Phong shading.

## Changed
- Rounding method for matrix testing now made public.

## [0.0.8] - 2023-04-25

## Added
- Archive directory containing output images from the 'Putting It Together' sections. Dumped affected files from before refactoring to `nalgebra`.

## Changed
- Refactored Matrix **yet again** to use highly optimised Matrix library from `nalgebra`. Hopefully this should reduce Big-O complexity. Also allows Copy trait. I could also have reverted to my v2 matrix file which uses a single generic and also allows the Copy trait (as it doesn't Vec, so doesn't allocate to heap), but it would be better to use an optimised library. So now the 'matrix' file is essentially nothing but tests.
- Modified functionality of intersections to find hits directly from Objects rather than Primitives.

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