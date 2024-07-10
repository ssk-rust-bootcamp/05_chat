# Geektime Rust 语言训练营

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 cargo generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

在我们的课程中，新的项目会使用 `ssk-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate ssk-rust-bootcamp/template
```

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。
[text](../../AuroraPlan/rust-toolchain.toml) [text](../../AuroraPlan/.rustfmt.toml)
```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install` 即可。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

cargo nextest 是一个 Rust 增强测试工具。

```bash
cargo install cargo-nextest --locked
```
#### deny check
- rm -rf deny.toml 删除掉现有的 deny 配置
- cargo deny init 初始化适应现版本的配置文件
- ️在 deny.toml 中找到 git-fetch-with-cli = true 并取消注释
- 修改 [licenses] 字段
```
unlicensed = "allow"
allow = [
  "MIT",
  "Apache-2.0",
  "Unicode-DFS-2016",
  "MPL-2.0",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "CC0-1.0",
]

```
- 执行 cargo deny check

- 此时再执行 git commit -a 就没有问题了
