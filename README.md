# hagi

Claude Code向けセットアップツール - Rust/Shell環境の開発効率化

## 概要

hagiは、Claude Codeの開発環境を素早くセットアップするためのCLIツールです。MCP(Model Context Protocol)サーバーの設定、パーミッション管理、プロジェクトテンプレートの配置を自動化します。

### 主な機能

- **グローバルセットアップ**: `~/.claude/`配下にMCP設定とパーミッション設定を配置
- **プロジェクトセットアップ**: プロジェクトごとの`.claude/`ディレクトリとテンプレートを配置
- **MCP管理**: sequential-thinking、serena、file-search、git、github、context7の統合設定
- **安全な操作**: バックアップ、ドライラン、確認プロンプト機能

---

## インストール

### 前提条件

- Rust/cargo (1.80以降推奨)
- Node.js (v18以降推奨)
- uv (Python package manager)

### cargoでインストール

```bash
cargo install --git https://github.com/kiffveef/hagi hagi
```

### 更新

```bash
cargo install --git https://github.com/kiffveef/hagi hagi --force
```

---

## 使い方

### 1. グローバルセットアップ

Claude Code用のグローバル設定を`~/.claude/`に配置します。

```bash
# 設定をインストール
hagi install --global
# または
hagi install -g

# ドライラン（変更内容の確認のみ）
hagi install -g --dry-run
```

**セットアップ内容:**
- `~/.claude/mcp.json` - MCP設定（sequential-thinkingのみ有効化）
- `~/.claude/settings.json` - パーミッション設定

---

### 2. プロジェクトセットアップ

プロジェクトルートで実行し、`.claude/`ディレクトリとテンプレートを配置します。

```bash
cd /path/to/your/project
hagi install
```

**セットアップ内容:**
- `.claude/` ディレクトリ作成
- `.claude/CLAUDE.md` - プロジェクトガイドライン
- `.claude/instructions/` - 詳細インストラクション（10ファイル）
- `.claude/mcp.json` - プロジェクト用MCP設定
- `.claude/settings.local.json` - パーミッション設定
- `.gitignore` 更新(`/.claude/`, `/.serena/`, `/mcp.json`, `/settings.json`, `/settings.local.json`)

---

### 3. ステータス確認

```bash
hagi status
```

インストール状態、MCP設定、テンプレートの確認ができます。

---

### 4. アンインストール

```bash
# プロジェクト設定を削除
hagi uninstall

# グローバル設定を削除
hagi uninstall --global
# または
hagi uninstall -g

# 確認なしで削除
hagi uninstall -y
```

---

### 5. MCP管理（将来実装予定）

```bash
# MCPサーバー一覧
hagi mcp list

# MCPサーバー情報
hagi mcp info serena

# MCPサーバー有効化
hagi mcp enable serena
hagi mcp enable file-search

# MCPサーバー無効化
hagi mcp disable serena
```

---

## セットアップされるMCP

| MCP | 用途 | デフォルト状態 |
|-----|------|----------------|
| sequential-thinking | 構造化思考支援 | ✅ 有効 |
| serena | コード解析・セマンティック検索 | ❌ 無効 |
| file-search | 高速ファイル検索 | ❌ 無効 |
| git | Git操作 | ❌ 無効 |
| github | GitHub連携 | ❌ 無効 |
| context7 | ライブラリドキュメント検索 | ❌ 無効 |

詳細なインストール方法とトラブルシューティングは [MCP導入ガイド](./docs/mcp-setup.md) を参照してください。

---

## ドキュメント

- **[MCP導入ガイド](./docs/mcp-setup.md)**: MCPサーバーのインストール方法、設定、トラブルシューティング
- **インストールガイド** (作成予定): hagiのインストール詳細手順
- **コマンドリファレンス** (作成予定): 全コマンドの詳細説明
- **開発ガイド** (作成予定): hagiへの貢献方法

---

## コマンド一覧

```
hagi <COMMAND>

Commands:
  install    Install hagi configuration (global or project-specific)
  uninstall  Uninstall hagi configuration
  status     Show installation status
  update     Update hagi templates and configuration
  mcp        MCP server management commands
  config     Configuration management commands
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

詳細は `hagi <COMMAND> --help` で確認できます。

---

## ライセンス

MIT

---

## フィードバック・バグ報告

Issue・Pull Requestは歓迎します:
https://github.com/kiffveef/hagi/issues

---

## 参考リンク

- [Claude Code公式ドキュメント](https://docs.claude.com/en/docs/claude-code/)
- [Model Context Protocol](https://github.com/modelcontextprotocol)
