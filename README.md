# LastRay

### Description
Yet another ray tracer in Rust taking inspiration from [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 

### How to run
Since the scene is currently hard-coded in `main.rs`, the leftmost example image can be generated without configuration by running:
```
cargo run --release
```

### Short term goals:
- [x] Support sphere geometry
- [x] Add basic materials (Diffuse, Metal, Dielectric)
- [x] Support Multi-threaded rendering
- [x] Support triangle geometry
- [ ] Add acceleration structure (BVH)
- [x] Add loading from basic file formats (.obj, .mtl, etc.)
- [ ] Create or implement existing scene format
- [ ] Add texture support

### Long term goals:
- [ ] Add animation support
- [x] Vectorize intersection testing (targeting SSSE3 or AVX2)
- [ ] Implement more physically based rendering features
- [ ] More advanced materials

### Example Images
<div>
  <img src="https://raw.githubusercontent.com/DarioSucic/LastRay/master/out.png" width="320" height="320">
  <img src="https://raw.githubusercontent.com/DarioSucic/LastRay/master/spheres.png" width="320" height="320">
</div>
