# hagi

Claude Code向けセットアップツール - Rust/Shell環境の開発効率化

## 概要

hagiは、Claude Codeの開発環境を素早くセットアップするためのCLIツールです。MCP(Model Context Protocol)サーバーの設定、パーミッション管理、プロジェクトテンプレートの配置を自動化します。

### 主な機能

- **グローバルセットアップ**: `~/.claude/`配下にMCP設定とパーミッション設定を配置
- **プロジェクトセットアップ**: プロジェクトごとの`.claude/`ディレクトリとテンプレートを配置
- **MCP管理**: sequential-thinking、context7、serena、one-search、memory、gitの統合設定
- **安全な操作**: バックアップ自動作成・世代管理(最新3世代保持)、ドライラン、確認プロンプト機能

---

## インストール

### 前提条件

**必須:**
- Rust/cargo (1.80以降推奨) - hagiのビルド・インストールに必要

**推奨(MCP機能を使う場合):**
- Node.js (v18以降) - sequential-thinking、context7、one-search用
- uv (Python package manager) - mcp-memory-service用
- Python3 (3.10-3.13) - mcp-memory-service setup用
- Git - mcp-memory-service clone用

**Note**: `hagi install --global`実行時に、上記ツールの存在が自動チェックされます。不足している場合は警告とインストール手順が表示されますが、インストール処理は継続されます。

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

| MCP | 用途 | デフォルト状態 | 備考 |
|-----|------|----------------|------|
| sequential-thinking | 構造化思考支援 | ✅ 有効 | 軽量、起動即座 |
| context7 | 公式ドキュメント検索 | ✅ 有効 | API keyなしで基本機能利用可 |
| one-search | Web検索(DuckDuckGo他) | ❌ 無効 | 軽量、Puppeteerなし |
| memory | 長期記憶管理(完全ローカル) | ❌ 無効 | SQLite-vec + ONNX |
| serena | コード解析・セマンティック検索 | ❌ 無効 | トークン節約設定済み |
| git | Git操作 | ❌ 無効 | uvx経由 |

詳細なインストール方法とトラブルシューティングは [MCP導入ガイド](./docs/mcp-setup.md) を参照してください。

---

## ドキュメント

- **[インストールガイド](./docs/installation.md)**: hagiのインストール方法、前提条件、更新、アンインストール
- **[コマンドリファレンス](./docs/commands.md)**: 全コマンドの詳細説明、オプション、使用例
- **[使い方ガイド](./docs/usage.md)**: スラッシュコマンド(/st)の使い方、MCPサーバーの活用方法
- **[MCP導入ガイド](./docs/mcp-setup.md)**: MCPサーバーのインストール方法、設定、トラブルシューティング
- **[トラブルシューティング](./docs/troubleshooting.md)**: よくある問題と解決策、デバッグ方法
- **[開発ガイド](./docs/development.md)**: hagiへの貢献方法、開発環境のセットアップ

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
| `update` | hagiツール自体の更新 | ✅ 実装済 |

詳細は[コマンドリファレンス](./docs/commands.md)または`hagi <COMMAND> --help`で確認できます。

---

## ライセンス

MIT

---

## 参考リンク

- [Claude Code公式ドキュメント](https://docs.claude.com/en/docs/claude-code/)
- [Model Context Protocol](https://github.com/modelcontextprotocol)
