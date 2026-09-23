# MCP導入ガイド

hagiが管理するMCPサーバーの設定・活用方法。

---

## MCPサーバー一覧

| MCP | 用途 | 実行方法 | デフォルト |
|-----|------|----------|-----------|
| context7 | 公式ドキュメント検索 | npx | ✅ 有効 |
| memory | 長期記憶(プロジェクト横断検索) | npx | ❌ 無効 |
| serena | セマンティックコード解析 | uvx | ❌ 無効 |

Web検索は組み込みWebSearch/WebFetch、Git操作はBash経由のgitコマンド、
長期記憶は組み込みauto memoryを使用する(専用MCPは不要)。

**前提条件:** npx系はNode.js v18以降、uvx系はuvが必要。

---

## セットアップ

```bash
# プロジェクトに.claude/mcp.jsonを配置
hagi install

# 有効化/無効化
hagi mcp enable serena
hagi mcp disable serena

# 状態確認
hagi mcp list
```

変更後はClaude Codeの再起動が必要。

---

## 各MCPサーバーの詳細

### context7

API keyなしで基本機能が使える。制限緩和にはAPI keyを設定する。

```json
"context7": {
  "env": {
    "CONTEXT7_API_KEY": "your_api_key_here"
  }
}
```

### memory

完全ローカルの長期記憶(memento)。SQLiteベース、XDG Base Directory準拠。**デフォルト無効**。

- 通常の記憶は組み込みauto memory(`~/.claude/projects/<project>/memory/MEMORY.md`)を使用
- プロジェクト横断のセマンティック検索が必要な場合のみ有効化: `hagi mcp enable memory`
- データ: `~/.local/share/claude-memory/memory.db`

### serena

セマンティックコード解析。LSPベースで40以上の言語に対応。

**組み込みLSPとの使い分け:**

Claude Codeは組み込みLSPツール(2.0.74以降)を持ち、定義ジャンプ・参照検索・シンボル一覧はserenaと重複する。
組み込みLSPは言語ごとの公式プラグインと言語サーバー本体が必要。

| 言語 | 推奨 |
|------|------|
| 公式プラグインあり(Rust、C/C++、TypeScript/JavaScript、Python、Go、Java、Kotlin、C#、PHP、Swift、Lua) | 公式LSPプラグイン(例: `/plugin install rust-analyzer-lsp@claude-plugins-official`)。serena不要 |
| 公式プラグインなし(Ruby、Dart/Flutter、Scalaなど) | 必要なら `hagi mcp enable serena` |

serena固有の機能はシンボル単位の編集(`replace_symbol_body`、`rename_symbol`等)。これが必要な場合も有効化する。
プラグイン名と必要な言語サーバーは[公式ドキュメント](https://code.claude.com/docs/en/discover-plugins#code-intelligence)を参照。

- キャッシュ: `.serena/`(`.gitignore`に自動追加済み)
- 初回起動はGitHubからクローンするため遅い(2回目以降はキャッシュ済み)
- `/serena`, `/review`と連携

**キャッシュクリーンアップ:**
```bash
find .serena/ -type f -mtime +30 -delete
```

---

## 手動編集

`.claude/mcp.json`を直接編集しても良い。`.mcp.json`はシンボリックリンクなので同一ファイル。

```json
{
  "mcpServers": {
    "serena": {
      "disabled": false
    }
  }
}
```

問題が発生した場合は[トラブルシューティング](./troubleshooting.md#mcp関連の問題)を参照。
