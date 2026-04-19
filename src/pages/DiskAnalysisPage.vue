<script setup lang="ts">
import { useDiskAnalysis } from "../composables/useDiskAnalysis";
import type { AlertOptions, FileNode } from "../types/cleaner";

const props = defineProps<{
  showAlert: (options: AlertOptions) => void;
}>();

const emit = defineEmits<{
  "open-context-menu": [event: MouseEvent, node: FileNode];
}>();

const { isFullScanning, fullScanProgress, treeData, startFullDiskScan, toggleNode } =
  useDiskAnalysis(props.showAlert);
</script>

<template>
  <section class="page-container full-width">
    <div class="page-header">
      <div class="header-info">
        <h1>查找大目录</h1>
        <p>查看 C 盘目录大小，适合技术人员细节分析空间占用情况。</p>
      </div>
      <div class="header-actions">
        <button class="btn-primary btn-sm" :disabled="isFullScanning" @click="startFullDiskScan">
          {{ isFullScanning ? "正在扫描..." : "开始扫描" }}
        </button>
      </div>
    </div>

    <div v-if="treeData.length > 0 || isFullScanning" class="tree-table-container shadow-card">
      <div v-if="isFullScanning" class="scanning-overlay">
        <div class="spinner"></div>
        <div class="scanning-status">
          <p class="scanning-main-text">正在扫描 C 盘文件...</p>
          <div class="scanning-stats-row">
            <span class="stat-badge">已扫描：{{ fullScanProgress.fileCount.toLocaleString() }} 个文件</span>
          </div>
          <p v-if="fullScanProgress.currentPath" class="scanning-current-path">
            当前：{{ fullScanProgress.currentPath }}
          </p>
        </div>
      </div>

      <div v-else class="tree-content-wrapper">
        <div class="tree-header">
          <span class="col-name">文件/文件夹名称</span>
          <span class="col-size">大小</span>
          <span class="col-graph">相对于父目录占比</span>
        </div>
        <div class="tree-body">
          <div
            v-for="(node, index) in treeData"
            :key="node.path"
            class="tree-row"
            :class="{ 'is-file': !node.is_dir }"
            :style="{ paddingLeft: `${node.level * 20 + 16}px` }"
            @contextmenu="emit('open-context-menu', $event, node)"
          >
            <div class="col-name" @click="toggleNode(index)">
              <span v-if="node.is_dir" class="node-toggle">
                {{ node.isLoading ? "⌛" : node.isOpen ? "▼" : "▶" }}
              </span>
              <span v-else class="node-icon svg-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg>
              </span>
              <span class="node-text">{{ node.name }}</span>
            </div>
            <div class="col-size">{{ node.size_str }}</div>
            <div class="col-graph">
              <div class="mini-bar-bg">
                <div class="mini-bar-fill" :style="{ width: `${node.percent}%` }"></div>
              </div>
              <span class="percent-text">{{ Math.round(node.percent) }}%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.tree-table-container {
  background: #fff;
  border-radius: 24px;
  overflow: hidden;
  margin-top: 8px;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  box-shadow: var(--card-shadow);
  border: 1px solid rgba(0, 0, 0, 0.02);
}

.tree-content-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.tree-header {
  display: flex;
  background: #f9fafb;
  padding: 16px 24px;
  font-size: 12px;
  font-weight: 700;
  color: var(--text-sec);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.tree-body {
  flex: 1;
  overflow-y: auto;
}

.tree-row {
  display: flex;
  align-items: center;
  padding: 14px 24px;
  border-bottom: 1px solid #f5f5f7;
  font-size: 14px;
  transition: background 0.15s ease;
}

.tree-row:hover {
  background: #f9f9fb;
}

.tree-row.is-file {
  color: #424245;
}

.col-name {
  flex: 2;
  display: flex;
  align-items: center;
  font-weight: 500;
  min-width: 0;
}

.node-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.col-size {
  width: 100px;
  text-align: right;
  font-weight: 600;
  color: var(--text-main);
  flex-shrink: 0;
}

.col-graph {
  width: 180px;
  display: flex;
  align-items: center;
  gap: 12px;
  padding-left: 32px;
  flex-shrink: 0;
}

.mini-bar-bg {
  flex: 1;
  height: 6px;
  background: #f0f0f2;
  border-radius: 3px;
  overflow: hidden;
}

.mini-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #007aff, #5856d6);
  border-radius: 3px;
  transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
}

.percent-text {
  font-size: 11px;
  color: var(--text-sec);
  width: 32px;
  font-weight: 600;
  text-align: right;
}

.node-toggle {
  width: 24px;
  cursor: pointer;
  color: #c1c1c1;
  display: inline-block;
  text-align: center;
  font-size: 10px;
  transition: color 0.2s;
}

.node-toggle:hover {
  color: var(--primary-color);
}

.node-icon {
  width: 24px;
  font-size: 14px;
  opacity: 0.7;
}
</style>
