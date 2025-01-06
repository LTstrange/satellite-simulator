# Roadmap

## V0.1.0
- [x] Satellites motion simulation.
- [x] ISLs topology construct.
- [x] get topology from network.
- [x] Camera control.
- [x] Satellite orbit display.
- [x] ISL display.

## V0.2.0
- [x] Brp based network control.
- [x] add satellites using python.
- [ ] fetch connections using python.
- [ ] Better UI.

## Uncategorized

Recording planned and future tasks.

Functionality:
- Implement some kind of double-pass connection for ISL simulation. (and set it as the default for simulation stability)
- Realtime mode: Cant change the time speed, sync with system time, maybe useful for realtime accurate simulation.
- Add statistic Plugin to track some important Data.

Bug Fix:
- Maintain simulation when window is minimized.

Perfomance:
- Resolve performance issues caused by gizmos. (Partialy fixed, waiting for (component_gizmo/GizmoAsset))

UI:
- Tend to use ui rather than config file.
- Connection saturation rate
- Implement mouse drag on macos.
