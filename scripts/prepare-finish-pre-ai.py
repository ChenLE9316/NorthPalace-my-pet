from pathlib import Path

root = Path(__file__).resolve().parents[1]
base = root / "scripts/finish-pre-ai.py"
text = base.read_text(encoding="utf-8")

queue_block = '''literal(\n    "src-tauri/src/persistence.rs",\n    ''' + "'''" + '''    #[allow(dead_code)]\\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\\n        self.tx\\n            .send(PersistenceCommand::StoreMemory(memory))\\n            .map_err(|_| \"persistence worker channel is unavailable\".to_owned())\\n    }\\n\\n''' + "'''" + ''',\n    "",\n    expected=2,\n)'''
if text.count(queue_block) != 1:
    raise RuntimeError("queue_memory precondition block changed")
text = text.replace(queue_block, queue_block.replace("expected=2", "expected=1"))

# README validation wording advanced after the base patch was authored; the current-state finalizer
# owns that paragraph now, so remove only the obsolete literal from the temporary base script.
start_marker = '''literal(\n    "README.md",\n    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate'''
next_marker = '''literal(\n    "README.md",\n    "canonical Lenvu production master'''
start = text.find(start_marker)
end = text.find(next_marker, start + 1)
if start < 0 or end < 0:
    raise RuntimeError("could not isolate obsolete README bundle literal")
text = text[:start] + text[end:]

needle = 'print("Pre-AI finish patch applied.")\n'
chained = needle + '''\nimport runpy
runpy.run_path(str(ROOT / "scripts/finish-pre-ai-finalize.py"), run_name="__main__")
runpy.run_path(str(ROOT / "scripts/finish-pre-ai-hardening.py"), run_name="__main__")
runpy.run_path(str(ROOT / "scripts/finish-pre-ai-gate-fix.py"), run_name="__main__")
'''
if text.count(needle) != 1:
    raise RuntimeError("could not chain stable pre-AI finalizers")
text = text.replace(needle, chained)
base.write_text(text, encoding="utf-8", newline="\n")

# The workflow still invokes this historical path after the base patch. Make it an explicit no-op;
# all semantic hardening now runs once from the stable scripts above.
(root / "scripts/finish-pre-ai-followup.py").write_text(
    'print("Legacy follow-up disabled; stable hardening already applied by base patch.")\n',
    encoding="utf-8",
    newline="\n",
)

print("Prepared simplified base + hardening + final gate-fix patch chain.")
