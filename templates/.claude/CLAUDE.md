# Project Guidelines

このファイルは、プロジェクト固有のルール・ガイドラインを記述するためのテンプレートです。
必要に応じてカスタマイズしてください。

---

## 🚨 Pre-Work Checklist - READ THIS FIRST

**Before ANY file edit or code change, ALWAYS check:**

### Git Branch Check
```bash
# STEP 1: Check current branch BEFORE editing
git branch --show-current

# STEP 2: If on main, CREATE BRANCH IMMEDIATELY
git checkout -b <prefix>/descriptive-name
```

### Task Management Check (when using /st --todo)
- [ ] Read `.claude/TODO.md` if it exists
- [ ] Use TodoWrite tool to track progress
- [ ] **IMMEDIATELY** update `.claude/TODO.md` after EVERY TodoWrite call
- [ ] Keep TodoWrite ↔ TODO.md synchronized throughout session

### Documentation Check
- [ ] After implementing feature, update user-facing docs BEFORE merging to main
- [ ] README.md, docs/usage.md, docs/mcp-setup.md as needed

**Remember:**
- 🔴 Editing files on `main` breaks workflow
- 🔴 TodoWrite without TODO.md sync loses progress
- 🔴 Missing documentation blocks users from using features

---

## 📋 Current Tasks

@TODO.md

---

## Top-Level Rules

@instructions/efficiency.md

@instructions/communication.md

@instructions/documentation.md

@instructions/tools.md

---

## Programming Rules

@instructions/code-quality.md

@instructions/code-style.md

@instructions/best-practices.md

@instructions/task-management.md

@instructions/security.md

@instructions/git-workflow.md

---

## プロジェクト概要

<!-- プロジェクトの目的、主な機能、技術スタックなどを記述 -->

---

## コーディング規約

<!-- プロジェクト固有のコーディング規約を記述 -->

### 命名規則

<!-- 変数名、関数名、クラス名などの命名規則 -->

### ファイル構成

<!-- ディレクトリ構造、ファイル配置のルール -->

---

## ブランチ運用

<!-- プロジェクト固有のブランチ運用ルール -->

---

## コミットメッセージ

<!-- プロジェクト固有のコミットメッセージ形式 -->

---

## タスク管理

<!-- プロジェクトのタスク管理方法 -->

---

## テスト

<!-- テスト方針、テストの実行方法 -->

---

## デプロイ

<!-- デプロイ手順、注意事項 -->

---

## 参考リンク

<!-- プロジェクト関連のドキュメント、リポジトリへのリンク -->
