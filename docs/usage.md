# 使い方ガイド

---

## スキル

`hagi install`で`.claude/skills/`に配置される。Claude Code起動時に自動読み込み。

### /st - 構造化思考

複雑な問題を段階的に分析・解決する。文脈に応じてWebSearch、Context7、TodoWriteを自動選択。memory MCPで過去パターンを自動検索・保存。

```
/st データベース設計を最適化する方法
/st Axum0.7でWebSocket実装
/st "似た問題" --fresh          # memory検索スキップ
```

### /design - 設計文書

設計決定や仕様を`.claude/designs/`に文書化する。memory MCPに自動保存。

```
/design "authentication flow"
/design "error handling strategy"
```

### /note - 会話メモ

直近の会話をマークダウンにまとめる。

```
/note                    # note-YYYYMMDD-HHMMSS.md
/note chat-mode.md       # 指定ファイル名
```

### /serena - コード分析

serena + memory MCPでコードパターンを検索・分析する。分析結果はmemoryに自動保存。

```
/serena "error handling in async functions"
/serena "database pooling"
```

### /review - コードレビュー

サードパーティ視点でレビューと改善提案を行う。

```
/review src/commands/install.rs
/review src/commands/ --focus architecture
/review --diff
/review src/utils.rs --refactor
```

| オプション | 説明 |
|-----------|------|
| `--strict` | 軽微な問題も含める |
| `--focus <area>` | security / performance / readability / architecture |
| `--refactor` | 具体的なコード改善案を提示 |
| `--diff` | git diffをレビュー |

---

## チャットモード

プロジェクトの制約なくClaudeと会話するための専用スペース。

```bash
hagi install --chat
cd ~/.chat && claude
```

軽量MCP(memory + one-search)のみ。`~/.chat/.claude/CLAUDE.md`で応答スタイルをカスタマイズ可能。

---

## プロジェクトテンプレート

`hagi install`で配置されるファイルの概要。

| ファイル | 内容 |
|----------|------|
| `CLAUDE.md` | プロジェクトガイドライン |
| `instructions/` | Git操作、タスク管理、推奨ツール等のルール |
| `skills/` | スラッシュコマンド |
| `mcp.json` | MCP設定 |
| `settings.local.json` | パーミッション、auto memory無効化 |

カスタマイズ後に特定カテゴリだけ更新する場合は`--only`で選択できる。CLAUDE.mdはプロジェクト固有セクション(`<!-- hagi:project:start/end -->`)を保持したまま更新される。

```bash
# instructionsテンプレートのみ更新(CLAUDE.mdも自動更新、プロジェクトセクション保持)
hagi install --only instructions

# skills + instructionsを更新
hagi install --only instructions skills

# 特定ファイルを除外しつつ更新
hagi install --only instructions --skip instructions/simplicity.md
```

`--skip`による除外指定も引き続き利用可能。

```bash
hagi install --skip CLAUDE.md --skip instructions
```

---

## パーミッション

`settings.local.json`で危険コマンドを制限している。

**拒否**: `rm -f`, `sudo`, `chmod 777`, `git push --force`, `git reset`

**許可**: `cargo`, `git add/commit/status/diff/log/branch/checkout/merge`, `rg/fd/bat`, `npx/uv`

詳細は`.claude/settings.local.json`を参照。
