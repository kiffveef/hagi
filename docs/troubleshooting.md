# hagiトラブルシューティング

hagiの使用中に発生する可能性のある問題と解決策をまとめています。

---

## インストールに関する問題

### jq: command not found

**症状:**
```
Warning: jq not found, .claude/ git protection disabled
```

**原因:** jqがインストールされていない

**解決策:**
1. jqをインストール
   ```bash
   sudo apt install jq
   ```

2. インストール確認
   ```bash
   jq --version
   ```

**Note**: jqがなくてもhagiは動作しますが、`.claude/` git操作防止機能(Layer 1)が無効になります。

---

### cargo: command not found

**症状:**
```
bash: cargo: command not found
```

**原因:** Rustがインストールされていない、またはPATHが通っていない

**解決策:**
1. Rustをインストール
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. PATHを設定
   ```bash
   source ~/.cargo/env
   ```

3. シェルを再起動して確認
   ```bash
   cargo --version
   ```

---

### error: could not download from GitHub

**症状:**
```
error: failed to download from `https://github.com/kiffveef/hagi`
```

**原因:** インターネット接続の問題、またはGitHubへのアクセス制限

**解決策:**
1. インターネット接続を確認
2. GitHubのステータスを確認: https://www.githubstatus.com/
3. プロキシ設定を確認
   ```bash
   echo $HTTP_PROXY
   echo $HTTPS_PROXY
   ```

---

### permission denied

**症状:**
```
error: failed to create directory `/usr/local/bin`
Permission denied
```

**原因:** インストールディレクトリへの書き込み権限がない

**解決策:**
- `sudo`を使わずにインストール(推奨):
  ```bash
  cargo install --git https://github.com/kiffveef/hagi hagi
  ```
  (デフォルトで`~/.cargo/bin`にインストールされます)

- または、`~/.cargo/bin`がPATHに含まれているか確認:
  ```bash
  echo $PATH | grep -o ".cargo/bin"
  ```

---

## セットアップに関する問題

### hagi install が失敗する

**症状:**
```
Error: Failed to create directory
```

**原因:** ディレクトリの権限、またはディスク容量の問題

**解決策:**
1. ドライランで何が起きるか確認
   ```bash
   hagi install -g --dry-run
   ```

2. ディレクトリの権限を確認
   ```bash
   ls -la ~/.claude/
   ```

3. ディスク容量を確認
   ```bash
   df -h ~
   ```

---

### バックアップファイルが多すぎる

**症状:**
`.backup.20250110120000`のようなファイルが大量にある

**原因:** 過去のバージョンのhagiで作成されたバックアップ

**解決策:**
hagiは自動的に最新3世代のバックアップのみを保持します。古いバックアップは自動削除されますが、手動で削除したい場合:

```bash
# 古いバックアップを手動削除
fd -e backup -X rm

# または、特定のパターンで削除
find ~/.claude -name "*.backup.*" -type f -delete
```

---

## MCP関連の問題

### MCP設定が反映されない

**症状:**
`hagi install -g`後もMCPサーバーが動作しない

**解決策:**
1. Claude Codeを再起動

2. MCP設定ファイルを確認
   ```bash
   bat -p ~/.claude/mcp.json
   ```

3. JSON構文を検証
   ```bash
   hagi config validate mcp
   ```

4. MCPサーバーのコマンドが実行可能か確認
   ```bash
   # sequential-thinkingの場合
   npx -y @modelcontextprotocol/server-sequential-thinking --help
   ```

---

### MCPサーバーが起動しない

**症状:**
Claude Code起動時にMCPサーバーのエラーが表示される

**原因:** Node.js、uv等の依存関係が不足している

**解決策:**
1. 依存関係を確認
   ```bash
   node --version  # sequential-thinking, github, context7に必要
   uv --version    # gitに必要
   ```

2. 不足している場合はインストール
   ```bash
   # Node.js (Ubuntu/Debian)
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs

   # uv
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

3. MCPサーバーを再度有効化
   ```bash
   hagi mcp enable sequential-thinking
   ```

---

### 環境変数が設定されていない

**症状:**
GitHub MCPサーバーで認証エラーが発生

**原因:** `GITHUB_PERSONAL_ACCESS_TOKEN`が未設定

**解決策:**
1. GitHub Personal Access Tokenを作成
   - https://github.com/settings/tokens にアクセス
   - 「Generate new token (classic)」をクリック
   - 必要なスコープを選択(repo、read:org等)

2. `mcp.json`に設定
   ```bash
   hagi config edit mcp
   ```

   以下のように編集:
   ```json
   "github": {
     "command": "npx",
     "args": ["-y", "@modelcontextprotocol/server-github"],
     "env": {
       "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_your_token_here"
     }
   }
   ```

3. Claude Codeを再起動

---

## コマンド実行時の問題

### hagi mcp enable が失敗する

**症状:**
```
❌ MCP server not found: server-name
```

**原因:** サーバー名が間違っている

**解決策:**
1. 利用可能なサーバー一覧を確認
   ```bash
   hagi mcp list
   ```

2. 正しい名前で再実行
   ```bash
   hagi mcp enable sequential-thinking
   ```

---

### hagi config edit でエディタが起動しない

**症状:**
エディタが開かない、または`vim: command not found`

**原因:** `$EDITOR`環境変数が未設定、またはエディタが存在しない

**解決策:**
1. `$EDITOR`を設定
   ```bash
   export EDITOR=nano  # nanoを使用
   # または
   export EDITOR=code  # VS Codeを使用
   ```

2. `.bashrc`または`.zshrc`に追加
   ```bash
   echo 'export EDITOR=nano' >> ~/.bashrc
   source ~/.bashrc
   ```

3. 手動で編集
   ```bash
   nano ~/.claude/mcp.json
   ```

---

### hagi update が失敗する

**症状:**
```
❌ Update failed.
```

**原因:** インターネット接続、cargo、GitHubへのアクセス問題

**解決策:**
1. インターネット接続を確認

2. cargoが動作するか確認
   ```bash
   cargo --version
   ```

3. 手動で更新
   ```bash
   cargo install --git https://github.com/kiffveef/hagi hagi --force
   ```

---

## 設定ファイルに関する問題

### JSON構文エラー

**症状:**
```
❌ Configuration is invalid
Error: Expected value at line 5 column 3
```

**原因:** JSON構文が不正(カンマ、括弧の不一致等)

**解決策:**
1. バックアップから復元
   ```bash
   # 最新のバックアップを確認
   ls -lt ~/.claude/*.backup.* | head -1

   # 復元(例)
   cp ~/.claude/mcp.json.backup.20250110120000 ~/.claude/mcp.json
   ```

2. jqで構文チェック・整形
   ```bash
   jq . ~/.claude/mcp.json
   ```

3. 手動で修正
   ```bash
   hagi config edit mcp
   ```

---

## パフォーマンスに関する問題

### .serenaディレクトリが肥大化

**症状:**
`.serena/`ディレクトリが数GB以上になっている

**原因:** serena MCPのキャッシュが蓄積

**解決策:**
1. ディスク使用量を確認
   ```bash
   du -sh .serena/
   ```

2. 古いキャッシュを削除
   ```bash
   # 30日以上古いファイルを削除
   find .serena -type f -mtime +30 -delete
   ```

3. serenaを一時的に無効化
   ```bash
   hagi mcp disable serena
   ```

---

## .claude/ git保護に関する問題

### git add .claude/ がブロックされる

**症状:**
```
❌ .claude/ is outside git workflow. Edit = done. No git operation needed.
📖 See: .claude/instructions/git-workflow.md
```

**原因:** `.claude/`ディレクトリはgit管理対象外のため、Claude Code hookが操作をブロックしています

**これは正常な動作です。** `.claude/`はローカル設定ファイルであり、git追跡すべきではありません。

**もし意図的にコミットしたい場合:**

通常はこの操作は不要ですが、特殊なケースで必要な場合:

1. Layer 1(Claude Code hook)を一時的にバイパス:
   - `.claude/settings.local.json`のhooks設定を一時的にコメントアウト
   - Claude Codeを再起動

2. Layer 2(git pre-commit hook)を一時的にバイパス:
   ```bash
   git commit --no-verify -m "Special case: commit .claude/"
   ```

**推奨:** `.claude/`を同期したい場合は`hagi sync`コマンドを使用してください。

---

### git commit が .claude/ ファイルでエラーになる

**症状:**
```
❌ ERROR: .claude/ files should not be committed!
To unstage: git restore --staged .claude/
```

**原因:** `.claude/`ファイルがステージングされており、git pre-commit hookがブロックしています

**解決策:**
```bash
# ステージングを解除
git restore --staged .claude/

# 確認
git status
```

---

## その他の問題

### Claude Codeでスラッシュコマンドが使えない

**症状:**
`/st`コマンドが認識されない

**原因:** `.claude/commands/`ディレクトリが存在しない

**解決策:**
1. プロジェクトセットアップを実行
   ```bash
   cd /path/to/project
   hagi install
   ```

2. `.claude/commands/st.md`が存在するか確認
   ```bash
   ls -la .claude/commands/
   ```

---

### 設定が意図せず上書きされる

**症状:**
カスタマイズした設定が`hagi install -g`で消える

**原因:** hagiは既存設定とマージしますが、同じキーは上書きされます

**解決策:**
1. 事前にバックアップ
   ```bash
   cp ~/.claude/mcp.json ~/.claude/mcp.json.backup
   ```

2. ドライランで確認
   ```bash
   hagi install -g --dry-run
   ```

3. 手動でマージ
   ```bash
   # 既存設定を確認
   bat -p ~/.claude/mcp.json

   # 新規設定を取得
   hagi config show mcp > /tmp/new-mcp.json

   # jqで手動マージ
   jq -s '.[0] * .[1]' ~/.claude/mcp.json /tmp/new-mcp.json > ~/.claude/mcp.merged.json
   ```

---

## ログとデバッグ

### 詳細なエラー情報を取得

```bash
# Rustのバックトレースを有効化
RUST_BACKTRACE=1 hagi install -g

# より詳細なバックトレース
RUST_BACKTRACE=full hagi install -g
```

### hagiのバージョン確認

```bash
hagi --version
```

### 環境情報の収集

```bash
# OS情報
uname -a

# Rustバージョン
cargo --version
rustc --version

# Node.jsバージョン
node --version
npm --version

# uvバージョン
uv --version

# hagiバージョン
hagi --version
```

---

## サポート

問題が解決しない場合は、以下の情報を含めてIssueを作成してください:

**Issue作成時に含める情報:**
- OS名とバージョン
- hagiのバージョン(`hagi --version`)
- 実行したコマンド
- エラーメッセージ全文
- `RUST_BACKTRACE=1`でのエラー出力

**GitHub Issues:** https://github.com/kiffveef/hagi/issues

---

## 関連ドキュメント

- **[README](../README.md)**: プロジェクト概要
- **[インストールガイド](./installation.md)**: インストール方法
- **[コマンドリファレンス](./commands.md)**: 全コマンドの詳細
- **[MCP導入ガイド](./mcp-setup.md)**: MCPサーバーのセットアップ
- **[開発ガイド](./development.md)**: hagiへの貢献方法
