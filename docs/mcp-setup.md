# MCP サーバー導入ガイド

このドキュメントでは、hagiがセットアップするMCPサーバーのインストール方法と設定方法を説明します。

---

## 目次

1. [前提条件](#前提条件)
2. [MCPサーバー一覧](#mcpサーバー一覧)
3. [インストール手順](#インストール手順)
4. [hagiでのセットアップ](#hagiでのセットアップ)
5. [個別の有効化・無効化](#個別の有効化無効化)
6. [トラブルシューティング](#トラブルシューティング)

---

## 前提条件

以下のツールが必要です：

### 必須
- **Node.js** (v18以降推奨)
  ```bash
  node --version
  ```

### 推奨
- **uv** (Python package manager)
  ```bash
  curl -LsSf https://astral.sh/uv/install.sh | sh
  ```

- **Rust/cargo** (file-search MCP用)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

---

## MCPサーバー一覧

### 🔥 推奨構成(Windows + WSL2最適化)

軽量・高速・完全ローカル動作を重視した構成です。

| MCP | 用途 | インストール方法 | デフォルト状態 | 備考 |
|-----|------|------------------|----------------|------|
| sequential-thinking | 構造化思考支援 | npx (自動) | ✅ 有効 | 軽量、起動即座 |
| context7 | 公式ドキュメント検索 | npx (自動) | ✅ 有効 | 軽量、API keyなしで基本機能利用可 |
| one-search | Web検索 | npx (自動) | ❌ 無効 | DuckDuckGo推奨(Puppeteerなし) |
| memory | 長期記憶管理 | uv + Git (手動) | ❌ 無効 | 完全ローカル(SQLite-vec + ONNX) |
| serena | セマンティックコード解析 | npx (自動) | ❌ 無効 | XDG準拠、キャッシュ管理 |

### その他のMCP

| MCP | 用途 | インストール方法 | デフォルト状態 |
|-----|------|------------------|----------------|
| file-search | 高速ファイル検索 | cargo install (手動) | ❌ 無効 |
| git | Git操作 | uvx (自動) | ❌ 無効 |
| github | GitHub REST API連携 | npx (自動) | ❌ 無効 |
| github-graphql | GitHub GraphQL API連携 | uvx (自動) | ❌ 無効 |

---

## インストール手順

### 1. sequential-thinking (自動インストール)

`hagi install`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @modelcontextprotocol/server-sequential-thinking
```

---

### 2. serena (uvx経由インストール、デフォルト無効) - Phase 2e

有効化時にuvx経由でGitHubリポジトリから自動的にインストールされます。

**前提条件:**
- uv (Python package manager)

**インストール方法:**

serenaはuvxが自動的にGitHubから取得するため、手動インストールは不要です。

**手動確認:**
```bash
uvx --from git+https://github.com/oraios/serena serena start-mcp-server --help
```

**特徴:**
- セマンティックコード検索・解析
- LSP対応言語: Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++
- プロジェクトごとのキャッシュ管理

**キャッシュ管理:**
- デフォルトキャッシュ: `~/.cache/serena/`
- プロジェクトキャッシュ: `.serena/` (自動的に`.gitignore`に追加済み)
- 定期クリーンアップ推奨: 30日以上経過したファイル削除

**クリーンアップスクリプト:**
```bash
# プロジェクトルートで実行
find .serena/ -type f -mtime +30 -delete
```

**有効化方法:**
```bash
hagi mcp enable serena

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**連携:**
- `/code-pattern`コマンド: serena + mcp-memory-serviceで過去パターン検索
- `/research`コマンド: Step 3bで現在のコードベースとの統合提案

**トラブルシューティング:**

uvが見つからない場合:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
source ~/.cargo/env
```

初回起動が遅い場合:
- GitHubからクローンするため初回は時間がかかります
- 2回目以降はuvxがキャッシュするため高速になります

---

### 3. file-search (手動ビルド必須、上級者向け)

**リポジトリ:** https://github.com/Kurogoma4D/file-search-mcp

**特徴:**
- Rust + Tantivy実装による高速全文検索
- スコアベースのランキング
- バイナリファイル自動除外

⚠️ **注意:** ソースからのビルドが必要です。事前ビルド済みバイナリは提供されていません。

**前提条件:**
- Rust toolchain (rustup経由)

**インストール手順:**

```bash
# 1. Rustツールチェインをインストール(未インストールの場合)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. インストール先ディレクトリを作成
mkdir -p ~/.local/opt/mcp-servers

# 3. リポジトリクローン
cd ~/.local/opt/mcp-servers
git clone https://github.com/Kurogoma4D/file-search-mcp.git
cd file-search-mcp

# 4. リリースビルド
cargo build --release

# 5. バイナリ確認
ls -la target/release/file-search-mcp
```

**有効化方法:**
```bash
hagi mcp enable file-search

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**トラブルシューティング:**

ビルドエラーが出る場合:
```bash
# Rustツールチェインを最新化
rustup update

# クリーンビルド
cargo clean && cargo build --release
```

---

### 4. git (自動インストール)

有効化時にuvx経由で自動的にインストールされます。

**前提条件:**
- uvのインストールが必要

**手動確認:**
```bash
uvx mcp-server-git --repository .
```

---

### 5. github (GitHub REST API、自動インストール + .env PAT対応)

⚠️ **非推奨警告:**
現在使用しているnpmパッケージ`@modelcontextprotocol/server-github`は非推奨です。GitHubは公式MCPを`github/github-mcp-server`に移行していますが、DockerまたはHTTP経由での利用が必要なため、hagiの軽量セットアップの方針に合わず、既存設定を維持しています。

有効化時にnpx経由で自動的にインストールされます。

**特徴:**
- GitHub REST API v3を使用
- Issues、Pull Requests、Repositoriesの操作
- Web検索に比べて10-50倍トークン効率が高い

**前提条件:**
- GitHub Personal Access Token(PAT)の発行が必要

**PAT発行手順:**
1. GitHub → Settings → Developer settings → Personal access tokens → Tokens(classic)
2. "Generate new token (classic)" をクリック
3. スコープを選択:
   - `repo` (リポジトリアクセス)
   - `read:org` (組織情報読み取り)
4. トークンをコピー

**設定方法:**

**方法1: .envファイルで管理(推奨)**

プロジェクトルートに`.env`ファイルを作成:

```bash
# .env
GITHUB_PERSONAL_ACCESS_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**重要:** `.env`ファイルは必ず`.gitignore`に追加してください:

```bash
echo ".env" >> .gitignore
```

**方法2: シェル環境変数で管理**

```bash
# ~/.bashrc または ~/.zshrc
export GITHUB_PERSONAL_ACCESS_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# 設定反映
source ~/.bashrc
```

**方法3: mcp.jsonに直接記載(非推奨)**

⚠️ セキュリティリスクがあるため、.envまたはシェル環境変数を推奨します。

```json
"github": {
  "env": {
    "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  }
}
```

**有効化方法:**

```bash
# .envファイルを作成してから有効化
hagi mcp enable github

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**hagiのトークン検証機能:**

`hagi mcp enable github`実行時、以下の順序でトークンの存在を確認します:

1. プロジェクトルートの`.env`ファイルをチェック
2. シェル環境変数 `GITHUB_PERSONAL_ACCESS_TOKEN`をチェック
3. トークンが見つかれば、確認メッセージを表示
4. 見つからなければ、設定方法のガイダンスを表示

**重要:** hagiはトークンを`~/.claude/mcp.json`に書き込みません。トークンは.envファイルまたはシェル環境変数で管理し、Claude Codeが実行時に読み込みます。

**動作確認:**

```bash
# トークンがロードされたか確認
hagi mcp info github

# 出力例:
# MCP Server: github
# Status: enabled
# Command: npx -y @modelcontextprotocol/server-github
# Environment:
#   GITHUB_PERSONAL_ACCESS_TOKEN: *** (set)
# Description: GitHub REST API integration (issues, PRs, repos)
```

**将来的な対応:**
- パッケージが削除された場合は、GitHub公式MCPへの移行を検討
- または別のGitHub連携ツールへの切り替えを検討

---

### 6. github-graphql (GitHub GraphQL API、自動インストール + .env PAT対応)

**特徴:**
- GitHub GraphQL API v4を使用
- REST APIより10-50倍トークン効率が高い(必要なフィールドのみ取得)
- バッチクエリ対応(複数リソースを1回のリクエストで取得)
- 複雑な条件検索が可能

**前提条件:**
- GitHub Personal Access Token(PAT)の発行が必要

**PAT発行手順:**

github MCPと同じ手順でトークンを発行できます。

1. GitHub → Settings → Developer settings → Personal access tokens → Tokens(classic)
2. "Generate new token (classic)" をクリック
3. スコープを選択:
   - `repo` (リポジトリアクセス)
   - `read:org` (組織情報読み取り)
4. トークンをコピー

**設定方法:**

**方法1: .envファイルで管理(推奨)**

プロジェクトルートに`.env`ファイルを作成:

```bash
# .env
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# 注意: github REST APIとは環境変数名が異なります
# - github REST: GITHUB_PERSONAL_ACCESS_TOKEN
# - github GraphQL: GITHUB_TOKEN
```

**重要:** `.env`ファイルは必ず`.gitignore`に追加してください:

```bash
echo ".env" >> .gitignore
```

**方法2: シェル環境変数で管理**

```bash
# ~/.bashrc または ~/.zshrc
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# 設定反映
source ~/.bashrc
```

**方法3: mcp.jsonに直接記載(非推奨)**

⚠️ セキュリティリスクがあるため、.envまたはシェル環境変数を推奨します。

```json
"github-graphql": {
  "env": {
    "GITHUB_TOKEN": "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  }
}
```

**有効化方法:**

```bash
# .envファイルを作成してから有効化
hagi mcp enable github-graphql

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**hagiのトークン検証機能:**

`hagi mcp enable github-graphql`実行時、以下の順序でトークンの存在を確認します:

1. プロジェクトルートの`.env`ファイルをチェック
2. シェル環境変数 `GITHUB_TOKEN`をチェック
3. トークンが見つかれば、確認メッセージを表示
4. 見つからなければ、設定方法のガイダンスを表示

**重要:** hagiはトークンを`~/.claude/mcp.json`に書き込みません。トークンは.envファイルまたはシェル環境変数で管理し、Claude Codeが実行時に読み込みます。

**動作確認:**

```bash
# トークンがロードされたか確認
hagi mcp info github-graphql

# 出力例:
# MCP Server: github-graphql
# Status: enabled
# Command: uvx mcp-server-github-graphql
# Environment:
#   GITHUB_TOKEN: *** (set)
# Description: GitHub GraphQL API (advanced queries, batch ops)
```

**GraphQL vs REST APIの比較:**

| 項目 | REST API(github) | GraphQL API(github-graphql) |
|------|------------------|-----------------------------|
| トークン効率 | 標準 | 10-50倍高い |
| バッチクエリ | 不可 | 可能 |
| 必要なフィールドのみ取得 | 不可 | 可能 |
| 複雑な検索 | 制限あり | 柔軟 |
| インストール | npx(自動) | uvx(自動) |
| 環境変数名 | `GITHUB_PERSONAL_ACCESS_TOKEN` | `GITHUB_TOKEN` |

**どちらを使うべきか:**

- **github-graphql推奨**: トークン効率を重視する場合、複雑なクエリが必要な場合
- **github使用**: シンプルな操作のみの場合、既存のREST API連携がある場合

**両方を同時に有効化することも可能です。**

```bash
hagi mcp enable github github-graphql
```

---

### 7. context7 (自動インストール、デフォルト有効)

`hagi install --global`実行時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y @upstash/context7-mcp
```

**特徴:**
- 公式ドキュメント検索(バージョン指定対応)
- API keyなしで基本機能が使える(制限あり)
- 軽量、起動高速

**API key設定(オプション):**

より高度な機能を使用したい場合、`~/.claude/mcp.json`を編集:
```json
"context7": {
  "env": {
    "CONTEXT7_API_KEY": "your_api_key_here"
  }
}
```

---

### 8. one-search (自動インストール、デフォルト無効)

有効化時にnpx経由で自動的にインストールされます。

**手動確認:**
```bash
npx -y one-search-mcp
```

**特徴:**
- マルチエンジンWeb検索(DuckDuckGo、Bing、SearXNG、Tavily)
- Windows + WSL2推奨: DuckDuckGoプロバイダー(Puppeteerなし)

**プロバイダー選択:**

`~/.claude/mcp.json`で設定変更可能:
```json
"one-search": {
  "disabled": false,
  "env": {
    "SEARCH_PROVIDER": "duckduckgo"  // 推奨: 軽量、高速
  }
}
```

**プロバイダー一覧:**
- `duckduckgo` - 推奨(WSL2環境で軽量)
- `bing` - Bing API keyが必要
- `searxng` - セルフホストSearXNGインスタンスが必要
- `tavily` - Tavily API keyが必要

**有効化方法:**

```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable one-search

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

---

### 9. memory (mcp-memory-service) (手動インストール、デフォルト無効)

完全ローカルで動作する長期記憶管理システムです。

**特徴:**
- 完全ローカル動作(SQLite-vec + ONNX埋め込み)
- 外部LLM不要
- プライバシー保護(クラウドにデータ送信なし)
- XDG Base Directory準拠
- 軽量(~50MB)

**前提条件:**
- **Python 3.10-3.13** (3.14は未対応)
  - 推奨: Python 3.13
  - 理由: PyTorch 2.8.0がPython 3.14をサポートしていない
- uv (Python package manager)
- Git

**インストール手順:**

```bash
# 1. リポジトリをクローン
mkdir -p ~/.local/opt/mcp-servers
cd ~/.local/opt/mcp-servers
git clone https://github.com/doobidoo/mcp-memory-service.git

# 2. Python 3.13で仮想環境を作成し、依存関係をインストール
cd mcp-memory-service
uv venv --python 3.13
uv pip install -e .

# 3. パッチ適用(HF_HOME環境変数問題の修正)
# v8.4.3およびv8.5.0でHF_HOMEが上書きされる問題を修正
curl -fsSL https://raw.githubusercontent.com/kiffveef/hagi/main/patches/mcp-memory-service-hf-home-fix.patch | git apply

# 4. .env ファイルを作成
curl -fsSL https://raw.githubusercontent.com/kiffveef/hagi/main/templates/mcp-memory-service/.env.template -o .env

# または手動で .env を作成:
cat > .env << 'EOF'
# MCP Memory Service Configuration
MCP_MEMORY_STORAGE_BACKEND=sqlite_vec
HF_HOME=${HOME}/.cache/huggingface
TRANSFORMERS_CACHE=${HOME}/.cache/huggingface
SENTENCE_TRANSFORMERS_HOME=${HOME}/.cache/huggingface
MCP_MEMORY_SQLITE_PATH=${HOME}/.local/share/mcp-memory-service/primary_sqlite_vec.db
MCP_MEMORY_CHROMA_PATH=${HOME}/.local/share/mcp-memory-service/chroma_db
MCP_MEMORY_BACKUPS_PATH=${HOME}/.local/share/mcp-memory-service/backups
EOF

# 5. データディレクトリを作成
mkdir -p ~/.local/share/mcp-memory-service/{chroma_db,backups}

# 6. 埋め込みモデルをダウンロード(初回のみ、~50MB)
# sentence-transformers/all-MiniLM-L6-v2モデルを事前にダウンロード
uv run python -c "from sentence_transformers import SentenceTransformer; print('Downloading model...'); model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2'); print('Model downloaded successfully')"

# 7. 動作確認
uv run memory server --help
```

**パッチ詳細:**

mcp-memory-serviceのv8.4.3およびv8.5.0には、低メモリシステム(8GB未満)で環境変数を無条件に上書きする問題があります。このパッチは、ユーザーが設定したHF_HOME等の環境変数を尊重するように修正します。

- **影響を受けるバージョン**: v8.4.3, v8.5.0
- **修正内容**: `server.py`の環境変数設定を既存の値を確認してから設定するように変更
- **パッチファイル**: [patches/mcp-memory-service-hf-home-fix.patch](https://github.com/kiffveef/hagi/blob/main/patches/mcp-memory-service-hf-home-fix.patch)

**データ保存場所:**
- データベース: `~/.local/share/mcp-memory-service/chroma_db/`
- バックアップ: `~/.local/share/mcp-memory-service/backups/`
- Hugging Faceキャッシュ: `~/.cache/huggingface/`

**環境変数の説明:**

mcp-memory-serviceは以下の環境変数で動作を制御します(すべてXDG Base Directory準拠):

- `MCP_MEMORY_STORAGE_BACKEND`: ストレージバックエンド
  - 設定値: `sqlite_vec`(推奨)
  - 用途: ベクトル検索エンジンの選択

- `MCP_MEMORY_CHROMA_PATH`: データベース保存先
  - デフォルト: `~/.local/share/mcp-memory-service/chroma_db/`
  - XDG準拠: `${XDG_DATA_HOME:-$HOME/.local/share}/mcp-memory-service/chroma_db`
  - 用途: 記憶データの永続化

- `MCP_MEMORY_BACKUPS_PATH`: バックアップ保存先
  - デフォルト: `~/.local/share/mcp-memory-service/backups/`
  - XDG準拠: `${XDG_DATA_HOME:-$HOME/.local/share}/mcp-memory-service/backups`
  - 用途: データのバックアップ

- `HF_HOME`: Hugging Faceモデルキャッシュ
  - デフォルト: `~/.cache/huggingface/`
  - XDG準拠: `${XDG_CACHE_HOME:-$HOME/.cache}/huggingface`
  - 用途: ONNX埋め込みモデルのキャッシュ
  - **重要**: 未設定の場合、非推奨の`TRANSFORMERS_CACHE`が使用され警告が表示されます

すべての環境変数は`~/.claude/mcp.json`に設定済みです。

**有効化方法:**

```bash
# 将来のhagiコマンド(実装予定)
hagi mcp enable memory

# または手動で~/.claude/mcp.jsonを編集
# "disabled": true → false に変更
```

**スラッシュコマンド連携:**

`/research`コマンドがmemoryと自動連携します:
- 調査結果を自動保存
- 同一トピック検索時に過去の調査を表示
- メモリ更新機能

詳細は`templates/.claude/commands/research.md`を参照してください。

---

## hagiでのセットアップ

### グローバルセットアップ

```bash
# グローバル設定を~/.claude/にインストール
hagi install --global

# ドライラン(変更内容の確認のみ)
hagi install --global --dry-run
```

**セットアップ内容:**
- `~/.claude/mcp.json`の作成・マージ
- `~/.claude/settings.json`の作成・マージ
- デフォルト有効MCP:
  - sequential-thinking(構造化思考支援)
  - context7(公式ドキュメント検索)
- デフォルト無効MCP(手動で有効化可能):
  - one-search(Web検索)
  - memory(長期記憶管理)
  - serena、file-search、git、github
- 既存ファイルの自動バックアップ(タイムスタンプ付き、最新3世代のみ保持)
- 依存関係チェック(Node.js、uv、Python3、Git)と警告表示

---

### プロジェクト個別セットアップ

```bash
# プロジェクトルートで実行
cd /path/to/your/project
hagi install
```

**セットアップ内容:**
- `.claude/`ディレクトリ作成
- `.claude/CLAUDE.md`、`.claude/instructions/`のコピー
- `.claude/settings.local.json`のコピー
- `.gitignore`の更新

---

## 個別の有効化・無効化

### 方法1: hagiコマンド(将来実装予定)

```bash
# MCPサーバーを有効化
hagi mcp enable serena
hagi mcp enable file-search

# MCPサーバーを無効化
hagi mcp disable serena

# 一覧表示
hagi mcp list
```

### 方法2: 手動編集

`~/.claude/mcp.json`を編集:

```json
{
  "mcpServers": {
    "serena": {
      "command": "npx",
      "args": ["-y", "serena-mcp-server"],
      "disabled": false  // true → false に変更
    }
  }
}
```

**変更後の反映:**
- Claude Codeを再起動

---

## トラブルシューティング

### file-search MCPが動作しない

**症状:**
```
Command not found: file-search-mcp
```

**解決方法:**
1. インストール確認:
   ```bash
   cargo install --list | grep file-search
   ```

2. パス確認:
   ```bash
   which file-search-mcp
   ```

3. パスが通っていない場合:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

4. シェル設定ファイルに追加:
   ```bash
   # ~/.bashrc または ~/.zshrc
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

---

### github/github-graphql MCPでPATエラー

**症状:**
```
Error: GITHUB_PERSONAL_ACCESS_TOKEN is not set
# または
Error: GITHUB_TOKEN is not set
```

**解決方法:**

1. `.env`ファイル確認:
   ```bash
   # プロジェクトルートで確認
   cat .env

   # github REST APIの場合
   # GITHUB_PERSONAL_ACCESS_TOKEN=ghp_...

   # github-graphqlの場合
   # GITHUB_TOKEN=ghp_...
   ```

2. トークンの再発行:
   - 有効期限切れの場合、GitHubで再発行
   - github: `GITHUB_PERSONAL_ACCESS_TOKEN`
   - github-graphql: `GITHUB_TOKEN`

3. シェル環境変数確認:
   ```bash
   echo $GITHUB_PERSONAL_ACCESS_TOKEN
   echo $GITHUB_TOKEN
   ```

4. mcp.json確認:
   ```bash
   cat ~/.claude/mcp.json | jq '.mcpServers.github.env'
   cat ~/.claude/mcp.json | jq '.mcpServers["github-graphql"].env'
   ```

5. 再有効化(トークンを再注入):
   ```bash
   # .envファイルを修正してから
   hagi mcp enable github
   # または
   hagi mcp enable github-graphql
   ```

---

### serenaのメモリ肥大化

**症状:**
- `.serena/`ディレクトリが肥大化

**解決方法:**
1. 古いファイルを削除:
   ```bash
   find .serena/ -type f -mtime +30 -delete
   ```

2. 定期クリーンアップスクリプト作成:
   ```bash
   # cleanup-serena.sh
   #!/bin/bash
   find .serena/ -type f -mtime +30 -delete
   echo "Cleaned up old serena files"
   ```

3. cron設定(週次実行):
   ```bash
   crontab -e
   # 毎週日曜 0:00に実行
   0 0 * * 0 /path/to/cleanup-serena.sh
   ```

---

### MCP接続エラー

**症状:**
```
Failed to connect to MCP server
```

**解決方法:**
1. 依存関係確認:
   ```bash
   # Node.js
   node --version

   # uv
   uv --version

   # cargo
   cargo --version
   ```

2. MCP設定確認:
   ```bash
   cat ~/.claude/mcp.json | jq
   ```

3. Claude Code再起動

4. ログ確認:
   ```bash
   # Claude Codeのログを確認
   # パスは環境により異なる
   tail -f ~/.local/state/claude-code/logs/*.log
   ```

---

### one-search接続エラー

**症状:**
```
Failed to connect to one-search MCP
```

**解決方法:**
1. Node.js確認:
   ```bash
   node --version  # v18以降推奨
   ```

2. プロバイダー設定確認:
   ```bash
   cat ~/.claude/mcp.json | jq '.mcpServers["one-search"].env'
   ```

3. DuckDuckGoプロバイダーに変更(WSL2推奨):
   ```json
   "one-search": {
     "env": {
       "SEARCH_PROVIDER": "duckduckgo"
     }
   }
   ```

4. Puppeteer依存のプロバイダーを避ける(WSL2環境)

---

### mcp-memory-service起動エラー

**症状:**
```
Failed to start memory server
```

**解決方法:**
1. インストール確認:
   ```bash
   ls ~/.local/opt/mcp-servers/mcp-memory-service/
   ```

2. uv環境確認:
   ```bash
   cd ~/.local/opt/mcp-servers/mcp-memory-service
   uv sync
   ```

3. Python3確認:
   ```bash
   python3 --version
   ```

4. データディレクトリ作成:
   ```bash
   mkdir -p ~/.local/share/mcp-memory-service/{chroma_db,backups}
   ```

5. 手動起動テスト:
   ```bash
   cd ~/.local/opt/mcp-servers/mcp-memory-service
   uv run memory server
   ```

---

### mcp-memory-service で TRANSFORMERS_CACHE 警告

**症状:**
```
FutureWarning: Using `TRANSFORMERS_CACHE` is deprecated and will be removed in v5 of Transformers. Use `HF_HOME` instead.
```

**原因:**
- `HF_HOME`環境変数が未設定
- Transformers v5で`TRANSFORMERS_CACHE`が削除される予定

**解決方法:**

最新のhagiテンプレートでは`HF_HOME`が設定済みです。以下で確認:

```bash
cat ~/.claude/mcp.json | grep -A 5 '"memory"'
```

`HF_HOME`が含まれていない場合、手動で追加:

```json
"memory": {
  "env": {
    "MCP_MEMORY_STORAGE_BACKEND": "sqlite_vec",
    "MCP_MEMORY_CHROMA_PATH": "${XDG_DATA_HOME:-$HOME/.local/share}/mcp-memory-service/chroma_db",
    "MCP_MEMORY_BACKUPS_PATH": "${XDG_DATA_HOME:-$HOME/.local/share}/mcp-memory-service/backups",
    "HF_HOME": "${XDG_CACHE_HOME:-$HOME/.cache}/huggingface"
  }
}
```

変更後、Claude Codeを再起動すると警告が消えます。

---

### context7 API制限

**症状:**
```
Rate limit exceeded
```

**解決方法:**
1. API keyなしの基本機能を使用している場合、レート制限があります

2. API keyを取得して設定:
   ```bash
   # ~/.claude/mcp.jsonを編集
   {
     "context7": {
       "env": {
         "CONTEXT7_API_KEY": "your_api_key"
       }
     }
   }
   ```

---

### Windows + WSL2環境でのMCP起動遅延

**症状:**
- MCP起動に時間がかかる(3秒以上)

**解決方法:**
1. Docker依存のMCPを無効化
2. DuckDuckGoプロバイダー使用(Puppeteerなし)
3. npx版MCPを優先(sequential-thinking、context7、one-search)
4. WSL2のメモリ制限を確認:
   ```bash
   # ~/.wslconfig
   [wsl2]
   memory=4GB
   processors=2
   ```

---

## 参考リンク

### 推奨構成(Phase 2d-2e)
- [context7 MCP](https://github.com/upstash/context7-mcp) - 公式ドキュメント検索 (Phase 2d)
- [one-search MCP](https://github.com/supercorp-ai/one-search-mcp) - マルチエンジンWeb検索 (Phase 2d)
- [mcp-memory-service](https://github.com/doobidoo/mcp-memory-service) - 完全ローカル長期記憶管理 (Phase 2d)
- [serena MCP](https://github.com/serena-ai/serena-mcp) - セマンティックコード解析 (Phase 2e)

### その他のMCP
- [sequential-thinking MCP](https://github.com/modelcontextprotocol/servers)
- [file-search MCP](https://github.com/Kurogoma4D/file-search-mcp)
- [git MCP](https://github.com/modelcontextprotocol/servers)
- [github MCP](https://github.com/modelcontextprotocol/servers)

---

## フィードバック

不明点や問題があれば、以下にissueを作成してください:
https://github.com/kiffveef/hagi/issues
