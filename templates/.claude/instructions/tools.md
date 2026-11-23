# Preferred Tools

Shell: `grep`→`rg`, `cat`→`bat -p`, `find`→`fd`, `pip`→`uv`

File operations: use Read/Edit/Write/Grep/Glob tools. Never read, write, search or edit files through Bash: no Python or other scripts, no heredoc or `cat > file` writes, no `sed -i`. Several edits go as parallel Edit calls in one message.
Bash only for: system commands, package managers, git, build tools.
