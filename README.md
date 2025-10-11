# hagi

Claude Code向けセットアップツール - Rust/Shell環境の開発効率化

## 概要

hagiは、Claude Codeの開発環境を素早くセットアップするためのCLIツールです。MCP(Model Context Protocol)サーバーの設定、パーミッション管理、プロジェクトテンプレートの配置を自動化します。

### 主な機能

- **グローバルセットアップ**: `~/.claude/`配下にMCP設定とパーミッション設定を配置
- **プロジェクトセットアップ**: プロジェクトごとの`.claude/`ディレクトリとテンプレートを配置
- **MCP管理**: sequential-thinking、serena、file-search、git、github、context7の統合設定
- **安全な操作**: バックアップ自動作成・世代管理(最新3世代保持)、ドライラン、確認プロンプト機能

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

## クイックスタート

### 1. グローバルセットアップ

```bash
hagi install --global
```

`~/.claude/`配下にMCP設定とパーミッション設定を配置します。

### 2. プロジェクトセットアップ

```bash
cd /path/to/your/project
hagi install
```

プロジェクトに`.claude/`ディレクトリとテンプレートを配置します。

詳細な使い方は[コマンドリファレンス](./docs/commands.md)を参照してください。

---

## セットアップされるMCP

| MCP | 用途 | デフォルト状態 |
|-----|------|----------------|
| sequential-thinking | 構造化思考支援 | ✅ 有効 |
| context7 | ライブラリドキュメント検索 | ✅ 有効 |
| serena | コード解析・セマンティック検索 | ❌ 無効 |
| file-search | 高速ファイル検索 | ❌ 無効 |
| git | Git操作 | ❌ 無効 |
| github | GitHub連携 | ❌ 無効 |

詳細なインストール方法とトラブルシューティングは [MCP導入ガイド](./docs/mcp-setup.md) を参照してください。

---

## ドキュメント

- **[コマンドリファレンス](./docs/commands.md)**: 全コマンドの詳細説明、オプション、使用例
- **[使い方ガイド](./docs/usage.md)**: スラッシュコマンド(/st)の使い方、MCPサーバーの活用方法
- **[MCP導入ガイド](./docs/mcp-setup.md)**: MCPサーバーのインストール方法、設定、トラブルシューティング
- **インストールガイド** (作成予定): hagiのインストール詳細手順
- **開発ガイド** (作成予定): hagiへの貢献方法

---

## コマンド一覧

| コマンド | 説明 | 状態 |
|---------|------|------|
| `install` | グローバル/プロジェクト設定のインストール | ✅ 実装済 |
| `uninstall` | 設定の削除 | ✅ 実装済 |
| `status` | インストール状態確認 | ✅ 実装済 |
| `mcp list` | MCPサーバー一覧表示 | ✅ 実装済 |
| `mcp info` | MCPサーバー詳細表示 | ✅ 実装済 |
| `mcp enable` | MCPサーバー有効化 | ✅ 実装済 |
| `mcp disable` | MCPサーバー無効化 | ✅ 実装済 |
| `config show` | 設定内容表示 | ✅ 実装済 |
| `config validate` | 設定検証 | ✅ 実装済 |
| `config edit` | 設定編集 | ✅ 実装済 |
| `update` | テンプレート更新 | ⏳ 将来実装 |

詳細は[コマンドリファレンス](./docs/commands.md)または`hagi <COMMAND> --help`で確認できます。

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
