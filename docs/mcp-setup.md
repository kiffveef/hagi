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

---

## MCPサーバー一覧

### 推奨構成

軽量・高速・完全ローカル動作を重視した構成です。

| MCP | 用途 | インストール方法 | デフォルト状態 | 備考 |
|-----|------|------------------|----------------|------|
| sequential-thinking | 構造化思考支援 | npx (自動) | ✅ 有効 | 軽量、起動即座 |
| context7 | 公式ドキュメント検索 | npx (自動) | ✅ 有効 | 軽量、API keyなしで基本機能利用可 |
| memory | 長期記憶管理 | npx (自動) | ✅ 有効 | Memento(BGE-M3埋め込み、SQLite) |
| one-search | Web検索 | npx (自動) | ❌ 無効 | DuckDuckGo推奨(Puppeteerなし) |
| serena | セマンティックコード解析 | uvx (自動) | ❌ 無効 | XDG準拠、キャッシュ管理 |

### その他のMCP

| MCP | 用途 | インストール方法 | デフォルト状態 |
|-----|------|------------------|----------------|
| git | Git操作 | uvx (自動) | ❌ 無効 |

---

## インストール手順

### 1. sequential-thinking (自動インストール)

`hagi install`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @modelcontextprotocol/server-sequential-thinking
```

---

### 2. serena (uvx経由インストール、デフォルト無効)

有効化時にuvx経由でGitHubリポジトリから自動的にインストールされます。

**前提条件:**
- uv (Python package manager)

**インストール方法:**

serenaはuvxが自動的にGitHubから取得するため、手動インストールは不要です。

**手動確認:**
```bash
uvx --from git+https://github.com/oraios/serena serena start-mcp-server --help
```

**特徴:**
- セマンティックコード検索・解析
- LSP対応言語: Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++
- プロジェクトごとのキャッシュ管理

**キャッシュ管理:**
- デフォルトキャッシュ: `~/.cache/serena/`
- プロジェクトキャッシュ: `.serena/` (自動的に`.gitignore`に追加済み)
- 定期クリーンアップ推奨: 30日以上経過したファイル削除

**クリーンアップスクリプト:**
```bash
# プロジェクトルートで実行
find .serena/ -type f -mtime +30 -delete
```

**有効化方法:**
```bash
hagi mcp enable serena

# または手動で.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**連携:**
- `/serena`コマンド: serena + Mementoで過去パターン検索

**トラブルシューティング:**

uvが見つからない場合:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
source ~/.cargo/env
```

初回起動が遅い場合:
- GitHubからクローンするため初回は時間がかかります
- 2回目以降はuvxがキャッシュするため高速になります

---

### 3. git (自動インストール)

有効化時にuvx経由で自動的にインストールされます。

**前提条件:**
- uvのインストールが必要

**手動確認:**
```bash
uvx mcp-server-git --repository .
```

---

### 4. context7 (自動インストール、デフォルト有効)

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

より高度な機能を使用したい場合、`.claude/mcp.json`を編集:
```json
"context7": {
  "env": {
    "CONTEXT7_API_KEY": "your_api_key_here"
  }
}
```

---

### 5. one-search (自動インストール、デフォルト無効)

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y one-search-mcp
```

**特徴:**
- マルチエンジンWeb検索(DuckDuckGo、Bing、SearXNG、Tavily)
- DuckDuckGoプロバイダー推奨(Puppeteerなし、軽量)

**プロバイダー選択:**

`.claude/mcp.json`で設定変更可能:
```json
"one-search": {
  "disabled": false,
  "env": {
    "SEARCH_PROVIDER": "duckduckgo"  // 推奨: 軽量、高速
  }
}
```

**プロバイダー一覧:**
- `duckduckgo` - 推奨(軽量)
- `bing` - Bing API keyが必要
- `searxng` - セルフホストSearXNGインスタンスが必要
- `tavily` - Tavily API keyが必要

**有効化方法:**

```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable one-search

# または手動で.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

---

### 6. memory (Memento) (自動インストール、デフォルト有効)

シンプルで軽量な長期記憶管理システムです。

**特徴:**
- npx経由の自動インストール(手動セットアップ不要)
- BGE-M3多言語埋め込み(日本語・英語対応)
- SQLiteベース(軽量、高速)
- 完全ローカル動作(クラウド不要)
- XDG Base Directory準拠

**前提条件:**
- Node.js v18以降

**インストール:**

手動インストール不要。`hagi mcp enable memory`または設定変更時にnpxが自動でインストールします。

**手動確認:**
```bash
npx -y @iachilles/memento@latest
```

**データ保存場所:**
- データベース: `~/.local/share/claude-memory/memory.db`
- XDG準拠: `${XDG_DATA_HOME:-$HOME/.local/share}/claude-memory/memory.db`

**有効化方法:**

```bash
hagi mcp enable memory

# または手動で.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**主要API:**

| ツール | 用途 |
|--------|------|
| `search_nodes` | エンティティ検索 |
| `create_entities` | エンティティ作成 |
| `add_observations` | 既存エンティティに情報追加 |
| `delete_entities` | エンティティ削除 |

**スラッシュコマンド連携:**

`/st`と`/design`がmemoryと連携します:
- `/st --save`: 思考パターンを保存
- `/st`: 類似パターンを自動検索
- `/design --memory`: 設計決定をプロジェクト横断で保存

**旧バージョン(mcp-memory-service)からの移行:**

mcp-memory-serviceを使用していた場合、データの自動移行はサポートされていません。
旧設定は`icebox/mcp-memory-service`ブランチに保存されています。

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
- `~/.claude/settings.json`の作成・マージ (パーミッション、hooks設定)
- 既存ファイルの自動バックアップ(タイムスタンプ付き、最新3世代のみ保持)
- 依存関係チェック(Node.js、uv、Python3、Git)と警告表示

**Note**: グローバルMCP設定は管理しません。MCPサーバーはプロジェクトごとに`.claude/mcp.json`で設定します。

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
- `.claude/mcp.json` - プロジェクト用MCP設定
- `.mcp.json` → `.claude/mcp.json` (シンボリックリンク) - Claude Code 2.1+互換
- `.claude/settings.local.json`のコピー
- `.gitignore`の更新

**デフォルト有効MCP:**
- sequential-thinking(構造化思考支援)
- context7(公式ドキュメント検索)
- memory(長期記憶管理)

**デフォルト無効MCP(手動で有効化可能):**
- one-search(Web検索)
- serena、git

---

## 個別の有効化・無効化

### 方法1: hagiコマンド

```bash
# MCPサーバーを有効化
hagi mcp enable serena
hagi mcp enable memory

# MCPサーバーを無効化
hagi mcp disable serena

# 一覧表示
hagi mcp list
```

### 方法2: 手動編集

`.claude/mcp.json`を編集:

```json
{
  "mcpServers": {
    "serena": {
      "command": "uvx",
      "args": ["--from", "git+https://github.com/oraios/serena", "serena", "start-mcp-server", "--context", "claude-code"],
      "disabled": false  // true → false に変更
    }
  }
}
```

**Note**: `.mcp.json`は`.claude/mcp.json`へのシンボリックリンクです。どちらを編集しても同じです。

**変更後の反映:**
- Claude Codeを再起動

---

## トラブルシューティング

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
   cat .claude/mcp.json | jq
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
   cat .claude/mcp.json | jq '.mcpServers["one-search"].env'
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

### Memento (memory) 起動エラー

**症状:**
```
Failed to start memory server
```

**解決方法:**
1. Node.jsバージョン確認:
   ```bash
   node --version  # v18以降推奨
   ```

2. 手動起動テスト:
   ```bash
   npx -y @iachilles/memento@latest
   ```

3. データディレクトリの権限確認:
   ```bash
   ls -la ~/.local/share/claude-memory/
   # 存在しない場合は自動作成されます
   ```

4. Claude Code再起動

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
   # .claude/mcp.jsonを編集
   {
     "context7": {
       "env": {
         "CONTEXT7_API_KEY": "your_api_key"
       }
     }
   }
   ```

---

## 参考リンク

- [context7 MCP](https://github.com/upstash/context7-mcp) - 公式ドキュメント検索
- [one-search MCP](https://github.com/supercorp-ai/one-search-mcp) - マルチエンジンWeb検索
- [Memento (@iachilles/memento)](https://www.npmjs.com/package/@iachilles/memento) - BGE-M3多言語メモリ管理
- [serena MCP](https://github.com/oraios/serena) - セマンティックコード解析
- [sequential-thinking MCP](https://github.com/modelcontextprotocol/servers)
- [git MCP](https://github.com/modelcontextprotocol/servers)

---

## フィードバック

不明点や問題があれば、以下にissueを作成してください:
https://github.com/kiffveef/hagi/issues
