# openppp2-client

一个简陋的、非官方的 [openppp2](https://github.com/liulilittle/openppp2) 客户端，主要用于切换多个配置。

## 安装

从 Release 中下载对应平台的二进制文件，解压后运行，其会创建 `~/.config/openppp2-client.toml` 用于存放配置。

## 配置与使用

需要保证环境变量或执行目录存在 openppp2 的可执行文件/启动脚本。

配置说明：

```toml
config_dirs = [".", "~/.config"]       # ppp 配置文件存放目录（将这些目录下的所有 `.json` 文件视为配置）
args = [                               # 命令行参数，还有 --config 与内置的 --dns-rules 将被自动添加，无需在此处给出。
  "--mode=client",
  "--tun-ip=10.0.0.2",
  "--tun-gw=10.0.0.0",
  "--tun-mask=24",
  "--tun-host=yes",
  "--tun-vnet=yes",
  "--block-quic=yes",
  "--set-http-proxy=no",
]

[[defaults]]              # 使用默认的配置文件，只需配置 ip 与端口即可。
name = "example1"
ip = "127.0.0.1"
port = 2777

[[defaults]]              # 多个默认配置
name = "example2"
ip = "127.0.0.1"
port = 2888
```

## TODO

- [ ] 分流
