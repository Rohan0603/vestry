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
const scannable = /\.(md|toml|rs|yml|yaml|json|js|mjs|ts|txt)$|^(LICENSE|\.gitignore|\.gitattributes)$/;
for (const f of files.filter((f) => scannable.test(f))) {
  const text = readFileSync(f, 'utf8');
  for (const [pattern, label] of secretPatterns) {
    if (pattern.test(text)) fail(`${f}: possible ${label} (pattern match; inspect and redact)`);
  }
}

if (files.includes('REQUIREMENTS.md')) {
  const req = readFileSync('REQUIREMENTS.md', 'utf8');
  const ids = [...req.matchAll(/^\| D(\d+) \|/gm)].map((m) => Number(m[1]));
  if (ids.length === 0) fail('REQUIREMENTS.md: no decision rows found');
  const dupes = [...new Set(ids.filter((id, i) => ids.indexOf(id) !== i))];
  if (dupes.length) fail(`REQUIREMENTS.md: duplicate decision numbers: D${dupes.join(', D')}`);
  if (!/\*\*Revision:\*\*/.test(req)) fail('REQUIREMENTS.md: missing revision header');
}

const checks = 5;
if (failures.length) {
  console.error(`governance: FAIL (${failures.length} violation(s))`);
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}
console.log(`governance: PASS (${files.length} tracked files; hygiene, markdown, evidence naming, secret scan, decision-row uniqueness — ${checks} check groups)`);