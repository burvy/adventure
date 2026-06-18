# How to Create a New Object
1. Under the `objects/` folder, create a module (folder)
2. Register that module in `objects/mod.rs`
3. Create a resource that has reusable assets to initialize with `FromWorld`
4. Implement `ObjectBlueprint` so it can be spawned from a config
5. Attach `Interactable` to the object if it has click logic from the hero
6. Register systems in the object plugin if it needs startup or update logic

# How to Modify an Object
- Edit `assets/layouts/lobby.ron` if the object already exists and you want new pos/rot/scales/counts/regions
- Edit `definition.rs` or `logic.rs`
`definition.rs` controls components, materials, colliders
`logic.rs` controls logic, update/click behavior

# Layout
- Fixed placements like hero/register/map are in `assets/layouts/lobby.ron`
- Repeated groups like Ferris waves are in `ferris_spawn_groups`
- Random repeated objects like paper are in `paper_scatter_regions`
- `src/build/layout.rs` parses the layout file and turns it into runtime spawn data

# Textures
- Textures can be from `assets/images`, baked into the GLB scene, or coded in
- `Oube`, `Paper`, and the hero arm have code created materials
- Change the textures by updating the model/materials in `assets`
- Then assign to `StandardMaterial`

# Expansion
Creating a new object requires:
- new object module
- asset resource
- `ObjectBlueprint`
- `Interactable` if it is interactable
- plugin registration in `objects/mod.rs`

If the object is layout driven:
- add placements to `assets/layouts/lobby.ron`

If the object repeats:
- add fixed repeated transforms or random scatter regions to the layout file
- do not copy `Paper` or Ferris just to get repeated spawning unless the spawn behavior itself is different
