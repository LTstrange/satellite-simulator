# Roadmap

## V0.1.0
- [x] Satellites motion simulation.
- [x] ISLs topology construct.
- [x] get topology from network.
- [x] Camera control.
- [x] Satellite orbit display.
- [x] ISL display.

## V0.2.0
- [x] Separate orbit and satellites functionality
- [ ] Brp based network control.
- [ ] Implement static and/or dynamic ISL for xGrid constellation. 
    By using some group logic, satellites in one groud can only link to another group.
- [ ] Python code interaction: fetch/create ISLs, add satellites/orbits, and more.
- [ ] Better UI.
- [ ] Better Documentation. (mdbook)
- [ ] Accept TLE file

## Uncategorized

Recording planned and future tasks.

Functionality:
- python control:
    - create satellites
    - create ISLs between satellites
- Realtime mode: Cant change the time speed, sync with system time, maybe useful for realtime accurate simulation.

Bug Fix:
- Maintain simulation when window is minimized.

Perfomance:

UI:
- Implement mouse drag on macos.
- Tend to use ui and python code rather than config file.
