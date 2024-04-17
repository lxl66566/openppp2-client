# openppp2-client

一个简陋的、非官方的 [openppp2](https://github.com/liulilittle/openppp2) 客户端。

## 安装

从 Release 中下载对应平台的二进制文件，解压后运行，其会在当前文件夹下创建 `client-config.toml` 用于存放配置。

## 配置与使用

需要保证环境变量或执行目录存在 openppp2 的可执行文件。

配置说明：

```toml
config_dirs = ["."]       # ppp 配置文件存放目录，openppp2-client 将这些目录下的所有 `.json` 文件视为配置。
args = [                  # 命令行参数，还有 --config 与 --dns-rules 将被添加，此处未写出。
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
name = "example"
ip = "127.0.0.1"
port = 2777

[[defaults]]              # 可以添加多个默认配置。
name = "example2"
ip = "127.0.0.1"
port = 2778
```

## TODO

- [ ] 分流
