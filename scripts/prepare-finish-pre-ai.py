from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "scripts/finish-pre-ai.py"
text = path.read_text(encoding="utf-8")

queue_block = '''literal(\n    "src-tauri/src/persistence.rs",\n    ''' + "'''" + '''    #[allow(dead_code)]\\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\\n        self.tx\\n            .send(PersistenceCommand::StoreMemory(memory))\\n            .map_err(|_| \"persistence worker channel is unavailable\".to_owned())\\n    }\\n\\n''' + "'''" + ''',\n    "",\n    expected=2,\n)'''
if text.count(queue_block) != 1:
    raise RuntimeError("queue_memory precondition block changed")
text = text.replace(queue_block, queue_block.replace("expected=2", "expected=1"))

start_marker = '''literal(\n    "README.md",\n    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate'''
next_marker = '''literal(\n    "README.md",\n    "canonical Lenvu production master'''
start = text.find(start_marker)
end = text.find(next_marker, start + 1)
if start < 0 or end < 0:
    raise RuntimeError("could not isolate obsolete README bundle literal")
text = text[:start] + text[end:]

needle = 'print("Pre-AI finish patch applied.")\n'
replacement = needle + '\nimport runpy\nrunpy.run_path(str(ROOT / "scripts/finish-pre-ai-finalize.py"), run_name="__main__")\n'
if text.count(needle) != 1:
    raise RuntimeError("could not chain current-state README finalizer")
text = text.replace(needle, replacement)
path.write_text(text, encoding="utf-8", newline="\n")

robust = root / "scripts/finish-pre-ai-followup-v2.py"
robust_text = robust.read_text(encoding="utf-8")

# The evaluator has this column list in the production INSERT and manual-authority test seed.
old_call = '''replace(\n    "src-tauri/src/memory_evaluator.rs",\n    "INSERT INTO memories (kind, content, importance, source_event_id, created_at_ms, updated_at_ms)",\n    "INSERT INTO memories (kind, content, importance, source_event_id, origin, created_at_ms, updated_at_ms)",\n)'''
new_call = old_call[:-1] + '    count=2,\n)'
if robust_text.count(old_call) != 1:
    raise RuntimeError("could not adjust evaluator origin column-list count")
robust_text = robust_text.replace(old_call, new_call)

# The parameterized production VALUES become automatic; the literal manual-authority seed gets
# an explicit manual origin so its column count stays valid after the shared column rewrite.
manual_anchor = '''replace(\n    "src-tauri/src/memory_evaluator.rs",\n    "VALUES (?1, ?2, ?3, ?4, ?5, ?5)",\n    "VALUES (?1, ?2, ?3, ?4, 'automatic', ?5, ?5)",\n)'''
manual_patch = manual_anchor + '''\nreplace(\n    "src-tauri/src/memory_evaluator.rs",\n    "VALUES ('preference', 'The user repeatedly chooses to play with Lenvu.', 0.9, NULL, 1, 1)",\n    "VALUES ('preference', 'The user repeatedly chooses to play with Lenvu.', 0.9, NULL, 'manual', 1, 1)",\n)'''
if robust_text.count(manual_anchor) != 1:
    raise RuntimeError("could not add evaluator manual-origin seed")
robust_text = robust_text.replace(manual_anchor, manual_patch)

# The two test schemas are Rust string literals. Match only the unique SQLite token so the
# follow-up does not depend on whether the generated Rust source keeps a line-continuation escape.
for path_name in ["src-tauri/src/memory_evaluator.rs", "src-tauri/src/memory_admin.rs"]:
    brittle = f'''replace(\n    "{path_name}",\n    "                   source_event_id INTEGER,\\n                   created_at_ms INTEGER NOT NULL,",\n    "                   source_event_id INTEGER,\\n                   origin TEXT NOT NULL DEFAULT 'manual',\\n                   created_at_ms INTEGER NOT NULL,",\n)'''
    stable = f'''replace(\n    "{path_name}",\n    "source_event_id INTEGER,",\n    "source_event_id INTEGER, origin TEXT NOT NULL DEFAULT 'manual',",\n)'''
    if robust_text.count(brittle) != 1:
        raise RuntimeError(f"could not stabilize schema origin rewrite for {path_name}")
    robust_text = robust_text.replace(brittle, stable)

compile(robust_text, "finish-pre-ai-followup.py", "exec")
(root / "scripts/finish-pre-ai-followup.py").write_text(
    robust_text,
    encoding="utf-8",
    newline="\n",
)
print("Corrected finish patch with token-stable semantic hardening.")
