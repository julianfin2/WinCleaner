<script setup lang="ts">
import { ref } from "vue";
import AppModal from "./components/common/AppModal.vue";
import AppSidebar from "./components/layout/AppSidebar.vue";
import AdvancedCleanPage from "./pages/AdvancedCleanPage.vue";
import BrowserCleanPage from "./pages/BrowserCleanPage.vue";
import DiskAnalysisPage from "./pages/DiskAnalysisPage.vue";
import FastCleanPage from "./pages/FastCleanPage.vue";
import MemoryCleanPage from "./pages/MemoryCleanPage.vue";
import { openInExplorer, openSearch } from "./services/tauri/cleaner";
import type {
  AlertOptions,
  ConfirmOptions,
  FileNode,
  ModalMode,
  ModalType,
  Tab,
} from "./types/cleaner";

const activeTab = ref<Tab>("clean-c-fast");
const isCMenuOpen = ref(true);
const isBrowserMenuOpen = ref(true);

const showModal = ref(false);
const modalTitle = ref("");
const modalMessage = ref("");
const modalType = ref<ModalType>("info");
const modalMode = ref<ModalMode>("alert");
const modalConfirmText = ref("确定");
const modalCancelText = ref("取消");
let modalResolver: ((confirmed: boolean) => void) | null = null;

const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  node: null as FileNode | null,
});

function showAlert({ title, message, type = "info" }: AlertOptions) {
  modalMode.value = "alert";
  modalTitle.value = title;
  modalMessage.value = message;
  modalType.value = type;
  modalConfirmText.value = "确定";
  showModal.value = true;
}

function requestConfirm({
  title,
  message,
  type = "info",
  confirmText = "确定",
  cancelText = "取消",
}: ConfirmOptions) {
  if (modalResolver) {
    modalResolver(false);
    modalResolver = null;
  }

  modalMode.value = "confirm";
  modalTitle.value = title;
  modalMessage.value = message;
  modalType.value = type;
  modalConfirmText.value = confirmText;
  modalCancelText.value = cancelText;
  showModal.value = true;

  return new Promise<boolean>((resolve) => {
    modalResolver = resolve;
  });
}

function closeModal() {
  showModal.value = false;
  if (modalResolver) {
    modalResolver(false);
    modalResolver = null;
  }
}

function confirmModal() {
  showModal.value = false;
  if (modalResolver) {
    modalResolver(true);
    modalResolver = null;
  }
}

function handleContextMenu(event: MouseEvent, node: FileNode) {
  event.preventDefault();
  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    node,
  };

  const close = () => {
    contextMenu.value.show = false;
    window.removeEventListener("click", close);
  };

  window.addEventListener("click", close);
}

async function openNodeInExplorer() {
  if (!contextMenu.value.node) return;

  try {
    await openInExplorer(contextMenu.value.node.path);
  } catch (err) {
    console.error(err);
  }
}

async function searchNode(provider: "google" | "perplexity") {
  if (!contextMenu.value.node) return;

  const query = `Windows 文件或目录 ${contextMenu.value.node.name} 是做什么用的，我可以删除吗`;

  try {
    await openSearch(query, provider);
  } catch (err) {
    console.error(err);
  }
}
</script>

<template>
  <div class="app-container">
    <AppSidebar
      :active-tab="activeTab"
      :is-c-menu-open="isCMenuOpen"
      :is-browser-menu-open="isBrowserMenuOpen"
      @update:active-tab="activeTab = $event"
      @toggle-c-menu="isCMenuOpen = !isCMenuOpen"
      @toggle-browser-menu="isBrowserMenuOpen = !isBrowserMenuOpen"
    />

    <main class="content">
      <FastCleanPage
        v-if="activeTab === 'clean-c-fast'"
        :show-alert="showAlert"
        :request-confirm="requestConfirm"
      />
      <AdvancedCleanPage v-else-if="activeTab === 'clean-c-advanced'" :show-alert="showAlert" />
      <BrowserCleanPage
        v-else-if="activeTab === 'clean-browser-chrome'"
        browser="chrome"
        :show-alert="showAlert"
        :request-confirm="requestConfirm"
      />
      <BrowserCleanPage
        v-else-if="activeTab === 'clean-browser-edge'"
        browser="edge"
        :show-alert="showAlert"
        :request-confirm="requestConfirm"
      />
      <DiskAnalysisPage
        v-else-if="activeTab === 'clean-c-deep'"
        :show-alert="showAlert"
        @open-context-menu="handleContextMenu"
      />
      <MemoryCleanPage v-else-if="activeTab === 'clean-memory'" :show-alert="showAlert" />
      <section v-else class="placeholder-page">
        <div class="empty-state">
          <span class="empty-icon">🛠️</span>
          <h1>功能开发中</h1>
          <p>此模块正在逐步完善，敬请期待。</p>
        </div>
      </section>
    </main>

    <div
      v-if="contextMenu.show"
      class="context-menu shadow-card"
      :style="{ top: `${contextMenu.y}px`, left: `${contextMenu.x}px` }"
      @click.stop
    >
      <div class="menu-item" @click="openNodeInExplorer">
        <span class="menu-icon svg-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
        </span>
        <span>在文件夹中打开</span>
      </div>
      <div class="menu-divider"></div>
      <div class="menu-item" @click="searchNode('google')">
        <span class="menu-icon svg-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
        </span>
        <span>用 Google 搜索</span>
      </div>
      <div class="menu-item" @click="searchNode('perplexity')">
        <span class="menu-icon svg-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>
        </span>
        <span>询问 Perplexity</span>
      </div>
    </div>

    <AppModal
      :open="showModal"
      :title="modalTitle"
      :message="modalMessage"
      :type="modalType"
      :mode="modalMode"
      :confirm-text="modalConfirmText"
      :cancel-text="modalCancelText"
      @close="closeModal"
      @confirm="confirmModal"
    />
  </div>
</template>

