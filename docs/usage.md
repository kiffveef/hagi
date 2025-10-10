# hagi使い方ガイド

hagiインストール後の詳しい使い方、MCPサーバーの活用方法について説明します。

---

## スラッシュコマンド

hagiでセットアップすると、`.claude/commands/`ディレクトリにスラッシュコマンドが配置されます。

### /st - Sequential Thinking

sequential-thinking MCPを使った構造化思考支援コマンド。複雑な問題を段階的に分析・解決します。

#### 基本的な使い方

```
/st データベース設計を最適化する方法を考えてください
```

Claude Codeが以下のステップで問題を分析します:
1. 問題の分析
2. 論理的なステップへの分解
3. アプローチの計画
4. 体系的な実行
5. 結果の検証

#### オプション

**`--search`: Web検索とライブラリドキュメント検索**

問題解決に外部情報が必要な場合に使用します。

```
/st --search Axum0.7でWebSocketを実装する方法
```

以下のツールを使用します:
- **WebSearch**: Web検索で最新情報を収集
- **Context7 MCP**: ライブラリの公式ドキュメントを取得

例:
- `Axum0.7`と指定すると、Context7がAxumの公式ドキュメントから情報を取得
- Web検索で実装例やベストプラクティスを検索

**`--todo`: TodoWriteツールで進捗管理**

各ステップの進捗をTodoWriteツールで追跡します。

```
/st --todo 新機能の実装計画を立ててください
```

動作:
- 分析開始時に全ステップのTodoリストを作成
- 各ステップ完了時にステータスを更新
- `.claude/TODO.md`が存在する場合、自動的に同期

**複数オプションの併用**

```
/st --search --todo Rust1.80の新機能を調査して実装計画を立ててください
```

---

## MCPサーバーの活用

hagiでセットアップされるMCPサーバーの使い方を説明します。

### sequential-thinking (デフォルト有効)

構造化思考を支援するMCPサーバー。

**用途:**
- 複雑な問題の段階的な分析
- 論理的な実装計画の立案
- エラー原因の体系的な調査

**使い方:**
`/st`コマンドまたは直接sequential-thinkingツールを使用

### context7 (デフォルト有効)

ライブラリ・フレームワークの公式ドキュメントを検索・取得するMCPサーバー。

**用途:**
- ライブラリのAPIドキュメント参照
- コード例の検索
- 最新バージョンの機能確認

**使い方:**
```
/st --search Tokio1.40のasync/await使い方
```

Context7が自動的にTokioの公式ドキュメントから情報を取得します。

### その他のMCPサーバー (デフォルト無効)

以下のMCPサーバーはデフォルトで無効化されています。必要に応じて有効化してください。

#### serena
コード解析・セマンティック検索を提供。

**有効化方法:**
`.claude/mcp.json`または`~/.claude/mcp.json`の`serena`セクションから`"disabled": true`を削除

**注意:**
- `.serena/`ディレクトリにキャッシュが作成される
- メモリ使用量が増加する可能性がある
- `.gitignore`に`.serena/`が自動追加される

#### file-search
高速ファイル検索を提供。

**有効化方法:**
`.claude/mcp.json`の`file-search`セクションから`"disabled": true`を削除

#### git
Git操作を提供。

**有効化方法:**
`.claude/mcp.json`の`git`セクションから`"disabled": true`を削除

#### github
GitHub連携(Issue/PR管理)を提供。

**有効化方法:**
1. GitHub Personal Access Tokenを取得
2. `.claude/mcp.json`の`GITHUB_PERSONAL_ACCESS_TOKEN`を設定
3. `"disabled": true`を削除

---

## プロジェクトガイドライン

`hagi install`実行後、`.claude/CLAUDE.md`が作成されます(将来実装予定)。

このファイルはプロジェクト固有のルール・ガイドラインを記述します:
- コーディング規約
- ブランチ運用ルール
- コミットメッセージ形式
- タスク管理方法

---

## パーミッション管理

`settings.local.json`で危険なコマンドを制限しています。

### デフォルトで拒否されるコマンド

- `rm -f`
- `sudo`
- `chmod 777`
- `git push --force`
- `git reset`

### 許可されるコマンド

- `cargo`関連
- `git add/commit/status/diff/log/branch/checkout/merge`
- `rg/fd/bat`
- `npx/uv`

詳細は`.claude/settings.local.json`を参照してください。

---

## トラブルシューティング

### /stコマンドが認識されない

Claude Codeを再起動してください。スラッシュコマンドは起動時に読み込まれます。

### Context7がライブラリを見つけられない

- ライブラリ名とバージョンを明示的に指定してください
  - 良い例: `Axum0.7`
  - 悪い例: `Axum`

### MCPサーバーが動作しない

1. `~/.claude/mcp.json`で該当MCPの`disabled`設定を確認
2. Node.js/uvがインストールされているか確認
3. Claude Codeを再起動

詳細は[MCP導入ガイド](./mcp-setup.md)を参照してください。

---

## 次のステップ

- [MCP導入ガイド](./mcp-setup.md): MCPサーバーの詳細設定
- [README](../README.md): hagiコマンドリファレンス
