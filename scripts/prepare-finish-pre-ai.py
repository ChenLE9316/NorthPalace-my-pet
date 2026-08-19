from pathlib import Path

path = Path(__file__).resolve().with_name("finish-pre-ai.py")
text = path.read_text(encoding="utf-8")

queue_block = '''literal(\n    "src-tauri/src/persistence.rs",\n    ''' + "'''" + '''    #[allow(dead_code)]\\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\\n        self.tx\\n            .send(PersistenceCommand::StoreMemory(memory))\\n            .map_err(|_| \"persistence worker channel is unavailable\".to_owned())\\n    }\\n\\n''' + "'''" + ''',\n    "",\n    expected=2,\n)'''
count = text.count(queue_block)
if count != 1:
    raise RuntimeError(f"expected one queue_memory precondition block, found {count}")
text = text.replace(queue_block, queue_block.replace("expected=2", "expected=1"))

# README's clean-bundle wording was already advanced after the original finish script was staged.
# Remove only that obsolete literal; the semantic hardening script patches the current wording.
start_marker = '''literal(\n    "README.md",\n    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate'''
next_marker = '''literal(\n    "README.md",\n    "canonical Lenvu production master'''
start = text.find(start_marker)
end = text.find(next_marker, start + 1)
if start < 0 or end < 0:
    raise RuntimeError("could not isolate obsolete README bundle literal")
text = text[:start] + text[end:]

path.write_text(text, encoding="utf-8", newline="\n")
print("Corrected finish-patch preconditions for the current repository state.")
