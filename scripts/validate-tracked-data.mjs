import { spawnSync } from 'node:child_process';

const git = spawnSync('git', ['ls-files', '-z'], {
  encoding: 'utf8',
  windowsHide: true,
});

if (git.error) {
  console.error(`Tracked-data guard could not start git: ${git.error.message}`);
  process.exit(1);
}

if (git.status !== 0) {
  const detail = git.stderr?.trim() || `git exited with status ${git.status}`;
  console.error(`Tracked-data guard could not enumerate repository files: ${detail}`);
  process.exit(1);
}

const trackedFiles = git.stdout.split('\0').filter(Boolean).map((file) => file.replaceAll('\\', '/'));

const rules = [
  {
    reason: 'local runtime output must not be tracked',
    matches: (file) => /^runtime\//i.test(file),
  },
  {
    reason: 'local logs must not be tracked',
    matches: (file) => /^logs\//i.test(file) || /\.log$/i.test(file),
  },
  {
    reason: 'local SQLite/database state must not be tracked',
    matches: (file) => /\.(?:sqlite3?|db)(?:-(?:wal|shm))?$/i.test(file),
  },
  {
    reason: 'environment files may contain secrets',
    matches: (file) => {
      if (/(^|\/)\.env\.example$/i.test(file)) return false;
      return /(^|\/)\.env(?:\.|$)/i.test(file);
    },
  },
  {
    reason: 'local model weights must not be tracked',
    matches: (file) => /^models\/.*\.(?:gguf|bin|safetensors)$/i.test(file),
  },
  {
    reason: 'privacy rules are user-local state',
    matches: (file) => /(^|\/)privacy-rules\.json$/i.test(file),
  },
  {
    reason: 'private signing/key material must not be tracked',
    matches: (file) => /\.(?:key|p12|pfx)$/i.test(file),
  },
];

const violations = [];
for (const file of trackedFiles) {
  for (const rule of rules) {
    if (rule.matches(file)) {
      violations.push({ file, reason: rule.reason });
    }
  }
}

if (violations.length > 0) {
  console.error('Tracked-data guard rejected repository content:');
  for (const violation of violations) {
    console.error(`- ${violation.file}: ${violation.reason}`);
  }
  process.exit(1);
}

console.log(`Tracked-data guard passed (${trackedFiles.length} tracked files checked).`);
