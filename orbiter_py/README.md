# Orbiter_py

`orbiter_py` 是一个用于连接和控制卫星模拟器的 Python 包。通过这个包，用户可以使用 Python 代码控制卫星模拟器，执行如建立连接、发送控制命令等操作。

## 安装


## 使用示例

以下是如何使用 `orbiter_py` 来连接并控制卫星模拟器的简单示例：

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

## 贡献

欢迎提出问题、提交 bug 报告或贡献代码。您可以通过以下步骤参与项目：

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature-xyz`)
3. 提交您的修改 (`git commit -am 'Add feature xyz'`)
4. 推送到您的分支 (`git push origin feature-xyz`)
5. 提交 pull request

---

如有任何问题或建议，欢迎通过 issues 提交反馈。

