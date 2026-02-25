# hagi

Claude Code向けセットアップツール - Rust/Shell環境の開発効率化

## 概要

hagiは、Claude Codeの開発環境を素早くセットアップするためのCLIツールです。MCP(Model Context Protocol)サーバーの設定、パーミッション管理、プロジェクトテンプレートの配置を自動化します。

### 主な機能

- **グローバルセットアップ**: `~/.claude/settings.json`にパーミッション設定とhooks設定を配置
- **プロジェクトセットアップ**: プロジェクトごとの`.claude/`ディレクトリとMCP設定、テンプレートを配置
- **MCP管理**: context7、memory、serena、one-search、gitの統合設定
- **複数マシン同期**: `.claude`ディレクトリをプライベートGitリポジトリで同期（設計ドキュメント、タスク管理を共有）
- **安全な操作**: バックアップ自動作成・世代管理(最新3世代保持)、ドライラン、確認プロンプト機能
- **git操作防止**: `.claude/`ディレクトリの誤コミットを2層防御で自動ブロック

---

## インストール

### 前提条件

**必須:**
- Rust/cargo (1.80以降推奨) - hagiのビルド・インストールに必要
- jq - Claude Code hookによる`.claude/` git操作防止に必要

**推奨(MCP機能を使う場合):**
- Node.js (v18以降) - context7、one-search、memory用
- uv (Python package manager) - serena用

**jqのインストール:**
```bash
sudo apt install jq
```

**Note**: `hagi install --global`実行時に、上記ツールの存在が自動チェックされます。不足している場合は警告とインストール手順が表示されますが、インストール処理は継続されます。jqがない場合、`.claude/` git操作防止機能は無効になります(警告が表示されます)。

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

`~/.claude/settings.json`にパーミッション設定とhooks設定を配置します。

### 2. プロジェクトセットアップ

```bash
cd /path/to/your/project
hagi install
```

プロジェクトに`.claude/`ディレクトリ、MCP設定、テンプレートを配置します。
Claude Code 2.1+互換のため`.mcp.json`シンボリックリンクも作成されます。

詳細な使い方は[コマンドリファレンス](./docs/commands.md)を参照してください。

---

## セットアップされるMCP

| MCP | 用途 | デフォルト状態 | 備考 |
|-----|------|----------------|------|
| context7 | 公式ドキュメント検索 | ✅ 有効 | API keyなしで基本機能利用可 |
| memory | 長期記憶管理(完全ローカル) | ✅ 有効 | Memento(BGE-M3多言語) |
| one-search | Web検索(DuckDuckGo他) | ❌ 無効 | 軽量、Puppeteerなし |
| serena | コード解析・セマンティック検索 | ❌ 無効 | トークン節約設定済み |
| git | Git操作 | ❌ 無効 | uvx経由 |

---

## .claude/ git操作防止

hagiは`.claude/`ディレクトリの誤コミットを防ぐため、2層の防御機構を提供します。

### Layer 1: Claude Code PreToolUse Hook

Claude Codeが`git add .claude/`を実行しようとした時点でブロック:

```
❌ .claude/ is outside git workflow. Edit = done. No git operation needed.
📖 See: .claude/instructions/git-workflow.md
```

### Layer 2: Git pre-commit Hook

手動操作や他ツールからの誤コミットをバックアップ防御:

```
❌ ERROR: .claude/ files should not be committed!
To unstage: git restore --staged .claude/
```

### なぜ.claude/はgit管理しないのか

- ローカル設定は開発者ごとに異なる
- MCP設定にはマシン固有のパスが含まれる
- 同期が必要な場合は`hagi sync`を使用

**Note**: この機能にはjqが必要です。jqがインストールされていない場合、Layer 1は無効になりますが警告が表示されます。

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

| コマンド | 説明 |
|---------|------|
| `install` | グローバル/プロジェクト設定のインストール |
| `uninstall` | 設定の削除 |
| `status` | インストール状態確認 |
| `sync init` | .claude同期の初期化 |
| `sync pull` | 最新の.claude設定を取得 |
| `sync push` | .claude変更をpush |
| `sync status` | 同期状態確認 |
| `mcp list` | MCPサーバー一覧表示 |
| `mcp info` | MCPサーバー詳細表示 |
| `mcp enable` | MCPサーバー有効化 |
| `mcp disable` | MCPサーバー無効化 |
| `config show` | 設定内容表示 |
| `config validate` | 設定検証 |
| `config edit` | 設定編集 |
| `update` | hagiツール自体の更新 |

詳細は[コマンドリファレンス](./docs/commands.md)または`hagi <COMMAND> --help`で確認できます。

---

## ライセンス

MIT

## 参考リンク

- [Claude Code公式ドキュメント](https://docs.claude.com/en/docs/claude-code/)
- [Model Context Protocol](https://github.com/modelcontextprotocol)

## フィードバック

バグ報告・機能要望: https://github.com/kiffveef/hagi/issues
