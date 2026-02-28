# hagi

Claude Code向けセットアップツール

## 概要

hagiは、Claude Codeの開発環境セットアップを自動化するCLIツールです。MCP設定、パーミッション管理、プロジェクトテンプレートの配置を一括で行います。

- **グローバル/プロジェクトセットアップ**: パーミッション、MCP設定、テンプレートを配置
- **MCP管理**: context7、memory、serena、one-search、gitの有効化/無効化
- **複数マシン同期**: `.claude`ディレクトリをプライベートGitリポジトリで同期
- **安全な操作**: バックアップ自動作成(最新1世代保持)、ドライラン対応
- **git操作防止**: `.claude/`の誤コミットを2層防御で自動ブロック

---

## クイックスタート

### 前提条件

- Rust/cargo(1.80以降推奨)
- jq(`.claude/` git保護に必要)

詳細は[インストールガイド](./docs/installation.md)を参照。

### インストール

```bash
cargo install --git https://github.com/kiffveef/hagi hagi
```

### セットアップ

```bash
# グローバル設定
hagi install --global

# プロジェクト設定
cd /path/to/your/project
hagi install
```

---

## ドキュメント

| ドキュメント | 内容 |
|-------------|------|
| [インストールガイド](./docs/installation.md) | 前提条件、インストール、更新 |
| [コマンドリファレンス](./docs/commands.md) | 全コマンドの詳細 |
| [使い方ガイド](./docs/usage.md) | スキル、チャットモード、ワークフロー |
| [MCP導入ガイド](./docs/mcp-setup.md) | MCPサーバーの設定と活用 |
| [トラブルシューティング](./docs/troubleshooting.md) | よくある問題と解決策 |
| [開発ガイド](./docs/development.md) | hagiへの貢献方法 |

---

## コマンド一覧

| コマンド | 説明 |
|---------|------|
| `install` | グローバル/プロジェクト設定のインストール |
| `uninstall` | 設定の削除 |
| `status` | インストール状態確認 |
| `update` | hagiツール自体の更新 |
| `sync` | .claude同期(init/pull/push/status) |
| `mcp` | MCP管理(list/info/enable/disable) |
| `config` | 設定管理(show/validate/edit) |

詳細は[コマンドリファレンス](./docs/commands.md)または`hagi <COMMAND> --help`で確認。

---

## ライセンス

MIT

## リンク

- [Claude Code公式ドキュメント](https://docs.claude.com/en/docs/claude-code/)
- [Model Context Protocol](https://github.com/modelcontextprotocol)
- [バグ報告・機能要望](https://github.com/kiffveef/hagi/issues)
