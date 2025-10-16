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
1. 依存関係チェック(Node.js、uv、Python3、Git)
2. `~/.claude/`ディレクトリを作成(存在しない場合)
3. 既存ファイルのバックアップを作成(タイムスタンプ付き)
4. 新規設定ファイルと既存設定をマージ
5. 古いバックアップを自動削除(最新3世代のみ保持)

**依存関係チェック:**

グローバルセットアップ時に以下のツールの存在を自動チェックします:

| ツール | 必要なMCP | インストールコマンド |
|--------|-----------|---------------------|
| Node.js | context7、sequential-thinking、one-search | `curl -fsSL https://deb.nodesource.com/setup_18.x \| sudo -E bash -` <br> `sudo apt-get install -y nodejs` |
| uv | mcp-memory-service | `curl -LsSf https://astral.sh/uv/install.sh \| sh` |
| Python3 | mcp-memory-service setup | `sudo apt-get update` <br> `sudo apt-get install -y python3 python3-pip` |
| Git | mcp-memory-service clone | `sudo apt-get install -y git` |

**重要**: 依存関係が不足していても、インストール処理は継続されます(警告のみ表示)。不足しているツールは後でインストール可能です。

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
1. gitリポジトリかチェック(非gitリポジトリの場合、自動的に`git init`を実行)
2. プロジェクトルートに`.claude/`ディレクトリを作成
3. テンプレートファイルをコピー(ディレクトリ構造を保持)
4. `.gitignore`に必要なパターンを追加(既存エントリは保持)
5. **Git hooksのインストール**(`.git/hooks/`配下)
6. セットアップ完了メッセージを表示

**Git Hooks自動インストール:**

`hagi install`実行時に以下のGit hooksが自動的にインストールされます:

| Hook | 機能 | 効果 |
|------|------|------|
| `pre-commit` | mainブランチへの直接コミット防止 | mainブランチで`git commit`を実行するとエラーで停止 |
| `commit-msg` | Claude Code署名の検出・ブロック | コミットメッセージに`Generated with [Claude Code]`や`Co-Authored-By: Claude`が含まれる場合、エラーで停止 |

**重要:** これらのhooksはプロジェクトワークフローを保護します:
- ✅ featureブランチでの開発を強制
- ✅ クリーンで専門的なコミットメッセージを保証
- ✅ Claude Codeの自動生成署名を除去

**Git自動初期化:**
- プロジェクトディレクトリがgitリポジトリでない場合、自動的に`git init`を実行します
- 初期化後、空の初期コミット(`git commit --allow-empty -m "🌱 init"`)を自動作成します
- ドライランモードでは以下が表示されます:
  - `Would run: git init`
  - `Would run: git commit --allow-empty -m "🌱 init"`
- gitが既に初期化済みの場合はスキップされます

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
  - **グローバルとローカルの設定比較**
  - 設定が異なるサーバーを警告表示
  - 有効化/無効化されているMCPサーバー一覧
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

インストール済みMCPサーバーの一覧を表示します。グローバル設定とプロジェクトローカル設定の両方を表示します。

**出力例:**
```
═══ Global Configuration (~/.claude/mcp.json) ═══

  sequential-thinking [enabled] - Structured thinking and problem-solving
  context7 [enabled] - Library documentation and code examples
  serena [enabled] - Code analysis and semantic search
  file-search [disabled] - Fast file search and analysis
  git [disabled] - Git operations and repository management
  github [disabled] - GitHub integration (issues, PRs, releases)

═══ Project-Local Configuration (.claude/mcp.json) ═══

  sequential-thinking [enabled] - Structured thinking and problem-solving
  context7 [enabled] - Library documentation and code examples
  serena [disabled] - Code analysis and semantic search
  file-search [disabled] - Fast file search and analysis
  git [disabled] - Git operations and repository management
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
hagi mcp enable <SERVER_NAME> [SERVER_NAME...] [--global]
```

指定したMCPサーバーを有効化します。複数のサーバーを同時に指定できます。

**スコープ:**
- **デフォルト**: プロジェクトローカル(`.claude/mcp.json`)を操作
- **`--global / -g`**: グローバル設定(`~/.claude/mcp.json`)を操作

**例:**
```bash
# プロジェクトローカルで有効化(デフォルト)
hagi mcp enable serena

# グローバル設定で有効化
hagi mcp enable serena --global
hagi mcp enable serena -g

# 複数のサーバーを同時に有効化
hagi mcp enable serena file-search git

# グローバルで複数サーバーを有効化
hagi mcp enable memory one-search --global
```

**動作:**
1. 対象の設定ファイルを読み込み(グローバルまたはローカル)
2. 指定サーバーの`"disabled": true`フィールドを削除
3. **github/github-graphql MCPの場合、.envファイルまたはシェル環境変数からPATの存在を確認(検証のみ、書き込みなし)**
4. バックアップを作成(タイムスタンプ付き、バッチ処理時は1回のみ)
5. 古いバックアップを自動削除(最新3世代のみ保持)
6. ファイルを保存
7. 成功/失敗の集計を表示
8. 再起動を促すメッセージを表示

**出力例:**
```bash
$ hagi mcp enable serena file-search git
✅ MCP server 'serena' enabled
✅ MCP server 'file-search' enabled
✅ MCP server 'git' enabled

✅ 3 server(s) enabled.

Note: Restart Claude Code to apply changes.
```

**GitHub MCPのトークン検証例:**
```bash
$ hagi mcp enable github github-graphql
✅ MCP server 'github' enabled
   ✓ GITHUB_PERSONAL_ACCESS_TOKEN found in environment
✅ MCP server 'github-graphql' enabled
   ✓ GITHUB_TOKEN found in environment

✅ 2 server(s) enabled.

Note: Restart Claude Code to apply changes.
```

**トークンが見つからない場合:**
```bash
$ hagi mcp enable github
✅ MCP server 'github' enabled
   ⚠ GITHUB_PERSONAL_ACCESS_TOKEN not found in .env or environment
   Set up with one of:
     1. Create .env file: echo 'GITHUB_PERSONAL_ACCESS_TOKEN=your_token' > .env
     2. Export in shell: export GITHUB_PERSONAL_ACCESS_TOKEN=your_token
     3. Edit ~/.claude/mcp.json manually

✅ 1 server(s) enabled.

⚠️ Warning: The following servers require environment variables:
  - github
Edit ~/.claude/mcp.json and configure required variables.

Note: Restart Claude Code to apply changes.
```

**エラー処理:**
存在しないサーバー名を指定した場合、そのサーバーのみエラーとなり、他のサーバーは正常に処理されます。

```bash
$ hagi mcp enable serena invalid-name file-search
✅ MCP server 'serena' enabled
❌ invalid-name - MCP server not found
✅ MCP server 'file-search' enabled

✅ 2 server(s) enabled.
❌ 1 server(s) failed.

Note: Restart Claude Code to apply changes.
```

**注意:**
- 環境変数が必要なサーバー(github等)を有効化する際は集約された警告が表示されます
- 設定変更後はClaude Codeの再起動が必要です

### disable - MCPサーバー無効化

```bash
hagi mcp disable <SERVER_NAME> [SERVER_NAME...] [--global]
```

指定したMCPサーバーを無効化します。複数のサーバーを同時に指定できます。

**スコープ:**
- **デフォルト**: プロジェクトローカル(`.claude/mcp.json`)を操作
- **`--global / -g`**: グローバル設定(`~/.claude/mcp.json`)を操作

**例:**
```bash
# プロジェクトローカルで無効化(デフォルト)
hagi mcp disable serena

# グローバル設定で無効化
hagi mcp disable serena --global
hagi mcp disable serena -g

# 複数のサーバーを同時に無効化
hagi mcp disable serena file-search git

# グローバルで複数サーバーを無効化
hagi mcp disable memory one-search --global
```

**動作:**
1. 対象の設定ファイルを読み込み(グローバルまたはローカル)
2. 指定サーバーに`"disabled": true`を追加
3. バックアップを作成(タイムスタンプ付き、バッチ処理時は1回のみ)
4. 古いバックアップを自動削除(最新3世代のみ保持)
5. ファイルを保存
6. 成功/失敗の集計を表示
7. 再起動を促すメッセージを表示

**出力例:**
```bash
$ hagi mcp disable file-search git github
✅ MCP server 'file-search' disabled
✅ MCP server 'git' disabled
✅ MCP server 'github' disabled

✅ 3 server(s) disabled.

Note: Restart Claude Code to apply changes.
```

**エラー処理:**
存在しないサーバー名を指定した場合、そのサーバーのみエラーとなり、他のサーバーは正常に処理されます。

```bash
$ hagi mcp disable serena invalid-name
✅ MCP server 'serena' disabled
❌ invalid-name - MCP server not found

✅ 1 server(s) disabled.
❌ 1 server(s) failed.

Note: Restart Claude Code to apply changes.
```

**注意:**
- 重要なサーバー(sequential-thinking、context7)を無効化する際は集約された警告が表示されます
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

# 単一サーバーを有効化
hagi mcp enable serena

# 複数のサーバーを同時に有効化
hagi mcp enable serena file-search git

# 詳細情報確認
hagi mcp info serena

# 使わないサーバーをまとめて無効化
hagi mcp disable git github memory

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
