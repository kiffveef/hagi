# Project Guidelines

**📌 About This File:**
- **Project-specific rules and guidelines go here**
- This overrides global `~/.claude/CLAUDE.md` settings
- Customize sections below for your project
- Delete unused sections, add project-specific ones

**📁 File Structure:**
- `CLAUDE.md` - Main project guidelines (this file)
- `instructions/` - Detailed rules imported via `@instructions/`
- `TODO.md` - Task tracking (optional, recommended for `/st --todo`)

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

