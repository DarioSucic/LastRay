# LastRay

### Description
Yet another ray tracer in Rust taking inspiration from [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 

### Example Images
<div>
  <img src="https://raw.githubusercontent.com/DarioSucic/LastRay/master/out_nice_denoised.png" width="320" height="320">
  <img src="https://raw.githubusercontent.com/DarioSucic/LastRay/master/spheres.png" width="320" height="320">
</div>

### How to run
The scene is currently hard-coded in `main.rs`, and changing constantly. Running the following command should generate some image:
```
cargo run --release
```

### Short term goals:
- [x] Support sphere geometry
- [x] Add basic materials (Diffuse, Metal, Dielectric)
- [x] Support Multi-threaded rendering
- [x] Support triangle geometry
- [x] Add loading from basic file formats (.obj, .mtl, etc.)
- [x] Vectorize intersection testing (targeting AVX2)
- [ ] Add acceleration structure (BVH)
- [x] Support light sources (basic)

### Longer term goals:
- [ ] Add animation support
- [ ] Implement more physically based rendering features
- [ ] More advanced materials
- [ ] Create or implement existing scene format
- [ ] Add texture support