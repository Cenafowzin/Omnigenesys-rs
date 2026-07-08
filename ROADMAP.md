# Omnigenesys Rust — Roadmap

## Visão geral

Migração do framework de geração procedural de Go para Rust. O objetivo é um produto de mercado: multi-engine (subprocess/FFI/WASM), 2D+3D, runtime chunk generation, e fácil de criar adapters pela comunidade.

---

## Arquitetura alvo

```
omnigenesys-rs/
├── crates/
│   ├── omnigenesys-core/        → Grid, Pipeline, Conditions, Operators (lib pura)
│   ├── omnigenesys-noise/       → Perlin 2D/3D, Simplex, Voronoi
│   ├── omnigenesys-pathfinding/ → A* 2D/3D, JPS, flow field
│   ├── omnigenesys-export/      → JSON, MessagePack, FlatBuffers
│   ├── omnigenesys-ffi/         → C ABI exports para native plugin (cdylib)
│   └── omnigenesys-wasm/        → WASM bindings (wasm-bindgen)
├── bins/
│   └── mapgen/                  → CLI subprocess (retrocompatível com o Go)
├── examples/
│   └── cornfield.rs             → Mapa example portado
├── Cargo.toml                   → Workspace
└── pipeline.schema.json         → JSON Schema do pipeline config
```

---

## Fase 1 — Paridade com Go (~13-18 dias)

O objetivo é ter o mesmo resultado que o Go: mesma pipeline JSON, mesmo output, mesma seed = mesmo mapa.

### 1.1 Core (2-3 dias)
- [ ] `GridSize` (width, height, depth — depth=1 para 2D)
- [ ] `Coord` (x, y, z — z=0 para 2D)
- [ ] `TileId = u16` com `EMPTY = 0`
- [ ] `TileRegistry` (string ↔ u16, interning no I/O)
- [ ] `Layer` (flat `Vec<TileId>`, indexação `z*W*H + y*W + x`)
- [ ] `Grid` (size, layers via `IndexMap<String, Layer>`, seed)
- [ ] `Context` (grid, rng `StdRng`, registry)
- [ ] `trait Operator: Send + Sync` com `fn execute(&self, ctx: &mut Context)`
- [ ] `enum Condition` (LayerIs, LayerNot, LayerEmpty, LayerClear, NearType, NotNearType)
- [ ] `Pipeline` (Vec de operators, execução sequencial)

### 1.2 Noise (1-2 dias)
- [ ] Perlin 2D (permutation table, fade, lerp, gradient)
- [ ] `Sampler` trait com `fn sample(&self, x: f64, y: f64) -> f64`
- [ ] `NoiseConfig` (type, scale) + `fn build(seed) -> Box<dyn Sampler>`
- [ ] Determinístico por seed (LCG para permutation shuffle)

### 1.3 Pathfinding (1-2 dias)
- [ ] `NeighborMode` enum (Four, Eight, Six, TwentySix)
- [ ] A* com `BinaryHeap`, `FxHashMap` para gScore/parent
- [ ] Heuristic: distância euclidiana
- [ ] 8 direções 2D (cardinal cost 1.0, diagonal 1.414)
- [ ] Cost function como `&dyn Fn(Coord) -> f64`
- [ ] Condition filtering por vizinho
- [ ] Path reconstruction via parent chain

### 1.4 Operators — Terrain (1 dia)
- [ ] `Fill` (preenche layer inteira com tile)
- [ ] `FillBorder` (preenche borda com espessura configurável)

### 1.5 Operators — Scatter (1 dia)
- [ ] `Scatter` (probabilidade por célula, conditions)
- [ ] `NoiseScatter` (Perlin threshold, conditions)

### 1.6 Operators — Placement (1-2 dias)
- [ ] `PlacePoint` (âncoras, offsets, conditions)
- [ ] `PlaceFixed` (retângulo uniforme com âncoras)
- [ ] `PlaceRoom` (retângulo com wall+floor)
- [ ] `PlaceStructures` (múltiplas estruturas, min_distance, avoid, max_attempts)

### 1.7 Operators — Paths (2-3 dias)
- [ ] `PathRepulsion` (layer, tile, distance, factor)
- [ ] `buildCost` (noise + repulsion → cost closure)
- [ ] `scanStructureBounds` (flood-fill, bounding boxes)
- [ ] `entryPointsFrom` (entry point no edge mais próximo)
- [ ] `PathConnect` (A* direto entre dois pontos)
- [ ] `ConnectToStructures` (scan + A* para cada estrutura)
- [ ] `BranchPaths` (múltiplos caminhos de source tiles)
- [ ] `ConnectPoints` (random walk com direct_chance)

### 1.8 Export + CLI (1-2 dias)
- [ ] JSON export (formato idêntico ao Go: width, height, seed, layers com cells)
- [ ] `ToJSON(grid, path)` com indentação
- [ ] `ToWriter(grid, writer)` compacto
- [ ] CLI `mapgen`: --config (arquivo ou stdin), --out (arquivo ou stdout)
- [ ] Parse de `PipelineConfig` JSON (serde) com dispatch de operators

### 1.9 Testes de paridade (2 dias)
- [ ] Teste: mesma seed + mesma pipeline = mesmo output Go vs Rust
- [ ] Portar mapa example (cornfield) como teste de integração
- [ ] Validar todos os operators individualmente
- [ ] Benchmark comparativo Go vs Rust

---

## Fase 2 — 3D e Voxel (~10-14 dias)

### 2.1 Generators 3D (3-4 dias)
- [ ] Perlin 3D (trilinear interpolation, 8 gradientes)
- [ ] A* 3D com NeighborMode::Six e TwentySix
- [ ] Heightmap → Voxel (noise 2D empilhado)

### 2.2 Operators 3D (3-4 dias)
- [ ] `FillVolume` (triple loop x,y,z)
- [ ] `FillShell` (6 faces em vez de 4 bordas)
- [ ] `PlacePoint3D` (anchor_z, offset_z)
- [ ] `PlaceVolumes` (caixas 3D com overlap check)
- [ ] `VolumeScatter` + `NoiseScatter3D`
- [ ] Pathfinding 3D nos operators de paths

### 2.3 Spatial Index (2-3 dias)
- [ ] `SpatialIndex` (spatial hash com buckets)
- [ ] `build_for_tile(layer, tile, cell_size)` — O(N)
- [ ] `any_within(center, distance)` — O(1) amortizado
- [ ] Integrar com Conditions (NotNearType, LayerClear)
- [ ] Benchmark: antes vs depois em volumes 100³+

### 2.4 Export 3D (1-2 dias)
- [ ] Sparse format (só células não-vazias)
- [ ] Campo `depth` no JSON config e output
- [ ] MessagePack como formato alternativo

---

## Fase 3 — Integração com engines (~6-7 dias)

### 3.1 FFI / Native Plugin (2-3 dias)
- [ ] Crate `omnigenesys-ffi` (cdylib)
- [ ] `omni_generate(config, len, &out, &out_len) -> i32`
- [ ] `omni_free(ptr, len)`
- [ ] Header C gerado automaticamente (cbindgen)
- [ ] Formato binário direto para zero-copy: `[header | layer0 tiles | layer1 tiles | ...]`

### 3.2 WASM (2 dias)
- [ ] Crate `omnigenesys-wasm` (wasm-bindgen)
- [ ] `generate(config_json: &str) -> String` (JSON in, JSON out)
- [ ] Build com `wasm-pack`
- [ ] Teste em browser (preview de mapa)

### 3.3 Adapters atualizados (2 dias)
- [ ] Atualizar adapter Unreal para usar .dll FFI (opcional, subprocess continua funcionando)
- [ ] Atualizar adapter Unity para usar .dll FFI
- [ ] Documentar: "How to build an adapter" para comunidade

---

## Fase 4 — Performance (~2-3 dias)

### 4.1 Rayon parallelism (2 dias)
- [ ] Scatter paralelo (partition por região, thread-local RNG derivado de seed+index)
- [ ] Pathfinding paralelo (múltiplos paths simultâneos em ConnectToStructures/BranchPaths)
- [ ] Benchmark: single-thread vs rayon em mapas 200×200 e volumes 100³

### 4.2 Otimizações (1 dia)
- [ ] `FxHashMap` no A* (rustc-hash, hashing mais rápido)
- [ ] Verificar autovectorization no noise sampling (flags SIMD no release build)
- [ ] Profile com `cargo flamegraph` e otimizar hotspots reais

---

## Fase 5 — Novos generators (~4-5 dias)
- [ ] 3D Cellular Automata (caves, túneis estilo Minecraft)
- [ ] Marching Cubes (voxel grid → mesh)
- [ ] 3D Voronoi (biomas volumétricos)
- [ ] SDF (Signed Distance Fields — formas orgânicas)

---

## Fase 6 — Produto (~5-7 dias)
- [ ] Interface web (visual pipeline editor, preview via WASM no browser)
- [ ] `pipeline.schema.json` (JSON Schema para validação e autocomplete)
- [ ] Documentação completa (PIPELINE.md portado + guia de adapters)
- [ ] Versionamento do formato JSON (`"version": 1` no root)
- [ ] Publicar no crates.io (`cargo add omnigenesys-core`)
- [ ] Landing page mínima do projeto

---

## Crates Rust recomendados

| Funcionalidade | Crate |
|---|---|
| RNG determinístico | `rand` + `rand_chacha` (ChaCha8Rng) |
| Serialização JSON | `serde` + `serde_json` |
| CLI args | `clap` |
| Paralelismo | `rayon` |
| Priority queue (A*) | `std::collections::BinaryHeap` |
| HashMap rápido | `rustc-hash` (FxHashMap) |
| Ordered map (layers) | `indexmap` |
| WASM bindings | `wasm-bindgen` + `wasm-pack` |
| Formato binário | `rmp-serde` (MessagePack) ou `flatbuffers` |
| C header gen | `cbindgen` |
| Profiling | `cargo-flamegraph` |

---

## Mapeamento Go → Rust

### Tipos

| Go | Rust |
|---|---|
| `grid.Cell { Type string, Metadata map[string]any }` | `TileId (u16)` + sparse metadata |
| `grid.Layer { Name, Cells [][]Cell }` | `Layer { name, cells: Vec<TileId>, size }` |
| `grid.Grid2D { Width, Height, Seed, Layers, LayerOrder }` | `Grid { size: GridSize, layers: IndexMap<String, Layer>, seed }` |
| `pipeline.Context { Grid, RNG }` | `Context { grid, rng: StdRng, registry: TileRegistry }` |
| `pipeline.Operator (interface)` | `trait Operator: Send + Sync` |
| `pipeline.Condition (structs com switch)` | `enum Condition` |
| `noise.Perlin { perm [512]int }` | `Perlin { perm: [u8; 512] }` |
| `pathfinding.Point { X, Y }` | `Coord { x: i32, y: i32, z: i32 }` |

### Operators — 1:1

| Go | Rust | Campos |
|---|---|---|
| `terrain.Fill` | `Fill` | layer, tile |
| `terrain.FillBorder` | `FillBorder` | layer, tile, thickness |
| `placement.PlacePoint` | `PlacePoint` | layer, tile, anchor_x/y/z, offset_x/y/z |
| `placement.PlaceRoom` | `PlaceRoom` | layer, x, y, width, height, floor, wall |
| `placement.PlaceFixed` | `PlaceFixed` | layer, tile, width, height, anchor, offset |
| `placement.PlaceStructures` | `PlaceStructures` | layer, structures[], min_distance, avoid_layer, avoid_type |
| `scatter.Scatter` | `Scatter` | layer, tile, chance, conditions |
| `scatter.NoiseScatter` | `NoiseScatter` | layer, tile, threshold, noise config, conditions |
| `paths.PathConnect` | `PathConnect` | layer, tile, from, to, noise_factor, noise config |
| `paths.ConnectToStructures` | `ConnectToStructures` | layer, tile, from, structures_layer, clearance, noise, repulsion |
| `paths.BranchPaths` | `BranchPaths` | source_layer, source_tile, layer, tile, branches, noise, repulsion |
| `paths.ConnectPoints` | `ConnectPoints` | layer, tile, from, to, direct_chance, max_steps, diagonal |

---

## Visão de longo prazo

```
Onirika (ecossistema)
├── Omnigenesys (módulos de procgen separados)
│   ├── omni-mapgen      ← mapas/terreno (este projeto)
│   ├── omni-dialog      ← diálogos procedurais
│   ├── omni-npc         ← comportamento/spawning de NPCs
│   ├── omni-loot        ← economia/drops procedurais
│   └── omni-quest       ← missões procedurais
│
├── Dream Box (core mínimo de engine — futuro)
│   ├── ECS + Game loop + Event bus + Plugin API
│   └── cada módulo Omni vira plugin nativo
│
└── Lost Fields (jogo — prova tudo em produção)
```

Cada módulo funciona standalone com qualquer engine (subprocess/FFI/WASM) hoje, e vira plugin nativo do Dream Box amanhã. O Dream Box não é projetado top-down — emerge bottom-up dos padrões reais descobertos construindo os módulos Omni.
