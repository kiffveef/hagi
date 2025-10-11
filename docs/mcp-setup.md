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

### 🔥 推奨構成(Windows + WSL2最適化)

軽量・高速・完全ローカル動作を重視した構成です。

| MCP | 用途 | インストール方法 | デフォルト状態 | 備考 |
|-----|------|------------------|----------------|------|
| sequential-thinking | 構造化思考支援 | npx (自動) | ✅ 有効 | 軽量、起動即座 |
| context7 | 公式ドキュメント検索 | npx (自動) | ✅ 有効 | 軽量、API keyなしで基本機能利用可 |
| one-search | Web検索 | npx (自動) | ❌ 無効 | DuckDuckGo推奨(Puppeteerなし) |
| memory | 長期記憶管理 | uv + Git (手動) | ❌ 無効 | 完全ローカル(SQLite-vec + ONNX) |
| serena | セマンティックコード解析 | npx (自動) | ❌ 無効 | XDG準拠、キャッシュ管理 |

### その他のMCP

| MCP | 用途 | インストール方法 | デフォルト状態 |
|-----|------|------------------|----------------|
| file-search | 高速ファイル検索 | cargo install (手動) | ❌ 無効 |
| git | Git操作 | uvx (自動) | ❌ 無効 |
| github | GitHub連携 | npx (自動) | ❌ 無効 |

---

## インストール手順

### 1. sequential-thinking (自動インストール)

`hagi install`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @modelcontextprotocol/server-sequential-thinking
```

---

### 2. serena (自動インストール、デフォルト無効) - Phase 2e

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y serena-mcp-server
```

**特徴:**
- セマンティックコード検索・解析
- LSP対応言語: Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++
- プロジェクトごとのキャッシュ管理

**キャッシュ管理:**
- デフォルトキャッシュ: `~/.cache/serena` (XDG Base Directory準拠)
- プロジェクトキャッシュ: `.serena/` (自動的に`.gitignore`に追加済み)
- 定期クリーンアップ推奨: 30日以上経過したファイル削除

**クリーンアップスクリプト:**
```bash
# プロジェクトルートで実行
find .serena/ -type f -mtime +30 -delete
```

**有効化方法:**
```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable serena

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**連携:**
- `/code-pattern`コマンド: serena + mcp-memory-serviceで過去パターン検索
- `/research`コマンド: Step 3bで現在のコードベースとの統合提案

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

### 6. context7 (自動インストール、デフォルト有効)

`hagi install --global`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @upstash/context7-mcp
```

**特徴:**
- 公式ドキュメント検索(バージョン指定対応)
- API keyなしで基本機能が使える(制限あり)
- 軽量、起動高速

**API key設定(オプション):**

より高度な機能を使用したい場合、`~/.claude/mcp.json`を編集:
```json
"context7": {
  "env": {
    "CONTEXT7_API_KEY": "your_api_key_here"
  }
}
```

---

### 7. one-search (自動インストール、デフォルト無効)

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y one-search-mcp
```

**特徴:**
- マルチエンジンWeb検索(DuckDuckGo、Bing、SearXNG、Tavily)
- Windows + WSL2推奨: DuckDuckGoプロバイダー(Puppeteerなし)

**プロバイダー選択:**

`~/.claude/mcp.json`で設定変更可能:
```json
"one-search": {
  "disabled": false,
  "env": {
    "SEARCH_PROVIDER": "duckduckgo"  // 推奨: 軽量、高速
  }
}
```

**プロバイダー一覧:**
- `duckduckgo` - 推奨(WSL2環境で軽量)
- `bing` - Bing API keyが必要
- `searxng` - セルフホストSearXNGインスタンスが必要
- `tavily` - Tavily API keyが必要

**有効化方法:**

```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable one-search

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

---

### 8. memory (mcp-memory-service) (手動インストール、デフォルト無効)

完全ローカルで動作する長期記憶管理システムです。

**特徴:**
- 完全ローカル動作(SQLite-vec + ONNX埋め込み)
- 外部LLM不要
- プライバシー保護(クラウドにデータ送信なし)
- XDG Base Directory準拠
- 軽量(~50MB)

**前提条件:**
- Python3
- uv (Python package manager)
- Git

**インストール手順:**

```bash
# 1. リポジトリをクローン
mkdir -p ~/.local/opt/mcp-servers
cd ~/.local/opt/mcp-servers
git clone https://github.com/doobidoo/mcp-memory-service.git

# 2. 依存関係をインストール
cd mcp-memory-service
uv sync

# 3. 動作確認
uv run memory server --help
```

**データ保存場所:**
- データベース: `~/.local/share/mcp-memory-service/chroma_db/`
- バックアップ: `~/.local/share/mcp-memory-service/backups/`

**有効化方法:**

```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable memory

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**スラッシュコマンド連携:**

`/research`コマンドがmemoryと自動連携します:
- 調査結果を自動保存
- 同一トピック検索時に過去の調査を表示
- メモリ更新機能

詳細は`templates/.claude/commands/research.md`を参照してください。

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
- デフォルト有効MCP:
  - sequential-thinking(構造化思考支援)
  - context7(公式ドキュメント検索)
- デフォルト無効MCP(手動で有効化可能):
  - one-search(Web検索)
  - memory(長期記憶管理)
  - serena、file-search、git、github
- 既存ファイルの自動バックアップ(タイムスタンプ付き、最新3世代のみ保持)
- 依存関係チェック(Node.js、uv、Python3、Git)と警告表示

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

### one-search接続エラー

**症状:**
```
Failed to connect to one-search MCP
```

**解決方法:**
1. Node.js確認:
   ```bash
   node --version  # v18以降推奨
   ```

2. プロバイダー設定確認:
   ```bash
   cat ~/.claude/mcp.json | jq '.mcpServers["one-search"].env'
   ```

3. DuckDuckGoプロバイダーに変更(WSL2推奨):
   ```json
   "one-search": {
     "env": {
       "SEARCH_PROVIDER": "duckduckgo"
     }
   }
   ```

4. Puppeteer依存のプロバイダーを避ける(WSL2環境)

---

### mcp-memory-service起動エラー

**症状:**
```
Failed to start memory server
```

**解決方法:**
1. インストール確認:
   ```bash
   ls ~/.local/opt/mcp-servers/mcp-memory-service/
   ```

2. uv環境確認:
   ```bash
   cd ~/.local/opt/mcp-servers/mcp-memory-service
   uv sync
   ```

3. Python3確認:
   ```bash
   python3 --version
   ```

4. データディレクトリ作成:
   ```bash
   mkdir -p ~/.local/share/mcp-memory-service/{chroma_db,backups}
   ```

5. 手動起動テスト:
   ```bash
   cd ~/.local/opt/mcp-servers/mcp-memory-service
   uv run memory server
   ```

---

### context7 API制限

**症状:**
```
Rate limit exceeded
```

**解決方法:**
1. API keyなしの基本機能を使用している場合、レート制限があります

2. API keyを取得して設定:
   ```bash
   # ~/.claude/mcp.jsonを編集
   {
     "context7": {
       "env": {
         "CONTEXT7_API_KEY": "your_api_key"
       }
     }
   }
   ```

---

### Windows + WSL2環境でのMCP起動遅延

**症状:**
- MCP起動に時間がかかる(3秒以上)

**解決方法:**
1. Docker依存のMCPを無効化
2. DuckDuckGoプロバイダー使用(Puppeteerなし)
3. npx版MCPを優先(sequential-thinking、context7、one-search)
4. WSL2のメモリ制限を確認:
   ```bash
   # ~/.wslconfig
   [wsl2]
   memory=4GB
   processors=2
   ```

---

## 参考リンク

### 推奨構成(Phase 2d-2e)
- [context7 MCP](https://github.com/upstash/context7-mcp) - 公式ドキュメント検索 (Phase 2d)
- [one-search MCP](https://github.com/supercorp-ai/one-search-mcp) - マルチエンジンWeb検索 (Phase 2d)
- [mcp-memory-service](https://github.com/doobidoo/mcp-memory-service) - 完全ローカル長期記憶管理 (Phase 2d)
- [serena MCP](https://github.com/serena-ai/serena-mcp) - セマンティックコード解析 (Phase 2e)

### その他のMCP
- [sequential-thinking MCP](https://github.com/modelcontextprotocol/servers)
- [file-search MCP](https://github.com/Kurogoma4D/file-search-mcp)
- [git MCP](https://github.com/modelcontextprotocol/servers)
- [github MCP](https://github.com/modelcontextprotocol/servers)

---

## フィードバック

不明点や問題があれば、以下にissueを作成してください:
https://github.com/kiffveef/hagi/issues
