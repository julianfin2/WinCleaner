<script setup lang="ts">
import { useMemoryClean } from "../composables/useMemoryClean";
import type { AlertOptions } from "../types/cleaner";
import { formatItemSize } from "../utils/format";

const props = defineProps<{
  showAlert: (options: AlertOptions) => void;
}>();

const { state, startClean } = useMemoryClean(props.showAlert);
</script>

<template>
  <section class="page-container memory-page-spread">
    <div class="page-header">
      <div class="header-info">
        <h1>清理内存</h1>
        <p>释放内存占用，不影响程序运行。但释放内存后重新打开之前的软件，会感到略微卡顿。</p>
      </div>
    </div>

    <div class="memory-layout-v2">
      <div class="memory-main-card shadow-card">
        <div class="gauge-section">
          <div class="memory-gauge" :style="{ '--percent': state.stats?.percent || 0 }">
            <svg viewBox="0 0 100 100">
              <circle class="gauge-bg" cx="50" cy="50" r="45"></circle>
              <circle class="gauge-fill" cx="50" cy="50" r="45" :style="{ strokeDashoffset: 283 - (283 * (state.stats?.percent || 0)) / 100 }"></circle>
            </svg>
            <div class="gauge-content">
              <span class="gauge-value">{{ Math.round(state.stats?.percent || 0) }}<small>%</small></span>
              <span class="gauge-label">内存占用</span>
            </div>
          </div>
        </div>

        <div class="stats-section">
          <div class="stat-box-v2">
            <span class="label">已用内存</span>
            <span class="value">{{ formatItemSize(state.stats?.used || 0) }}</span>
          </div>
          <div class="stat-divider-h"></div>
          <div class="stat-box-v2">
            <span class="label">可用内存</span>
            <span class="value">{{ formatItemSize(state.stats?.free || 0) }}</span>
          </div>
          <div class="stat-divider-h"></div>
          <div class="stat-box-v2">
            <span class="label">内存总量</span>
            <span class="value">{{ formatItemSize(state.stats?.total || 0) }}</span>
          </div>
        </div>
      </div>

      <div class="memory-actions-v2">
        <div class="action-card shadow-card" :class="{ cleaning: state.isCleaning }">
          <div class="action-info">
            <h3>普通加速</h3>
            <p>建议在需要开启更多软件，但内存占用居高不下时使用。</p>
          </div>
          <button class="btn-primary" :disabled="state.isCleaning" @click="startClean(false)">
            {{ state.cleaningType === "fast" ? "清理中..." : "立即加速" }}
          </button>
        </div>

        <div class="action-card shadow-card secondary" :class="{ cleaning: state.isCleaning }">
          <div class="action-info">
            <h3>深度加速</h3>
            <p>可以在长时间使用电脑后，感觉电脑有点卡顿时执行。</p>
          </div>
          <button class="btn-secondary" :disabled="state.isCleaning" @click="startClean(true)">
            {{ state.cleaningType === "deep" ? "清理中..." : "深度加速" }}
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.memory-layout-v2 {
  display: flex;
  flex-direction: column;
  gap: 32px;
  margin-top: 24px;
}

.memory-main-card {
  background: white;
  border-radius: 32px;
  padding: 48px;
  display: flex;
  align-items: center;
  gap: 60px;
  box-shadow: var(--card-shadow);
}

.gauge-section {
  flex: 0 0 auto;
}

.memory-gauge {
  position: relative;
  width: 240px;
  height: 240px;
}

.memory-gauge svg {
  transform: rotate(-90deg);
  width: 100%;
  height: 100%;
}

.gauge-bg {
  fill: none;
  stroke: #f2f2f7;
  stroke-width: 8;
}

.gauge-fill {
  fill: none;
  stroke: var(--primary-color);
  stroke-width: 8;
  stroke-linecap: round;
  stroke-dasharray: 283;
  transition: stroke-dashoffset 1s cubic-bezier(0.4, 0, 0.2, 1), stroke 0.3s;
}

.gauge-content {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  display: flex;
  flex-direction: column;
}

.gauge-value {
  font-size: 64px;
  font-weight: 800;
  color: var(--text-main);
  line-height: 1;
  letter-spacing: -2px;
}

.gauge-value small {
  font-size: 24px;
  margin-left: 2px;
  letter-spacing: 0;
}

.gauge-label {
  font-size: 14px;
  color: var(--text-sec);
  font-weight: 600;
  margin-top: 4px;
}

.stats-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.stat-box-v2 {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.stat-box-v2 .label {
  font-size: 15px;
  color: var(--text-sec);
  font-weight: 500;
}

.stat-box-v2 .value {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-main);
}

.stat-divider-h {
  height: 1px;
  background: #f2f2f7;
  width: 100%;
}

.memory-actions-v2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

.action-card {
  background: white;
  padding: 32px;
  border-radius: 28px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.action-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.06);
}

.action-card.secondary {
  background-color: #fbfbfd;
  border: 1px dashed var(--border-color);
  box-shadow: none;
}

.action-info {
  flex: 1;
}

.action-info h3 {
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 8px;
  color: var(--text-main);
}

.action-info p {
  font-size: 13px;
  color: var(--text-sec);
  line-height: 1.5;
}

.action-card :deep(.btn-primary),
.action-card :deep(.btn-secondary) {
  width: 100%;
  padding: 14px;
}

.action-card.cleaning {
  filter: grayscale(1);
  opacity: 0.7;
  pointer-events: none;
}
</style>
