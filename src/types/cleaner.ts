export type Tab =
  | "clean-c-fast"
  | "clean-c-advanced"
  | "clean-c-deep"
  | "clean-browser-chrome"
  | "clean-browser-edge"
  | "clean-memory";

export interface ScanItem {
  name: string;
  path: string;
  size: number;
  count: number;
  enabled: boolean;
}

export interface FastScanResult {
  items: ScanItem[];
  total_size: string;
  total_count: number;
}

export interface CleanResult {
  total_freed: string;
  success_count: number;
  fail_count: number;
}

export interface BrowserProfile {
  name: string;
  path_name: string;
  cache_size: number;
  cache_size_str: string;
  enabled: boolean;
}

export interface BrowserScanResult {
  profiles: BrowserProfile[];
  total_size: string;
}

export interface FileNode {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
  size_str: string;
  percent: number;
  has_children: boolean;
  level: number;
  isOpen: boolean;
  isLoading: boolean;
}

export interface MemoryStats {
  total: number;
  used: number;
  free: number;
  percent: number;
}

export interface ScanProgressPayload {
  file_count: number;
  current_path: string;
}

export interface ProjectCleanProgressPayload {
  completed_items: number;
  total_items: number;
  current_item: string;
  approx_completed_bytes: number;
}

export type ModalType = "info" | "success" | "error";
export type ModalMode = "alert" | "confirm";

export interface AlertOptions {
  title: string;
  message: string;
  type?: ModalType;
}

export interface ConfirmOptions extends AlertOptions {
  confirmText?: string;
  cancelText?: string;
}
