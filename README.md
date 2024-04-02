# AWS S3 Example


## 使用localStack本地开发测试

LocalStack 是一个云服务模拟器，可以在笔记本电脑或 CI 环境中的单个容器中运行。借助 LocalStack，可以在本地使用AWS Rust SDK进行基于AWS云服务的开发测试，无需链接到WAS云服务，节省成本，便于开发测试。

### Install LocalStack

Linux

Download:
```shell
curl -Lo localstack-cli-3.3.0-linux-amd64-onefile.tar.gz \ https://github.com/localstack/localstack-cli/releases/download/v3.3.0/localstack-cli-3.3.0-linux-amd64-onefile.tar.gz
```

Extract the CLI: 
```shell
sudo tar xvzf localstack-cli-3.3.0-linux-*-onefile.tar.gz -C /usr/local/bin
```

Check the CLI version:
```shell
localstack --version

# output:
3.3.0
```

### Start The LocalStack

Launch the LocalStack
```shell
localstack start
```

Check the LocalStack status
```shell
curl http://localhost:4566/_localstack/info | jq

# output:
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   271  100   271    0     0  83282      0 --:--:-- --:--:-- --:--:-- 90333
{
  "version": "3.3.1.dev:693772f27",
  "edition": "community",
  "is_license_activated": false,
  "session_id": "3ccda464-e803-4d21-b3ba-dd25d94d42c5",
  "machine_id": "dkr_49ff3d908e17",
  "system": "linux",
  "is_docker": true,
  "server_time_utc": "2024-04-01T18:40:42",
  "uptime": 73
}
```


LocalStack部署在远程服务器

> ⚠️启动的localstack工具拉起的 Docker容器绑定到主机127.0.0.1:4566，只能本地访问，如果想把localstack部署在其他机器上请在部署的机器转发一下请求，例如使用socat工具：

```shell
socat TCP-LISTEN:8090,fork TCP:127.0.0.1:4566
```
其S3客户端问该主机的ip:8090，然后socat转发到本地的127.0.0.1:4566，这样就能供其他机器的S3客户端访问该主机上Docker容器内的localstack服务了。
