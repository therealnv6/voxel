# voxel

a simple voxel game "engine" written in Rust, using
[Bevy](https://bevyengine.org/)

[[https://github.com/therealnv6/voxel/blob/master/.assets/screenshot.png|screenshot]]

# Roadmap

- [x] 3D chunk registry
- [x] 3D noise generation
- [ ] Good performance.
  - Performance is "acceptable" at best. It runs fine at 8x6 discovery radius,
    but you can clearly see the unrendered chunks which... is very suboptimal.
    We can hide this with fog, but that's cheating.
- [ ] Full occlusion culling
  - [x] Per-chunk occlusion culling (cpu-based)
  - [ ] Neighboring chunk occlusion culling (cpu-based)
    - This is going to need some tinkering. The meshing happens on another
      thread which does not have access to the registry, so we'd have to get the
      adjacent chunks their voxels, and pass them into the thread. It's easily
      doable but I'm too lazy right now.
- [ ] Biome Generation
  - Low priorty. Currently, we're just generating random "canyons" with 3D
    simplex noise, barely processed.
