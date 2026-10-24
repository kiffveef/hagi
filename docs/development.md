# 開発ガイド

---

## セットアップ

```bash
git clone https://github.com/kiffveef/hagi.git
cd hagi
cargo build
cargo test
```

前提条件: Rust1.85+(edition2024)、Git、Node.js v18+(MCPテスト用)、uv(MCPテスト用)

---

## プロジェクト構造

```
src/
├── main.rs              # CLI定義
├── utils.rs             # 共通ユーティリティ
├── templates.rs         # テンプレート埋め込み
└── commands/
    ├── install.rs
    ├── uninstall.rs
    ├── status.rs
    ├── update.rs
    ├── sync.rs
    ├── mcp.rs
    └── config.rs

templates/.claude/       # 埋め込みテンプレート
docs/                    # ドキュメント
```

---

## 開発ワークフロー

### ブランチ

`feature/`, `fix/`, `docs/`, `refactor/`, `test/`, `config/` プレフィックスを使用。mainブランチへの直接コミットは禁止。

### コミットメッセージ

```
<アイコン> <英語の要約>(50文字以内)

- 日本語の箇条書き(任意)
```

アイコン: 🌱first ✨add 🔄update 🐛fix 📝docs ♻️refactor 🧪test 🔧config

### コーディング規約

- `cargo fmt` + `cargo clippy`
- `anyhow::Result`でエラーハンドリング
- snake_case(関数、変数)、PascalCase(型、トレイト)

---

## よくある開発タスク

### 新コマンドの追加

1. `src/commands/newcommand.rs` を作成
2. `src/commands/mod.rs` にモジュール追加
3. `src/main.rs` にサブコマンド定義を追加
4. `docs/commands.md` に説明追加

### テンプレートファイルの追加

`templates/.claude/`にファイルを追加すると`cargo build`で自動埋め込み。

### 新MCPサーバーの追加

1. `templates/.claude/mcp.json` にエントリ追加
2. `src/commands/mcp.rs` の `get_server_description` に説明追加
3. `docs/mcp-setup.md` に記載

---

## テスト

```bash
cargo run -- install -g --dry-run
cargo run -- install --dry-run
cargo run -- mcp list
cargo run -- config validate mcp
```

---

## バージョン更新

```bash
# Cargo.tomlのversionを更新
git tag -a v0.2.1 -m "v0.2.1"
git push origin v0.2.1
```
