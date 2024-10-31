# 卫星模拟器

## 项目简介
这是一个基于 [Bevy](https://bevyengine.org/) 引擎开发的卫星通信模拟器，旨在模拟卫星绕地球的真实轨迹、通信链路、计算卸载等功能。该项目可以用于卫星通信网络等领域，提供了直观的 3D 可视化界面，帮助用户更好地理解卫星运行轨道和通信链路。并可以通过网络接口获取数据、输入指令，让用户可以基于模拟器生成的数据展开进一步的研究。

## 功能
- **卫星轨道模拟**：支持导入TLE格式的卫星数据，由此模拟卫星轨道。[WIP：目前仅支持json格式]
- **卫星轨道显示**：可视化显示卫星的轨道路径。
- **轨道相机**：支持旋转和缩放视角。
- **实时卫星通信链路**：模拟卫星之间的通信链接及信号延迟[WIP]。

## 安装

1. **克隆仓库**：
   ```bash
   git clone https://github.com/LTstrange/satellite-simulator.git
   cd satellite-simulator
   ```
2. **安装 Rust 工具链**：
   如果还未安装 Rust，可跟随Rust官方[安装指引](https://www.rust-lang.org/tools/install)。
3. **运行项目**：
   ```bash
   cargo run
   ```

## 使用方法

- **视角控制**：
   - Windows: 按住鼠标左键并拖动以旋转视角，鼠标滚轮缩放画面。
   - MacOS: 双指滑动旋转视角，两指捏合缩放。
- **轨迹展示**：卫星轨道会以白色显示，卫星位置会随时间模拟移动。
- **链路显示**：卫星间链接（ISL）以黄色双向箭头显示。链路以默认方法（最近邻）链接，并在超距后断开。
- **配置文件**：使用`config.toml`文件配置程序行为。

## 配置文件 ( `config.toml` )
   *修改配置文件* 是设置模拟器 *整体行为* 的方式，通过配置文件，可以设置：
   - [Dataset]：用于配置卫星星座的数据集
      - constellation_file：指向星座数据集的相对地址
   - [Display]：用于配置显示相关设置
      - orbit：是否启用轨道显示
      - connection：是否启用ISL显示
   - [Simulation]：用于配置模拟相关参数
      - time_speed：配置时间流速倍率。倍率通过调整时间片大小实现，不会使模拟卡顿。但时间片过大可能会导致模拟精度下降。
      - connection_distance：ISL链接最大距离
      - connection_number：每颗卫星最大ISL链接数量
   配置文件必须与可执行文件处于同一目录下。


## 未来工作
- **通信链路模拟**：支持卫星之间的通信链路动态显示，包括信号延迟和链路干扰。
- **计算卸载模拟**：支持模拟卫星的计算资源。
- **能源模拟**：支持电量模拟，模拟因传输和计算导致的能源损耗，与太阳能充能。
- **程序操控接口**：支持从网络接口读取命令，并进行相应 *精细* 设置与调整。例如，给特定卫星分配计算任务，给卫星设置路由算法等。

## 贡献
欢迎任何形式的贡献！请先 Fork 本仓库并提交 Pull Request，或提交issues。
