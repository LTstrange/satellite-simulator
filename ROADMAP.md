# Roadmap

Recording planned and future tasks.

Functionality:
- Implement some kind of double-pass connection for ISL simulation. (and set it as the default for connection and simulation result stability)
- Realtime mode: Cant change the time speed, sync with system time, maybe useful for accurate simulation.
- UI: tend to use ui rather than config file.

Bug Fix:
- Maintain simulator operation when window is minimized.
- Enhance netcode robustness. (May be just use bevy remote protocol is better)

Perfomance:
- Cache data to mitigate speed differences between simulator and Python code. (may not be necessary due to "double-pass" connection sim mod)
- Resolve performance issues caused by gizmos. (Partialy fixed, waiting for component gizmo)