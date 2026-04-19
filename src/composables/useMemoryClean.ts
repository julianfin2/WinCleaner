import { onMounted, onUnmounted, ref } from "vue";
import {
  getMemoryStats as fetchMemoryStats,
  runDeepMemoryClean,
  runMemoryClean,
} from "../services/tauri/cleaner";
import type { AlertOptions, MemoryStats } from "../types/cleaner";
import { formatItemSize } from "../utils/format";

interface MemoryState {
  stats: MemoryStats | null;
  isCleaning: boolean;
  cleaningType: "fast" | "deep" | null;
  lastFreed: string;
  isDone: boolean;
}

export function useMemoryClean(showAlert: (options: AlertOptions) => void) {
  const state = ref<MemoryState>({
    stats: null,
    isCleaning: false,
    cleaningType: null,
    lastFreed: "",
    isDone: false,
  });

  let memoryInterval: number | null = null;

  async function getStats() {
    try {
      state.value.stats = await fetchMemoryStats();
    } catch (err) {
      console.error("Failed to fetch memory stats", err);
    }
  }

  async function startClean(deep = false) {
    if (state.value.isCleaning) return;

    state.value.isCleaning = true;
    state.value.cleaningType = deep ? "deep" : "fast";

    try {
      const freedBytes = deep ? await runDeepMemoryClean() : await runMemoryClean();
      state.value.lastFreed = formatItemSize(freedBytes);
      showAlert({
        title: "优化完成",
        message: `已为您释放 ${state.value.lastFreed} 内存空间`,
        type: "success",
      });
      await getStats();
    } catch (err) {
      showAlert({
        title: "清理失败",
        message: String(err),
        type: "error",
      });
    } finally {
      state.value.isCleaning = false;
      state.value.cleaningType = null;
    }
  }

  onMounted(() => {
    void getStats();
    memoryInterval = window.setInterval(() => {
      void getStats();
    }, 3000);
  });

  onUnmounted(() => {
    if (memoryInterval) {
      window.clearInterval(memoryInterval);
    }
  });

  return {
    state,
    getStats,
    startClean,
  };
}
