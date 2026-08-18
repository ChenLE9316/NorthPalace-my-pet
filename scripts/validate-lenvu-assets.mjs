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
  sourceMeasurement: 'assets/runtime/lenvu/source-notes/source-measurement.json',
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

function approx(actual, expected, label, epsilon = 1e-6) {
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

if (!existsSync(resolve(root, paths.visualGroundTruth))) fail(`missing ${paths.visualGroundTruth}`);

const runtime = readJson(paths.runtime);
const master = readJson(paths.master);
const candidate = readJson(paths.candidate);
const landmarks = readJson(paths.landmarks);
const sourceMeasurement = readJson(paths.sourceMeasurement);
const references = readJson(paths.references);

equal(runtime.character.id, 'lenvu', 'runtime character id');
equal(master.characterId, runtime.character.id, 'master character id');
equal(candidate.characterId, runtime.character.id, 'candidate character id');
equal(landmarks.characterId, runtime.character.id, 'landmark character id');
equal(sourceMeasurement.characterId, runtime.character.id, 'source-measurement character id');

equal(master.authority.visualGroundTruth, paths.visualGroundTruth, 'master visual-ground-truth path');
equal(master.authority.sourceMeasurement, paths.sourceMeasurement, 'master source-measurement path');
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

// Bind source measurements to original reference provenance.
equal(sourceMeasurement.authority.visualGroundTruth, paths.visualGroundTruth, 'source-measurement ground-truth path');
equal(sourceMeasurement.authority.referenceManifest, paths.references, 'source-measurement reference-manifest path');
equal(sourceMeasurement.measurementMethod.sourceWasOriginalHighResolution, true, 'source-measurement original-source policy');
equal(sourceMeasurement.measurementMethod.sourceChecksumMatchedReferenceManifest, true, 'source-measurement checksum policy');
equal(sourceMeasurement.measurementPolicy.assistantMemoryMayFillMissingValues, false, 'source-measurement memory policy');
equal(sourceMeasurement.measurementPolicy.generatedCandidateMayFillMissingValues, false, 'source-measurement generated-candidate policy');
equal(sourceMeasurement.completion.allRequiredPixelMeasurementsPresent, true, 'source pixel-measurement completion');
equal(sourceMeasurement.completion.identityDetailsVerifiedFromSource, true, 'source identity verification');
equal(sourceMeasurement.completion.normalizedLandmarksUpdatedFromMeasurements, true, 'normalized landmark completion');
equal(sourceMeasurement.completion.completed, true, 'source-measurement completion');
for (const [check, value] of Object.entries(sourceMeasurement.measurements.identityDetails)) {
  equal(value, true, `source identity detail ${check}`);
}

// Deterministically remap source pixels to canonical coordinates.
const mapping = sourceMeasurement.canonicalMapping;
equal(mapping.preserveAspectRatio, true, 'canonical mapping aspect-ratio policy');
approx(mapping.targetTopY, master.canvas.safeVisualBounds.top, 'mapping target top');
approx(mapping.targetGroundY, master.anchors.groundY, 'mapping target ground');
approx(mapping.targetRootX, master.anchors.root.x, 'mapping target root x');

function transform(viewName, point) {
  const view = mapping.views[viewName];
  if (!view) fail(`missing canonical mapping view ${viewName}`);
  const scale = (mapping.targetGroundY - mapping.targetTopY) / (view.sourceGroundYPx - view.sourceTopYPx);
  approx(view.scaleNormalizedPerSourcePx, scale, `${viewName} mapping scale`, 1e-12);
  return {
    x: mapping.targetRootX + (point.x - view.sourceRootXPx) * scale,
    y: mapping.targetTopY + (point.y - view.sourceTopYPx) * scale,
  };
}

function verifyMapped(viewName, sourcePoint, landmarkPoint, label) {
  const mapped = transform(viewName, sourcePoint);
  approx(landmarkPoint.x, mapped.x, `${label}.x`);
  approx(landmarkPoint.y, mapped.y, `${label}.y`);
}

const smFront = sourceMeasurement.measurements.frontNeutral;
verifyMapped('frontNeutral', smFront.earTipRightPx, landmarks.frontNeutral.earTipRight, 'front earTipRight');
verifyMapped('frontNeutral', smFront.earTipLeftPx, landmarks.frontNeutral.earTipLeft, 'front earTipLeft');
verifyMapped('frontNeutral', smFront.hornTipRightPx, landmarks.frontNeutral.hornTipRight, 'front hornTipRight');
verifyMapped('frontNeutral', smFront.hornTipLeftPx, landmarks.frontNeutral.hornTipLeft, 'front hornTipLeft');
verifyMapped('frontNeutral', smFront.rightEyeCenterPx, landmarks.frontNeutral.rightEye, 'front rightEye');
verifyMapped('frontNeutral', smFront.leftEyeCenterPx, landmarks.frontNeutral.leftEye, 'front leftEye');
verifyMapped('frontNeutral', smFront.noseCenterPx, landmarks.frontNeutral.nose, 'front nose');
verifyMapped('frontNeutral', smFront.chinPx, landmarks.frontNeutral.chin, 'front chin');
verifyMapped('frontNeutral', smFront.shoulderRightPx, landmarks.frontNeutral.shoulderRight, 'front shoulderRight');
verifyMapped('frontNeutral', smFront.shoulderLeftPx, landmarks.frontNeutral.shoulderLeft, 'front shoulderLeft');
verifyMapped('frontNeutral', smFront.frontPawRightContactPx, landmarks.frontNeutral.frontPawRight, 'front pawRight');
verifyMapped('frontNeutral', smFront.frontPawLeftContactPx, landmarks.frontNeutral.frontPawLeft, 'front pawLeft');

const smProfile = sourceMeasurement.measurements.leftProfileNeutral;
verifyMapped('leftProfileNeutral', smProfile.noseTipPx, landmarks.leftProfileNeutral.nose, 'profile nose');
verifyMapped('leftProfileNeutral', smProfile.visibleEyeCenterPx, landmarks.leftProfileNeutral.visibleEye, 'profile visibleEye');
verifyMapped('leftProfileNeutral', smProfile.headCenterPx, landmarks.leftProfileNeutral.head, 'profile head');
verifyMapped('leftProfileNeutral', smProfile.shoulderPx, landmarks.leftProfileNeutral.shoulder, 'profile shoulder');
verifyMapped('leftProfileNeutral', smProfile.frontPawContactPx, landmarks.leftProfileNeutral.frontPaw, 'profile frontPaw');
verifyMapped('leftProfileNeutral', smProfile.hipPx, landmarks.leftProfileNeutral.hip, 'profile hip');
verifyMapped('leftProfileNeutral', smProfile.rearPawContactPx, landmarks.leftProfileNeutral.rearPaw, 'profile rearPaw');
verifyMapped('leftProfileNeutral', smProfile.tailBasePx, landmarks.leftProfileNeutral.tailBase, 'profile tailBase');
verifyMapped('leftProfileNeutral', smProfile.tailEffectPeakPx, landmarks.leftProfileNeutral.tailEffectPeak, 'profile tailEffectPeak');

const smBack = sourceMeasurement.measurements.backNeutral;
verifyMapped('backNeutral', smBack.spineCenterTopPx, landmarks.backNeutral.spineCenterTop, 'back spineTop');
verifyMapped('backNeutral', smBack.spineCenterMidPx, landmarks.backNeutral.spineCenterMid, 'back spineMid');
verifyMapped('backNeutral', smBack.tailBasePx, landmarks.backNeutral.tailBase, 'back tailBase');
verifyMapped('backNeutral', smBack.rearPawRightContactPx, landmarks.backNeutral.rearPawRight, 'back pawRight');
verifyMapped('backNeutral', smBack.rearPawLeftContactPx, landmarks.backNeutral.rearPawLeft, 'back pawLeft');

// The measured long side profile must fit without horizontal compression.
const profileBounds = smProfile.visibleBoundsPx;
const profileLeft = transform('leftProfileNeutral', { x: profileBounds.x, y: smProfile.groundContactYPx }).x;
const profileRight = transform('leftProfileNeutral', { x: profileBounds.x + profileBounds.width, y: smProfile.groundContactYPx }).x;
if (master.canvas.safeVisualBounds.left > profileLeft + 0.002) fail('safe visual left bound clips measured profile silhouette');
if (master.canvas.safeVisualBounds.right < profileRight - 0.002) fail('safe visual right bound clips measured profile silhouette');

// Landmarks are engineering normalization targets, never character-identity authority.
equal(landmarks.authority.visualGroundTruth, paths.visualGroundTruth, 'landmark visual-ground-truth path');
equal(landmarks.authority.primaryReference, master.authority.primaryReference, 'landmark primary-reference path');
equal(landmarks.authority.referenceManifest, paths.references, 'landmark reference-manifest path');
equal(landmarks.authority.sourceMeasurement, paths.sourceMeasurement, 'landmark source-measurement path');
equal(landmarks.provenance.measurementStatus, 'measured_from_original_high_resolution_source', 'landmark measurement status');
equal(landmarks.provenance.coordinatesAreIdentityAuthority, false, 'landmark identity-authority policy');
equal(landmarks.revisionPolicy.sourceVisualEvidenceOverridesCoordinates, true, 'source-over-coordinate policy');
equal(landmarks.revisionPolicy.candidateArtworkMayNotRedefineCoordinatesOrIdentity, true, 'candidate landmark-authority policy');
equal(landmarks.revisionPolicy.measureAgainstOriginalHighResolutionSourceBeforeMasterApproval, true, 'source-measurement approval policy');

const normalizedRemapComplete = sourceMeasurement.completion.normalizedLandmarksUpdatedFromMeasurements === true
  && landmarks.provenance.measurementStatus === 'measured_from_original_high_resolution_source';
if (candidate.review.approved && !normalizedRemapComplete) fail('approved canonical master requires source-measured landmarks');
if (master.runtimeAssetReady && !normalizedRemapComplete) fail('runtimeAssetReady=true requires source-measured landmarks');

equal(master.canvas.runtimeCellWidth, runtime.character.referenceCanvas.width, 'runtime cell width');
equal(master.canvas.runtimeCellHeight, runtime.character.referenceCanvas.height, 'runtime cell height');
approx(master.anchors.root.x, runtime.character.anchor.x, 'root anchor x');
approx(master.anchors.root.y, runtime.character.anchor.y, 'root anchor y');
approx(master.anchors.groundY, runtime.character.anchor.y, 'ground/root y');
approx(master.anchors.tailBase.x, landmarks.leftProfileNeutral.tailBase.x, 'master tail-base x');
approx(master.anchors.tailBase.y, landmarks.leftProfileNeutral.tailBase.y, 'master tail-base y');
approx(master.anchors.frontPawLeft.x, landmarks.frontNeutral.frontPawLeft.x, 'master front-paw-left x');
approx(master.anchors.frontPawLeft.y, landmarks.frontNeutral.frontPawLeft.y, 'master front-paw-left y');
approx(master.anchors.frontPawRight.x, landmarks.frontNeutral.frontPawRight.x, 'master front-paw-right x');
approx(master.anchors.frontPawRight.y, landmarks.frontNeutral.frontPawRight.y, 'master front-paw-right y');

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
equal(sourceMeasurement.authority.sourceSha256, referenceRecord.sourceSha256, 'source-measurement source SHA-256');
equal(sourceMeasurement.authority.sourceWidth, referenceRecord.sourceWidth, 'source-measurement source width');
equal(sourceMeasurement.authority.sourceHeight, referenceRecord.sourceHeight, 'source-measurement source height');
const primaryAbsolute = resolve(root, primaryReference);
if (!existsSync(primaryAbsolute)) fail(`primary reference file is missing: ${primaryReference}`);
equal(statSync(primaryAbsolute).size, referenceRecord.bytes, 'primary reference byte length');
const digest = createHash('sha256').update(readFileSync(primaryAbsolute)).digest('hex');
equal(digest, referenceRecord.sha256, 'primary reference SHA-256');

if (candidate.review.approved) {
  const requiredChecks = [
    'sourceReferenceMatched', 'speciesSilhouetteVerified', 'nonChibiProportionsVerified',
    'elongatedCanineMuzzleVerified', 'slateGrayWhitePaletteVerified', 'requiredViewsPresent',
    'rightEyeCyanVerified', 'leftEyeVioletVerified', 'leftHornGoldCrescentVerified',
    'foreheadGlyphVerified', 'darkSegmentedHornPairVerified', 'cyanEarCircuitryVerified',
    'cyanLumenCodePlacementVerified', 'physicalTailBaseVerified', 'cyanHolographicTailTerminalVerified',
    'silhouetteConsistencyVerified', 'groundAndAnchorsMeasured', 'desktopPreviewVerified',
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
  + `${references.assets.length} reference asset(s), sourcePixels=measured+remapped, `
  + `candidate=${candidate.status}, productionReady=${master.runtimeAssetReady}`,
);
