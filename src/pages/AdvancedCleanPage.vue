<script setup lang="ts">
import { useAdvancedClean } from "../composables/useAdvancedClean";
import type { AlertOptions } from "../types/cleaner";

const props = defineProps<{
  showAlert: (options: AlertOptions) => void;
}>();

const { expandedAdvanced, loading, runTask } = useAdvancedClean(props.showAlert);
</script>

<template>
  <section class="page-container">
    <div class="page-header">
      <div class="header-info">
        <h1>高级清理工具</h1>
        <p>针对特定系统区域执行清理，但都有注意事项和副作用，在不理解的情况下慎点。</p>
      </div>
    </div>

    <div class="adv-card-list">
      <div class="adv-card" :class="{ expanded: expandedAdvanced === 'dism' }">
        <div class="adv-card-main" @click="expandedAdvanced = expandedAdvanced === 'dism' ? null : 'dism'">
          <div class="adv-card-info">
            <span class="adv-card-icon">⚙️</span>
            <div class="adv-card-text">
              <h3>系统组件清理 <small class="detail-hint">(点击查看详情)</small></h3>
              <p>通过 DISM 命令移除不再需要的系统冗余组件。</p>
            </div>
          </div>
          <div class="adv-card-right">
            <span class="expand-icon" :class="{ rotated: expandedAdvanced === 'dism' }">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
            </span>
            <button class="btn-action" :disabled="loading.dism" @click.stop="runTask('dism')">
              {{ loading.dism ? "执行中..." : "执行" }}
            </button>
          </div>
        </div>
        <div v-show="expandedAdvanced === 'dism'" class="adv-card-detail">
          <div class="detail-content">
            <h4>详细信息：</h4>
            <p>Windows 在更新后会保留旧版本的组件。此操作会调用系统底层的 DISM 工具（StartComponentCleanup）进行物理移除。</p>
            <h4 class="warning-title">注意事项：</h4>
            <ul>
              <li>执行后将无法卸载已安装的 Windows 更新。</li>
              <li>过程可能较慢（需 1-5 分钟），请勿中途关闭程序。</li>
            </ul>
          </div>
        </div>
      </div>

      <div class="adv-card" :class="{ expanded: expandedAdvanced === 'thumb' }">
        <div class="adv-card-main" @click="expandedAdvanced = expandedAdvanced === 'thumb' ? null : 'thumb'">
          <div class="adv-card-info">
            <span class="adv-card-icon">🖼️</span>
            <div class="adv-card-text">
              <h3>清理缩略图缓存 <small class="detail-hint">(点击查看详情)</small></h3>
              <p>重置文件夹预览缩略图数据库以释放空间。</p>
            </div>
          </div>
          <div class="adv-card-right">
            <span class="expand-icon" :class="{ rotated: expandedAdvanced === 'thumb' }">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
            </span>
            <button class="btn-action" :disabled="loading.thumb" @click.stop="runTask('thumb')">执行</button>
          </div>
        </div>
        <div v-show="expandedAdvanced === 'thumb'" class="adv-card-detail">
          <div class="detail-content">
            <h4>详细信息：</h4>
            <p>系统会自动生成图片和视频的缩略图缓存（thumbcache_*.db）。当缓存过大或出现显示错误时，建议清理。</p>
            <h4 class="warning-title">注意事项：</h4>
            <ul>
              <li>清理后，再次打开图片文件夹时加载预览会稍慢。</li>
              <li>部分文件正被资源管理器使用时可能无法彻底删除。</li>
            </ul>
          </div>
        </div>
      </div>

      <div class="adv-card" :class="{ expanded: expandedAdvanced === 'hiber' }">
        <div class="adv-card-main" @click="expandedAdvanced = expandedAdvanced === 'hiber' ? null : 'hiber'">
          <div class="adv-card-info">
            <span class="adv-card-icon">🌙</span>
            <div class="adv-card-text">
              <h3>关闭休眠文件 <small class="detail-hint">(点击查看详情)</small></h3>
              <p>永久删除 hiberfil.sys 文件（大小等同于内存）。</p>
            </div>
          </div>
          <div class="adv-card-right">
            <span class="expand-icon" :class="{ rotated: expandedAdvanced === 'hiber' }">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
            </span>
            <button class="btn-action" :disabled="loading.hiber" @click.stop="runTask('hiber')">执行</button>
          </div>
        </div>
        <div v-show="expandedAdvanced === 'hiber'" class="adv-card-detail">
          <div class="detail-content">
            <h4>详细信息：</h4>
            <p>休眠文件（hiberfil.sys）占用大量 C 盘空间。对于使用 SSD且不常用休眠功能的用户，关闭它可以释放巨额空间。</p>
            <h4 class="warning-title">注意事项：</h4>
            <ul>
              <li>关闭后将无法使用“休眠”功能及“快速启动”技术。</li>
              <li>只需执行一次。</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.adv-card-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.adv-card {
  background: #fff;
  border-radius: 20px;
  box-shadow: var(--card-shadow);
  border: 1px solid rgba(0, 0, 0, 0.02);
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.adv-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.06);
}

.adv-card-main {
  padding: 24px 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
}

.adv-card-main:hover {
  background: #fafafa;
}

.adv-card-info {
  display: flex;
  align-items: center;
  gap: 24px;
}

.adv-card-icon {
  font-size: 32px;
}

.adv-card-text h3 {
  font-size: 18px;
  margin-bottom: 4px;
  font-weight: 700;
  color: var(--text-main);
  display: flex;
  align-items: center;
  gap: 8px;
}

.adv-card-text p {
  color: var(--text-sec);
  font-size: 14px;
}

.detail-hint {
  font-size: 12px;
  color: var(--text-sec);
  font-weight: 400;
  opacity: 0.7;
}

.adv-card-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.expand-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #c1c1c1;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border-radius: 50%;
  background: #f8f9fa;
}

.expand-icon svg {
  width: 18px;
  height: 18px;
}

.expand-icon.rotated {
  transform: rotate(180deg);
  background: #ebf4ff;
  color: var(--primary-color);
}

.btn-action {
  background-color: #f2f2f7;
  color: var(--primary-color);
  border: none;
  padding: 10px 24px;
  border-radius: 10px;
  font-weight: 700;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 90px;
}

.btn-action:hover {
  background-color: var(--primary-color);
  color: #fff;
  transform: scale(1.05);
}

.btn-action:disabled {
  background-color: #e5e5e7;
  color: #a1a1a1;
  cursor: not-allowed;
  transform: none;
}

.adv-card-detail {
  padding: 0 32px 32px 88px;
  border-top: 1px solid #f5f5f7;
  background: #fcfcfd;
}

.detail-content {
  padding-top: 24px;
}

.detail-content h4 {
  font-size: 14px;
  margin-bottom: 10px;
  color: var(--text-main);
  font-weight: 700;
}

.detail-content p {
  font-size: 14px;
  color: var(--text-sec);
  line-height: 1.6;
  margin-bottom: 16px;
}

.warning-title {
  color: #ff9500 !important;
  margin-top: 20px;
}

.detail-content ul {
  padding-left: 18px;
  color: var(--text-sec);
  font-size: 13px;
}

.detail-content li {
  margin-bottom: 8px;
  line-height: 1.4;
}
</style>
