# Svelte Diagnostic Final

- Source commit: 8aadc7c9a35a0f46b9478748d506f608bcb3e8d7
- prepare: success
- npm ci: success
- TS6: failure
- TS7 tsgo: failure
- build/repo: success

## TS6
``text
d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:1907:3
[31mError[39m: Subsequent property declarations must have the same type.  Property 'buffers' must be of type '(GPUVertexBufferLayout | null)[] | undefined', but here has type 'Iterable<GPUVertexBufferLayout | null | undefined> | undefined'. 
[36m   */
  [35mbuffers[36m?: Iterable<
    | GPUVertexBufferLayout[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2135:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPU' must be of type '{ new (): GPU; prototype: GPU; }', but here has type '{ new (): never; prototype: GPU; }'. 
[36m
declare var [35mGPU[36m: {
  prototype: GPU;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2176:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUAdapter' must be of type '{ new (): GPUAdapter; prototype: GPUAdapter; }', but here has type '{ new (): never; prototype: GPUAdapter; }'. 
[36m
declare var [35mGPUAdapter[36m: {
  prototype: GPUAdapter;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2216:12
[31mError[39m: All declarations of 'subgroupMinSize' must have identical modifiers. 
[36m   */
  readonly [35msubgroupMinSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2216:12
[31mError[39m: Subsequent property declarations must have the same type.  Property 'subgroupMinSize' must be of type 'number', but here has type 'number | undefined'. 
[36m   */
  readonly [35msubgroupMinSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2224:12
[31mError[39m: All declarations of 'subgroupMaxSize' must have identical modifiers. 
[36m   */
  readonly [35msubgroupMaxSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2224:12
[31mError[39m: Subsequent property declarations must have the same type.  Property 'subgroupMaxSize' must be of type 'number', but here has type 'number | undefined'. 
[36m   */
  readonly [35msubgroupMaxSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2231:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUAdapterInfo' must be of type '{ new (): GPUAdapterInfo; prototype: GPUAdapterInfo; }', but here has type '{ new (): never; prototype: GPUAdapterInfo; }'. 
[36m
declare var [35mGPUAdapterInfo[36m: {
  prototype: GPUAdapterInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2242:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBindGroup' must be of type '{ new (): GPUBindGroup; prototype: GPUBindGroup; }', but here has type '{ new (): never; prototype: GPUBindGroup; }'. 
[36m
declare var [35mGPUBindGroup[36m: {
  prototype: GPUBindGroup;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2253:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBindGroupLayout' must be of type '{ new (): GPUBindGroupLayout; prototype: GPUBindGroupLayout; }', but here has type '{ new (): never; prototype: GPUBindGroupLayout; }'. 
[36m
declare var [35mGPUBindGroupLayout[36m: {
  prototype: GPUBindGroupLayout;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2309:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBuffer' must be of type '{ new (): GPUBuffer; prototype: GPUBuffer; }', but here has type '{ new (): never; prototype: GPUBuffer; }'. 
[36m
declare var [35mGPUBuffer[36m: {
  prototype: GPUBuffer;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2354:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCanvasContext' must be of type '{ new (): GPUCanvasContext; prototype: GPUCanvasContext; }', but here has type '{ new (): never; prototype: GPUCanvasContext; }'. 
[36m
declare var [35mGPUCanvasContext[36m: {
  prototype: GPUCanvasContext;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2365:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCommandBuffer' must be of type '{ new (): GPUCommandBuffer; prototype: GPUCommandBuffer; }', but here has type '{ new (): never; prototype: GPUCommandBuffer; }'. 
[36m
declare var [35mGPUCommandBuffer[36m: {
  prototype: GPUCommandBuffer;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2483:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCommandEncoder' must be of type '{ new (): GPUCommandEncoder; prototype: GPUCommandEncoder; }', but here has type '{ new (): never; prototype: GPUCommandEncoder; }'. 
[36m
declare var [35mGPUCommandEncoder[36m: {
  prototype: GPUCommandEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2494:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCompilationInfo' must be of type '{ new (): GPUCompilationInfo; prototype: GPUCompilationInfo; }', but here has type '{ new (): never; prototype: GPUCompilationInfo; }'. 
[36m
declare var [35mGPUCompilationInfo[36m: {
  prototype: GPUCompilationInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2554:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCompilationMessage' must be of type '{ new (): GPUCompilationMessage; prototype: GPUCompilationMessage; }', but here has type '{ new (): never; prototype: GPUCompilationMessage; }'. 
[36m
declare var [35mGPUCompilationMessage[36m: {
  prototype: GPUCompilationMessage;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2605:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUComputePassEncoder' must be of type '{ new (): GPUComputePassEncoder; prototype: GPUComputePassEncoder; }', but here has type '{ new (): never; prototype: GPUComputePassEncoder; }'. 
[36m
declare var [35mGPUComputePassEncoder[36m: {
  prototype: GPUComputePassEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2617:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUComputePipeline' must be of type '{ new (): GPUComputePipeline; prototype: GPUComputePipeline; }', but here has type '{ new (): never; prototype: GPUComputePipeline; }'. 
[36m
declare var [35mGPUComputePipeline[36m: {
  prototype: GPUComputePipeline;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2804:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUDevice' must be of type '{ new (): GPUDevice; prototype: GPUDevice; }', but here has type '{ new (): never; prototype: GPUDevice; }'. 
[36m
declare var [35mGPUDevice[36m: {
  prototype: GPUDevice;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2816:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUDeviceLostInfo' must be of type '{ new (): GPUDeviceLostInfo; prototype: GPUDeviceLostInfo; }', but here has type '{ new (): never; prototype: GPUDeviceLostInfo; }'. 
[36m
declare var [35mGPUDeviceLostInfo[36m: {
  prototype: GPUDeviceLostInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2840:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUError' must be of type '{ new (): GPUError; prototype: GPUError; }', but here has type '{ new (): never; prototype: GPUError; }'. 
[36m
declare var [35mGPUError[36m: {
  prototype: GPUError;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2851:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUExternalTexture' must be of type '{ new (): GPUExternalTexture; prototype: GPUExternalTexture; }', but here has type '{ new (): never; prototype: GPUExternalTexture; }'. 
[36m
declare var [35mGPUExternalTexture[36m: {
  prototype: GPUExternalTexture;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2897:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUPipelineError' must be of type '{ new (message: string, options: GPUPipelineErrorInit): GPUPipelineError; prototype: GPUPipelineError; }', but here has type '{ new (message: string | undefined, options: GPUPipelineErrorInit): GPUPipelineError; prototype: GPUPipelineError; }'. 
[36m
declare var [35mGPUPipelineError[36m: {
  prototype: GPUPipelineError;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2913:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUPipelineLayout' must be of type '{ new (): GPUPipelineLayout; prototype: GPUPipelineLayout; }', but here has type '{ new (): never; prototype: GPUPipelineLayout; }'. 
[36m
declare var [35mGPUPipelineLayout[36m: {
  prototype: GPUPipelineLayout;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2936:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUQuerySet' must be of type '{ new (): GPUQuerySet; prototype: GPUQuerySet; }', but here has type '{ new (): never; prototype: GPUQuerySet; }'. 
[36m
declare var [35mGPUQuerySet[36m: {
  prototype: GPUQuerySet;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3012:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUQueue' must be of type '{ new (): GPUQueue; prototype: GPUQueue; }', but here has type '{ new (): never; prototype: GPUQueue; }'. 
[36m
declare var [35mGPUQueue[36m: {
  prototype: GPUQueue;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3023:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderBundle' must be of type '{ new (): GPURenderBundle; prototype: GPURenderBundle; }', but here has type '{ new (): never; prototype: GPURenderBundle; }'. 
[36m
declare var [35mGPURenderBundle[36m: {
  prototype: GPURenderBundle;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3045:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderBundleEncoder' must be of type '{ new (): GPURenderBundleEncoder; prototype: GPURenderBundleEncoder; }', but here has type '{ new (): never; prototype: GPURenderBundleEncoder; }'. 
[36m
declare var [35mGPURenderBundleEncoder[36m: {
  prototype: GPURenderBundleEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3136:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderPassEncoder' must be of type '{ new (): GPURenderPassEncoder; prototype: GPURenderPassEncoder; }', but here has type '{ new (): never; prototype: GPURenderPassEncoder; }'. 
[36m
declare var [35mGPURenderPassEncoder[36m: {
  prototype: GPURenderPassEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3148:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderPipeline' must be of type '{ new (): GPURenderPipeline; prototype: GPURenderPipeline; }', but here has type '{ new (): never; prototype: GPURenderPipeline; }'. 
[36m
declare var [35mGPURenderPipeline[36m: {
  prototype: GPURenderPipeline;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3159:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUSampler' must be of type '{ new (): GPUSampler; prototype: GPUSampler; }', but here has type '{ new (): never; prototype: GPUSampler; }'. 
[36m
declare var [35mGPUSampler[36m: {
  prototype: GPUSampler;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3176:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUShaderModule' must be of type '{ new (): GPUShaderModule; prototype: GPUShaderModule; }', but here has type '{ new (): never; prototype: GPUShaderModule; }'. 
[36m
declare var [35mGPUShaderModule[36m: {
  prototype: GPUShaderModule;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3241:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUSupportedLimits' must be of type '{ new (): GPUSupportedLimits; prototype: GPUSupportedLimits; }', but here has type '{ new (): never; prototype: GPUSupportedLimits; }'. 
[36m
declare var [35mGPUSupportedLimits[36m: {
  prototype: GPUSupportedLimits;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3311:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUTexture' must be of type '{ new (): GPUTexture; prototype: GPUTexture; }', but here has type '{ new (): never; prototype: GPUTexture; }'. 
[36m
declare var [35mGPUTexture[36m: {
  prototype: GPUTexture;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3322:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUTextureView' must be of type '{ new (): GPUTextureView; prototype: GPUTextureView; }', but here has type '{ new (): never; prototype: GPUTextureView; }'. 
[36m
declare var [35mGPUTextureView[36m: {
  prototype: GPUTextureView;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\pixi.js\lib\events\FederatedPointerEvent.d.ts[39m:48:22
[31mError[39m: Class 'FederatedPointerEvent' incorrectly implements interface 'PointerEvent'.
  Property 'persistentDeviceId' is missing in type 'FederatedPointerEvent' but required in type 'PointerEvent'. 
[36m */
export declare class [35mFederatedPointerEvent[36m extends FederatedMouseEvent implements PointerEvent {
    /**[39m

====================================
[31msvelte-check found 52 errors and 0 warnings in 4 files
[39m
``

## TS7 tsgo
``text

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2216:12
[31mError[39m: All declarations of 'subgroupMinSize' must have identical modifiers. (ts)
[36m   */
  readonly [35msubgroupMinSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2216:12
[31mError[39m: Subsequent property declarations must have the same type.  Property 'subgroupMinSize' must be of type 'number', but here has type 'number | undefined'. (ts)
[36m   */
  readonly [35msubgroupMinSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2224:12
[31mError[39m: All declarations of 'subgroupMaxSize' must have identical modifiers. (ts)
[36m   */
  readonly [35msubgroupMaxSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2224:12
[31mError[39m: Subsequent property declarations must have the same type.  Property 'subgroupMaxSize' must be of type 'number', but here has type 'number | undefined'. (ts)
[36m   */
  readonly [35msubgroupMaxSize[36m?: number;
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2231:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUAdapterInfo' must be of type '{ new (): GPUAdapterInfo; prototype: GPUAdapterInfo; }', but here has type '{ new (): never; prototype: GPUAdapterInfo; }'. (ts)
[36m
declare var [35mGPUAdapterInfo[36m: {
  prototype: GPUAdapterInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2242:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBindGroup' must be of type '{ new (): GPUBindGroup; prototype: GPUBindGroup; }', but here has type '{ new (): never; prototype: GPUBindGroup; }'. (ts)
[36m
declare var [35mGPUBindGroup[36m: {
  prototype: GPUBindGroup;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2253:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBindGroupLayout' must be of type '{ new (): GPUBindGroupLayout; prototype: GPUBindGroupLayout; }', but here has type '{ new (): never; prototype: GPUBindGroupLayout; }'. (ts)
[36m
declare var [35mGPUBindGroupLayout[36m: {
  prototype: GPUBindGroupLayout;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2309:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUBuffer' must be of type '{ new (): GPUBuffer; prototype: GPUBuffer; }', but here has type '{ new (): never; prototype: GPUBuffer; }'. (ts)
[36m
declare var [35mGPUBuffer[36m: {
  prototype: GPUBuffer;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2354:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCanvasContext' must be of type '{ new (): GPUCanvasContext; prototype: GPUCanvasContext; }', but here has type '{ new (): never; prototype: GPUCanvasContext; }'. (ts)
[36m
declare var [35mGPUCanvasContext[36m: {
  prototype: GPUCanvasContext;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2365:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCommandBuffer' must be of type '{ new (): GPUCommandBuffer; prototype: GPUCommandBuffer; }', but here has type '{ new (): never; prototype: GPUCommandBuffer; }'. (ts)
[36m
declare var [35mGPUCommandBuffer[36m: {
  prototype: GPUCommandBuffer;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2483:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCommandEncoder' must be of type '{ new (): GPUCommandEncoder; prototype: GPUCommandEncoder; }', but here has type '{ new (): never; prototype: GPUCommandEncoder; }'. (ts)
[36m
declare var [35mGPUCommandEncoder[36m: {
  prototype: GPUCommandEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2494:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCompilationInfo' must be of type '{ new (): GPUCompilationInfo; prototype: GPUCompilationInfo; }', but here has type '{ new (): never; prototype: GPUCompilationInfo; }'. (ts)
[36m
declare var [35mGPUCompilationInfo[36m: {
  prototype: GPUCompilationInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2554:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUCompilationMessage' must be of type '{ new (): GPUCompilationMessage; prototype: GPUCompilationMessage; }', but here has type '{ new (): never; prototype: GPUCompilationMessage; }'. (ts)
[36m
declare var [35mGPUCompilationMessage[36m: {
  prototype: GPUCompilationMessage;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2605:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUComputePassEncoder' must be of type '{ new (): GPUComputePassEncoder; prototype: GPUComputePassEncoder; }', but here has type '{ new (): never; prototype: GPUComputePassEncoder; }'. (ts)
[36m
declare var [35mGPUComputePassEncoder[36m: {
  prototype: GPUComputePassEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2617:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUComputePipeline' must be of type '{ new (): GPUComputePipeline; prototype: GPUComputePipeline; }', but here has type '{ new (): never; prototype: GPUComputePipeline; }'. (ts)
[36m
declare var [35mGPUComputePipeline[36m: {
  prototype: GPUComputePipeline;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2804:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUDevice' must be of type '{ new (): GPUDevice; prototype: GPUDevice; }', but here has type '{ new (): never; prototype: GPUDevice; }'. (ts)
[36m
declare var [35mGPUDevice[36m: {
  prototype: GPUDevice;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2816:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUDeviceLostInfo' must be of type '{ new (): GPUDeviceLostInfo; prototype: GPUDeviceLostInfo; }', but here has type '{ new (): never; prototype: GPUDeviceLostInfo; }'. (ts)
[36m
declare var [35mGPUDeviceLostInfo[36m: {
  prototype: GPUDeviceLostInfo;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2840:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUError' must be of type '{ new (): GPUError; prototype: GPUError; }', but here has type '{ new (): never; prototype: GPUError; }'. (ts)
[36m
declare var [35mGPUError[36m: {
  prototype: GPUError;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2851:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUExternalTexture' must be of type '{ new (): GPUExternalTexture; prototype: GPUExternalTexture; }', but here has type '{ new (): never; prototype: GPUExternalTexture; }'. (ts)
[36m
declare var [35mGPUExternalTexture[36m: {
  prototype: GPUExternalTexture;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2897:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUPipelineError' must be of type '{ new (message: string, options: GPUPipelineErrorInit): GPUPipelineError; prototype: GPUPipelineError; }', but here has type '{ new (message: string | undefined, options: GPUPipelineErrorInit): GPUPipelineError; prototype: GPUPipelineError; }'. (ts)
[36m
declare var [35mGPUPipelineError[36m: {
  prototype: GPUPipelineError;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2913:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUPipelineLayout' must be of type '{ new (): GPUPipelineLayout; prototype: GPUPipelineLayout; }', but here has type '{ new (): never; prototype: GPUPipelineLayout; }'. (ts)
[36m
declare var [35mGPUPipelineLayout[36m: {
  prototype: GPUPipelineLayout;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:2936:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUQuerySet' must be of type '{ new (): GPUQuerySet; prototype: GPUQuerySet; }', but here has type '{ new (): never; prototype: GPUQuerySet; }'. (ts)
[36m
declare var [35mGPUQuerySet[36m: {
  prototype: GPUQuerySet;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3012:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUQueue' must be of type '{ new (): GPUQueue; prototype: GPUQueue; }', but here has type '{ new (): never; prototype: GPUQueue; }'. (ts)
[36m
declare var [35mGPUQueue[36m: {
  prototype: GPUQueue;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3023:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderBundle' must be of type '{ new (): GPURenderBundle; prototype: GPURenderBundle; }', but here has type '{ new (): never; prototype: GPURenderBundle; }'. (ts)
[36m
declare var [35mGPURenderBundle[36m: {
  prototype: GPURenderBundle;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3045:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderBundleEncoder' must be of type '{ new (): GPURenderBundleEncoder; prototype: GPURenderBundleEncoder; }', but here has type '{ new (): never; prototype: GPURenderBundleEncoder; }'. (ts)
[36m
declare var [35mGPURenderBundleEncoder[36m: {
  prototype: GPURenderBundleEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3136:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderPassEncoder' must be of type '{ new (): GPURenderPassEncoder; prototype: GPURenderPassEncoder; }', but here has type '{ new (): never; prototype: GPURenderPassEncoder; }'. (ts)
[36m
declare var [35mGPURenderPassEncoder[36m: {
  prototype: GPURenderPassEncoder;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3148:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPURenderPipeline' must be of type '{ new (): GPURenderPipeline; prototype: GPURenderPipeline; }', but here has type '{ new (): never; prototype: GPURenderPipeline; }'. (ts)
[36m
declare var [35mGPURenderPipeline[36m: {
  prototype: GPURenderPipeline;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3159:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUSampler' must be of type '{ new (): GPUSampler; prototype: GPUSampler; }', but here has type '{ new (): never; prototype: GPUSampler; }'. (ts)
[36m
declare var [35mGPUSampler[36m: {
  prototype: GPUSampler;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3176:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUShaderModule' must be of type '{ new (): GPUShaderModule; prototype: GPUShaderModule; }', but here has type '{ new (): never; prototype: GPUShaderModule; }'. (ts)
[36m
declare var [35mGPUShaderModule[36m: {
  prototype: GPUShaderModule;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3181:6
[31mError[39m: Duplicate identifier 'GPUSupportedFeatures'. (ts)
[36m
type [35mGPUSupportedFeatures[36m =
  ReadonlySet<string>;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3241:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUSupportedLimits' must be of type '{ new (): GPUSupportedLimits; prototype: GPUSupportedLimits; }', but here has type '{ new (): never; prototype: GPUSupportedLimits; }'. (ts)
[36m
declare var [35mGPUSupportedLimits[36m: {
  prototype: GPUSupportedLimits;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3311:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUTexture' must be of type '{ new (): GPUTexture; prototype: GPUTexture; }', but here has type '{ new (): never; prototype: GPUTexture; }'. (ts)
[36m
declare var [35mGPUTexture[36m: {
  prototype: GPUTexture;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3322:13
[31mError[39m: Subsequent variable declarations must have the same type.  Variable 'GPUTextureView' must be of type '{ new (): GPUTextureView; prototype: GPUTextureView; }', but here has type '{ new (): never; prototype: GPUTextureView; }'. (ts)
[36m
declare var [35mGPUTextureView[36m: {
  prototype: GPUTextureView;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\@webgpu\types\dist\index.d.ts[39m:3359:6
[31mError[39m: Duplicate identifier 'WGSLLanguageFeatures'. (ts)
[36m
type [35mWGSLLanguageFeatures[36m =
  ReadonlySet<string>;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\pixi.js\lib\events\FederatedPointerEvent.d.ts[39m:48:22
[31mError[39m: Class 'FederatedPointerEvent' incorrectly implements interface 'PointerEvent'. (ts)
[36m */
export declare class [35mFederatedPointerEvent[36m extends FederatedMouseEvent implements PointerEvent {
    /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32msrc\lib\pet\bubble.ts[39m:15:3
[31mError[39m: Object literal may only specify known properties, and 'recovering' does not exist in type 'Record<"degraded" | "error", string>'. (ts)
[36m  degraded: '我還在，只是有些功能暫時休息。',
  [35mrecovering[36m: '等我一下，我正在恢復。',
  error: '我遇到一點問題，但 Pet Runtime 不會假裝沒事。',[39m

====================================
[31msvelte-check found 184 errors and 0 warnings in 4 files
[39m
``

## Build
``text

> northpalace-my-pet@0.1.0 build
> npm run validate:assets && vite build


> northpalace-my-pet@0.1.0 validate:assets
> node scripts/validate-lenvu-assets.mjs

[Lenvu asset contract] OK — 14 animation profiles, 1 reference asset(s), sourcePixels=measured+remapped, candidate=awaiting_source_faithful_candidate_artwork, productionReady=false
5:15:15 AM [vite-plugin-svelte] no Svelte config found at D:/a/NorthPalace-my-pet/NorthPalace-my-pet - using default configuration.
[36mvite v8.2.0 [32mbuilding client environment for production...[36m[39m
[2K
transforming...✓ 839 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                                     1.15 kB │ gzip:  0.48 kB
dist/assets/index-BvdaOVeU.css                     12.55 kB │ gzip:  2.96 kB
dist/assets/webworkerAll-zs0gkWkb.js                0.05 kB │ gzip:  0.06 kB
dist/assets/getTextureBatchBindGroup-CIgez0gS.js    0.40 kB │ gzip:  0.31 kB │ map:     1.78 kB
dist/assets/CanvasPool-DJhC69AR.js                  0.80 kB │ gzip:  0.45 kB │ map:     3.51 kB
dist/assets/canvasUtils-gItO4D4Z.js                 6.07 kB │ gzip:  2.06 kB │ map:    21.91 kB
dist/assets/BufferResource-EioRVO9d.js             10.57 kB │ gzip:  2.81 kB │ map:    25.61 kB
dist/assets/init-Bcz1Jwfn.js                       15.59 kB │ gzip:  4.91 kB │ map:    54.88 kB
dist/assets/init-C63Eqe5X.js                       24.72 kB │ gzip:  8.52 kB │ map:   105.83 kB
dist/assets/browserAll-BzSnKDMY.js                 42.63 kB │ gzip: 11.16 kB │ map:   156.60 kB
dist/assets/RenderTargetSystem-DI_S-4H-.js         71.11 kB │ gzip: 20.19 kB │ map:   262.25 kB
dist/assets/CanvasRenderer-MeLDGdTD.js             87.39 kB │ gzip: 27.41 kB │ map:   463.14 kB
dist/assets/Geometry-CUD3NC1u.js                  101.64 kB │ gzip: 31.16 kB │ map:   499.93 kB
dist/assets/index-TL9cjoZn.js                     210.03 kB │ gzip: 64.69 kB │ map: 1,094.86 kB

[32m✓ built in 734ms[39m
``
