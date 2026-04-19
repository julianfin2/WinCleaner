export function formatItemSize(bytes: number): string {
  if (bytes === 0) return "0 B";

  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}

export function splitSize(sizeStr: string | number) {
  const str = String(sizeStr);
  const parts = str.split(" ");

  if (parts.length === 2) {
    return { value: parts[0], unit: parts[1] };
  }

  return { value: str, unit: "" };
}
