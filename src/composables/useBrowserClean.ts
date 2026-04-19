import { computed, ref } from "vue";
import {
  startBrowserClean as runBrowserCleanCommand,
  startBrowserScan as runBrowserScanCommand,
  subscribeBrowserCleanProgress,
} from "../services/tauri/cleaner";
import type {
  AlertOptions,
  BrowserScanResult,
  CleanResult,
  ConfirmOptions,
  ProjectCleanProgressPayload,
} from "../types/cleaner";
import { formatItemSize } from "../utils/format";

interface BrowserState {
  isScanning: boolean;
  isCleaning: boolean;
  isDone: boolean;
  scanResult: BrowserScanResult | null;
  cleanResult: CleanResult | null;
}

interface BrowserCleanProgressState {
  completedItems: number;
  totalItems: number;
  currentItem: string;
  approxCompletedBytes: number;
}

export function useBrowserClean(
  browser: "chrome" | "edge",
  showAlert: (options: AlertOptions) => void,
  requestConfirm: (options: ConfirmOptions) => Promise<boolean>,
) {
  const state = ref<BrowserState>({
    isScanning: false,
    isCleaning: false,
    isDone: false,
    scanResult: null,
    cleanResult: null,
  });
  const cleanProgress = ref<BrowserCleanProgressState>({
    completedItems: 0,
    totalItems: 0,
    currentItem: "",
    approxCompletedBytes: 0,
  });

  const selectedStats = computed(() => {
    const scanResult = state.value.scanResult;
    if (!scanResult) return { totalBytes: 0, sizeStr: "0 B", count: 0, hasSelection: false };

    const enabledProfiles = scanResult.profiles.filter((profile) => profile.enabled);
    const totalBytes = enabledProfiles.reduce((acc, profile) => acc + profile.cache_size, 0);

    return {
      totalBytes,
      sizeStr: formatItemSize(totalBytes),
      count: enabledProfiles.length,
      hasSelection: enabledProfiles.length > 0,
    };
  });
  const cleanProgressSizeStr = computed(() => formatSizeValue(cleanProgress.value.approxCompletedBytes));

  function formatSizeValue(bytes: number) {
    return formatItemSize(bytes);
  }

  function resetCleanProgress() {
    cleanProgress.value = {
      completedItems: 0,
      totalItems: 0,
      currentItem: "",
      approxCompletedBytes: 0,
    };
  }

  function handleCleanProgress(payload: ProjectCleanProgressPayload) {
    cleanProgress.value = {
      completedItems: payload.completed_items,
      totalItems: payload.total_items,
      currentItem: payload.current_item,
      approxCompletedBytes: payload.approx_completed_bytes,
    };
  }

  async function startScan() {
    const current = state.value;
    current.isScanning = true;
    current.isDone = false;
    current.scanResult = null;
    current.cleanResult = null;

    try {
      const result = await runBrowserScanCommand(browser);
      current.scanResult = {
        ...result,
        profiles: result.profiles
          .map((profile) => ({ ...profile, enabled: true }))
          .sort((a, b) => b.cache_size - a.cache_size),
      };
    } catch (err) {
      showAlert({
        title: "扫描失败",
        message: String(err),
        type: "error",
      });
    } finally {
      current.isScanning = false;
    }
  }

  async function startClean() {
    const current = state.value;
    if (!current.scanResult || current.isCleaning) return;

    const selectedProfiles = current.scanResult.profiles
      .filter((profile) => profile.enabled)
      .map((profile) => profile.path_name);

    if (selectedProfiles.length === 0) {
      showAlert({
        title: "未选择",
        message: "请选择至少一个用户资料进行清理。",
        type: "info",
      });
      return;
    }

    const browserName = browser === "chrome" ? "谷歌浏览器" : "微软浏览器";
    const confirmed = await requestConfirm({
      title: "确认清理浏览器缓存",
      message: `即将清理 ${browserName} 的缓存和临时文件。\n\n建议先关闭浏览器，以避免部分文件被占用导致清理不完整。\n\n是否继续？`,
      type: "info",
      confirmText: "继续清理",
      cancelText: "暂不清理",
    });

    if (!confirmed) {
      return;
    }

    resetCleanProgress();
    current.isCleaning = true;
    const unlisten = await subscribeBrowserCleanProgress(handleCleanProgress);
    try {
      current.cleanResult = await runBrowserCleanCommand(browser, selectedProfiles);
      current.isDone = true;
      current.scanResult = null;
    } catch (err) {
      showAlert({
        title: "清理失败",
        message: String(err),
        type: "error",
      });
    } finally {
      unlisten();
      current.isCleaning = false;
    }
  }

  function toggleAllProfiles(enabled: boolean) {
    state.value.scanResult?.profiles.forEach((profile) => {
      profile.enabled = enabled;
    });
  }

  function invertProfiles() {
    state.value.scanResult?.profiles.forEach((profile) => {
      profile.enabled = !profile.enabled;
    });
  }

  function reset() {
    state.value = {
      isScanning: false,
      isCleaning: false,
      isDone: false,
      scanResult: null,
      cleanResult: null,
    };
    resetCleanProgress();
  }

  return {
    state,
    selectedStats,
    cleanProgress,
    cleanProgressSizeStr,
    startScan,
    startClean,
    toggleAllProfiles,
    invertProfiles,
    reset,
  };
}
