import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const files = execFileSync('git', ['ls-files', '-z'], { encoding: 'utf8' })
  .split('\0')
  .filter(Boolean);
const failures = [];
const fail = (msg) => failures.push(msg);

for (const required of ['README.md', 'AGENTS.md', 'CONTRIBUTING.md', 'LICENSE', 'REQUIREMENTS.md']) {
  if (!files.includes(required)) fail(`missing required repository file: ${required}`);
}

const decoder = new TextDecoder('utf-8', { fatal: true });
for (const f of files.filter((f) => f.endsWith('.md'))) {
  let text;
  try {
    text = decoder.decode(readFileSync(f));
  } catch {
    fail(`${f}: not valid UTF-8`);
    continue;
  }
  if (!text.endsWith('\n')) fail(`${f}: missing final newline`);
  text.split('\n').forEach((line, i) => {
    if (/[ \t]+$/.test(line)) fail(`${f}:${i + 1}: trailing whitespace`);
  });
}

for (const f of files.filter((f) => f.startsWith('evidence/'))) {
  if (!/^evidence\/\d{4}-\d{2}-\d{2}-[a-z0-9][a-z0-9.-]*\.md$/.test(f)) {
    fail(`${f}: evidence filename must be YYYY-MM-DD-<subject>.md`);
  }
}

const secretPatterns = [
  [/gh[pousr]_[A-Za-z0-9]{20,}/, 'GitHub token'],
  [/github_pat_[A-Za-z0-9_]{20,}/, 'GitHub fine-grained PAT'],
  [/\bsk-[A-Za-z0-9_-]{20,}/, 'secret API key'],
  [/\bAKIA[0-9A-Z]{16}\b/, 'AWS access key id'],
  [/-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/, 'private key block'],
];
// Supplemental high-signal check, never a replacement for required Gitleaks.
for (const f of files) {
  if (/(^|\/)\.env|\.(?:pem|key)$|(^|\/)auth\.json$/.test(f)) {
    fail(`${f}: forbidden credential-file class`);
  }
}
const scannable = /\.(md|toml|rs|yml|yaml|json|js|mjs|ts|txt|py|pem|key|env)$|^(LICENSE|\.gitignore|\.gitattributes)$/;
for (const f of files.filter((f) => scannable.test(f))) {
  const text = readFileSync(f, 'utf8');
  for (const [pattern, label] of secretPatterns) {
    if (pattern.test(text)) fail(`${f}: possible ${label} (pattern match; inspect and redact)`);
  }
}

if (files.includes('REQUIREMENTS.md')) {
  const req = readFileSync('REQUIREMENTS.md', 'utf8');
  const decisionRows = (text) => [...text.matchAll(/^\|\s*D(\d+)\s*\|[^\r\n]*/gm)];
  const rows = decisionRows(req);
  const ids = rows.map((m) => Number(m[1]));
  if (ids.length === 0) fail('REQUIREMENTS.md: no decision rows found');
  const dupes = [...new Set(ids.filter((id, i) => ids.indexOf(id) !== i))];
  if (dupes.length) fail(`REQUIREMENTS.md: duplicate decision numbers: D${dupes.join(', D')}`);
  if (ids.some((id, i) => id !== i + 1)) fail('REQUIREMENTS.md: decisions must be ordered and contiguous from D1');
  if (!/\*\*Revision:\*\*/.test(req)) fail('REQUIREMENTS.md: missing revision header');

  // CI supplies the observed PR base/pre-push SHA. Local checks compare HEAD.
  // This detects changes relative to that baseline, not all-history authority.
  try {
    const base = process.env.GOVERNANCE_BASE_SHA || execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
    if (!/^(?:[a-f0-9]{40}|[a-f0-9]{64})$/.test(base)) throw new Error('invalid base SHA');
    const previous = execFileSync('git', ['show', `${base}:REQUIREMENTS.md`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
    const previousRows = decisionRows(previous);
    if (!previousRows.length) throw new Error('baseline contains no decisions');
    const current = new Map(rows.map((m) => [m[1], m[0]]));
    for (const row of previousRows) {
      if (!current.has(row[1])) fail(`REQUIREMENTS.md: deleted prior decision D${row[1]}`);
      else if (current.get(row[1]) !== row[0]) fail(`REQUIREMENTS.md: modified prior decision D${row[1]}; append a superseding decision instead`);
    }
  } catch {
    fail('REQUIREMENTS.md: cannot verify prior decision rows against the supplied base (fetch full history and provide a valid commit SHA)');
  }
}

const checks = 5;
if (failures.length) {
  console.error(`governance: FAIL (${failures.length} violation(s))`);
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}
console.log(`governance: PASS (${files.length} tracked files; hygiene, markdown, evidence naming, secret scan, decision-row integrity (unique, ordered, prior rows preserved) — ${checks} check groups)`);