# Onscripter Web

Onscripter Web 是一个基于 Rust 和 JavaScript 的 Web 服务器项目，用于在网页上运行 Onscripter 游戏。它提供了一个简单的界面，允许用户选择游戏文件夹并启动游戏。

## 运行步骤

### 1. 克隆项目

```bash
git clone https://github.com/your-repo/onscripter-web.git
cd onscripter-web
```

### 2. 安装依赖

确保你已经安装了 Rust 和 Cargo。然后在项目根目录下运行以下命令：

```bash
cargo build
```

### 3. 配置服务器参数

你可以通过命令行参数来配置服务器的主机、端口和根目录。默认配置如下：

- 主机：`127.0.0.1`
- 端口：`9991`
- 资源根目录：`./assets`

你可以使用以下命令来修改配置：

```bash
cargo run -- -H <host> -p <port> -r <root_dir>
```

例如：

```bash
cargo run -- -H 0.0.0.0 -p 8080 -r /path/to/assets
```

### 4. 启动服务器

```bash
cargo run
```

### 5. 访问服务器

在浏览器中打开 `http://<host>:<port>`，你将看到游戏列表页面。选择一个游戏文件夹，点击进入即可开始游戏。

## 添加游戏资源

请将支持的ONS游戏包放到`./assets`文件夹内（文件夹名称无要求），它应该看起来是这样的：

```
assets
│   └── index.html
├── game1
│   ├── arc.nsa
│   ├── default.ttf
│   ├── list.txt
│   ├── nscript.dat
│   ├── onsyuri.html
│   ├── onsyuri.js
│   ├── onsyuri.wasm
│   ├── onsyuri_index.json
│   ├── ...
│   └── ...
├── game2
│   ├── ...
│   └── ...
├── ...
...
```

具体配置详见Onsyuri项目

## Onscripter-Yuri

![GitHub release](https://img.shields.io/github/v/release/YuriSizuku/OnscripterYuri?color=green&label=onsyuri&logo=4chan&style=flat-square)![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/YuriSizuku/OnscripterYuri/build_web.yml?label=web(wasm)&logo=firefox&style=flat-square)![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/YuriSizuku/OnscripterYuri/build_android.yml?label=android(arm|arm64)&&logo=android&style=flat-square)

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/YuriSizuku/OnscripterYuri/build_win.yml?label=win_mingw(x86|x64)&logo=mingww64&style=flat-square)![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/YuriSizuku/OnscripterYuri/build_win.yml?label=win_msvc(x86|x64|arm64)&logo=codeblocks&style=flat-square)

[https://github.com/YuriSizuku/OnscripterYuri](https://github.com/YuriSizuku/OnscripterYuri)

