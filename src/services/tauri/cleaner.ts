import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl } from "@tauri-apps/plugin-opener";
import type {
  BrowserScanResult,
  CleanResult,
  FastScanResult,
  FileNode,
  MemoryStats,
  ProjectCleanProgressPayload,
  ScanProgressPayload,
} from "../../types/cleaner";

export function startFastScan() {
  return invoke<FastScanResult>("start_fast_scan");
}

export function startFastClean(selectedPaths: string[]) {
  return invoke<CleanResult>("start_fast_clean", { selectedPaths });
}

export function cleanSystemComponents() {
  return invoke<string>("clean_system_components");
}

export function cleanThumbnails() {
  return invoke<string>("clean_thumbnails");
}

export function disableHibernation() {
  return invoke<string>("disable_hibernation");
}

export function startBrowserScan(browser: "chrome" | "edge") {
  return invoke<BrowserScanResult>("start_browser_scan", { browser });
}

export function startBrowserClean(browser: "chrome" | "edge", profiles: string[]) {
  return invoke<CleanResult>("start_browser_clean", { browser, profiles });
}

export function startFullDiskScan() {
  return invoke("start_full_disk_scan");
}

export function getTreeChildren(path: string) {
  return invoke<FileNode[]>("get_tree_children", { path });
}

export function subscribeScanProgress(
  handler: (payload: ScanProgressPayload) => void,
) {
  return listen<ScanProgressPayload>("scan-progress", (event) => {
    handler(event.payload);
  });
}

export function subscribeFastCleanProgress(
  handler: (payload: ProjectCleanProgressPayload) => void,
) {
  return listen<ProjectCleanProgressPayload>("fast-clean-progress", (event) => {
    handler(event.payload);
  });
}

export function subscribeBrowserCleanProgress(
  handler: (payload: ProjectCleanProgressPayload) => void,
) {
  return listen<ProjectCleanProgressPayload>("browser-clean-progress", (event) => {
    handler(event.payload);
  });
}

export function openInExplorer(path: string) {
  return invoke("open_in_explorer", { path });
}

export function openSearch(query: string, provider: "google" | "perplexity") {
  const encoded = encodeURIComponent(query);
  const url =
    provider === "google"
      ? `https://www.google.com/search?q=${encoded}`
      : `https://www.perplexity.ai/?q=${encoded}`;

  return openUrl(url);
}

export function getMemoryStats() {
  return invoke<MemoryStats>("get_memory_stats");
}

export function runMemoryClean() {
  return invoke<number>("run_memory_clean");
}

export function runDeepMemoryClean() {
  return invoke<number>("run_deep_memory_clean");
}
