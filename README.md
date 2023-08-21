# voxel

a simple voxel game "engine" written in Rust, using
[Bevy](https://bevyengine.org/)

![screenshot](https://github.com/therealnv6/voxel/blob/main/.assets/screenshot1.png)

# Roadmap

- [x] 3D chunk registry
- [x] 3D noise generation
- [x] CPU "frustum" culling
  - [discovery](https://github.com/therealnv6/voxel/blob/4d066d7b06bb6bd9b358d3f9c97532305b74026e/src/chunk/events/discovery.rs#L61)
  - ~~unloading~~ (todo)
- [ ] LOD
  - ~~Face culling is completely broken in LOD, which means it's basically
    unusable. Haven't really taken the time to look into this.~~
  - This is completely disabled for now. It accessed the ChunkRegistry mutably
    within the chunk discovery function, which had to be changed due to
    slowdowns when processing the chunk discovery queue. We could perhaps
    actively change the LOD of the loaded chunks instead
    (`Query<Entity, With<LoadedChunk>>` should suffice).
- [ ] Good performance.
  - Performance is "acceptable" at best. It runs fine at 8x6 discovery radius,
    but you can clearly see the unrendered chunks which... is very suboptimal.
    We can hide this with fog, but that's cheating.
- [ ] Face culling
  - [x] Per-chunk occlusion culling (cpu-based)
  - [ ] Neighboring chunk occlusion culling (cpu-based)
    - This is going to need some tinkering. The meshing happens on another
      thread which does not have access to the registry, so we'd have to get the
      adjacent chunks their voxels, and pass them into the thread. It's easily
      doable but I'm too lazy right now.
- [ ] Biome Generation
  - Low priorty. Currently, we're just generating random "canyons" with 3D
    simplex noise, barely processed.
