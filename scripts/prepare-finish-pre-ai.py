from pathlib import Path

root = Path(__file__).resolve().parents[1]
path = root / "scripts/finish-pre-ai.py"
text = path.read_text(encoding="utf-8")

queue_block = '''literal(\n    "src-tauri/src/persistence.rs",\n    ''' + "'''" + '''    #[allow(dead_code)]\\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\\n        self.tx\\n            .send(PersistenceCommand::StoreMemory(memory))\\n            .map_err(|_| \"persistence worker channel is unavailable\".to_owned())\\n    }\\n\\n''' + "'''" + ''',\n    "",\n    expected=2,\n)'''
count = text.count(queue_block)
if count != 1:
    raise RuntimeError(f"expected one queue_memory precondition block, found {count}")
text = text.replace(queue_block, queue_block.replace("expected=2", "expected=1"))

# README's clean-bundle wording was already advanced after the original finish script was staged.
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

# Replace the first brittle follow-up implementation only inside the clean runner worktree.
robust = root / "scripts/finish-pre-ai-followup-v2.py"
followup = root / "scripts/finish-pre-ai-followup.py"
followup.write_text(robust.read_text(encoding="utf-8"), encoding="utf-8", newline="\n")

print("Corrected finish-patch preconditions and selected robust semantic hardening v2.")
