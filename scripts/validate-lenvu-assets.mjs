import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { resolve } from 'node:path';

const root = process.cwd();

const paths = {
  runtime: 'src/lib/pet/lenvu.manifest.json',
  visualGroundTruth: 'docs/LENVU_VISUAL_GROUND_TRUTH.md',
  master: 'assets/runtime/lenvu/source-notes/canonical-master.json',
  candidate: 'assets/runtime/lenvu/source-notes/master-candidate.json',
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

function verifyArtifact(artifact, label) {
  if (!artifact || typeof artifact.path !== 'string' || artifact.path.length === 0) {
    fail(`${label} artifact path is missing`);
  }
  const absolutePath = resolve(root, artifact.path);
  if (!existsSync(absolutePath)) fail(`${label} artifact file is missing: ${artifact.path}`);
  if (!Number.isInteger(artifact.bytes) || artifact.bytes <= 0) fail(`${label} artifact byte length is invalid`);
  if (typeof artifact.sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(artifact.sha256)) {
    fail(`${label} artifact SHA-256 is invalid`);
  }
  equal(statSync(absolutePath).size, artifact.bytes, `${label} artifact byte length`);
  const digest = createHash('sha256').update(readFileSync(absolutePath)).digest('hex');
  equal(digest, artifact.sha256, `${label} artifact SHA-256`);
}

if (!existsSync(resolve(root, paths.visualGroundTruth))) {
  fail(`missing ${paths.visualGroundTruth}`);
}

const runtime = readJson(paths.runtime);
const master = readJson(paths.master);
const candidate = readJson(paths.candidate);
const landmarks = readJson(paths.landmarks);
const references = readJson(paths.references);

equal(runtime.character.id, 'lenvu', 'runtime character id');
equal(master.characterId, runtime.character.id, 'master character id');
equal(candidate.characterId, runtime.character.id, 'candidate character id');
equal(landmarks.characterId, runtime.character.id, 'landmark character id');

equal(master.authority.visualGroundTruth, paths.visualGroundTruth, 'master visual-ground-truth path');
equal(master.authority.candidateMetadata, paths.candidate, 'master candidate-metadata path');
equal(candidate.authority.visualGroundTruth, paths.visualGroundTruth, 'candidate visual-ground-truth path');
equal(candidate.authority.masterContract, paths.master, 'candidate master-contract path');
equal(candidate.authority.landmarks, paths.landmarks, 'candidate landmarks path');
equal(candidate.authority.referenceManifest, paths.references, 'candidate reference-manifest path');
equal(candidate.runtimeAsset, false, 'candidate runtimeAsset');

equal(candidate.promotionPolicy.generatedArtworkIsCandidateOnly, true, 'candidate-only promotion policy');
equal(candidate.promotionPolicy.blindMirroringForbidden, true, 'blind-mirroring policy');
equal(candidate.promotionPolicy.candidateMayBeUsedAsRuntimeTexture, false, 'candidate runtime-texture policy');
equal(candidate.promotionPolicy.candidateMayRedefineSourceIdentity, false, 'candidate identity-authority policy');
equal(candidate.promotionPolicy.sourceVisualEvidenceOverridesCandidate, true, 'source-evidence priority policy');

equal(master.identity.speciesSilhouette, 'tall_lean_long_legged_canine_dragon_digital', 'species silhouette');
equal(master.identity.baseFurPalette, 'cool_slate_blue_gray_and_white', 'base fur palette');
equal(master.identity.muzzle, 'elongated_canine', 'muzzle identity');
equal(master.identity.rightEye, runtime.identity.rightEye, 'identity right eye');
equal(master.identity.leftEye, runtime.identity.leftEye, 'identity left eye');
equal(master.identity.goldCrescentHorn, runtime.identity.goldCrescentHorn, 'identity gold horn side');
equal(master.identity.blindHorizontalMirrorAllowed, runtime.identity.blindHorizontalMirrorAllowed, 'blind mirror policy');
equal(master.identity.chibiOrCatLikeRedesignAllowed, false, 'chibi/cat redesign policy');
equal(master.identity.generatedCandidateMayRedefineIdentity, false, 'generated candidate identity policy');

// Landmarks are engineering normalization targets, never character-identity authority.
equal(landmarks.authority.visualGroundTruth, paths.visualGroundTruth, 'landmark visual-ground-truth path');
equal(landmarks.authority.primaryReference, master.authority.primaryReference, 'landmark primary-reference path');
equal(landmarks.authority.referenceManifest, paths.references, 'landmark reference-manifest path');
equal(landmarks.provenance.measurementStatus, 'inferred_not_pixel_traced', 'landmark measurement status');
equal(landmarks.provenance.coordinatesAreIdentityAuthority, false, 'landmark identity-authority policy');
equal(landmarks.revisionPolicy.sourceVisualEvidenceOverridesCoordinates, true, 'source-over-coordinate policy');
equal(landmarks.revisionPolicy.candidateArtworkMayNotRedefineCoordinatesOrIdentity, true, 'candidate landmark-authority policy');
equal(landmarks.revisionPolicy.measureAgainstOriginalHighResolutionSourceBeforeMasterApproval, true, 'source-measurement approval policy');

const landmarkNeedsSourceMeasurement = landmarks.provenance.measurementStatus !== 'measured_from_original_high_resolution_source';
if (candidate.review.approved && landmarkNeedsSourceMeasurement) {
  fail('approved canonical master requires landmarks measured from the original high-resolution source');
}

if (master.runtimeAssetReady && landmarkNeedsSourceMeasurement) {
  fail('runtimeAssetReady=true requires source-measured production landmarks');
}

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

for (const view of master.requiredMasterViews) {
  if (!candidate.requiredViews.includes(view)) fail(`candidate requiredViews is missing ${view}`);
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

if (candidate.review.approved) {
  const requiredChecks = [
    'sourceReferenceMatched',
    'speciesSilhouetteVerified',
    'nonChibiProportionsVerified',
    'elongatedCanineMuzzleVerified',
    'slateGrayWhitePaletteVerified',
    'requiredViewsPresent',
    'rightEyeCyanVerified',
    'leftEyeVioletVerified',
    'leftHornGoldCrescentVerified',
    'foreheadGlyphVerified',
    'darkSegmentedHornPairVerified',
    'cyanEarCircuitryVerified',
    'cyanLumenCodePlacementVerified',
    'physicalTailBaseVerified',
    'cyanHolographicTailTerminalVerified',
    'silhouetteConsistencyVerified',
    'groundAndAnchorsMeasured',
    'desktopPreviewVerified',
  ];
  for (const check of requiredChecks) equal(candidate.review[check], true, `approved candidate review ${check}`);
  verifyArtifact(candidate.artifact, 'approved candidate');
}

const configuredAssets = Object.entries(runtime.animations)
  .filter(([, profile]) => profile.asset !== null)
  .map(([id]) => id);

if (!master.runtimeAssetReady && configuredAssets.length > 0) {
  fail(`runtimeAssetReady=false but production animation assets are configured: ${configuredAssets.join(', ')}`);
}

if (master.runtimeAssetReady && !candidate.review.approved) {
  fail('runtimeAssetReady=true requires an approved canonical master candidate');
}

console.log(
  `[Lenvu asset contract] OK — ${Object.keys(runtime.animations).length} animation profiles, `
  + `${references.assets.length} reference asset(s), landmarks=${landmarks.provenance.measurementStatus}, `
  + `candidate=${candidate.status}, productionReady=${master.runtimeAssetReady}`,
);
