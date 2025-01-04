# Roadmap

## V0.1.0
- [x] Satellites motion simulation.
- [x] ISLs topology construct.
- [x] get topology from network.
- [x] Camera control.
- [x] Satellite orbit display.
- [x] ISL display.

## V0.2.0
- [] Brp based network control.
- [] add satellites using python.
- [] Better UI.

## Uncategorized

Recording planned and future tasks.

Functionality:
- Implement some kind of double-pass connection for ISL simulation. (and set it as the default for simulationa stability)
- Realtime mode: Cant change the time speed, sync with system time, maybe useful for realtime accurate simulation.
- Add statistic Plugin to track some important Data.

Bug Fix:
- Maintain simulator operation when window is minimized.
- Enhance netcode robustness. (May be just use bevy remote protocol is better)
- Cant use mouse drag on macos.

Perfomance:
- Resolve performance issues caused by gizmos. (Partialy fixed, waiting for component gizmo)
- Cache data to mitigate speed differences between simulator and Python code. (may not be necessary due to "double-pass" connection sim mod)

UI:
- Connection saturation rate
- Tend to use ui rather than config file.
