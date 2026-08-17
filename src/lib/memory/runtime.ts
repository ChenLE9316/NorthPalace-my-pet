import { invoke } from '@tauri-apps/api/core';

export type MemoryKind = 'episodic' | 'semantic' | 'preference' | 'relationship';

export interface MemoryInput {
  kind: MemoryKind;
  content: string;
  importance: number;
}

export interface MemoryRecord extends MemoryInput {
  id: number;
  createdAtMs: number;
  updatedAtMs: number;
  sourceEventId: number | null;
}

export async function listMemories(kind: MemoryKind | null = null, limit = 50): Promise<MemoryRecord[]> {
  return invoke<MemoryRecord[]>('memory_list', { kind, limit });
}

export async function searchMemories(query: string, limit = 50): Promise<MemoryRecord[]> {
  return invoke<MemoryRecord[]>('memory_search', { query, limit });
}

export async function createMemory(input: MemoryInput): Promise<number> {
  return invoke<number>('memory_create', { input });
}

export async function updateMemory(id: number, input: MemoryInput): Promise<void> {
  await invoke('memory_update', { id, input });
}

export async function deleteMemory(id: number): Promise<void> {
  await invoke('memory_delete', { id });
}
