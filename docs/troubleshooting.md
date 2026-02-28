# トラブルシューティング

---

## インストール

### cargo: command not found

Rustがインストールされていない、またはPATHが通っていない。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### error: failed to download from GitHub

インターネット接続またはGitHubアクセスの問題。GitHubステータス: https://www.githubstatus.com/

### permission denied

`sudo`を使わずインストールする(デフォルトで`~/.cargo/bin`にインストールされる)。

---

## セットアップ

### hagi install が失敗する

```bash
# 何が起きるか確認
hagi install -g --dry-run

# ディレクトリ権限確認
ls -la ~/.claude/
```

### jq: command not found

jqがないと`.claude/` git保護(Layer 1)が無効になる。hagiの動作自体には影響しない。

```bash
sudo apt install jq
```

### バックアップファイルが多すぎる

hagiは最新1世代のみ保持する。過去バージョンで作成された余分なバックアップは手動削除:

```bash
fd -e backup -X rm
```

---

## MCP関連の問題

### MCP設定が反映されない

1. Claude Codeを再起動
2. 設定ファイルを確認: `hagi config validate mcp`
3. 依存関係を確認:
   ```bash
   node --version  # context7, one-search, memory
   uv --version    # serena, git
   ```

### MCPサーバーが起動しない

依存関係が不足している可能性がある。

```bash
# Node.js (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# uv
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### context7 API制限

API keyなしの場合、レート制限がある。`.claude/mcp.json`にAPI keyを設定して解除。

### one-search接続エラー

WSL2環境ではPuppeteer依存のプロバイダーを避け、DuckDuckGoを使用する。

```json
"one-search": {
  "env": { "SEARCH_PROVIDER": "duckduckgo" }
}
```

### Memento (memory) 起動エラー

```bash
# 手動起動テスト
npx -y @iachilles/memento@latest

# データディレクトリ確認
ls -la ~/.local/share/claude-memory/
```

### .serenaディレクトリが肥大化

serenaのキャッシュ蓄積が原因。

```bash
du -sh .serena/
find .serena -type f -mtime +30 -delete
```

---

## コマンド実行

### hagi mcp enable/disable が失敗する

`hagi mcp list`でサーバー名を確認して正しい名前で再実行。

### hagi config edit でエディタが起動しない

`$EDITOR`を設定する:

```bash
export EDITOR=nano
echo 'export EDITOR=nano' >> ~/.bashrc
```

### hagi update が失敗する

手動更新:

```bash
cargo install --git https://github.com/kiffveef/hagi hagi --force
```

---

## 設定ファイル

### JSON構文エラー

```bash
# 構文チェック
hagi config validate mcp

# バックアップから復元
ls -lt ~/.claude/*.backup.* | head -1
cp ~/.claude/mcp.json.backup.YYYYMMDD_HHMMSS ~/.claude/mcp.json
```

### 設定が意図せず上書きされる

hagiは既存設定とマージするが、同じキーは上書きされる。事前に`hagi install -g --dry-run`で確認する。

---

## .claude/ git保護

### git add .claude/ がブロックされる

これは正常な動作。`.claude/`はgit管理対象外。同期が必要な場合は`hagi sync`を使用する。

### git commit が .claude/ でエラーになる

```bash
git restore --staged .claude/
```

---

## スキル

### スラッシュコマンドが認識されない

Claude Codeを再起動する。スキルは起動時に読み込まれる。

### Context7がライブラリを見つけられない

ライブラリ名とバージョンを明示する(例: `Axum0.7`, `Tokio1.40`)。

---

## デバッグ

```bash
# 詳細エラー
RUST_BACKTRACE=1 hagi install -g

# 環境情報
hagi --version
cargo --version
node --version
uv --version
```

Issue作成時はエラーメッセージ全文と環境情報を含めること: https://github.com/kiffveef/hagi/issues
