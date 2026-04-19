import { ref } from "vue";
import {
  getTreeChildren,
  startFullDiskScan as runFullDiskScanCommand,
  subscribeScanProgress,
} from "../services/tauri/cleaner";
import type { AlertOptions, FileNode } from "../types/cleaner";

export function useDiskAnalysis(showAlert: (options: AlertOptions) => void) {
  const isFullScanning = ref(false);
  const fullScanProgress = ref({ fileCount: 0, currentPath: "" });
  const treeData = ref<FileNode[]>([]);

  async function startFullDiskScan() {
    isFullScanning.value = true;
    treeData.value = [];
    fullScanProgress.value = { fileCount: 0, currentPath: "" };

    const unlisten = await subscribeScanProgress((payload) => {
      fullScanProgress.value.fileCount = payload.file_count;
      fullScanProgress.value.currentPath = payload.current_path;
    });

    try {
      await runFullDiskScanCommand();
      const rootChildren = await getTreeChildren("C:\\");
      treeData.value = rootChildren.map((node) => ({
        ...node,
        level: 0,
        isOpen: false,
        isLoading: false,
      }));
    } catch {
      showAlert({
        title: "扫描失败",
        message: "请确保以管理员身份运行程序。",
        type: "error",
      });
    } finally {
      isFullScanning.value = false;
      unlisten();
    }
  }

  async function toggleNode(index: number) {
    const node = treeData.value[index];
    if (!node?.is_dir || node.isLoading) return;

    if (node.isOpen) {
      let removeCount = 0;
      for (let i = index + 1; i < treeData.value.length; i += 1) {
        if (treeData.value[i].level > node.level) removeCount += 1;
        else break;
      }
      treeData.value.splice(index + 1, removeCount);
      node.isOpen = false;
      return;
    }

    node.isLoading = true;
    try {
      const children = await getTreeChildren(node.path);
      const mappedChildren = children.map((child) => ({
        ...child,
        level: node.level + 1,
        isOpen: false,
        isLoading: false,
      }));
      treeData.value.splice(index + 1, 0, ...mappedChildren);
      node.isOpen = true;
    } catch (err) {
      showAlert({
        title: "展开失败",
        message: String(err),
        type: "error",
      });
    } finally {
      node.isLoading = false;
    }
  }

  return {
    isFullScanning,
    fullScanProgress,
    treeData,
    startFullDiskScan,
    toggleNode,
  };
}
