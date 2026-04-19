import { computed, ref } from "vue";
import {
  startFastClean as runFastCleanCommand,
  startFastScan as runFastScanCommand,
  subscribeFastCleanProgress,
} from "../services/tauri/cleaner";
import type {
  AlertOptions,
  CleanResult,
  ConfirmOptions,
  FastScanResult,
  ProjectCleanProgressPayload,
} from "../types/cleaner";
import { formatItemSize } from "../utils/format";

interface FastState {
  isScanning: boolean;
  isCleaning: boolean;
  isDone: boolean;
  progress: number;
  scanResult: FastScanResult | null;
  cleanResult: CleanResult | null;
}

interface FastCleanProgressState {
  completedItems: number;
  totalItems: number;
  currentItem: string;
  approxCompletedBytes: number;
}

export function useFastClean(
  showAlert: (options: AlertOptions) => void,
  requestConfirm: (options: ConfirmOptions) => Promise<boolean>,
) {
  const state = ref<FastState>({
    isScanning: false,
    isCleaning: false,
    isDone: false,
    progress: 0,
    scanResult: null,
    cleanResult: null,
  });
  const cleanProgress = ref<FastCleanProgressState>({
    completedItems: 0,
    totalItems: 0,
    currentItem: "",
    approxCompletedBytes: 0,
  });

  const selectedStats = computed(() => {
    const scanResult = state.value.scanResult;
    if (!scanResult) return { totalBytes: 0, sizeStr: "0 B", count: 0, hasSelection: false };

    const enabledItems = scanResult.items.filter((item) => item.enabled);
    const totalBytes = enabledItems.reduce((acc, item) => acc + item.size, 0);
    const totalCount = enabledItems.reduce((acc, item) => acc + item.count, 0);

    return {
      totalBytes,
      sizeStr: formatItemSize(totalBytes),
      count: totalCount,
      hasSelection: enabledItems.length > 0,
    };
  });
  const cleanProgressSizeStr = computed(() => formatItemSize(cleanProgress.value.approxCompletedBytes));

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
    current.progress = 0;
    current.scanResult = null;

    const interval = window.setInterval(() => {
      if (current.progress < 95) {
        current.progress += Math.floor(Math.random() * 5);
      }
    }, 100);

    try {
      current.scanResult = await runFastScanCommand();
      current.progress = 100;
    } catch {
      showAlert({
        title: "扫描失败",
        message: "请尝试以管理员身份运行程序。",
        type: "error",
      });
    } finally {
      window.clearInterval(interval);
      current.isScanning = false;
    }
  }

  async function startClean() {
    const current = state.value;
    if (current.isCleaning || !current.scanResult) return;

    const selectedPaths = current.scanResult.items
      .filter((item) => item.enabled)
      .map((item) => item.path);

    if (selectedPaths.length === 0) {
      showAlert({
        title: "未选择任何项",
        message: "请至少勾选一个需要清理的项目。",
        type: "info",
      });
      return;
    }

    if (selectedPaths.includes("C:\\$Recycle.Bin")) {
      const confirmed = await requestConfirm({
        title: "确认清空回收站",
        message: "当前勾选项包含回收站。\n\n清空后，回收站中的文件将被永久删除，通常无法直接恢复。\n\n是否继续清理？",
        type: "info",
        confirmText: "继续清理",
        cancelText: "返回检查",
      });

      if (!confirmed) {
        return;
      }
    }

    resetCleanProgress();
    current.isCleaning = true;
    const unlisten = await subscribeFastCleanProgress(handleCleanProgress);
    try {
      current.cleanResult = await runFastCleanCommand(selectedPaths);
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

  function reset() {
    state.value = {
      isScanning: false,
      isCleaning: false,
      isDone: false,
      progress: 0,
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
    reset,
  };
}
