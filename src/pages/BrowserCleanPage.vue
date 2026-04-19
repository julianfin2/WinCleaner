<script setup lang="ts">
import { useBrowserClean } from "../composables/useBrowserClean";
import type { AlertOptions, ConfirmOptions } from "../types/cleaner";
import { splitSize } from "../utils/format";

const props = defineProps<{
  browser: "chrome" | "edge";
  showAlert: (options: AlertOptions) => void;
  requestConfirm: (options: ConfirmOptions) => Promise<boolean>;
}>();

const { state, selectedStats, cleanProgress, cleanProgressSizeStr, startScan, startClean, toggleAllProfiles, invertProfiles, reset } =
  useBrowserClean(props.browser, props.showAlert, props.requestConfirm);

const browserName = props.browser === "chrome" ? "谷歌浏览器" : "微软浏览器";
</script>

<template>
  <section class="page-container">
    <div class="page-header">
      <div class="header-info">
        <h1>清理{{ browserName }}</h1>
        <p>安全清理浏览器缓存、临时文件等，不会删除账号和插件数据。注意，清理前需要关闭浏览器。</p>
      </div>
    </div>

    <div class="main-action">
      <div v-if="!state.scanResult && !state.isDone" class="scan-circle-container">
        <div class="scan-circle" :class="{ scanning: state.isScanning }">
          <div class="scan-inner" @click="!state.isScanning && startScan()">
            <span v-if="!state.isScanning" class="scan-btn-text">开始扫描</span>
            <span v-else class="spinner"></span>
          </div>
        </div>
      </div>

      <div v-else-if="state.scanResult && !state.isDone" class="result-card">
        <div class="result-header">
          <span class="result-icon">{{ state.isCleaning ? "🧹" : "🌍" }}</span>
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
            <span class="stat-label">{{ state.isCleaning ? "已完成资料" : "用户资料数量" }}</span>
          </div>
        </div>
        <p v-if="state.isCleaning" class="cleaning-note">
          正在清理：{{ cleanProgress.currentItem || "准备开始..." }}，建议保持浏览器关闭以减少文件占用。
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
      <div class="list-header">
        <h3>用户资料列表</h3>
        <div class="list-actions">
          <button class="btn-text" @click="toggleAllProfiles(true)">全选</button>
          <button class="btn-text" @click="toggleAllProfiles(false)">取消</button>
          <button class="btn-text" @click="invertProfiles()">反选</button>
        </div>
      </div>
      <div
        v-for="profile in state.scanResult?.profiles || []"
        :key="profile.path_name"
        class="detail-item"
        :class="{ disabled: !profile.enabled }"
        @click="profile.enabled = !profile.enabled"
      >
        <div class="item-info">
          <label class="checkbox-container" @click.stop>
            <input v-model="profile.enabled" type="checkbox">
            <span class="checkmark"></span>
          </label>
          <span>{{ profile.name }}</span>
        </div>
        <span class="item-size">{{ profile.cache_size_str }}</span>
      </div>
      <div v-if="state.isScanning" class="scanning-placeholder">正在定位并分析浏览器用户资料...</div>
    </div>
  </section>
</template>
