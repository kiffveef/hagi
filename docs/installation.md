# インストールガイド

---

## 前提条件

### 必須

| ツール | 用途 | インストール |
|--------|------|-------------|
| Rust/cargo 1.80+ | hagiのビルド | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| jq | .claude/ git保護 | `sudo apt install jq` |

### MCP利用時

| ツール | 必要なMCP | インストール |
|--------|-----------|-------------|
| Node.js v18+ | context7, one-search, memory | `curl -fsSL https://deb.nodesource.com/setup_18.x \| sudo -E bash -` |
| uv | serena, git | `curl -LsSf https://astral.sh/uv/install.sh \| sh` |

---

## インストール

### cargoでインストール(推奨)

```bash
cargo install --git https://github.com/kiffveef/hagi hagi
hagi --version
```

### リポジトリからビルド

```bash
git clone https://github.com/kiffveef/hagi.git
cd hagi
cargo install --path .
```

---

## 更新

```bash
# 推奨
hagi update

# 手動
cargo install --git https://github.com/kiffveef/hagi hagi --force
```

---

## アンインストール

```bash
# バイナリ削除
cargo uninstall hagi

# 設定も削除する場合
hagi uninstall --global
cd /path/to/project && hagi uninstall
```

問題が発生した場合は[トラブルシューティング](./troubleshooting.md#インストール)を参照。
