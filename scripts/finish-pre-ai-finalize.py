from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def patch(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"{path}: expected one match, found {count}: {old!r}")
    target.write_text(text.replace(old, new), encoding="utf-8", newline="\n")

patch(
    "README.md",
    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate, `docs/VALIDATION_BASELINE.md` records the clean frontend/Rust gate, and `docs/WINDOWS_BUNDLE_BASELINE.md` records a clean GitHub-hosted Windows Tauri release + NSIS build, artifact discovery and an 8-second release-executable smoke launch. The manual Windows Bundle workflow consumes the same committed dependency graphs rather than generating independent candidates.\n",
    "`docs/SVELTE_DIAGNOSTIC_BASELINE.md` records the clean dual-Svelte gate, `docs/VALIDATION_BASELINE.md` records the clean frontend/Rust gate, and `docs/WINDOWS_BUNDLE_BASELINE.md` records a clean GitHub-hosted Windows Tauri release + NSIS build, artifact discovery and an 8-second release-executable smoke launch. The permanent Windows Bundle workflow consumes the same committed dependency graphs and now requires both production WebViews to execute and cross Tauri IPC under the production CSP before bundle artifacts are accepted.\n",
)

patch(
    "README.md",
    "The activity journal is intentionally bounded and ignores high-frequency noise. Long-term memory currently supports episodic, semantic, preference and relationship records with FTS5/BM25 retrieval plus importance/recency metadata. A future Memory Evaluator will decide whether automatic candidates should be stored, merged or discarded.\n",
    "The activity journal is intentionally bounded and ignores high-frequency noise. Long-term memory supports episodic, semantic, preference and relationship records with FTS5/BM25 retrieval plus importance/recency metadata. A deterministic non-LLM Memory Evaluator turns repeated interaction evidence into store/merge/discard decisions while a durable `manual` / `automatic` origin preserves user-authored memory authority independently from journal retention.\n",
)

patch(
    "README.md",
    "Memory Evaluator / relationship evolution\n  ↓\nMiniCPM5-1B isolated text worker",
    "relationship evolution / target-machine tuning\n  ↓\nMiniCPM5-1B isolated text worker",
)

print("Current-state README semantics finalized.")
