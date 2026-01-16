# hagi使い方ガイド

hagiインストール後の詳しい使い方、MCPサーバーの活用方法について説明します。

---

## スラッシュコマンド (Skills)

hagiでセットアップすると、`.claude/skills/`ディレクトリにスキルが配置されます。

> **Note**: Claude Code 2.1.3以降、commandsはskillsに統合されました。hagiは自動的に古い`commands/`を`commands.bak/`にバックアップし、新しい`skills/`形式をインストールします。

### スキルの更新(既存インストール済みの場合)

```bash
# カスタマイズ済みのCLAUDE.mdとinstructionsを保持しつつ更新
hagi install --skip CLAUDE.md --skip instructions
```

### /st - Structured Thinking

sequential-thinking MCPを使った構造化思考支援。複雑な問題を段階的に分析・解決します。memory連携で過去のパターンを再利用できます。

#### 使い方

```
/st データベース設計を最適化する方法を考えてください
/st Axum0.7でWebSocketを実装する方法
/st "認証フローの設計" --save    # パターンをmemoryに保存
/st "似た問題" --fresh           # memory検索をスキップ
```

#### オプション

| オプション | 説明 |
|-----------|------|
| `--save` | 思考パターンをmemoryに保存 |
| `--fresh` | memory検索をスキップして新規分析 |

#### 自動ツール選択

フラグ指定は不要です。文脈に応じて自動的に適切なツールを選択します:

| 状況 | 自動選択されるツール |
|------|---------------------|
| 外部情報・ドキュメントが必要 | WebSearch, Context7 MCP |
| 複数ステップのタスク | TodoWrite |
| `.claude/TODO.md`が存在 | 自動同期 |
| 類似パターンがmemoryに存在 | 自動検索・再利用提案 |

---

### /design - 設計文書

設計決定や仕様を`.claude/designs/`に文書化します。

#### 使い方

```
/design "authentication flow"
/design "error handling strategy" --memory   # memoryにも保存
```

#### オプション

| オプション | 説明 |
|-----------|------|
| `--memory` | 設計をmemoryにも保存(プロジェクト横断参照用) |

#### 特徴

- 設計意図を永続化(git追跡可能)
- 既存設計があれば自動検出して更新
- ADR(Architecture Decision Records)として活用可能

---

### /note - 会話メモ

直近の会話を簡潔なマークダウンファイルにまとめます。

#### 使い方

```
/note                    # note-YYYYMMDD-HHMMSS.md として保存
/note chat-mode.md       # 指定したファイル名で保存
```

#### 出力内容

- 概要(1〜2文)
- 議論の要点、決定事項
- 成果物、変更内容
- 次のアクション(あれば)

---

### /serena - コード分析

serena + memento でコードパターンを検索・分析します。

#### 使い方

```
/serena "error handling in async functions"
/serena "REST API pagination"
/serena "authentication middleware" --skip-memory
/serena "database pooling" --save-pattern
```

#### 特徴

- 現在のコードベースをserenaで検索
- 過去パターンをmementoから自動検索
- 実装アプローチの比較・改善提案
- `--save-pattern`で分析結果を長期記憶に保存

---

### /review - コードレビュー

サードパーティ視点でのコードレビューと改善提案。

#### 使い方

```
/review src/commands/install.rs
/review src/commands/ --focus architecture
/review --diff                              # 直近の変更をレビュー
/review src/utils.rs --refactor             # 具体的なリファクタリング提案
```

#### オプション

| オプション | 説明 |
|-----------|------|
| `--strict` | 軽微な問題も含める |
| `--focus <area>` | `security`, `performance`, `readability`, `architecture` |
| `--refactor` | 具体的なコード改善案を提示 |
| `--diff` | git diffをレビュー |

---

## チャットモード

プロジェクトの制約なく、Claudeと気軽に会話するための専用スペースです。

### セットアップ

```bash
hagi install --chat
```

これにより `~/.chat` ディレクトリが作成され、軽量な設定が配置されます。

### 使い方

```bash
cd ~/.chat && claude
```

### 特徴

- **MCPサーバーなし**: 純粋な会話のみ、軽量起動
- **プロジェクト文脈なし**: 開発ルールに縛られない自由な対話
- **カスタマイズ可能**: `~/.chat/CLAUDE.md` を編集して独自ルールを設定

### カスタマイズ例

`~/.chat/CLAUDE.md` を編集:

```markdown
# Chat Mode

## 会話スタイル
- 敬語で話してください
- 技術的な質問には詳しく回答
- 雑談は短めに

## 興味のあるトピック
- Rust
- システム設計
- 機械学習
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
/st Tokio1.40のasync/await使い方を調べて
```

Context7が自動的にTokioの公式ドキュメントから情報を取得します。

### one-search (デフォルト無効)

マルチエンジンWeb検索。DuckDuckGo、Bing、SearXNG、Tavily対応。

**用途:** Web検索

**有効化:**
```bash
hagi mcp enable one-search
```

### memory (デフォルト有効)

Memento - 完全ローカルの長期記憶管理。BGE-M3多言語埋め込み。

**用途:** 思考パターン・設計決定の長期保存、`/st`・`/design`・`/serena`と連携

**特徴:** 完全ローカル、npx一発起動、多言語対応

**無効化(必要な場合のみ):**
```bash
hagi mcp disable memory
```

### serena (デフォルト無効)

セマンティックコード解析・検索。

**用途:** コード検索・解析、`/serena`・`/review`と連携

**有効化:**
```bash
hagi mcp enable serena
```

**キャッシュ:** `.serena/` (自動的に`.gitignore`追加)

**LSP対応:** Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++

### その他のMCPサーバー (デフォルト無効)

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

`hagi install`実行後、以下のファイルが作成されます:

### .claude/CLAUDE.md

プロジェクト固有のルール・ガイドラインテンプレート。
必要に応じてカスタマイズしてください:
- プロジェクト概要
- コーディング規約
- ブランチ運用ルール
- コミットメッセージ形式
- タスク管理方法

### .claude/instructions/

プロジェクト開発時にClaude Codeが自動的に読み込むインストラクションファイル。

**git-workflow.md**:
- Git操作の重要ルール(MUST/NEVER形式)
- ブランチワークフロー
- コミットメッセージ形式
- `.claude/`をgit追跡しないルール

**task-management.md**:
- TodoWriteツールの使い方
- タスク状態管理
- `.claude/TODO.md`との同期方法

**tools.md**:
- 推奨ツールの使い方
- `rg`(ripgrep), `bat`, `fd`の使用法
- Claude Codeツールの活用

これらのファイルは新規セッションで自動的に読み込まれ、開発ルールが徹底されます。

---

## 複数マシン間での.claude同期

複数のマシン(デスクトップ、ノートPC、リモートサーバーなど)でプロジェクトを開発する場合、`.claude`ディレクトリの同期が重要です。設計ドキュメント、タスク管理、プロジェクト固有の設定を共有できます。

### なぜ.claudeを同期するのか？

`.claude`ディレクトリには以下のような重要な情報が含まれます:

- **CLAUDE.md**: プロジェクトの設計指針、ガイドライン
- **TODO.md**: タスク管理、進捗状況
- **instructions/**: プロジェクト固有のルール
- **skills/**: スラッシュコマンド (スキル)
- **mcp.json**: プロジェクトローカルのMCP設定

これらをマシン間で同期することで、一貫した開発環境を維持できます。

### セットアップ手順

#### 初回セットアップ（Machine A）

```bash
cd myproject

# 1. プロジェクト設定をインストール
hagi install

# 2. 同期を初期化
hagi sync init

# 対話的に確認プロンプトが表示されます:
# 📦 Creating private repository
#   Repository name: myproject-claude
#   Visibility: Private
# Proceed? [Y/n]: y

# → GitHubにmyproject-claudeリポジトリが自動作成されます
# → .claude内でgit init + push完了
```

**gh CLIがない場合:**

手動でGitHubリポジトリを作成してから:
```bash
hagi sync init git@github.com:yourname/myproject-claude.git
```

#### 別マシン（Machine B）

```bash
# 1. プロジェクトをクローン
git clone git@github.com:yourname/myproject.git
cd myproject

# 2. .claudeを取得
hagi sync pull

# 確認
ls -la .claude
```

### 日常のワークフロー

#### 作業開始前

```bash
# 最新の.claudeを取得
hagi sync pull
```

これにより、他のマシンで更新されたTODO.md、CLAUDE.mdなどが同期されます。

#### 作業中

通常通り開発を進めます:
- TODO.mdを更新
- CLAUDE.mdに設計メモを追加
- instructions/を編集

#### 作業終了時

```bash
# 状態確認
hagi sync status

# 変更をpush
hagi sync push -m "Update TODO: Complete authentication feature"

# または、デフォルトメッセージで
hagi sync push
```

### 実践例

#### ケース1: デスクトップ → ノートPC

**デスクトップ（朝）:**
```bash
cd ~/projects/myproject
hagi sync pull  # 前日のノートPCでの変更を取得

# 作業...
# TODO.mdを更新

hagi sync push -m "Add database migration tasks"
```

**ノートPC（夜）:**
```bash
cd ~/projects/myproject
hagi sync pull  # デスクトップでの変更を取得

# 作業...
# TODO.mdをさらに更新

hagi sync push -m "Complete migration implementation"
```

#### ケース2: チーム開発（複数人）

`.claude`リポジトリへのアクセス権を付与することで、チーム全体で設計ドキュメントを共有できます:

```bash
# チームメンバーがリポジトリをクローン
git clone git@github.com:yourname/myproject-claude.git .claude

# または
hagi sync pull
```

**注意:**
- チーム開発では、コンフリクトが発生する可能性があります
- 定期的に`hagi sync pull`で同期することを推奨

#### ケース3: リモートサーバーでの開発

```bash
# ローカルマシン
hagi sync push -m "Add deployment instructions"

# SSH接続してリモートサーバーへ
ssh user@remote-server
cd /path/to/project

# リモートで変更を取得
hagi sync pull

# リモートで作業...
# 環境固有の設定をCLAUDE.mdに追加

hagi sync push -m "Add remote server setup notes"
```

### トラブルシューティング

#### コンフリクトが発生した場合

```bash
$ hagi sync pull
# エラー: conflict in TODO.md

# .claudeディレクトリ内で手動解決
cd .claude
git status
# コンフリクトファイルを編集

git add TODO.md
git rebase --continue

# または、自分の変更を優先
git rebase --skip

# 完了したらpush
hagi sync push
```

#### 同期状態の確認

```bash
hagi sync status

# 出力例:
# 📊 .claude sync status:
#
# On branch main
# Your branch is ahead of 'origin/main' by 2 commits.
#
# Changes not staged for commit:
#   modified:   TODO.md
```

#### .claudeをGitリポジトリ化し忘れた場合

```bash
# 後から初期化も可能
hagi sync init
```

既存の`.claude`内容は保持されます。

### ベストプラクティス

1. **定期的な同期**: 作業開始前と終了時に必ず`pull`と`push`
2. **意味のあるコミットメッセージ**: 変更内容がわかるメッセージを記述
3. **コンフリクト回避**: 同じファイルを同時に編集しない
4. **プライベートリポジトリ必須**: `.claude`には設計情報が含まれるため

### セキュリティ考慮事項

- ✅ プライベートリポジトリを使用（公開しない）
- ✅ 機密情報（APIキー等）は`.claude`に含めない
- ✅ チーム開発では適切なアクセス制御を設定

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

1. `hagi mcp list`で該当MCPの有効/無効状態を確認
2. Node.js/uvがインストールされているか確認
3. Claude Codeを再起動

詳細は[MCP導入ガイド](./mcp-setup.md)を参照してください。

---

## 次のステップ

- [コマンドリファレンス](./commands.md): hagiコマンドの詳細説明
- [MCP導入ガイド](./mcp-setup.md): MCPサーバーの詳細設定
- [README](../README.md): hagiの概要と基本的な使い方
