<script setup lang="ts">
import type { ModalMode, ModalType } from "../../types/cleaner";

defineProps<{
  open: boolean;
  title: string;
  message: string;
  type: ModalType;
  mode?: ModalMode;
  confirmText?: string;
  cancelText?: string;
}>();

const emit = defineEmits<{
  close: [];
  confirm: [];
}>();
</script>

<template>
  <div v-if="open" class="modal-overlay" @click.self="emit('close')">
    <div class="modal-card" :class="type">
      <div class="modal-header">
        <span class="modal-icon">
          <template v-if="type === 'success'">✅</template>
          <template v-else-if="type === 'error'">❌</template>
          <template v-else>ℹ️</template>
        </span>
        <h3>{{ title }}</h3>
      </div>
      <div class="modal-body">
        <p>{{ message }}</p>
      </div>
      <div class="modal-footer" :class="{ 'is-confirm': mode === 'confirm' }">
        <button v-if="mode === 'confirm'" class="btn-secondary modal-btn" @click="emit('close')">
          {{ cancelText || "取消" }}
        </button>
        <button class="btn-primary modal-btn" @click="mode === 'confirm' ? emit('confirm') : emit('close')">
          {{ mode === "confirm" ? confirmText || "确定清理" : confirmText || "确定" }}
        </button>
      </div>
    </div>
  </div>
</template>
