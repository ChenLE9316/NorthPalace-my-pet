# Svelte Diagnostic Bootstrap

- Source commit: 1873b0a7cd4696f7c5ba92d42685dd8eeb574ffc
- resolve: success
- npm ci: success
- TS6 svelte-check: failure
- TS7 tsgo svelte-check: failure
- build: success
- tracked-data guard: success

## Resolve
``text

added 63 packages, and audited 64 packages in 11s

14 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
``

## TS6 svelte-check
``text

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:13:35
[31mError[39m: Cannot find name 'node:http2'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { Agent, ClientRequest, ClientRequestArgs, OutgoingHttpHeaders, ServerResponse } from "node:http";
import { Http2SecureServer } from [35m"node:http2"[36m;
import * as fs from "node:fs";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:14:21
[31mError[39m: Cannot find name 'node:fs'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { Http2SecureServer } from "node:http2";
import * as fs from [35m"node:fs"[36m;
import { EventEmitter } from "node:events";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:15:30
[31mError[39m: Cannot find name 'node:events'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport * as fs from "node:fs";
import { EventEmitter } from [35m"node:events"[36m;
import { Server as HttpsServer, ServerOptions as HttpsServerOptions } from "node:https";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:16:76
[31mError[39m: Cannot find name 'node:https'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { EventEmitter } from "node:events";
import { Server as HttpsServer, ServerOptions as HttpsServerOptions } from [35m"node:https"[36m;
import * as net from "node:net";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:17:22
[31mError[39m: Cannot find name 'node:net'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { Server as HttpsServer, ServerOptions as HttpsServerOptions } from "node:https";
import * as net from [35m"node:net"[36m;
import { Duplex, DuplexOptions, Stream } from "node:stream";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:18:47
[31mError[39m: Cannot find name 'node:stream'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport * as net from "node:net";
import { Duplex, DuplexOptions, Stream } from [35m"node:stream"[36m;
import { FetchFunction, FetchFunctionOptions, FetchResult, FetchResult as moduleRunner_FetchResult, ModuleEvaluator, ModuleRunner, ModuleRunnerHmr, ModuleRunnerOptions } from "vite/module-runner";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:21:38
[31mError[39m: Cannot find name 'node:tls'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { DepsOptimizerEsbuildOptions, EsbuildTarget, EsbuildTransformOptions, EsbuildTransformOptions as esbuildOptions_EsbuildTransformOptions, EsbuildTransformResult } from "#types/internal/esbuildOptions";
import { SecureContextOptions } from [35m"node:tls"[36m;
import { URL as url_URL } from "node:url";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:22:32
[31mError[39m: Cannot find name 'node:url'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { SecureContextOptions } from "node:tls";
import { URL as url_URL } from [35m"node:url"[36m;
import { ZlibOptions } from "node:zlib";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:23:29
[31mError[39m: Cannot find name 'node:zlib'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mimport { URL as url_URL } from "node:url";
import { ZlibOptions } from [35m"node:zlib"[36m;
import { ChunkMetadata, CustomPluginOptionsVite } from "#types/metadata";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:285:35
[31mError[39m: Cannot find namespace 'NodeJS'. 
[36m  }
  export interface Server extends [35mNodeJS[36m.EventEmitter {
    (req: http.IncomingMessage, res: http.ServerResponse, next?: Function): void;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:347:9
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  passphrase?: string;
  pfx?: [35mBuffer[36m | string;
  cert?: string;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:463:105
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36mtype PassFunctions<TIncomingMessage extends typeof http.IncomingMessage = typeof http.IncomingMessage, TServerResponse extends typeof http.ServerResponse = typeof http.ServerResponse, TError = Error> = {
  ws: (req: InstanceType<TIncomingMessage>, socket: net.Socket, options: NormalizedServerOptions, head: [35mBuffer[36m | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
  web: (req: InstanceType<TIncomingMessage>, res: InstanceType<TServerResponse>, options: NormalizedServerOptions, head: Buffer | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:464:122
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  ws: (req: InstanceType<TIncomingMessage>, socket: net.Socket, options: NormalizedServerOptions, head: Buffer | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
  web: (req: InstanceType<TIncomingMessage>, res: InstanceType<TServerResponse>, options: NormalizedServerOptions, head: [35mBuffer[36m | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
};[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1336:40
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  constructor(address: string | url_URL, protocols?: string | string[], options?: WebSocket.ClientOptions | ClientRequestArgs);
  close(code?: number, data?: string | [35mBuffer[36m): void;
  ping(data?: any, mask?: boolean, cb?: (err: Error) => void): void;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1368:72
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  // Events
  on(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  on(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1373:64
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  on(event: 'open', listener: (this: WebSocket) => void): this;
  on(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  on(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1376:74
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  on(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  once(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  once(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1381:66
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  once(event: 'open', listener: (this: WebSocket) => void): this;
  once(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  once(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1384:73
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  once(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  off(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  off(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1389:65
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  off(event: 'open', listener: (this: WebSocket) => void): this;
  off(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  off(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1392:64
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  off(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  addListener(event: 'close', listener: (code: number, reason: [35mBuffer[36m) => void): this;
  addListener(event: 'error', listener: (err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1397:56
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  addListener(event: 'open', listener: () => void): this;
  addListener(event: 'ping' | 'pong', listener: (data: [35mBuffer[36m) => void): this;
  addListener(event: 'unexpected-response', listener: (request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1400:67
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  addListener(event: string | symbol, listener: (...args: any[]) => void): this;
  removeListener(event: 'close', listener: (code: number, reason: [35mBuffer[36m) => void): this;
  removeListener(event: 'error', listener: (err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1405:59
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m  removeListener(event: 'open', listener: () => void): this;
  removeListener(event: 'ping' | 'pong', listener: (data: [35mBuffer[36m) => void): this;
  removeListener(event: 'unexpected-response', listener: (request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1415:18
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type RawData = [35mBuffer[36m | ArrayBuffer | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1415:41
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type RawData = Buffer | ArrayBuffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1419:24
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type Data = string | [35mBuffer[36m | ArrayBuffer | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1419:47
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type Data = string | Buffer | ArrayBuffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1423:39
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type CertMeta = string | string[] | [35mBuffer[36m | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1423:48
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  type CertMeta = string | string[] | Buffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1447:25
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m    followRedirects?: boolean | undefined;
    generateMask?(mask: [35mBuffer[36m): void;
    handshakeTimeout?: number | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1478:20
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m      strategy?: number | undefined;
      dictionary?: [35mBuffer[36m | Buffer[] | DataView | undefined;
      info?: boolean | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1478:29
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m      strategy?: number | undefined;
      dictionary?: Buffer | [35mBuffer[36m[] | DataView | undefined;
      info?: boolean | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1538:79
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m    close(cb?: (err?: Error) => void): void;
    handleUpgrade(request: http.IncomingMessage, socket: Duplex, upgradeHead: [35mBuffer[36m, callback: (client: T, request: http.IncomingMessage) => void): void;
    shouldHandle(request: http.IncomingMessage): boolean | Promise<boolean>;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:2126:61
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m   */
  assetsInlineLimit?: number | ((filePath: string, content: [35mBuffer[36m) => boolean | undefined);
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:3972:89
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. 
[36m}
declare function send(req: http.IncomingMessage, res: ServerResponse, content: string | [35mBuffer[36m, type: string, options: SendOptions): void;
//#endregion[39m

====================================
[31msvelte-check found 93 errors and 5 warnings in 12 files
[39m
``

## TS7 tsgo svelte-check
``text
import { EventEmitter } from "node:events";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:15:30
[31mError[39m: Cannot find name 'node:events'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport * as fs from "node:fs";
import { EventEmitter } from [35m"node:events"[36m;
import { Server as HttpsServer, ServerOptions as HttpsServerOptions } from "node:https";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:16:76
[31mError[39m: Cannot find name 'node:https'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport { EventEmitter } from "node:events";
import { Server as HttpsServer, ServerOptions as HttpsServerOptions } from [35m"node:https"[36m;
import * as net from "node:net";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:17:22
[31mError[39m: Cannot find name 'node:net'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport { Server as HttpsServer, ServerOptions as HttpsServerOptions } from "node:https";
import * as net from [35m"node:net"[36m;
import { Duplex, DuplexOptions, Stream } from "node:stream";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:18:47
[31mError[39m: Cannot find name 'node:stream'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport * as net from "node:net";
import { Duplex, DuplexOptions, Stream } from [35m"node:stream"[36m;
import { FetchFunction, FetchFunctionOptions, FetchResult, FetchResult as moduleRunner_FetchResult, ModuleEvaluator, ModuleRunner, ModuleRunnerHmr, ModuleRunnerOptions } from "vite/module-runner";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:21:38
[31mError[39m: Cannot find name 'node:tls'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport { DepsOptimizerEsbuildOptions, EsbuildTarget, EsbuildTransformOptions, EsbuildTransformOptions as esbuildOptions_EsbuildTransformOptions, EsbuildTransformResult } from "#types/internal/esbuildOptions";
import { SecureContextOptions } from [35m"node:tls"[36m;
import { URL as url_URL } from "node:url";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:22:32
[31mError[39m: Cannot find name 'node:url'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport { SecureContextOptions } from "node:tls";
import { URL as url_URL } from [35m"node:url"[36m;
import { ZlibOptions } from "node:zlib";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:23:29
[31mError[39m: Cannot find name 'node:zlib'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mimport { URL as url_URL } from "node:url";
import { ZlibOptions } from [35m"node:zlib"[36m;
import { ChunkMetadata, CustomPluginOptionsVite } from "#types/metadata";[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:285:35
[31mError[39m: Cannot find namespace 'NodeJS'. (ts)
[36m  }
  export interface Server extends [35mNodeJS[36m.EventEmitter {
    (req: http.IncomingMessage, res: http.ServerResponse, next?: Function): void;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:347:9
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  passphrase?: string;
  pfx?: [35mBuffer[36m | string;
  cert?: string;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:463:105
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36mtype PassFunctions<TIncomingMessage extends typeof http.IncomingMessage = typeof http.IncomingMessage, TServerResponse extends typeof http.ServerResponse = typeof http.ServerResponse, TError = Error> = {
  ws: (req: InstanceType<TIncomingMessage>, socket: net.Socket, options: NormalizedServerOptions, head: [35mBuffer[36m | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
  web: (req: InstanceType<TIncomingMessage>, res: InstanceType<TServerResponse>, options: NormalizedServerOptions, head: Buffer | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:464:122
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  ws: (req: InstanceType<TIncomingMessage>, socket: net.Socket, options: NormalizedServerOptions, head: Buffer | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
  web: (req: InstanceType<TIncomingMessage>, res: InstanceType<TServerResponse>, options: NormalizedServerOptions, head: [35mBuffer[36m | undefined, server: ProxyServer<TIncomingMessage, TServerResponse, TError>, cb?: ErrorCallback<TIncomingMessage, TServerResponse, TError>) => unknown;
};[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1336:40
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  constructor(address: string | url_URL, protocols?: string | string[], options?: WebSocket.ClientOptions | ClientRequestArgs);
  close(code?: number, data?: string | [35mBuffer[36m): void;
  ping(data?: any, mask?: boolean, cb?: (err: Error) => void): void;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1368:72
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  // Events
  on(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  on(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1373:64
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  on(event: 'open', listener: (this: WebSocket) => void): this;
  on(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  on(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1376:74
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  on(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  once(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  once(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1381:66
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  once(event: 'open', listener: (this: WebSocket) => void): this;
  once(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  once(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1384:73
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  once(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  off(event: 'close', listener: (this: WebSocket, code: number, reason: [35mBuffer[36m) => void): this;
  off(event: 'error', listener: (this: WebSocket, err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1389:65
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  off(event: 'open', listener: (this: WebSocket) => void): this;
  off(event: 'ping' | 'pong', listener: (this: WebSocket, data: [35mBuffer[36m) => void): this;
  off(event: 'unexpected-response', listener: (this: WebSocket, request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1392:64
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  off(event: string | symbol, listener: (this: WebSocket, ...args: any[]) => void): this;
  addListener(event: 'close', listener: (code: number, reason: [35mBuffer[36m) => void): this;
  addListener(event: 'error', listener: (err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1397:56
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  addListener(event: 'open', listener: () => void): this;
  addListener(event: 'ping' | 'pong', listener: (data: [35mBuffer[36m) => void): this;
  addListener(event: 'unexpected-response', listener: (request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1400:67
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  addListener(event: string | symbol, listener: (...args: any[]) => void): this;
  removeListener(event: 'close', listener: (code: number, reason: [35mBuffer[36m) => void): this;
  removeListener(event: 'error', listener: (err: Error) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1405:59
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m  removeListener(event: 'open', listener: () => void): this;
  removeListener(event: 'ping' | 'pong', listener: (data: [35mBuffer[36m) => void): this;
  removeListener(event: 'unexpected-response', listener: (request: ClientRequest, response: http.IncomingMessage) => void): this;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1415:18
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type RawData = [35mBuffer[36m | ArrayBuffer | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1415:41
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type RawData = Buffer | ArrayBuffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1419:24
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type Data = string | [35mBuffer[36m | ArrayBuffer | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1419:47
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type Data = string | Buffer | ArrayBuffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1423:39
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type CertMeta = string | string[] | [35mBuffer[36m | Buffer[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1423:48
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  type CertMeta = string | string[] | Buffer | [35mBuffer[36m[];
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1447:25
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m    followRedirects?: boolean | undefined;
    generateMask?(mask: [35mBuffer[36m): void;
    handshakeTimeout?: number | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1478:20
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m      strategy?: number | undefined;
      dictionary?: [35mBuffer[36m | Buffer[] | DataView | undefined;
      info?: boolean | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1478:29
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m      strategy?: number | undefined;
      dictionary?: Buffer | [35mBuffer[36m[] | DataView | undefined;
      info?: boolean | undefined;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:1538:79
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m    close(cb?: (err?: Error) => void): void;
    handleUpgrade(request: http.IncomingMessage, socket: Duplex, upgradeHead: [35mBuffer[36m, callback: (client: T, request: http.IncomingMessage) => void): void;
    shouldHandle(request: http.IncomingMessage): boolean | Promise<boolean>;[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:2126:61
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m   */
  assetsInlineLimit?: number | ((filePath: string, content: [35mBuffer[36m) => boolean | undefined);
  /**[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32mnode_modules\vite\dist\node\index.d.ts[39m:3972:89
[31mError[39m: Cannot find name 'Buffer'. Do you need to install type definitions for node? Try `npm i --save-dev @types/node` and then add 'node' to the types field in your tsconfig. (ts)
[36m}
declare function send(req: http.IncomingMessage, res: ServerResponse, content: string | [35mBuffer[36m, type: string, options: SendOptions): void;
//#endregion[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32msrc\lib\pet\bubble.ts[39m:15:3
[31mError[39m: Object literal may only specify known properties, and 'recovering' does not exist in type 'Record<"degraded" | "error", string>'. (ts)
[36m  degraded: '我還在，只是有些功能暫時休息。',
  [35mrecovering[36m: '等我一下，我正在恢復。',
  error: '我遇到一點問題，但 Pet Runtime 不會假裝沒事。',[39m

d:\a\NorthPalace-my-pet\NorthPalace-my-pet\[32msrc\main.ts[39m:1:8
[31mError[39m: Cannot find module or type declarations for side-effect import of './app.css'. (ts)
[36mimport [35m'./app.css'[36m;
import App from './App.svelte';[39m

====================================
[31msvelte-check found 225 errors and 5 warnings in 12 files
[39m
``

## Build
``text

> northpalace-my-pet@0.1.0 build
> npm run validate:assets && vite build


> northpalace-my-pet@0.1.0 validate:assets
> node scripts/validate-lenvu-assets.mjs

[Lenvu asset contract] OK — 14 animation profiles, 1 reference asset(s), sourcePixels=measured+remapped, candidate=awaiting_source_faithful_candidate_artwork, productionReady=false
5:10:26 AM [vite-plugin-svelte] no Svelte config found at D:/a/NorthPalace-my-pet/NorthPalace-my-pet - using default configuration.
[36mvite v8.2.0 [32mbuilding client environment for production...[36m[39m
[2K
5:10:26 AM [vite-plugin-svelte] src/lib/ui/CompanionView.svelte:120:4 Non-interactive element `<nav>` cannot have interactive role 'tablist'
https://svelte.dev/e/a11y_no_noninteractive_element_to_interactive_role
118:     </section>
119: 
120:     <nav class="companion-tabs" role="tablist" aria-label="Companion 功能">
               ^
121:       <button
122:         role="tab"
5:10:26 AM [vite-plugin-svelte] src/lib/ui/companion/SettingsSection.svelte:59:0 Non-interactive element `<section>` cannot have interactive role 'tabpanel'
https://svelte.dev/e/a11y_no_noninteractive_element_to_interactive_role
57: </script>
58: 
59: <section class="companion-section" role="tabpanel" aria-label="Settings">
              ^
60:   <section class="settings-panel">
61:     <div class="section-heading">
5:10:26 AM [vite-plugin-svelte] src/lib/ui/companion/HomeSection.svelte:13:0 Non-interactive element `<section>` cannot have interactive role 'tabpanel'
https://svelte.dev/e/a11y_no_noninteractive_element_to_interactive_role
11: </script>
12: 
13: <section class="companion-section home-section" role="tabpanel" aria-label="Home">
              ^
14:   <div class="section-heading section-heading--compact">
15:     <div>
5:10:27 AM [vite-plugin-svelte] src/lib/ui/companion/MemorySection.svelte:156:0 Non-interactive element `<section>` cannot have interactive role 'tabpanel'
https://svelte.dev/e/a11y_no_noninteractive_element_to_interactive_role
154: </script>
155: 
156: <section class="companion-section" role="tabpanel" aria-label="Memory">
               ^
157:   <section class="memory-panel">
158:     <div class="section-heading">
5:10:27 AM [vite-plugin-svelte] src/lib/ui/companion/ActivitySection.svelte:71:0 Non-interactive element `<section>` cannot have interactive role 'tabpanel'
https://svelte.dev/e/a11y_no_noninteractive_element_to_interactive_role
69: </script>
70: 
71: <section class="companion-section" role="tabpanel" aria-label="Activity">
              ^
72:   <section class="activity-panel">
73:     <div class="section-heading">
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
dist/assets/index-By8D--0g.js                     209.77 kB │ gzip: 64.63 kB │ map: 1,095.94 kB

[32m✓ built in 764ms[39m
``
