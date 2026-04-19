import { ref } from "vue";
import {
  cleanSystemComponents,
  cleanThumbnails,
  disableHibernation,
} from "../services/tauri/cleaner";
import type { AlertOptions } from "../types/cleaner";

export function useAdvancedClean(showAlert: (options: AlertOptions) => void) {
  const expandedAdvanced = ref<string | null>(null);
  const loading = ref<Record<string, boolean>>({});

  async function runTask(task: string) {
    loading.value[task] = true;

    try {
      let title = "";
      let result = "";

      if (task === "dism") {
        title = "系统组件清理";
        result = await cleanSystemComponents();
      } else if (task === "thumb") {
        title = "缩略图清理";
        result = await cleanThumbnails();
      } else if (task === "hiber") {
        title = "休眠文件优化";
        result = await disableHibernation();
      }

      showAlert({ title, message: result, type: "success" });
    } catch (err) {
      showAlert({
        title: "任务失败",
        message: String(err),
        type: "error",
      });
    } finally {
      loading.value[task] = false;
    }
  }

  return {
    expandedAdvanced,
    loading,
    runTask,
  };
}
