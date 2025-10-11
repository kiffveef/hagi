# hagi開発ガイド

hagiへの貢献方法と開発環境のセットアップ方法です。

---

## 開発環境のセットアップ

### 前提条件

- Rust 1.80以降
- Git
- Node.js v18以降(MCPテスト用)
- uv(Python package manager、MCPテスト用)

### リポジトリのクローン

```bash
git clone https://github.com/kiffveef/hagi.git
cd hagi
```

### ビルド

```bash
cargo build
```

### テスト実行

```bash
# ユニットテスト
cargo test

# 統合テスト(実際にコマンドを実行)
cargo run -- --help
cargo run -- install --dry-run
```

---

## プロジェクト構造

```
hagi/
├── src/
│   ├── main.rs              # エントリーポイント、CLI定義
│   ├── utils.rs             # 共通ユーティリティ関数
│   ├── templates.rs         # テンプレート埋め込み機能
│   └── commands/            # コマンド実装
│       ├── mod.rs           # コマンドモジュール定義
│       ├── install.rs       # install コマンド
│       ├── uninstall.rs     # uninstall コマンド
│       ├── status.rs        # status コマンド
│       ├── mcp.rs           # mcp コマンド
│       ├── config.rs        # config コマンド
│       └── update.rs        # update コマンド
├── templates/.claude/       # テンプレートファイル(埋め込み対象)
│   ├── mcp.json             # MCP設定テンプレート
│   ├── settings.local.json  # パーミッション設定
│   ├── CLAUDE.md            # プロジェクトガイドライン
│   ├── instructions/        # 詳細インストラクション
│   └── commands/            # スラッシュコマンド
├── docs/                    # ドキュメント
├── Cargo.toml               # Rust依存関係
└── README.md                # プロジェクト概要
```

---

## 開発ワークフロー

### 1. ブランチ戦略

**mainブランチ:**
- 安定版のコードのみ
- リリース可能な状態を維持

**機能ブランチ:**
- 新機能、バグ修正、ドキュメント更新用
- 命名規則:
  - `feature/<name>` - 新機能
  - `fix/<name>` - バグ修正
  - `docs/<name>` - ドキュメント更新
  - `refactor/<name>` - リファクタリング

**例:**
```bash
git checkout -b feature/add-clean-command
# 実装
git add .
git commit -m "✨ Add clean command"
git checkout main
git merge feature/add-clean-command
```

### 2. コミットメッセージ規約

**形式:**
```
<アイコン> <英語の要約>(50文字以内)

- 日本語の箇条書き
- 変更内容を簡潔に記述
```

**アイコン:**
- 🌱 `first` - Initial commit
- ✨ `add` - 新機能追加
- 🔄 `update` - 既存機能更新
- 🐛 `fix` - バグ修正
- 📝 `docs` - ドキュメント
- ♻️ `refactor` - リファクタリング
- 🧪 `test` - テスト追加・更新
- 🔧 `config` - 設定変更

**例:**
```
✨ Add MCP management commands

- hagi mcp list/info/enable/disable を実装
- 自動バックアップと世代管理を統合
- docs/commands.md を更新
```

### 3. コーディング規約

**Rustスタイル:**
- `cargo fmt` でフォーマット
- `cargo clippy` でリント
- `anyhow::Result` でエラーハンドリング
- snake_case (関数、変数)
- PascalCase (型、トレイト)

**ファイル構成:**
- 新しいコマンドは `src/commands/` に追加
- 共通ロジックは `src/utils.rs` に追加
- テンプレートは `templates/.claude/` に配置

---

## 新機能の追加

### 例: 新しいコマンドを追加

**1. コマンド実装ファイルを作成**

`src/commands/newcommand.rs`:
```rust
use anyhow::Result;
use colored::*;

pub fn newcommand() -> Result<()> {
    println!("{}", "New command executed!".green());
    Ok(())
}
```

**2. mod.rsに追加**

`src/commands/mod.rs`:
```rust
pub mod newcommand;
```

**3. main.rsに定義追加**

`src/main.rs`:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... 既存のコマンド

    /// New command description
    NewCommand,
}

// match文に追加
Commands::NewCommand => {
    commands::newcommand::newcommand()?;
}
```

**4. テスト**

```bash
cargo run -- newcommand
```

**5. ドキュメント更新**

- `docs/commands.md` に説明追加
- `README.md` のコマンド一覧に追加

---

## テスト

### 手動テスト

```bash
# ビルド
cargo build

# グローバルインストールテスト(ドライラン)
cargo run -- install -g --dry-run

# プロジェクトインストールテスト
cd /tmp/test-project
cargo run -- install --dry-run

# MCPコマンドテスト
cargo run -- mcp list
cargo run -- mcp info sequential-thinking

# 設定コマンドテスト
cargo run -- config validate mcp
```

### 自動テスト(将来実装予定)

```bash
cargo test
```

---

## リリース

### バージョン更新

`Cargo.toml`のバージョンを更新:
```toml
[package]
name = "hagi"
version = "0.2.0"  # 更新
```

### タグ作成

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### GitHub Releasesで公開

1. GitHubの「Releases」ページへ
2. 「Draft a new release」をクリック
3. タグを選択、リリースノートを記述
4. 「Publish release」

---

## 貢献ガイドライン

### Issue作成

バグ報告や機能要望は、GitHubのIssuesで受け付けています。

**バグ報告テンプレート:**
```
## 症状
何が起きたか

## 再現手順
1. ...
2. ...

## 期待する動作
何が起きるべきか

## 環境
- OS:
- hagiバージョン:
- Rustバージョン:
```

**機能要望テンプレート:**
```
## 概要
何を実現したいか

## 動機
なぜ必要か

## 提案する実装
どのように実装するか(任意)
```

### Pull Request

1. Issueを作成(既存のIssueがあればスキップ)
2. フォークしてブランチ作成
3. 実装・テスト
4. ドキュメント更新
5. PRを作成

**PRテンプレート:**
```
## 変更内容
何を変更したか

## 関連Issue
Closes #123

## テスト
どのようにテストしたか

## チェックリスト
- [ ] cargo fmt 実行済み
- [ ] cargo clippy 実行済み
- [ ] 動作テスト完了
- [ ] ドキュメント更新済み
```

---

## よくある開発タスク

### テンプレートファイルの追加

1. `templates/.claude/` にファイルを追加
2. `cargo build` で自動的に埋め込まれる
3. テスト実行

### 新しいMCPサーバーの追加

1. `templates/.claude/mcp.json` にエントリ追加
2. `src/commands/mcp.rs` の `get_server_description` 関数に説明追加
3. `docs/mcp-setup.md` にセットアップ手順追加

### 共通ユーティリティの追加

1. `src/utils.rs` に関数追加
2. 既存のコマンドで使用例を確認
3. テストを追加(将来実装予定)

---

## デバッグ

### ログ出力

```rust
use colored::*;

println!("{}", "Debug: value = {}".yellow(), value);
eprintln!("{}", "Error: {}".red(), error);
```

### cargo run での実行

```bash
# 詳細なエラー表示
RUST_BACKTRACE=1 cargo run -- install -g

# リリースビルドで実行
cargo run --release -- install -g
```

---

## 関連ドキュメント

- **[README](../README.md)**: プロジェクト概要
- **[インストールガイド](./installation.md)**: インストール方法
- **[コマンドリファレンス](./commands.md)**: 全コマンドの詳細
- **[トラブルシューティング](./troubleshooting.md)**: よくある問題と解決策

---

## 質問・サポート

- **GitHub Issues**: https://github.com/kiffveef/hagi/issues
- **GitHub Discussions**: https://github.com/kiffveef/hagi/discussions
