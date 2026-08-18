import { Application, Container, Graphics, type Ticker } from 'pixi.js';
import type { PetRuntimeSnapshot } from '../types';
import { animationProfile, resolveAnimation, type LenvuAnimationId } from './animation';
import { hitTestLenvu } from './hitTest';
import { lenvuManifest, type LenvuHitZoneId } from './manifest';

/**
 * PixiJS owns Lenvu's high-frequency presentation layer.
 *
 * This remains a procedural placeholder. The manifest already defines the stable renderer contract,
 * so production sprites/atlases can replace these primitives without changing Pet Brain.
 * Direction changes are semantic rather than blind root mirroring because Lenvu has asymmetric
 * identity features (heterochromia and the left-side crescent-horn ornament).
 */
export class PetRenderer {
  private readonly app = new Application();
  private readonly root = new Container();
  private readonly symmetricLayer = new Container();
  private readonly identityLayer = new Container();
  private readonly focusRing = new Graphics()
    .circle(0, 54, 58)
    .stroke({ color: 0x4edbff, width: 2, alpha: 0.55 });
  private readonly leftEye = new Graphics().circle(-18, -26, 6).fill(0xb991ff);
  private readonly rightEye = new Graphics().circle(18, -26, 6).fill(0x67e9ff);
  private readonly leftCrescent = new Graphics()
    .circle(-24, -83, 8)
    .stroke({ color: 0xd8b86b, width: 2.2, alpha: 0.95 });
  private snapshot: PetRuntimeSnapshot | null = null;
  private animation: LenvuAnimationId = 'idle';
  private elapsedSeconds = 0;
  private poseScaleX = 1;
  private poseScaleY = 1;

  async mount(container: HTMLElement) {
    await this.app.init({
      resizeTo: container,
      backgroundAlpha: 0,
      preference: 'webgl',
      powerPreference: 'low-power',
      antialias: false,
      resolution: 1,
      autoDensity: false,
      sharedTicker: false,
      autoStart: true,
    });

    this.app.canvas.setAttribute('aria-hidden', 'true');
    container.appendChild(this.app.canvas);

    this.focusRing.scale.y = 0.32;
    this.focusRing.visible = false;

    const body = new Graphics()
      .roundRect(-42, -18, 84, 92, 34)
      .fill({ color: 0x8ba7b8, alpha: 0.96 })
      .stroke({ color: 0x6ee3ff, width: 1.5, alpha: 0.55 });

    const chest = new Graphics()
      .roundRect(-27, 10, 54, 54, 24)
      .fill({ color: 0xd5e5ee, alpha: 0.85 });

    const head = new Graphics()
      .circle(0, -34, 47)
      .fill({ color: 0xb8cbd7, alpha: 0.98 })
      .stroke({ color: 0x72e8ff, width: 1.5, alpha: 0.6 });

    const leftEar = new Graphics()
      .circle(-36, -68, 21)
      .fill({ color: 0x6f899b, alpha: 0.98 });
    leftEar.scale.y = 1.4;

    const rightEar = new Graphics()
      .circle(36, -68, 21)
      .fill({ color: 0x6f899b, alpha: 0.98 });
    rightEar.scale.y = 1.4;

    // Placeholder horns deliberately remain dark and separate from the gold crescent ornament.
    const leftHorn = new Graphics().circle(-19, -82, 5).fill(0x26313d);
    const rightHorn = new Graphics().circle(19, -82, 5).fill(0x26313d);

    const tail = new Graphics()
      .circle(47, 37, 19)
      .fill({ color: 0x77e9ff, alpha: 0.48 });

    this.symmetricLayer.addChild(
      this.focusRing,
      tail,
      body,
      chest,
      head,
      leftEar,
      rightEar,
      leftHorn,
      rightHorn,
    );
    this.identityLayer.addChild(this.leftEye, this.rightEye, this.leftCrescent);
    this.root.addChild(this.symmetricLayer, this.identityLayer);

    this.root.position.set(container.clientWidth / 2, container.clientHeight / 2 + 8);
    this.app.stage.addChild(this.root);
    this.app.ticker.minFPS = 2;
    this.app.ticker.maxFPS = lenvuManifest.render.idleFrameBudgetFps;
    this.app.ticker.add(this.animate);
  }

  update(snapshot: PetRuntimeSnapshot) {
    this.snapshot = snapshot;
    this.animation = resolveAnimation(snapshot);
    this.focusRing.visible = snapshot.state.mode === 'focus_guard' && snapshot.state.posture !== 'held';

    const profile = animationProfile(this.animation);
    const lowPower = snapshot.behavior === null && (snapshot.state.posture === 'sleep' || snapshot.state.posture === 'lie');
    this.app.ticker.maxFPS = Math.max(2, lowPower ? profile.lowPowerFps : profile.fps);

    const eyeScale = snapshot.state.emotion === 'happy' ? 1.18 : 1;
    this.leftEye.scale.set(eyeScale);
    this.rightEye.scale.set(eyeScale);

    switch (snapshot.state.posture) {
      case 'sleep':
        this.root.rotation = -0.12;
        this.poseScaleX = 1.02;
        this.poseScaleY = 0.72;
        break;
      case 'lie':
        this.root.rotation = -0.06;
        this.poseScaleX = 1.03;
        this.poseScaleY = 0.82;
        break;
      case 'sit':
        this.root.rotation = 0;
        this.poseScaleX = 0.96;
        this.poseScaleY = 0.94;
        break;
      case 'held':
        this.root.rotation = 0.04;
        this.poseScaleX = 0.91;
        this.poseScaleY = 0.94;
        break;
      default:
        this.root.rotation = 0;
        this.poseScaleX = 1;
        this.poseScaleY = 1;
    }

    this.applyFacingScale();
  }

  currentAnimation(): LenvuAnimationId {
    return this.animation;
  }

  hitTest(clientX: number, clientY: number): LenvuHitZoneId | null {
    const view = this.app.canvas.parentElement;
    if (!view) return null;
    return hitTestLenvu(
      clientX,
      clientY,
      view.getBoundingClientRect(),
      this.snapshot?.state.facing ?? 'right',
    );
  }

  destroy() {
    this.app.ticker.remove(this.animate);
    this.app.destroy(true, { children: true });
  }

  private applyFacingScale() {
    const facingSign = this.snapshot?.state.facing === 'left' ? -1 : 1;
    // Only symmetric placeholder geometry is mirrored. Identity-bearing features stay on their
    // canonical semantic sides and are repositioned explicitly below.
    this.symmetricLayer.scale.x = facingSign;
    this.root.scale.set(this.poseScaleX, this.poseScaleY);

    const leftEyeX = -18 * facingSign;
    const rightEyeX = 18 * facingSign;
    this.leftEye.position.x = leftEyeX - (-18);
    this.rightEye.position.x = rightEyeX - 18;
    this.leftCrescent.position.x = -24 * facingSign - (-24);
  }

  private animate = (ticker: Ticker) => {
    this.elapsedSeconds += ticker.deltaMS / 1000;
    const elapsed = this.elapsedSeconds;
    const profile = animationProfile(this.animation);
    const posture = this.snapshot?.state.posture;

    let bob = Math.sin(elapsed * 2.1) * profile.bodyBob;
    let sway = Math.sin(elapsed * 1.25) * profile.sway;

    if (this.animation === 'play') {
      bob = Math.abs(Math.sin(elapsed * 6)) * -profile.bodyBob;
      sway = Math.sin(elapsed * 5) * profile.sway;
    } else if (this.animation === 'pet_receive') {
      bob = Math.sin(elapsed * 4) * profile.bodyBob;
    } else if (this.animation === 'sleep') {
      bob = Math.sin(elapsed * 1.2) * profile.bodyBob;
      sway = -0.12;
    } else if (this.animation === 'held') {
      bob = Math.sin(elapsed * 3.2) * profile.bodyBob;
      sway = Math.sin(elapsed * 2.4) * profile.sway + 0.04;
    } else if (this.animation === 'jump') {
      bob = -Math.abs(Math.sin(elapsed * 4.5)) * profile.bodyBob;
    }

    const view = this.app.canvas.parentElement;
    if (view) {
      this.root.position.set(view.clientWidth / 2, view.clientHeight / 2 + 8 + bob);
    }
    if (posture !== 'sleep' && posture !== 'lie') {
      const facingSign = this.snapshot?.state.facing === 'left' ? -1 : 1;
      this.root.rotation = sway * facingSign;
    }

    this.focusRing.alpha = 0.45 + Math.sin(elapsed * 2.4) * 0.18;
  };
}
