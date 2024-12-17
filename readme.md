# Solana 程序示例

本项目展仅为了测试go与合约交互而用


## 安装与运行

### 先决条件

- **Rust**：需要安装 Rust 语言工具链，具体安装步骤请参考 [Rust 官网](https://www.rust-lang.org/learn/get-started)。
- **Solana CLI**：需要安装 Solana 命令行工具，参考 [Solana 官方文档](https://docs.solana.com/cli/install-solana-cli-tools)。
- **Solana 本地测试网**：需要运行 Solana 本地节点来进行测试。

### 安装依赖

1. 在项目根目录下，首先使用 Cargo 安装所有依赖：

    ```
    cargo build-sbf --manifest-path=Cargo.toml --sbf-out-dir=dist/program -- -Znext-lockfile-bump
    ```

2. 上述命令会编译程序并生成与 Solana 区块链兼容的 BPF（Berkeley Packet Filter）目标文件。

### 启动 Solana 本地测试网

1. 首先，确保你已经安装并配置好 [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)。
2. 启动本地 Solana 测试网络（`solana-test-validator`）。执行以下命令来启动 Solana 本地验证节点，它将模拟一个 Solana 网络：

    ```bash
    solana-test-validator
    ```

   默认情况下，Solana 本地验证节点将会在 `localhost:8899` 上监听 RPC 请求。

### 创建钱包并获取测试代币

在本地 Solana 网络启动后，你需要创建一个新的钱包，并为它获取一些测试用的 SOL 代币。

1. 创建一个新的 Solana 钱包并将其保存在 `~/my-wallet.json`：

    ```bash
    solana-keygen new --outfile ~/my-wallet.json
    ```

2. 通过以下命令获取 10 SOL 的测试代币：

    ```bash
    solana airdrop 10
    ```

   上述命令会为你生成的钱包地址空投 10 SOL，供你在本地测试使用。

### 部署程序到本地 Solana 网络

1. 在成功启动 Solana 本地网络并创建钱包后，接下来是部署你的 Solana 程序。首先，确保你已经使用 `cargo build-bpf` 命令编译了程序。

2. 部署编译好的 Solana 程序到本地网络。运行以下命令来部署程序：

    ```bash
    solana program deploy dist/program/solana_simple_contract.so --url http://127.0.0.1:8899
    ```

   其中，`solana_simple_contract.so` 是你通过 `cargo build-bpf` 命令生成的目标文件。部署命令会输出程序的 **Program ID**

