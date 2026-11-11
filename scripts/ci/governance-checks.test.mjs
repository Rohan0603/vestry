import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const checker = fileURLToPath(new URL('./governance-checks.mjs', import.meta.url));
const baseline = '**Revision:** 1.0\n\n| D1 | Original decision |\n| D2 | Second decision |\n';

function fixture(t) {
  const cwd = mkdtempSync(join(tmpdir(), 'vestry-governance-'));
  t.after(() => rmSync(cwd, { recursive: true, force: true }));
  const env = {
    ...process.env,
    GIT_AUTHOR_NAME: 'Governance Fixture',
    GIT_AUTHOR_EMAIL: 'fixture@example.invalid',
    GIT_COMMITTER_NAME: 'Governance Fixture',
    GIT_COMMITTER_EMAIL: 'fixture@example.invalid',
  };
  const git = (...args) => execFileSync('git', args, { cwd, env, encoding: 'utf8', stdio: ['pipe', 'pipe', 'pipe'] }).trim();
  git('init', '--quiet');
  for (const name of ['README.md', 'AGENTS.md', 'CONTRIBUTING.md', 'LICENSE']) writeFileSync(join(cwd, name), 'Fixture\n');
  writeFileSync(join(cwd, 'REQUIREMENTS.md'), baseline);
  git('add', 'README.md', 'AGENTS.md', 'CONTRIBUTING.md', 'LICENSE', 'REQUIREMENTS.md');
  // Create only fixture objects; never change user identity, hooks or config.
  const tree = git('write-tree');
  const sha = git('commit-tree', tree, '-m', 'governance fixture');
  git('update-ref', 'HEAD', sha);
  return {
    write(name, content) { writeFileSync(join(cwd, name), content); git('add', '--', name); },
    run(extraEnv = {}) {
      return spawnSync(process.execPath, [checker], { cwd, env: { ...env, GOVERNANCE_BASE_SHA: sha, ...extraEnv }, encoding: 'utf8' });
    },
  };
}

function rejected(result, message) {
  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.match(result.stderr, message);
}

test('unchanged baseline passes against explicit SHA and local HEAD', (t) => {
  const f = fixture(t);
  assert.equal(f.run().status, 0);
  assert.equal(f.run({ GOVERNANCE_BASE_SHA: '' }).status, 0);
});

test('additive ordered decision passes without changing prior rows', (t) => {
  const f = fixture(t);
  f.write('REQUIREMENTS.md', baseline + '| D3 | Supersedes D1 explicitly |\n');
  assert.equal(f.run().status, 0);
});

test('edited prior decision fails even when numbers stay unique', (t) => {
  const f = fixture(t);
  f.write('REQUIREMENTS.md', baseline.replace('Original decision', 'Different decision'));
  rejected(f.run(), /modified prior decision D1/);
});

test('deleted final decision fails even when remaining IDs are contiguous', (t) => {
  const f = fixture(t);
  f.write('REQUIREMENTS.md', baseline.replace('| D2 | Second decision |\n', ''));
  rejected(f.run(), /deleted prior decision D2/);
});

test('duplicate decision fails', (t) => {
  const f = fixture(t);
  f.write('REQUIREMENTS.md', baseline + '| D2 | Duplicate |\n');
  rejected(f.run(), /duplicate decision numbers: D2/);
});

test('out-of-order or gapped additions fail', (t) => {
  const f = fixture(t);
  f.write('REQUIREMENTS.md', baseline + '| D4 | Missing D3 |\n');
  rejected(f.run(), /ordered and contiguous/);
});

test('unavailable and malformed baselines fail closed', (t) => {
  const f = fixture(t);
  for (const value of ['1'.repeat(40), '--all']) rejected(f.run({ GOVERNANCE_BASE_SHA: value }), /cannot verify prior decision rows/);
});

test('markdown final newline and trailing whitespace remain checked', (t) => {
  const f = fixture(t);
  f.write('README.md', 'Trailing space \nNo newline');
  const result = f.run();
  rejected(result, /missing final newline/);
  assert.match(result.stderr, /trailing whitespace/);
});

test('invalid UTF-8 fails without printing file content', (t) => {
  const f = fixture(t);
  f.write('README.md', Buffer.from([0xff, 0x0a]));
  rejected(f.run(), /not valid UTF-8/);
});

test('credential filename classes fail even with placeholder content', (t) => {
  const f = fixture(t);
  for (const name of ['.env', '.env.example', '.envrc', 'private.pem', 'private.key', 'auth.json']) f.write(name, 'placeholder\n');
  const result = f.run();
  rejected(result, /forbidden credential-file class/);
  for (const name of ['.env', '.env.example', '.envrc', 'private.pem', 'private.key', 'auth.json']) assert.ok(result.stderr.includes(name));
});

test('supplemental scanner detects Python patterns without disclosing match', (t) => {
  const f = fixture(t);
  const synthetic = ['gh', 'p_', 'A'.repeat(25)].join('');
  f.write('example.py', `value = '${synthetic}'\n`);
  const result = f.run();
  rejected(result, /possible GitHub token/);
  assert.ok(!result.stderr.includes(synthetic));
});

// Line endings differ between Windows checkouts and CI; assert logical content.
const readWorkflow = () => readFileSync(new URL('../../.github/workflows/governance.yml', import.meta.url), 'utf8').replace(/\r\n/g, '\n');

test('workflow preserves required policy name and does not skip missing live config', () => {
  const workflow = readWorkflow();
  assert.match(workflow, /name: Policy validator tests/);
  assert.match(workflow, /node --test scripts\/ci\/governance-checks.test.mjs/);
  assert.match(workflow, /fetch-depth: 0/);
  assert.ok(!workflow.includes("&& vars.GH_PROJECT_NODE_ID != ''"));
  assert.match(workflow, /Requested Project audit did not succeed/);
  assert.match(workflow, /live Project audit NOT RUN/);
});

test('aggregate job is exactly named governance and always runs', () => {
  const workflow = readWorkflow();
  // The aggregate job keeps the exact name "governance" for continuity with
  // the historical required-check contract. Current branch protection requires
  // "Policy validator tests" and "Gitleaks secret scan" (verified 2026-11-11).
  assert.match(workflow, /^  governance:\n    name: governance\n    if: \$\{\{ always\(\) \}\}\n    needs: \[policy-tests, live-audit\]$/m);
});

test('remote audit runs only on schedule or manual dispatch, never on PR code', () => {
  const workflow = readWorkflow();
  assert.match(workflow, /if: \$\{\{ github\.event_name == 'schedule' \|\| github\.event_name == 'workflow_dispatch' \}\}/);
  assert.match(workflow, /GH_TOKEN: \$\{\{ secrets\.GH_PROJECTS_READ_TOKEN \}\}/);
});

test('aggregate result matrix fails closed per event type', () => {
  const workflow = readWorkflow();
  // Offline policy result is required on every event, with no exception branch.
  assert.match(workflow, /if \[ "\$POLICY_RESULT" != "success" \]; then\n            echo "::error::Governance validator did not succeed"\n            exit 1\n          fi/);
  // A requested remote audit must succeed on schedule/dispatch; skipped or
  // failed audits cannot pass the aggregate job on those events.
  assert.match(workflow, /if \[ "\$EVENT_NAME" = "schedule" \] \|\| \[ "\$EVENT_NAME" = "workflow_dispatch" \]; then\n            if \[ "\$AUDIT_RESULT" != "success" \]; then\n              echo "::error::Requested Project audit did not succeed"\n              exit 1\n            fi/);
  // On pull_request/push the audit is explicitly reported NOT RUN, never implied passed.
  assert.match(workflow, /else\n            echo "::notice::Offline checks passed; live Project audit NOT RUN on this event"\n          fi/);
});
