# コマンドリファレンス

---

## install

設定ファイルとテンプレートをインストールする。

### グローバルセットアップ

```bash
hagi install --global
hagi install -g
```

`~/.claude/settings.json`にパーミッション設定とhooks設定を配置する。依存関係(Node.js、uv、jq)を自動チェックし、不足時は警告を表示する。インストール処理は継続される。

| ツール | 必要なMCP | インストール |
|--------|-----------|-------------|
| Node.js | context7、one-search、memory | `curl -fsSL https://deb.nodesource.com/setup_18.x \| sudo -E bash -` |
| uv | serena、git | `curl -LsSf https://astral.sh/uv/install.sh \| sh` |
| jq | .claude/ git保護 | `sudo apt install jq` |

### プロジェクトセットアップ

```bash
cd /path/to/project
hagi install
```

**配置されるファイル:**
- `.claude/CLAUDE.md`, `instructions/`, `skills/` - テンプレート
- `.claude/mcp.json` - MCP設定
- `.claude/settings.local.json` - パーミッション設定
- `.mcp.json` → `.claude/mcp.json` (シンボリックリンク)
- `.gitignore` 更新(`/.claude/`, `/.serena/`)
- Git hooks(`pre-commit`, `commit-msg`)

非gitリポジトリの場合、自動的に`git init`を実行する。

### チャットモードセットアップ

```bash
hagi install --chat
```

`~/.chat/.claude/`に軽量設定(memory + one-search)を配置する。

### オプション

| オプション | 説明 |
|-----------|------|
| `--global`, `-g` | グローバルセットアップ |
| `--chat` | チャットモードセットアップ |
| `--dry-run` | 変更内容の確認のみ |
| `--skip <PATH>` | 指定ファイル/ディレクトリをスキップ(複数指定可) |

**`--skip`の指定方法:**

| 指定 | 効果 |
|------|------|
| `CLAUDE.md` | `.claude/CLAUDE.md`をスキップ |
| `instructions` | `.claude/instructions/`全体をスキップ |
| `instructions/git-workflow.md` | 特定ファイルのみスキップ |
| `git` | git自動初期化をスキップ |

```bash
# カスタマイズ済みファイルを保持しつつ更新
hagi install --skip CLAUDE.md --skip instructions
```

---

## uninstall

設定を削除する。

```bash
hagi uninstall           # プロジェクト設定(.claude/全体)
hagi uninstall --global  # グローバル設定
```

| オプション | 説明 |
|-----------|------|
| `--global`, `-g` | グローバル設定を削除 |
| `-y`, `--yes` | 確認プロンプトをスキップ |

---

## status

インストール状態を確認する。

```bash
hagi status
```

グローバル設定、プロジェクト設定、MCP設定、テンプレートの状態を表示する。

---

## update

hagiを最新版に更新する。

```bash
hagi update
```

GitHubから最新版を取得し`cargo install --force`で上書きする。

---

## mcp

MCPサーバーを管理する。プロジェクトごとの`.claude/mcp.json`を操作する。

### list

```bash
hagi mcp list
```

### info

```bash
hagi mcp info <SERVER_NAME>
```

### enable / disable

```bash
hagi mcp enable <SERVER_NAME> [SERVER_NAME...]
hagi mcp disable <SERVER_NAME> [SERVER_NAME...]
```

複数サーバーを同時に指定可能。存在しないサーバー名はエラーとなるが、他のサーバーは正常に処理される。変更後はClaude Codeの再起動が必要。

```bash
# 例
hagi mcp enable serena memory git
hagi mcp disable git one-search
```

---

## config

設定ファイルを管理する。

### show

```bash
hagi config show <TYPE>   # mcp | global
```

### validate

```bash
hagi config validate <TYPE>
```

JSON構文チェックを実行する。

### edit

```bash
hagi config edit <TYPE>
```

`$EDITOR`(未設定時はvim)で設定ファイルを開く。編集後は`hagi config validate`で構文チェックを推奨。

---

## sync

複数マシン間で`.claude`ディレクトリを同期する。プライベートGitリポジトリを使用。

### init

```bash
hagi sync init              # gh CLIで自動セットアップ
hagi sync init <REMOTE_URL> # 手動指定
```

`gh`コマンドがある場合、`<project>-claude`リポジトリを自動作成する。ない場合はリモートURLを手動指定する。

### pull / push

```bash
hagi sync pull
hagi sync push [-m <MESSAGE>]
```

`pull`は`git pull --rebase`で取得。`push`は変更をステージング・コミット・pushする。

### status

```bash
hagi sync status
```

`.claude`のgit状態を表示する。

### ワークフロー

```bash
# 作業開始前
hagi sync pull

# 作業終了時
hagi sync push -m "Update TODO"
```

別マシンでの初回セットアップ:

```bash
git clone git@github.com:yourname/myproject.git && cd myproject
hagi sync init git@github.com:yourname/myproject-claude.git
```
