import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { resolve } from 'node:path';

const root = process.cwd();

const paths = {
  runtime: 'src/lib/pet/lenvu.manifest.json',
  master: 'assets/runtime/lenvu/source-notes/canonical-master.json',
  landmarks: 'assets/runtime/lenvu/source-notes/master-landmarks.json',
  references: 'assets/reference/manifest.json',
};

function fail(message) {
  throw new Error(`[Lenvu asset contract] ${message}`);
}

function readJson(relativePath) {
  const absolutePath = resolve(root, relativePath);
  if (!existsSync(absolutePath)) fail(`missing ${relativePath}`);
  try {
    return JSON.parse(readFileSync(absolutePath, 'utf8'));
  } catch (error) {
    fail(`invalid JSON in ${relativePath}: ${error instanceof Error ? error.message : String(error)}`);
  }
}

function equal(actual, expected, label) {
  if (actual !== expected) fail(`${label}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
}

function approx(actual, expected, label, epsilon = 1e-9) {
  if (typeof actual !== 'number' || typeof expected !== 'number' || Math.abs(actual - expected) > epsilon) {
    fail(`${label}: expected ${expected}, got ${actual}`);
  }
}

const runtime = readJson(paths.runtime);
const master = readJson(paths.master);
const landmarks = readJson(paths.landmarks);
const references = readJson(paths.references);

equal(runtime.character.id, 'lenvu', 'runtime character id');
equal(master.characterId, runtime.character.id, 'master character id');
equal(landmarks.characterId, runtime.character.id, 'landmark character id');

equal(master.canvas.runtimeCellWidth, runtime.character.referenceCanvas.width, 'runtime cell width');
equal(master.canvas.runtimeCellHeight, runtime.character.referenceCanvas.height, 'runtime cell height');
approx(master.anchors.root.x, runtime.character.anchor.x, 'root anchor x');
approx(master.anchors.root.y, runtime.character.anchor.y, 'root anchor y');
approx(master.anchors.groundY, runtime.character.anchor.y, 'ground/root y');

equal(master.render.nominalDesktopWidthLogicalPx, runtime.render.nominalWidth, 'nominal render width');
equal(master.render.nominalDesktopHeightLogicalPx, runtime.render.nominalHeight, 'nominal render height');

for (const field of ['rightEye', 'leftEye', 'goldCrescentHorn', 'blindHorizontalMirrorAllowed']) {
  equal(master.identity[field], runtime.identity[field], `identity ${field}`);
  equal(landmarks.identityConstraints[field], runtime.identity[field], `landmark identity ${field}`);
}

approx(landmarks.coordinateSpace.groundY, master.anchors.groundY, 'landmark ground y');
approx(landmarks.frontNeutral.root.x, master.anchors.root.x, 'front landmark root x');
approx(landmarks.frontNeutral.root.y, master.anchors.root.y, 'front landmark root y');
approx(landmarks.leftProfileNeutral.root.x, master.anchors.root.x, 'profile landmark root x');
approx(landmarks.leftProfileNeutral.root.y, master.anchors.root.y, 'profile landmark root y');
approx(landmarks.backNeutral.root.x, master.anchors.root.x, 'back landmark root x');
approx(landmarks.backNeutral.root.y, master.anchors.root.y, 'back landmark root y');

equal(references.scope, 'reference-only', 'reference manifest scope');
equal(references.runtimeAsset, false, 'reference manifest runtimeAsset');
equal(master.authority.referenceManifest, paths.references, 'master reference-manifest path');

const primaryReference = master.authority.primaryReference;
const referenceRecord = references.assets.find((asset) => asset.path === primaryReference);
if (!referenceRecord) fail(`primary reference ${primaryReference} is missing from reference manifest`);

const primaryAbsolute = resolve(root, primaryReference);
if (!existsSync(primaryAbsolute)) fail(`primary reference file is missing: ${primaryReference}`);
equal(statSync(primaryAbsolute).size, referenceRecord.bytes, 'primary reference byte length');

const digest = createHash('sha256').update(readFileSync(primaryAbsolute)).digest('hex');
equal(digest, referenceRecord.sha256, 'primary reference SHA-256');

const configuredAssets = Object.entries(runtime.animations)
  .filter(([, profile]) => profile.asset !== null)
  .map(([id]) => id);

if (!master.runtimeAssetReady && configuredAssets.length > 0) {
  fail(`runtimeAssetReady=false but production animation assets are configured: ${configuredAssets.join(', ')}`);
}

console.log(
  `[Lenvu asset contract] OK — ${Object.keys(runtime.animations).length} animation profiles, `
  + `${references.assets.length} reference asset(s), productionReady=${master.runtimeAssetReady}`,
);
