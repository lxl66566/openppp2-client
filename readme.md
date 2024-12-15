# openppp2-client

一个简陋的、非官方的 [openppp2](https://github.com/liulilittle/openppp2) 客户端，主要用于**切换多个配置**，**使用默认配置**，**大陆直连**。

## 安装

1. 安装 [openppp2](https://github.com/liulilittle/openppp2)。需要保证环境变量或执行目录存在 openppp2 的可执行文件，且名称为 `ppp`（或 `ppp.exe`, `ppp.cmd`, `ppp.sh`）。
2. 从 Release 中下载 `openppp2-client` 对应平台的二进制文件，解压。

## 配置与使用

下载后，直接运行 `openppp2-client`，其会自动创建 `~/.config/openppp2-client.toml` 用于存放配置。请根据你的需要进行修改。

配置说明：

```toml
config_dirs = [".", "~/.config"]        # ppp 配置文件存放目录（将这些目录下的所有 `.json` 文件视为配置）
args = [                                # 命令行参数，还有 --config 与内置的 --dns-rules 将被自动添加，无需在此处给出。
  "--mode=client",
  "--tun-ip=10.0.0.2",
  "--tun-gw=10.0.0.0",
  "--tun-mask=24",
  "--tun-host=yes",
  "--tun-vnet=yes",
  "--block-quic=yes",
  "--set-http-proxy=no",
]
default_port_for_ssh = 80               # openppp2-client 会读取 ssh 中的 Host 与 HostName 构建 ppp defaults，并使用设置的默认端口进行配置生成，免去了 ~/.ssh/config, ~/.config/openppp2-client.toml 两头写的困扰。
enable_chnroutes_by_default = false     # 是否默认开启大陆直连分流，默认关闭。

[[defaults]]                # 使用默认的 ppp 配置文件，只需配置 ip 与端口即可。
name = "example1"
ip = "127.0.0.1"
port = 2777

[[defaults]]                # 多个 ppp 默认配置
name = "example2"
ip = "127.0.0.1"
port = 2888
```

也可以通过命令行强制开启大陆直连分流（注意，修改系统路由表需要管理员权限）：

```sh
openppp2-client --enable-chnroutes
```

如果你不想看到 TUI 面板而希望通过命令行直接启动 openppp2：

```sh
openppp2-client use 127.0.0.1:2777          # 使用默认配置，只需要提供 ip 与端口即可
openppp2-client use openppp2-client.json    # 使用自定义配置文件
```

更多使用方法，请参考 `openppp2-client -h`。

## TODO

- [x] 分流
