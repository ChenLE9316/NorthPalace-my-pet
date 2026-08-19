from pathlib import Path

path = Path(__file__).resolve().with_name("finish-pre-ai.py")
text = path.read_text(encoding="utf-8")
old = '''literal(\n    "src-tauri/src/persistence.rs",\n    ''' + "'''" + '''    #[allow(dead_code)]\\n    pub fn queue_memory(&self, memory: MemoryDraft) -> Result<(), String> {\\n        self.tx\\n            .send(PersistenceCommand::StoreMemory(memory))\\n            .map_err(|_| \"persistence worker channel is unavailable\".to_owned())\\n    }\\n\\n''' + "'''" + ''',\n    "",\n    expected=2,\n)'''
new = old.replace("expected=2", "expected=1")
count = text.count(old)
if count != 1:
    raise RuntimeError(f"expected one queue_memory precondition block, found {count}")
path.write_text(text.replace(old, new), encoding="utf-8", newline="\n")
print("Corrected queue_memory patch precondition for this clean-runner worktree.")
