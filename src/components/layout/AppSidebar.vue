<script setup lang="ts">
import pkg from "../../../package.json";
import type { Tab } from "../../types/cleaner";

defineProps<{
  activeTab: Tab;
  isCMenuOpen: boolean;
  isBrowserMenuOpen: boolean;
}>();

const emit = defineEmits<{
  "update:activeTab": [tab: Tab];
  "toggle-c-menu": [];
  "toggle-browser-menu": [];
}>();
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2 class="brand">Windows 清理工具</h2>
    </div>

    <nav class="sidebar-nav">
      <div class="nav-group">
        <div class="nav-item-header" @click="emit('toggle-c-menu')">
          <span class="icon svg-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="20" height="8" rx="2" ry="2"/><rect x="2" y="14" width="20" height="8" rx="2" ry="2"/><line x1="6" y1="6" x2="6.01" y2="6"/><line x1="6" y1="18" x2="6.01" y2="18"/></svg>
          </span>
          <span class="label">清理 C 盘</span>
          <span class="arrow" :class="{ open: isCMenuOpen }">▾</span>
        </div>
        <div class="nav-sub-items" v-show="isCMenuOpen">
          <div class="nav-sub-item" :class="{ active: activeTab === 'clean-c-fast' }" @click="emit('update:activeTab', 'clean-c-fast')">
            快速模式
          </div>
          <div class="nav-sub-item" :class="{ active: activeTab === 'clean-c-advanced' }" @click="emit('update:activeTab', 'clean-c-advanced')">
            高级模式
          </div>
          <div class="nav-sub-item" :class="{ active: activeTab === 'clean-c-deep' }" @click="emit('update:activeTab', 'clean-c-deep')">
            查找大目录
          </div>
        </div>
      </div>

      <div class="nav-group">
        <div class="nav-item-header" @click="emit('toggle-browser-menu')">
          <span class="icon svg-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="16" rx="2"/><line x1="3" y1="10" x2="21" y2="10"/><path d="M7 7h.01"/><path d="M11 7h.01"/></svg>
          </span>
          <span class="label">清理浏览器</span>
          <span class="arrow" :class="{ open: isBrowserMenuOpen }">▾</span>
        </div>
        <div class="nav-sub-items" v-show="isBrowserMenuOpen">
          <div class="nav-sub-item" :class="{ active: activeTab === 'clean-browser-chrome' }" @click="emit('update:activeTab', 'clean-browser-chrome')">
            谷歌浏览器
          </div>
          <div class="nav-sub-item" :class="{ active: activeTab === 'clean-browser-edge' }" @click="emit('update:activeTab', 'clean-browser-edge')">
            微软浏览器
          </div>
        </div>
      </div>

      <div class="nav-item" :class="{ active: activeTab === 'clean-memory' }" @click="emit('update:activeTab', 'clean-memory')">
        <span class="icon svg-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
        </span>
        <span class="label">清理内存</span>
      </div>
    </nav>

    <div class="sidebar-footer">
      <span class="version">v{{ pkg.version }}</span>
    </div>
  </aside>
</template>
