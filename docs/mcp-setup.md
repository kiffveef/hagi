# MCP導入ガイド

hagiが管理するMCPサーバーの設定・活用方法。

---

## MCPサーバー一覧

| MCP | 用途 | 実行方法 | デフォルト |
|-----|------|----------|-----------|
| context7 | 公式ドキュメント検索 | npx | ✅ 有効 |
| memory | 長期記憶(Memento、BGE-M3多言語) | npx | ✅ 有効 |
| one-search | Web検索(DuckDuckGo他) | npx | ❌ 無効 |
| serena | セマンティックコード解析 | uvx | ❌ 無効 |
| git | Git操作 | uvx | ❌ 無効 |

**前提条件:** npx系はNode.js v18以降、uvx系はuvが必要。

---

## セットアップ

```bash
# プロジェクトに.claude/mcp.jsonを配置
hagi install

# 有効化/無効化
hagi mcp enable serena
hagi mcp disable git

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

### memory (Memento)

完全ローカルの長期記憶。SQLiteベース、XDG Base Directory準拠。

- データ: `~/.local/share/claude-memory/memory.db`
- `/st`, `/design`, `/serena`と連携

### one-search

マルチエンジンWeb検索。DuckDuckGoプロバイダーが推奨(Puppeteerなし、軽量)。

`.claude/mcp.json`でプロバイダーを変更可能:

```json
"one-search": {
  "env": {
    "SEARCH_PROVIDER": "duckduckgo"
  }
}
```

他のプロバイダー: `bing`(API key必要)、`searxng`(セルフホスト)、`tavily`(API key必要)

### serena

セマンティックコード解析。LSP対応(Python、TypeScript/JavaScript、Rust、Go、PHP、Java、C/C++)。

- キャッシュ: `.serena/`(`.gitignore`に自動追加済み)
- 初回起動はGitHubからクローンするため遅い(2回目以降はキャッシュ済み)
- `/serena`, `/review`と連携

**キャッシュクリーンアップ:**
```bash
find .serena/ -type f -mtime +30 -delete
```

### git

uvx経由のGit操作MCP。

```bash
hagi mcp enable git
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
