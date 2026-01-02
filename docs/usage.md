# hagi使い方ガイド

hagiインストール後の詳しい使い方、MCPサーバーの活用方法について説明します。

---

## スラッシュコマンド

hagiでセットアップすると、`.claude/commands/`ディレクトリにスラッシュコマンドが配置されます。

### コマンドの更新(既存インストール済みの場合)

新しいスラッシュコマンドが追加された場合、以下で更新できます:

```bash
# カスタマイズ済みのCLAUDE.mdとinstructionsを保持しつつ更新
hagi install --skip CLAUDE.md --skip instructions
```

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

### /research - 統合調査 (Phase 2d)

one-search + context7 + mcp-memory-serviceを組み合わせた包括的なリサーチコマンド。

#### 基本的な使い方

```
/research Rust async programming
/research "Axum0.7 CORS configuration"
```

#### ワークフロー

1. **Step 0**: 自動メモリチェック(過去の調査を確認)
2. **Step 1**: Web検索(one-search) - 実践例、チュートリアル
3. **Step 2**: 公式ドキュメント(context7) - 正確な仕様
4. **Step 3**: 統合・分析 - 両方の情報を総合
5. **Step 3b**: 現在のコードベース統合(serena) - プロジェクトへの適用提案 [Phase 2e]
6. **Step 4**: メモリ保存(mcp-memory-service) - 長期記憶に保存

#### オプション

```
/research <topic> --no-save          # メモリに保存しない
/research <topic> --force            # メモリチェックをスキップ
/research <topic> --memory-only      # メモリ検索のみ
```

#### 特徴

- 過去の調査結果を自動検出して再利用
- 公式ドキュメント(context7)とWeb検索(one-search)の両方を活用
- 調査結果を長期記憶に自動保存
- 現在のプロジェクトへの適用提案(serena統合、Phase 2e)

---

### /note - 会話メモ

直近の会話を簡潔なマークダウンファイルにまとめます。

#### 基本的な使い方

```
/note                    # note-YYYYMMDD-HHMMSS.md として保存
/note chat-mode.md       # 指定したファイル名で保存
```

#### 出力内容

- 概要(1〜2文)
- 議論の要点、決定事項
- 成果物、変更内容
- 次のアクション(あれば)

#### 特徴

- 2〜3会話分を一貫性のある文章で要約
- 技術的な詳細(ファイルパス、コマンド等)を適宜含む
- 文脈を知らない人が読んでも理解できる形式

---

### /code-pattern - パターン検索 (Phase 2e)

serena + mcp-memory-serviceで過去のコーディングパターンを検索・分析します。

#### 基本的な使い方

```
/code-pattern error handling in async functions
/code-pattern "REST API pagination implementation"
/code-pattern authentication middleware
```

#### ワークフロー

1. **Step 1**: 現在のコードベース検索(serena) - 現在のプロジェクトのパターン
2. **Step 2**: 過去パターン検索(mcp-memory-service) - 長期記憶からパターン検索
3. **Step 3**: パターン比較・分析 - 実装アプローチの比較
4. **Step 4**: 推奨事項 - ベストプラクティスと改善提案

#### オプション

```
/code-pattern <description> --current-only   # 現在のコードベースのみ
/code-pattern <description> --memory-only    # 過去パターンのみ
```

#### 特徴

- 現在のコードと過去のパターンを横断的に検索
- 実装アプローチの比較・分析
- ベストプラクティスの提案
- リファクタリング・改善の具体的な提案
- パターン分析結果を長期記憶に保存

#### 使用例

```
# エラーハンドリングパターンを検索
/code-pattern error handling in async functions

# 出力例:
## Error Handling Pattern Analysis
### Current Project (serena)
- src/handlers.rs:45 - Result<T, E>パターン
- src/api.rs:123 - anyhowパターン

### Past Projects (memory)
- web-api-v2: thiserrorカスタムエラー型
- data-processor: バックトレース付きエラー

### Recommendations
1. thiserror + anyhowの組み合わせを推奨
2. 具体的なリファクタリング提案
3. 再利用可能なコンポーネント
```

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
/st --search Tokio1.40のasync/await使い方
```

Context7が自動的にTokioの公式ドキュメントから情報を取得します。

### Phase 2d追加MCPサーバー (デフォルト無効)

#### one-search

マルチエンジンWeb検索を提供。DuckDuckGo、Bing、SearXNG、Tavilyに対応。

**用途:**
- Web検索で実践例、チュートリアルを探す
- `/research`コマンドと連携

**有効化方法:**
```bash
# 将来のhagiコマンド
hagi mcp enable one-search

# または手動で~/.claude/mcp.jsonを編集
```

**推奨設定:**
Windows + WSL2環境では`DuckDuckGo`プロバイダーを推奨(Puppeteerなし、軽量)

#### memory (mcp-memory-service)

完全ローカルの長期記憶管理。SQLite-vec + ONNX埋め込み。

**用途:**
- 調査結果、コーディングパターンの長期保存
- プロジェクト横断的な知識管理
- `/research`、`/code-pattern`コマンドと連携

**セットアップ:**
```bash
# 1. リポジトリクローン
mkdir -p ~/.local/opt/mcp-servers
git clone https://github.com/doobidoo/mcp-memory-service.git ~/.local/opt/mcp-servers/mcp-memory-service

# 2. 依存関係インストール
cd ~/.local/opt/mcp-servers/mcp-memory-service
uv sync
```

**有効化方法:**
```bash
hagi mcp enable memory
```

**特徴:**
- 完全ローカル(外部API不要、プライバシー保護)
- 軽量(~50MB、Docker不要)
- XDG Base Directory準拠

### Phase 2e追加MCPサーバー (デフォルト無効)

#### serena

セマンティックコード解析・検索を提供。

**用途:**
- 現在のプロジェクトのコード検索・解析
- `/code-pattern`コマンドで現在のパターン検索
- `/research`コマンドStep 3bで適用提案

**有効化方法:**
```bash
hagi mcp enable serena
```

**キャッシュ管理:**
- グローバルキャッシュ: `~/.cache/serena` (XDG準拠)
- プロジェクトキャッシュ: `.serena/` (自動的に`.gitignore`追加)
- 定期クリーンアップ推奨: `find .serena/ -type f -mtime +30 -delete`

**LSP対応言語:**
Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++

**連携:**
- `memory` (mcp-memory-service)と組み合わせて短期・長期記憶統合
- 現在のコード(serena) + 過去のパターン(memory)で最適解を提案

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

- [コマンドリファレンス](./commands.md): hagiコマンドの詳細説明
- [MCP導入ガイド](./mcp-setup.md): MCPサーバーの詳細設定
- [README](../README.md): hagiの概要と基本的な使い方
