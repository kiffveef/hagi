# MCP サーバー導入ガイド

このドキュメントでは、hagiがセットアップするMCPサーバーのインストール方法と設定方法を説明します。

---

## 目次

1. [前提条件](#前提条件)
2. [MCPサーバー一覧](#mcpサーバー一覧)
3. [インストール手順](#インストール手順)
4. [hagiでのセットアップ](#hagiでのセットアップ)
5. [個別の有効化・無効化](#個別の有効化無効化)
6. [トラブルシューティング](#トラブルシューティング)

---

## 前提条件

以下のツールが必要です：

### 必須
- **Node.js** (v18以降推奨)
  ```bash
  node --version
  ```

### 推奨
- **uv** (Python package manager)
  ```bash
  curl -LsSf https://astral.sh/uv/install.sh | sh
  ```

- **Rust/cargo** (file-search MCP用)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

---

## MCPサーバー一覧

| MCP | 用途 | インストール方法 | デフォルト状態 |
|-----|------|------------------|----------------|
| sequential-thinking | 構造化思考支援 | npx (自動) | ✅ 有効 |
| serena | コード解析・セマンティック検索 | npx (自動) | ❌ 無効 |
| file-search | 高速ファイル検索 | cargo install (手動) | ❌ 無効 |
| git | Git操作 | uvx (自動) | ❌ 無効 |
| github | GitHub連携 | npx (自動) | ❌ 無効 |
| context7 | ライブラリドキュメント検索 | npx (自動) | ❌ 無効 |

---

## インストール手順

### 1. sequential-thinking (自動インストール)

`hagi install`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @modelcontextprotocol/server-sequential-thinking
```

---

### 2. serena (自動インストール)

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y serena-mcp-server
```

**注意事項:**
- プロジェクトルートに`.serena/`ディレクトリが作成されます
- メモリ肥大化を防ぐため、`.gitignore`に自動追加されます
- 定期的なクリーンアップ推奨(30日以上経過したファイル削除)

---

### 3. file-search (手動インストール必須)

**インストール手順:**

#### 方法1: cargo install (推奨)
```bash
cargo install --git https://github.com/Kurogoma4D/file-search-mcp.git
```

#### 方法2: ソースからビルド
```bash
git clone https://github.com/Kurogoma4D/file-search-mcp.git
cd file-search-mcp
cargo build --release
# バイナリをパスに追加
cp target/release/file-search-mcp ~/.cargo/bin/
```

**インストール確認:**
```bash
which file-search-mcp
# 出力例: /home/user/.cargo/bin/file-search-mcp
```

**パスが通っていない場合:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# .bashrcや.zshrcに追加推奨
```

---

### 4. git (自動インストール)

有効化時にuvx経由で自動的にインストールされます。

**前提条件:**
- uvのインストールが必要

**手動確認:**
```bash
uvx mcp-server-git --repository .
```

---

### 5. github (自動インストール + PAT設定)

有効化時にnpx経由で自動的にインストールされます。

**前提条件:**
- GitHub Personal Access Token (PAT)の発行が必要

**PAT発行手順:**
1. GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. "Generate new token (classic)" をクリック
3. スコープを選択:
   - `repo` (リポジトリアクセス)
   - `read:org` (組織情報読み取り)
4. トークンをコピー

**設定方法:**

プロジェクトルートに`.env`ファイルを作成:
```bash
echo "GITHUB_PERSONAL_ACCESS_TOKEN=your_token_here" > .env
```

または、`~/.claude/mcp.json`を直接編集:
```json
"github": {
  "env": {
    "GITHUB_PERSONAL_ACCESS_TOKEN": "your_token_here"
  }
}
```

---

### 6. context7 (自動インストール)

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @upshift-protocol/context7
```

---

## hagiでのセットアップ

### グローバルセットアップ

```bash
# グローバル設定を~/.claude/にインストール
hagi install --global

# ドライラン(変更内容の確認のみ)
hagi install --global --dry-run
```

**セットアップ内容:**
- `~/.claude/mcp.json`の作成・マージ
- `~/.claude/settings.json`の作成・マージ
- sequential-thinking MCPのみ有効化

---

### プロジェクト個別セットアップ

```bash
# プロジェクトルートで実行
cd /path/to/your/project
hagi install
```

**セットアップ内容:**
- `.claude/`ディレクトリ作成
- `.claude/CLAUDE.md`、`.claude/instructions/`のコピー
- `.claude/settings.local.json`のコピー
- `.gitignore`の更新

---

## 個別の有効化・無効化

### 方法1: hagiコマンド(将来実装予定)

```bash
# MCPサーバーを有効化
hagi mcp enable serena
hagi mcp enable file-search

# MCPサーバーを無効化
hagi mcp disable serena

# 一覧表示
hagi mcp list
```

### 方法2: 手動編集

`~/.claude/mcp.json`を編集:

```json
{
  "mcpServers": {
    "serena": {
      "command": "npx",
      "args": ["-y", "serena-mcp-server"],
      "disabled": false  // true → false に変更
    }
  }
}
```

**変更後の反映:**
- Claude Codeを再起動

---

## トラブルシューティング

### file-search MCPが動作しない

**症状:**
```
Command not found: file-search-mcp
```

**解決方法:**
1. インストール確認:
   ```bash
   cargo install --list | grep file-search
   ```

2. パス確認:
   ```bash
   which file-search-mcp
   ```

3. パスが通っていない場合:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

4. シェル設定ファイルに追加:
   ```bash
   # ~/.bashrc または ~/.zshrc
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

---

### github MCPでPATエラー

**症状:**
```
Error: GITHUB_PERSONAL_ACCESS_TOKEN is not set
```

**解決方法:**
1. `.env`ファイル確認:
   ```bash
   cat .env
   ```

2. トークンの再発行:
   - 有効期限切れの場合、GitHubで再発行

3. mcp.json確認:
   ```bash
   cat ~/.claude/mcp.json | jq '.mcpServers.github.env'
   ```

---

### serenaのメモリ肥大化

**症状:**
- `.serena/`ディレクトリが肥大化

**解決方法:**
1. 古いファイルを削除:
   ```bash
   find .serena/ -type f -mtime +30 -delete
   ```

2. 定期クリーンアップスクリプト作成:
   ```bash
   # cleanup-serena.sh
   #!/bin/bash
   find .serena/ -type f -mtime +30 -delete
   echo "Cleaned up old serena files"
   ```

3. cron設定(週次実行):
   ```bash
   crontab -e
   # 毎週日曜 0:00に実行
   0 0 * * 0 /path/to/cleanup-serena.sh
   ```

---

### MCP接続エラー

**症状:**
```
Failed to connect to MCP server
```

**解決方法:**
1. 依存関係確認:
   ```bash
   # Node.js
   node --version

   # uv
   uv --version

   # cargo
   cargo --version
   ```

2. MCP設定確認:
   ```bash
   cat ~/.claude/mcp.json | jq
   ```

3. Claude Code再起動

4. ログ確認:
   ```bash
   # Claude Codeのログを確認
   # パスは環境により異なる
   tail -f ~/.local/state/claude-code/logs/*.log
   ```

---

## 参考リンク

- [sequential-thinking MCP](https://github.com/modelcontextprotocol/servers)
- [serena MCP](https://github.com/serena-ai/serena-mcp)
- [file-search MCP](https://github.com/Kurogoma4D/file-search-mcp)
- [git MCP](https://github.com/modelcontextprotocol/servers)
- [github MCP](https://github.com/modelcontextprotocol/servers)
- [context7 MCP](https://github.com/upshift-protocol/context7)

---

## フィードバック

不明点や問題があれば、以下にissueを作成してください:
https://github.com/kiffveef/hagi/issues
