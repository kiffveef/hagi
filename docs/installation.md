# hagiインストールガイド

hagiのインストール方法と前提条件の詳細説明です。

---

## 前提条件

hagiを使用するには、以下のツールがインストールされている必要があります。

### 必須

#### Rust/cargo (1.80以降推奨)

Rustのツールチェーン(rustc、cargo)が必要です。

**インストール確認:**
```bash
cargo --version
```

**インストール方法:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

公式サイト: https://www.rust-lang.org/tools/install

### MCP利用時に必要

#### Node.js (v18以降推奨)

sequential-thinking、github、context7などのMCPサーバーに必要です。

**インストール確認:**
```bash
node --version
npm --version
```

**インストール方法:**

**Ubuntu/Debian:**
```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**macOS (Homebrew):**
```bash
brew install node
```

**Windows (nvm-windows):**
```bash
# https://github.com/coreybutler/nvm-windows/releases から最新版をダウンロード
nvm install 18
nvm use 18
```

#### uv (Python package manager)

git MCPサーバーに必要です。

**インストール確認:**
```bash
uv --version
```

**インストール方法:**
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

公式サイト: https://github.com/astral-sh/uv

---

## hagiのインストール

### 方法1: cargoでインストール(推奨)

GitHubリポジトリから直接インストールします。

```bash
cargo install --git https://github.com/kiffveef/hagi hagi
```

**インストール確認:**
```bash
hagi --version
```

### 方法2: リポジトリをクローンしてビルド

開発版や独自のカスタマイズをしたい場合に使用します。

```bash
# リポジトリをクローン
git clone https://github.com/kiffveef/hagi.git
cd hagi

# ビルド
cargo build --release

# インストール
cargo install --path .
```

---

## 更新

hagiを最新版に更新します。

### 方法1: hagi updateコマンド(推奨)

```bash
hagi update
```

確認プロンプトが表示され、最新版をGitHubから取得します。

### 方法2: cargo install --force

```bash
cargo install --git https://github.com/kiffveef/hagi hagi --force
```

`--force`フラグで既存のバージョンを上書きします。

---

## アンインストール

hagiを削除します。

```bash
cargo uninstall hagi
```

**設定ファイルも削除する場合:**

```bash
# グローバル設定削除
hagi uninstall --global

# プロジェクト設定削除(各プロジェクトで実行)
cd /path/to/project
hagi uninstall
```

---

## トラブルシューティング

### cargo: command not found

**原因:** Rustがインストールされていない、またはPATHが通っていない

**解決策:**
1. Rustをインストール
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. PATHを設定
   ```bash
   source ~/.cargo/env
   ```

3. シェルを再起動

### error: could not find `Cargo.toml`

**原因:** 間違ったディレクトリでコマンドを実行している

**解決策:**
- `cargo install --git`を使用する(リポジトリのクローン不要)
- または、クローンしたhagiディレクトリ内で実行

### error: failed to download from GitHub

**原因:** インターネット接続の問題、またはGitHubへのアクセス制限

**解決策:**
1. インターネット接続を確認
2. プロキシ設定を確認
3. GitHubのステータスを確認: https://www.githubstatus.com/

### permission denied

**原因:** cargoのインストールディレクトリへの書き込み権限がない

**解決策:**
- `sudo`を使わずにインストール(推奨)
- `~/.cargo/bin`が書き込み可能か確認
- 環境変数`CARGO_HOME`を確認

---

## 次のステップ

インストールが完了したら:

1. **[クイックスタート](../README.md#クイックスタート)**: 基本的な使い方
2. **[コマンドリファレンス](./commands.md)**: 全コマンドの詳細
3. **[MCP導入ガイド](./mcp-setup.md)**: MCPサーバーのセットアップ

---

## 関連ドキュメント

- **[README](../README.md)**: hagiの概要
- **[使い方ガイド](./usage.md)**: スラッシュコマンド、MCPの活用方法
- **[開発ガイド](./development.md)**: hagiへの貢献方法
- **[トラブルシューティング](./troubleshooting.md)**: よくある問題と解決策
