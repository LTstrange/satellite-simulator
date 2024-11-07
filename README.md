# Satellite Simulator

## Project Overview
This simulator, built on the [Bevy engine](https://bevyengine.org/), offers a realistic simulation of satellite orbits around Earth, covering essential functions such as communication links, computation offloading, and energy consumption. Designed to support research in satellite communication networks, it provides an intuitive 3D visualization interface that allows users to clearly observe satellite operations and communication pathways. Future updates will enable data retrieval and command inputs through a network interface, giving users access to real-time simulation data for further research and analysis.

## Features
- **Satellite Orbit Simulation**: Supports the import of satellite data in JSON format to simulate realistic orbital paths.
- **Orbit Visualization**: Displays the precise orbital trajectories of satellites in an interactive view.
- **Orbit Camera**: Enables rotation and zoom controls, allowing users to adjust their perspective.
- **Real-Time Satellite Communication Links**: Simulates the dynamic establishment and disconnection of communication links between satellites.

## Installation

1. **Clone the Repository**

   Begin by cloning the repository:
   ```bash
   git clone https://github.com/LTstrange/satellite-simulator.git
   cd satellite-simulator
   ```
2. **Install the Rust Toolchain**

   If Rust is not yet installed, follow the official [installation guide](https://www.rust-lang.org/tools/install) to set up the Rust toolchain.
3. **Build the Project**

   Compile the project:
   ```bash
   cargo build
   ```
4. **Run the Simulator**

   Start the simulator by running:
   ```bash
   cargo run
   ```

## Usage

- **View Controls**
   - Windows: Hold the left mouse button and drag to rotate the view; use the mouse wheel to zoom.
   - MacOS: Use two-finger swipe to rotate the view and pinch to zoom.
- **Orbit Display**: Satellite orbits are displayed in white, with satellite positions dynamically updated over time.
- **Link Display**: Inter-satellite links (ISL) are shown as yellow bidirectional arrows. Satellites connect to their nearest neighbors by default, with links breaking once they exceed the communication range.
- **Configuration File**: Use the `config.toml` file to customize simulator settings.

## Configuration File (`config.toml`)
   The config.toml file allows for configuring the simulator’s overall behavior. The following settings can be adjusted within the configuration file:
   - **[Dataset]**: Defines satellite constellation data settings.
      - **constellation_file**：Specifies the relative path to the constellation dataset.
   - **[Display]**: Controls display-related settings.
      - **orbit**: Enables or disables orbit display.
      - **connection**: Enables or disables ISL (Inter-Satellite Link) display.
   - **[Simulation]**: Configures simulation parameters.
      - **time_speed**：This multiplier adjusts the time step size without causing simulation lag; however, setting it too high may reduce simulation accuracy.
      - **connection_distance**: Specifies the maximum distance for ISL connections.
      - **connection_number**: Defines the maximum number of ISL connections per satellite.
   Ensure that the `config.toml` is located in the same directory as the executable file.

## Future Work
- **Enhanced Communication Link Modeling**: Add simulations for transmission delays and signal interference within established satellite links.
- **Computation Offloading Simulation**: Model the computational resources available on satellites.
- **Energy Simulation**: Implement energy modeling to account for power consumption due to transmissions and computations, along with solar energy recharging for satellites.
- **Control Interface**: Enable command input via a network interface to allow detailed adjustments, such as assigning computation tasks to specific satellites or configuring routing algorithms.

## Contributing
All contributions are welcome! Please begin by forking this repository and submitting a pull request, or open an issue to start a discussion.
