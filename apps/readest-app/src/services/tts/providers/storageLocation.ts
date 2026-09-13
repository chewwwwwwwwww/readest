import { isTauriAppPlatform } from '@/services/environment';
import type { BaseDir } from '@/types/system';

// Native preparation migrates the complete SQLite/pack tree before opening
// any database and excludes downloadable media from iOS backups.
export const storageLocation = async (): Promise<{ root: string; base: BaseDir }> => {
  if (!isTauriAppPlatform()) return { root: 'tts-cache', base: 'Cache' };
  const { invoke } = await import('@tauri-apps/api/core');
  return { root: await invoke<string>('prepare_tts_storage'), base: 'None' };
};
