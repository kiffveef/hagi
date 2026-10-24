# hagi

Claude Code向けセットアップツール。パーミッション、hook、MCP設定、指示書・スキルのテンプレートを一括配置する。

## 特徴

- **一括セットアップ**: グローバル(`~/.claude`)とプロジェクト(`.claude/`)の設定をコマンド1つで配置
- **MCP管理**: context7、memory、serenaの有効化/無効化
- **複数マシン同期**: `.claude/`をプライベートGitリポジトリで同期
- **安全な操作**: 上書き前にバックアップ(最新1世代保持)、ドライラン対応
- **誤コミット防止**: `.claude/`へのgit操作をPreToolUse hookとgit pre-commitの2層でブロック
- **更新検知**: 配置時のhagiバージョンを記録し、hagiが新しくなるとセッション開始時に`hagi install --refresh`を促す

## クイックスタート

前提条件: Rust1.85以降(edition2024)、jq(`.claude/` git保護に使用)

```bash
cargo install --git https://github.com/kiffveef/hagi hagi

hagi install --global        # グローバル設定
cd /path/to/your/project
hagi install                 # プロジェクト設定
```

## コマンド

| コマンド | 説明 |
|---------|------|
| `install` | グローバル/プロジェクト設定のインストール(`--refresh`で導入済み設定の一括更新、`--only`でカテゴリ選択) |
| `uninstall` | 設定の削除 |
| `status` | インストール状態の確認 |
| `update` | hagi自体の更新 |
| `sync` | `.claude/`の同期(init/pull/push/status) |
| `mcp` | MCP管理(list/info/enable/disable) |
| `config` | 設定管理(show/validate/edit) |

詳細は`hagi <COMMAND> --help`か[コマンドリファレンス](./docs/commands.md)を参照。

## ドキュメント

| ドキュメント | 内容 |
|-------------|------|
| [インストールガイド](./docs/installation.md) | 前提条件、インストール、更新 |
| [コマンドリファレンス](./docs/commands.md) | 全コマンドの詳細 |
| [使い方ガイド](./docs/usage.md) | スキル、チャットモード、ワークフロー |
| [MCP導入ガイド](./docs/mcp-setup.md) | MCPサーバーの設定と活用 |
| [トラブルシューティング](./docs/troubleshooting.md) | よくある問題と解決策 |
| [開発ガイド](./docs/development.md) | hagiへの貢献方法 |

## ライセンス

MIT

## リンク

- [Claude Code公式ドキュメント](https://code.claude.com/docs/en/)
- [Model Context Protocol](https://github.com/modelcontextprotocol)
- [バグ報告・機能要望](https://github.com/kiffveef/hagi/issues)
