# voxel

a simple voxel game "engine" written in Rust, using
[Bevy](https://bevyengine.org/)

![screenshot](https://github.com/therealnv6/voxel/blob/main/.assets/screenshot1.png)

# Roadmap

- [x] 3D chunk registry
- [x] 3D noise generation
- [x] CPU "frustum" culling
  - [discovery](https://github.com/therealnv6/voxel/blob/4d066d7b06bb6bd9b358d3f9c97532305b74026e/src/chunk/events/discovery.rs#L61)
  - [unloading](https://github.com/therealnv6/voxel/blob/7bb1704d12a0f1bf77acc6bdcc87e483758c5a0e/src/chunk/discovery.rs#L75)
- [ ] LOD [^3]
- [ ] Good performance.
  - Currently, performance scales pretty bad. 8x4x8[^2] discovery radius rather
    easily achieves framerates of ~1500[^1], whereas 8x8x8 gets around ~700[^1],
    and 12x12x12 gets ~130[^1]. 6x6x6[^4] discovery radius manages to achieve
    ~800. A big part here is still not having LOD implemented, and not culling
    the occluded faces that are only occluded by the adjacent chunk(s).
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

[^1]:
    This is based on my personal computer; i7 10700k, RTX 3070, 16 GB @3600
    MHz

[^2]:
    8x4x8 in a 18x18x18 chunk hierarchy; meaning 8 chunks _ 4 chunks _ 8
    chunks _ 18 voxels _ 18 voxels \* 18 voxels = 3,686,400 voxels. [^5]

[^3]:
    Face culling is completely broken in LOD, which means it's basically
    unusable. Haven't really taken the time to look into this.

[^4]:
    6x6x6 in a 32x32x32 chunk hierarchy: meaning 6 chunks \* 6 chunks \* 6
    chunks \* 32 voxels \* 32 voxels \* 32 voxels = 7077888 voxels. [^5]

[^5]:
    This is worst case scenario, which can never really happen due to face
    culling.
