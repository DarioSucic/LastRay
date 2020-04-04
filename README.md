# LastRay

### Description
Yet another ray tracer in Rust taking inspiration from [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 

### How to run
Since the scene is currently hard-coded in `main.rs`, the output image can be generated without configuration by running:
```
cargo run --release
```

### Short term goals:
- [x] Support sphere geometry
- [x] Add basic materials (Diffuse, Metal, Dielectric)
- [x] Support Multi-threaded rendering
- [ ] Support triangle geometry
- [ ] Add acceleration structure (BVH)
- [ ] Add loading from basic file formats (.obj, .mtl, etc.)
- [ ] Create or implement existing scene format

### Long term goals:
- [ ] Add animation support
- [ ] Vectorize intersection testing (targeting SSSE3 or AVX2)
- [ ] Implement more physically based rendering features
- [ ] More advanced materials

### Example image (1080x1080, 128 samples per pixel)
<img src="https://raw.githubusercontent.com/DarioSucic/LastRay/master/out.png" width="320" height="320">
