# Pong — Rust + Bevy

A complete keyboard-controlled Pong game with a plain Rust rules crate and a Bevy 0.20 desktop adapter using Avian 0.8 physics.

- Solo play against a speed-limited computer, or two players on one keyboard.
- First to seven, a short countdown between rallies, pause, restart, and rematches.
- Paddle impact position changes the bounce angle; rallies gradually speed up.
- Fixed 120 Hz Avian simulation, continuous collision detection, and interpolated rendering.
- Automatic pause when the window loses focus.
- All visuals use rectangle meshes and Bevy's default font. No asset downloads are needed.

## Run

Open the repository in RustRover and run `src/main.rs`, or run this in the repository terminal:

```powershell
cargo run -p pong
```

For an optimized build:

```powershell
cargo run -p pong --release
```

The first release build compiles Bevy separately and can take a while. On Windows, Rust's MSVC toolchain requires the Visual Studio C++ build tools and a Windows SDK. This project was developed with Rust 1.99.1, Bevy 0.20.0, and Avian 0.8.0; `Cargo.lock` records the resolved dependencies.

The app selects DirectX 12 by default on Windows after a Vulkan device-loss failure on the development machine. This is an app setting; it does not change Windows or your graphics driver. An explicit `WGPU_BACKEND` environment variable overrides the selection. Shader pipelines compile synchronously at startup so the court does not initially appear in pieces.

The shapes use `Mesh2d` and `ColorMaterial`: the `Sprite` path produced incorrect sizes/colors on the tested Bevy 0.20 setup. The graphics test also checks the pixels of both paddles and the ball to catch missing game pieces.

## Controls

| Key | Action |
| --- | --- |
| Space | Start, pause, resume, or play again after a win |
| W / S | Move the left paddle |
| Up / Down | Move the left paddle in solo mode; the right paddle in two-player mode |
| Tab | Switch solo/two-player mode and reset the match |
| R | Reset scores and return to the ready screen, keeping the selected mode |
| Esc | Quit |

The game starts in solo mode. Opposite movement keys cancel each other. Returning to the window does not resume automatically: press Space when ready.

## Project map

```text
pong/
├── src/                         Desktop host: Bevy and Avian
│   ├── main.rs                  Entry point
│   ├── desktop_app.rs           DesktopApp and window/renderer configuration
│   └── game/
│       ├── mod.rs               Module declarations and exports
│       ├── composition/
│       │   ├── pong_plugin.rs  Plugin wiring and system ordering
│       │   ├── match_driver.rs  Binds input and fixed ticks to PlayMatch + AvianCourt
│       │   ├── presentation_bindings.rs  Supplies core values to keyboard and HUD
│       │   ├── court_setup.rs  CourtSetup: startup assembly
│       │   ├── paddle_factory.rs  Complete paddle creation
│       │   ├── ball_factory.rs    Complete ball creation
│       │   └── tests.rs         Factory contracts without game startup
│       ├── infrastructure/
│       │   ├── avian/
│       │   │   ├── avian_court.rs   PongPhysics adapter: physical state access
│       │   │   ├── paddle_contacts.rs  Collision events -> application calls
│       │   │   ├── previous_position.rs  Fixed-tick position snapshots
│       │   │   ├── body_interpolation.rs  Avian positions -> Bevy transforms
│       │   │   ├── avian_plugin.rs  Engine configuration and schedule gating
│       │   │   ├── court_walls.rs   Static wall construction
│       │   │   └── tests.rs         Real-engine integration tests
│       │   └── bevy/           MatchResource, CourtEntities, body markers
│       ├── presentation/
│       │   ├── input/          Keyboard and window-focus handling
│       │   ├── court_scene.rs   CourtScene: camera and court markings
│       │   ├── scene_assets.rs  SceneAssets and mesh/material construction
│       │   ├── palette.rs      Shared colors
│       │   └── hud/            Hud, HudSnapshot, HudText, Overlay (one file each)
│       └── tests/              Input scenarios, fixtures, opt-in GPU smoke test
├── crates/pong_core/            Engine-independent application + domain
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs               Stable public exports
│   │   ├── application/
│   │   │   ├── match_play/
│   │   │   │   ├── for_playing_pong.rs  Driving port
│   │   │   │   ├── play_match.rs    Application entry point
│   │   │   │   ├── match_effects.rs  Executes domain decisions through physics
│   │   │   │   └── use_cases/      HandleMatchAction, MovePaddles, BounceBall, AdvanceMatch
│   │   │   └── physics/        PongPhysics trait consumed by use cases
│   │   └── domain/
│   │       ├── ball/           BallMotion and bounce/serve rules
│   │       ├── paddle/         PaddleHit and movement policies
│   │       ├── controls/       Action, Controls, Direction
│   │       ├── match_play/
│   │       │   ├── pong_match.rs  Match data, scoring, mode, and lifecycle decisions
│   │       │   ├── mode.rs, phase.rs, effect.rs  Domain values and decisions
│   │       │   └── state_machine/
│   │       │       ├── machine.rs  Holds and delegates to the current state
│   │       │       ├── state.rs, active_state.rs  State traits in separate files
│   │       │       ├── event.rs, transition.rs  Transition input and result
│   │       │       └── states/     ReadyState, ServingState, PlayingState, PausedState, FinishedState
│   │       ├── physics/        Position and Velocity values
│   │       ├── rules.rs        Dimensions and tuning
│   │       └── side.rs         Side and its operations
│   └── tests/
│       ├── match_flow.rs        Core test suite entry point
│       ├── scenarios/          Lifecycle, movement, scoring, bounce scenarios
│       └── support/            Separate fixture, mock, and command types
├── scripts/check-source-layout.ps1  One type or trait per Rust file
├── scripts/check-architecture.ps1   Allowed dependency directions
├── docs/LEARNING_GUIDE.md       Guided tour
└── .github/workflows/ci.yml     Format, tests, lint, layout, dependency checks
```

The dependency direction is `pong (Bevy adapter) -> pong_core`. The core has no dependencies and cannot import Bevy without someone changing its manifest. Avian components in the Bevy world own positions and velocities. The core uses its own `PongPhysics` trait; `AvianCourt` translates that contract to component access. There is no second authoritative simulation. Before each active step the core requests paddle velocities; new paddle contacts invoke arcade bounce rules before the solver runs, and the completed step feeds scoring and countdown rules. Inactive phases skip integration entirely.

| Layer | Responsibility |
| --- | --- |
| Domain | `Match` owns scores, mode, serve policy, and the state machine; pure policies own bounce and movement rules |
| Application | `PlayMatch` exposes named use cases that coordinate the domain and the `PongPhysics` port |
| Presentation | Keyboard translation, HUD, camera, and scene visuals; consumes core values |
| Infrastructure | Avian adapters, physics-to-transform synchronization, and ECS storage |
| Composition | Binds input/tick/view calls to concrete storage and physics; creates entities and registers systems |

Application depends on domain. Neither imports Bevy, Avian, or the host. Presentation and infrastructure do not import each other or composition. Presentation can use Bevy for input and visuals, but cannot use Avian or the project's infrastructure types. Composition connects the concrete implementations. Rust visibility protects module internals; the architecture script checks explicit source paths, and CI separately verifies that the core manifest has no dependencies.

`PresentationBindings` reads `MatchResource` and passes its core `Mode` to keyboard handling and a borrowed domain `Match` to the HUD. Presentation therefore needs no knowledge of how the match is stored. `MatchDriver` belongs to composition because it connects queued input, the `PlayMatch` application entry point, and the concrete `AvianCourt`. These bindings borrow the existing match; they add no second world or duplicate match state.

The application use cases live in `crates/pong_core/src/application/match_play/use_cases/`:

| Use case | Responsibility |
| --- | --- |
| `HandleMatchAction` | Submit start/pause/restart/mode input to the domain and carry out its reset or serve decisions |
| `MovePaddles` | Read physical observations, use domain movement policies, and request paddle velocities |
| `BounceBall` | Apply the domain's paddle-hit policy through the physics port during play |
| `AdvanceMatch` | Supply the completed step's ball position to the domain and apply scoring/countdown effects |

`PlayMatch` implements `ForPlayingPong` and delegates to these use cases. For example: `PlayMatch::new(&mut game).act(Action::Primary, &mut physics)`. `Match` has no physics parameter and never imports the application. `MatchEffects` translates domain decisions such as `ResetBall` into calls on `PongPhysics`; it contains no scoring or transition policy.

Group files by the game concept they serve. A `struct`, `enum`, or `trait` gets its own file; a structure's inherent methods and trait implementations stay with that structure. `mod.rs` files declare modules and their exports. Constants and test scenarios stay in their own appropriate files. The layout check in CI enforces at most one type or trait declaration per Rust file, including test helpers.

The composition and presentation modules stay private to the Bevy host. `CourtSetup` assembles startup using `PaddleFactory` and `BallFactory`; each factory creates a complete physics root and its visual child in one call. Physics components are included in the root's spawn bundle, so no later system needs to finish attaching them. `CourtScene` builds the camera and court markings, `CourtWalls` creates static colliders, and `Hud` builds and updates the scoreboard. `HudSnapshot` owns state-dependent text. Core imports such as `pong_core::Match`, `pong_core::Controls`, and `pong_core::PongPhysics` remain stable through explicit re-exports in `lib.rs`.

Only composition assembles complete game pieces: it may use the infrastructure components and presentation assets. The concrete factories use private helpers within their own files; they do not introduce factory traits or engine types into `pong_core`.

`AvianCourt` only implements the physics port and its reset operations. `PaddleContacts` translates Avian collisions into `PaddleHit` application input. `PreviousPosition` captures physical history; resetting a body updates this history to prevent interpolation across a teleport. Infrastructure's `BodyInterpolation` turns Avian coordinates into Bevy transforms; `AvianPlugin` registers that engine synchronization. `MatchResource` only stores the domain `Match`; composition's `MatchDriver` invokes the `PlayMatch` application service.

The domain state machine holds a `Box<dyn State>`. Each of `ReadyState`, `ServingState`, `PlayingState`, `PausedState`, and `FinishedState` has its own file and handles its own events. A pause retains the interrupted active object, including any remaining serve ticks. `Phase` remains a read-only enum for observation; it does not implement transitions. Bevy reads that status without maintaining a second gameplay state machine. See the [state-machine walkthrough](docs/LEARNING_GUIDE.md#14-the-game-state-machine).

This state-object design uses dynamic dispatch and allocations when entering states with stored data. Ordinary countdown ticks reuse the existing state allocation. An enum-based machine is also valid Rust; this project chooses separate state types to keep each state's behavior together. The [Rust Book compares State-pattern implementations](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html).

## Check your changes

```powershell
cargo test -p pong_core
cargo test --workspace
cargo fmt --all -- --check
./scripts/check-source-layout.ps1
./scripts/check-architecture.ps1
cargo clippy --workspace --all-targets -- -D warnings
```

Tests follow **Given / When / Then**: establish the situation, perform the action, assert the observable result. Core scenarios share a `MatchFixture` and a hand-written `PhysicsMock` of the application-owned physics port. Expectations check meaningful commands, such as one ball reset per point and no physics writes while paused. Scenario values and floating-point tolerances have descriptive names.

The core tests require no engine, window, or GPU. Adapter fixtures create a fresh Bevy ECS `App` with real Avian physics and controlled time, without the renderer. They cover collisions, moving paddles, high-speed tunneling, pause/resume, resets, full-frame scheduling, and a complete match. See the [testing strategy](docs/LEARNING_GUIDE.md#9-how-the-tests-divide-responsibility) for what each layer proves and how to add a test.

Factory tests use a fresh ECS world and real components without running the game's startup systems. They check that each returned entity already has its physics body and correctly sized visual child once its queued commands are applied.

Presentation tests supply a core mode or match directly, without infrastructure resources or Avian startup. Integration tests verify that the composition bindings deliver input, quit requests, and HUD updates correctly.

The optional desktop check opens a real window, captures ready/playing/paused screens into `target`, and exits:

```powershell
cargo test -p pong render_smoke -- --ignored --nocapture
```

It requires a working desktop and GPU. Pixel checks verify that the paddles and ball are present; visual inspection is still needed to assess the layout. Keep its window focused during the check.

## Learn and extend

Start with [the learning guide](docs/LEARNING_GUIDE.md). It explains the Rust syntax, ownership, ECS, dependency boundaries, physics, tests, and a suggested reading order using this project's code.

Change gameplay tuning in `crates/pong_core/src/domain/rules.rs`, key bindings in `src/game/presentation/input/keyboard_controls.rs`, and shared colors in `src/game/presentation/palette.rs`. Court layout belongs in `src/game/presentation/court_scene.rs`; HUD wording belongs in `src/game/presentation/hud/hud_snapshot.rs` and its layout in `src/game/presentation/hud/panel.rs`. Add domain behavior tests before changing gameplay rules.

Avian handles rectangular contacts and wall reflection. The core retains paddle-angle and speed policy, so this is arcade physics rather than an unmodified rigid-body response. Corner and moving-paddle behavior can differ from the previous custom solver. See the [physics walkthrough](docs/LEARNING_GUIDE.md#8-what-avian-does-and-what-remains-a-game-rule).

This version is a local arcade game. Network play, sound, and persistent settings are outside its current scope. Bevy's existing default features are retained; narrowing them is a possible future build-size optimization, not a prerequisite for playing or learning.


