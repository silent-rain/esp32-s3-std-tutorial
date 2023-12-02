# esp-idf 环境配置

注意： 这里搭建 esp-idf 环境配置仅用于可视化配置单片机的 SDK 配置。

## 开发环境

- IDE: vscode
- 语言: C+Python
- 框架: esp-idf v5.1

## 开发环境搭建

### 安装 python 环境

系统默认已安装 python3;
仅安装虚拟环境, 防止本地环境混乱；

- 安装虚拟环境

```shell
sudo pacman -S extra/python-virtualenvwrapper
```

- 配置环境变量
  > vim ~/.zshrc

```shell
# >>> virtualenvwrapper initialize >>>
export WORKON_HOME=$HOME/.virtualenvs
VIRTUALENVWRAPPER_PYTHON=/usr/bin/python3
source $HOME/.local/bin/virtualenvwrapper.sh
PATH=$HOME/.local/bin:$PATH
# <<< virtualenvwrapper initialize <<<
```

- 激活环境变量

```shell
source ~/.zshrc
```

- 创建虚拟环境

```shell
mkvirtualenv esp32s3
```

### 搭建 esp-idf 开发环境

- [Linux 和 macOS 平台工具链的标准设置](https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/get-started/linux-macos-setup.html)

- 提示代码示例位置

```shell
esp-idf/examples/get-started/hello_world
```

## 编译与运行

### 编译

```shell
# 进入python虚拟环境
workon esp32s3

# 进入esp-idf环境
source ~/.espup/esp-idf/export.sh

# 设置“目标”芯片
idf.py set-target esp32s3

## 芯片SDK配置
idf.py menuconfig

# 编译
idf.py build

# 烧录
idf.py -p PORT monitor
```
