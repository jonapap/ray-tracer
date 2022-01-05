# Ray Tracer Engine

![](./renders/book1.png)

This is a ray tracer engine based on the books [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
and [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html#rectanglesandlights) 
by Peter Shirley.

The engine is written in Rust, adapted from the C++ version of the book. A few changes have been made from the provided
implementation:
  - Multi-Threading support
  - SAH (Surface Area Heuristic) for building the BVH tree
  - Various optimizations for improving the run-time performance

Future plans:
  - Add support for other objects
  - Add support for more textures
  - Improve performance using Monte Carlo and other methods

## Build and Run

First, install the latest stable version of Rust. Next, in this folder, run:

```bash
cargo run --release -- cornell-box
```

This will render the Cornell Box scene and output it under `./out.png`. Simply open the image using your preferred
image viewer.

Other scenes are provided. Simply run the following command to see all the possible command line arguments:
```bash
cargo run --release -- --help
```
