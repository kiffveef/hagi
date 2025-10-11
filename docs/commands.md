# hagiコマンドリファレンス

hagiの全コマンド、オプション、使用例の詳細説明です。

---

## グローバルオプション

### ヘルプ表示

```bash
hagi --help
hagi -h
```

各コマンドの詳細ヘルプ:
```bash
hagi <COMMAND> --help
hagi <COMMAND> -h
```

### バージョン表示

```bash
hagi --version
hagi -V
```

---

## install - インストール

Claude Code向けの設定ファイルとテンプレートをインストールします。

### グローバルセットアップ

`~/.claude/`配下にMCP設定とパーミッション設定を配置します。

```bash
hagi install --global
hagi install -g
```

**セットアップ内容:**
- `~/.claude/mcp.json` - MCP設定(sequential-thinking、context7を有効化)
- `~/.claude/settings.json` - パーミッション設定

**動作:**
1. `~/.claude/`ディレクトリを作成(存在しない場合)
2. 既存ファイルのバックアップを作成(タイムスタンプ付き)
3. 新規設定ファイルと既存設定をマージ
4. 古いバックアップを自動削除(最新3世代のみ保持)

**オプション:**

**`--dry-run`**: ドライラン(変更内容の確認のみ)

```bash
hagi install -g --dry-run
```

実際にファイルを変更せず、以下の情報のみを表示します:
- 作成されるファイル
- バックアップされるファイル
- マージされる設定内容

### プロジェクトセットアップ

プロジェクトルートに`.claude/`ディレクトリとテンプレートを配置します。

```bash
cd /path/to/your/project
hagi install
```

**セットアップ内容:**
- `.claude/` ディレクトリ作成
- `.claude/CLAUDE.md` - プロジェクトガイドライン(テンプレート)
- `.claude/instructions/` - 詳細インストラクション
  - `git-workflow.md` - Git操作ルール(MUST/NEVER形式)
  - `task-management.md` - TodoWriteツール使い方
  - `tools.md` - 推奨ツール(rg/bat/fd等)
- `.claude/commands/st.md` - sequential-thinkingスラッシュコマンド
- `.claude/mcp.json` - プロジェクト用MCP設定
- `.claude/settings.local.json` - パーミッション設定
- `.gitignore` 更新(以下を追加)
  - `/.claude/`
  - `/.serena/`

**動作:**
1. プロジェクトルートに`.claude/`ディレクトリを作成
2. テンプレートファイルをコピー(ディレクトリ構造を保持)
3. `.gitignore`に必要なパターンを追加(既存エントリは保持)
4. セットアップ完了メッセージを表示

**オプション:**

**`--dry-run`**: ドライラン(変更内容の確認のみ)

```bash
hagi install --dry-run
```

実際にファイルを作成せず、以下の情報のみを表示します:
- 作成されるディレクトリ
- コピーされるファイル一覧
- `.gitignore`に追加されるパターン

**`--skip <PATH>`**: 特定のファイルやディレクトリをスキップ(複数指定可能)

```bash
# CLAUDE.mdをスキップ
hagi install --skip CLAUDE.md

# instructionsディレクトリをスキップ
hagi install --skip instructions

# 複数のファイル/ディレクトリをスキップ
hagi install --skip CLAUDE.md --skip instructions

# 特定のファイルをスキップ
hagi install --skip instructions/git-workflow.md
```

**スキップ対象の指定方法:**
- ファイル名のみ: `CLAUDE.md` (`.claude/CLAUDE.md`がスキップされる)
- ディレクトリ名: `instructions` (`.claude/instructions/`配下すべてがスキップされる)
- 相対パス: `instructions/git-workflow.md` (特定ファイルのみスキップ)

**ユースケース:**
- 既存のCLAUDE.mdを保持したい場合: `--skip CLAUDE.md`
- カスタマイズ済みのinstructionsを保持したい場合: `--skip instructions`
- mcp.jsonのみを更新したい場合: `--skip CLAUDE.md --skip instructions --skip commands`

### 使用例

**初回セットアップ(推奨フロー):**

```bash
# 1. グローバル設定をインストール(MCP設定)
hagi install --global

# 2. プロジェクトに移動
cd ~/projects/my-rust-project

# 3. プロジェクト設定をインストール
hagi install

# 4. Claude Codeを起動
claude-code
```

**ドライランで確認してからインストール:**

```bash
# 変更内容を確認
hagi install -g --dry-run

# 問題なければ実行
hagi install -g
```

**既存プロジェクトに追加:**

```bash
cd /path/to/existing/project
hagi install
# .gitignore、.claude/が作成される
```

**カスタマイズ済みファイルを保持しながら更新:**

```bash
# CLAUDE.mdとinstructionsは既存のものを保持
hagi install --skip CLAUDE.md --skip instructions

# または、ドライランで確認してから実行
hagi install --dry-run --skip CLAUDE.md --skip instructions
hagi install --skip CLAUDE.md --skip instructions
```

---

## uninstall - アンインストール

hagiでインストールした設定を削除します。

### グローバル設定の削除

`~/.claude/`配下の設定を削除します。

```bash
hagi uninstall --global
hagi uninstall -g
```

**削除対象:**
- `~/.claude/mcp.json`
- `~/.claude/settings.json`
- `~/.claude/`ディレクトリ(空の場合)

**動作:**
1. 確認プロンプトを表示
2. 削除前にバックアップを作成
3. 指定ファイルを削除

### プロジェクト設定の削除

プロジェクトルートの`.claude/`ディレクトリを削除します。

```bash
hagi uninstall
```

**削除対象:**
- `.claude/`ディレクトリ全体
- `.gitignore`からhagi関連のパターンを削除
  - `/.claude/`
  - `/.serena/`

**動作:**
1. 確認プロンプトを表示
2. `.claude/`ディレクトリを削除
3. `.gitignore`から関連パターンを削除(他のエントリは保持)

### オプション

**`-y, --yes`**: 確認プロンプトをスキップ

```bash
hagi uninstall -y
hagi uninstall --global -y
```

確認なしで即座に削除を実行します。

### 使用例

**プロジェクト設定のみ削除:**

```bash
cd /path/to/project
hagi uninstall
# 確認プロンプトが表示される
```

**グローバル設定を確認なしで削除:**

```bash
hagi uninstall -g -y
# 即座に削除される
```

**完全にクリーンアップ:**

```bash
# プロジェクト設定を削除
hagi uninstall

# グローバル設定も削除
hagi uninstall --global
```

---

## status - ステータス確認

hagiのインストール状態を確認します。

```bash
hagi status
```

**表示内容:**
- グローバル設定のインストール状態
  - `~/.claude/mcp.json`の存在確認
  - `~/.claude/settings.json`の存在確認
- プロジェクト設定のインストール状態
  - `.claude/`ディレクトリの存在確認
  - 各テンプレートファイルの存在確認
- MCP設定の詳細
  - 有効化されているMCPサーバー一覧
  - 無効化されているMCPサーバー一覧
- テンプレートファイルの状態
  - `.claude/CLAUDE.md`
  - `.claude/instructions/`
  - `.claude/commands/`

### 使用例

**インストール状態の確認:**

```bash
cd /path/to/project
hagi status
```

出力例:
```
[Global Configuration]
✅ ~/.claude/mcp.json - installed
✅ ~/.claude/settings.json - installed

[Project Configuration]
✅ .claude/ - installed
✅ .claude/CLAUDE.md - installed
✅ .claude/instructions/ - installed
✅ .claude/commands/ - installed

[MCP Servers]
✅ sequential-thinking - enabled
✅ context7 - enabled
❌ serena - disabled
❌ file-search - disabled
❌ git - disabled
❌ github - disabled
```

---

## update - 更新

hagiツール自体を最新版に更新します。

```bash
hagi update
```

**動作:**
1. 現在のバージョンを表示
2. 更新確認プロンプトを表示
3. GitHub リポジトリから最新版を取得
4. `cargo install --force`で上書きインストール
5. 新しいバージョンを表示

**出力例:**
```
Updating hagi...

Current version: 0.1.0

Do you want to update hagi to the latest version? [Y/n]: y

Fetching latest version from GitHub...
(cargo installの出力)

✅ hagi updated successfully!

New version:
  hagi 0.2.0
```

**注意:**
- インターネット接続が必要です
- cargo がインストールされている必要があります
- 更新には数分かかる場合があります

**将来実装予定:**
- テンプレートファイルの最新版への更新
- MCP設定の新規サーバー追加
- 既存プロジェクトへの変更反映オプション

---

## mcp - MCP管理

MCPサーバーの管理コマンド。

### list - MCPサーバー一覧

```bash
hagi mcp list
```

インストール済みMCPサーバーの一覧を表示します。

**出力例:**
```
MCP Servers:

  sequential-thinking [enabled] - Structured thinking and problem-solving
  context7 [enabled] - Library documentation and code examples
  serena [disabled] - Code analysis and semantic search
  file-search [disabled] - Fast file search and analysis
  git [disabled] - Git operations and repository management
  github [disabled] - GitHub integration (issues, PRs, releases)
```

### info - MCPサーバー情報

```bash
hagi mcp info <SERVER_NAME>
```

特定のMCPサーバーの詳細情報を表示します。

**例:**
```bash
hagi mcp info github
```

**出力例:**
```
MCP Server: github
Status: disabled
Command: npx -y @modelcontextprotocol/server-github
Environment:
  GITHUB_PERSONAL_ACCESS_TOKEN: <your-github-token> (not set)
Description: GitHub integration (issues, PRs, releases)
```

### enable - MCPサーバー有効化

```bash
hagi mcp enable <SERVER_NAME>
```

指定したMCPサーバーを有効化します。

**例:**
```bash
hagi mcp enable serena
hagi mcp enable file-search
```

**動作:**
1. `~/.claude/mcp.json`を読み込み
2. 指定サーバーの`"disabled": true`フィールドを削除
3. バックアップを作成(タイムスタンプ付き)
4. 古いバックアップを自動削除(最新3世代のみ保持)
5. ファイルを保存
6. 再起動を促すメッセージを表示

**注意:**
- 環境変数が必要なサーバー(github等)を有効化する際は警告が表示されます
- 設定変更後はClaude Codeの再起動が必要です

### disable - MCPサーバー無効化

```bash
hagi mcp disable <SERVER_NAME>
```

指定したMCPサーバーを無効化します。

**例:**
```bash
hagi mcp disable serena
```

**動作:**
1. `~/.claude/mcp.json`を読み込み
2. 指定サーバーに`"disabled": true`を追加
3. バックアップを作成(タイムスタンプ付き)
4. 古いバックアップを自動削除(最新3世代のみ保持)
5. ファイルを保存
6. 再起動を促すメッセージを表示

**注意:**
- 重要なサーバー(sequential-thinking、context7)を無効化する際は警告が表示されます
- 設定変更後はClaude Codeの再起動が必要です

---

## config - 設定管理

hagiの設定管理コマンド。

### show - 設定表示

```bash
hagi config show <CONFIG_TYPE>
```

指定した設定の内容を表示します。

**対応する設定タイプ:**
- `mcp` - `~/.claude/mcp.json` (MCP設定)
- `global` - `~/.claude/settings.json` (グローバル設定)
- `hook` - フック設定 (将来実装予定)

**例:**
```bash
hagi config show mcp        # MCP設定表示
hagi config show global     # グローバル設定表示
```

**出力例:**
```
Configuration: mcp
File: /home/user/.claude/mcp.json

{
  "mcpServers": {
    "sequential-thinking": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"]
    },
    ...
  }
}
```

### validate - 設定検証

```bash
hagi config validate <CONFIG_TYPE>
```

設定ファイルのJSON構文チェックを行います。

**例:**
```bash
hagi config validate mcp    # MCP設定検証
hagi config validate global # グローバル設定検証
```

**出力例:**
```
✅ Configuration is valid: /home/user/.claude/mcp.json
```

**エラー時:**
```
❌ Configuration is invalid: /home/user/.claude/mcp.json

Error: Expected value at line 5 column 3

Tip: Use 'jq' to format and validate JSON manually:
  jq . /home/user/.claude/mcp.json
```

### edit - 設定編集

```bash
hagi config edit <CONFIG_TYPE>
```

指定した設定を`$EDITOR`で開きます。

**例:**
```bash
hagi config edit mcp        # MCP設定編集
hagi config edit global     # グローバル設定編集
```

**動作:**
1. 設定ファイルのバックアップを作成
2. 古いバックアップを自動削除(最新3世代のみ保持)
3. `$EDITOR`でファイルを開く(`$EDITOR`未設定時はvim)
4. 編集完了後、validateを実行して確認することを推奨

**注意:**
- 編集後は`hagi config validate`で構文チェックを推奨
- 設定変更後はClaude Codeの再起動が必要です

---

## 実践的な使用例

### ケース1: 新規プロジェクトのセットアップ

```bash
# 1. グローバル設定をインストール
hagi install --global

# 2. 新規プロジェクト作成
cargo new my-project
cd my-project

# 3. プロジェクト設定をインストール
hagi install

# 4. 状態確認
hagi status

# 5. Claude Code起動
claude-code
```

### ケース2: 既存プロジェクトへの追加

```bash
# 既存プロジェクトに移動
cd ~/projects/existing-project

# ドライランで確認
hagi install --dry-run

# 問題なければインストール
hagi install

# .gitignoreが更新されていることを確認
bat -p .gitignore
```

### ケース3: 設定の更新

```bash
# グローバル設定を再インストール(マージされる)
hagi install -g

# プロジェクト設定を更新
cd /path/to/project
hagi install

# 古いバックアップは自動削除される(最新3世代のみ保持)
```

### ケース4: クリーンアップ

```bash
# プロジェクト設定のみ削除
cd /path/to/project
hagi uninstall

# グローバル設定も削除(確認なし)
hagi uninstall -g -y
```

### ケース5: MCPサーバーの管理

```bash
# MCPサーバー一覧確認
hagi mcp list

# serenaを有効化
hagi mcp enable serena

# 詳細情報確認
hagi mcp info serena

# 状態確認
hagi status

# Claude Code再起動して設定を反映
```

### ケース6: 設定の確認と編集

```bash
# MCP設定の内容確認
hagi config show mcp

# 設定の妥当性検証
hagi config validate mcp

# MCP設定を編集
hagi config edit mcp
# (エディタが開く)

# 編集後に検証
hagi config validate mcp

# Claude Code再起動して設定を反映
```

---

## トラブルシューティング

### インストールに失敗する

**症状:**
```
Error: Failed to create directory
```

**解決策:**
1. ディレクトリの権限を確認
2. `--dry-run`で何が起きるか確認
3. 既存ファイルをバックアップしてから再実行

### バックアップファイルが多すぎる

**症状:**
`.backup.20250110120000`のようなファイルが大量にある

**解決策:**
hagiは自動的に最新3世代のバックアップのみを保持します。古いバックアップは自動削除されますが、手動で削除したい場合:

```bash
# 古いバックアップを手動削除
fd -e backup -X rm
```

### MCP設定が反映されない

**症状:**
`hagi install -g`後もMCPサーバーが動作しない

**解決策:**
1. Claude Codeを再起動
2. MCP設定ファイルを確認
   ```bash
   bat -p ~/.claude/mcp.json
   ```
3. Node.js、uvがインストールされているか確認
   ```bash
   node --version
   uv --version
   ```

詳細は[MCP導入ガイド](./mcp-setup.md)を参照してください。

---

## 関連ドキュメント

- **[使い方ガイド](./usage.md)**: スラッシュコマンド(/st)の使い方、MCPサーバーの活用方法
- **[MCP導入ガイド](./mcp-setup.md)**: MCPサーバーのインストール方法、設定、トラブルシューティング
- **[README](../README.md)**: hagiの概要と基本的な使い方

---

## フィードバック

コマンドに関する質問、バグ報告、機能要望は以下でお願いします:
https://github.com/kiffveef/hagi/issues
