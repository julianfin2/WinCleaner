<script setup lang="ts">
import { useFastClean } from "../composables/useFastClean";
import type { AlertOptions, ConfirmOptions } from "../types/cleaner";
import { splitSize, formatItemSize } from "../utils/format";

const props = defineProps<{
  showAlert: (options: AlertOptions) => void;
  requestConfirm: (options: ConfirmOptions) => Promise<boolean>;
}>();

const { state, selectedStats, cleanProgress, cleanProgressSizeStr, startScan, startClean, reset } = useFastClean(
  props.showAlert,
  props.requestConfirm,
);
</script>

<template>
  <section class="page-container">
    <div class="page-header">
      <div class="header-info">
        <h1>清理系统盘</h1>
        <p>快速清理 C 盘缓存，不影响系统运行。</p>
      </div>
    </div>

    <div class="main-action">
      <div v-if="!state.scanResult && !state.isDone" class="scan-circle-container">
        <div class="scan-circle" :class="{ scanning: state.isScanning }">
          <div class="scan-inner" @click="!state.isScanning && startScan()">
            <span v-if="!state.isScanning" class="scan-btn-text">开始扫描</span>
            <span v-else class="scan-percent">{{ state.progress }}%</span>
          </div>
        </div>
      </div>

      <div v-else-if="state.scanResult && !state.isDone" class="result-card">
        <div class="result-header">
          <span class="result-icon">{{ state.isCleaning ? "🧹" : "📋" }}</span>
          <h2>{{ state.isCleaning ? "正在清理" : "扫描完成" }}</h2>
        </div>
        <div class="result-stats">
          <div class="stat-item">
            <span class="stat-value">
              {{ splitSize(state.isCleaning ? cleanProgressSizeStr : selectedStats.sizeStr).value }}
              <span class="unit">{{ splitSize(state.isCleaning ? cleanProgressSizeStr : selectedStats.sizeStr).unit }}</span>
            </span>
            <span class="stat-label">{{ state.isCleaning ? "已处理约" : "预计释放" }}</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-value">{{ state.isCleaning ? `${cleanProgress.completedItems}/${cleanProgress.totalItems}` : selectedStats.count }}</span>
            <span class="stat-label">{{ state.isCleaning ? "已完成项目" : "文件数量" }}</span>
          </div>
        </div>

        <p v-if="state.isCleaning" class="cleaning-note">
          正在清理：{{ cleanProgress.currentItem || "准备开始..." }}，请稍候，不要关闭程序。
        </p>

        <button class="btn-primary main-btn" :disabled="state.isCleaning || !selectedStats.hasSelection" @click="startClean">
          {{ state.isCleaning ? "正在清理..." : "立即清理" }}
        </button>
      </div>

      <div v-else-if="state.isDone && state.cleanResult" class="result-card done-card">
        <div class="result-header">
          <span class="result-icon success">🎉</span>
          <h2>清理完成</h2>
        </div>

        <div class="result-stats">
          <div class="stat-item">
            <span class="stat-value">
              {{ splitSize(state.cleanResult.total_freed).value }}
              <span class="unit">{{ splitSize(state.cleanResult.total_freed).unit }}</span>
            </span>
            <span class="stat-label">释放空间</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-value">{{ state.cleanResult.success_count }}</span>
            <span class="stat-label">成功清理</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-value highlight-gray">{{ state.cleanResult.fail_count }}</span>
            <span class="stat-label">跳过/失败</span>
          </div>
        </div>
        <button class="btn-secondary" @click="reset">返回</button>
      </div>
    </div>

    <div v-if="(state.isScanning || state.scanResult) && !state.isDone && !state.isCleaning" class="detail-list">
      <h3>清理项详情</h3>
      <div
        v-for="item in state.scanResult?.items || []"
        :key="item.path"
        class="detail-item"
        :class="{ disabled: !item.enabled }"
        @click="item.enabled = !item.enabled"
      >
        <div class="item-info">
          <label class="checkbox-container" @click.stop>
            <input v-model="item.enabled" type="checkbox">
            <span class="checkmark"></span>
          </label>
          <span>{{ item.name }}</span>
        </div>
        <span class="item-size">{{ formatItemSize(item.size) }}</span>
      </div>
      <div v-if="state.isScanning" class="scanning-placeholder">正在深度扫描文件系统...</div>
    </div>
  </section>
</template>
