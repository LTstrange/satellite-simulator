# Orbiter-py

`orbiter-py` 是一个用于连接和控制卫星模拟器的 Python 包。通过这个包，用户可以使用 Python 代码控制卫星模拟器，执行如建立连接、发送控制命令等操作。

## 安装

### 从源代码安装

从源代码安装，可以使用以下命令：

```bash
git clone https://github.com/yourusername/orbiter-py.git
cd orbiter-py
pip install -e .
```

## 使用示例

以下是如何使用 `orbiter-py` 来连接并控制卫星模拟器的简单示例：

### 1. 建立连接

```python
from orbiter.connection import establish_connection

# 建立与卫星模拟器的连接
connection = establish_connection("192.168.1.100", port=9000)

if connection.is_connected():
    print("成功连接到卫星模拟器!")
else:
    print("连接失败!")
```

### 2. 发送控制命令

```python
from orbiter.control import send_command

# 发送控制命令到卫星模拟器
response = send_command(connection, "START_SIMULATION")

print(f"模拟器响应: {response}")
```

### 3. 关闭连接

```python
from orbiter.connection import close_connection

# 关闭与卫星模拟器的连接
close_connection(connection)
print("连接已关闭.")
```

## 包结构

以下是 `orbiter-py` 包的目录结构：

```
orbiter-py/
│
├── orbiter/            # 包的主要代码目录
│   ├── __init__.py     # 包的初始化文件
│   ├── connection.py   # 用于与卫星模拟器连接的模块
│   ├── control.py      # 用于控制卫星模拟器的模块
│   └── utils.py        # 辅助工具模块（可选）
│
├── tests/              # 测试目录
│   ├── __init__.py     # 测试初始化文件
│   ├── test_connection.py  # 测试与卫星模拟器连接的功能
│   ├── test_control.py     # 测试控制卫星模拟器的功能
│   └── test_utils.py       # 测试辅助工具函数
│
├── setup.py            # 包的安装脚本
├── README.md           # 项目的说明文件
└── LICENSE             # 项目的许可证文件（可选）
```

## 贡献

欢迎提出问题、提交 bug 报告或贡献代码。您可以通过以下步骤参与项目：

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature-xyz`)
3. 提交您的修改 (`git commit -am 'Add feature xyz'`)
4. 推送到您的分支 (`git push origin feature-xyz`)
5. 提交 pull request

---

如有任何问题或建议，欢迎通过 issues 提交反馈。

