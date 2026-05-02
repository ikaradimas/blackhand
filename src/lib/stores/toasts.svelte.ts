// Lightweight toast queue. Components push messages, ToastStack renders.

export type ToastKind = "error" | "warn" | "info" | "ok";

export type Toast = {
  id: string;
  kind: ToastKind;
  message: string;
  ttlMs: number;
};

class ToastStore {
  list = $state<Toast[]>([]);

  #push(kind: ToastKind, message: string, ttlMs: number): string {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    this.list = [...this.list, { id, kind, message, ttlMs }];
    if (ttlMs > 0) {
      setTimeout(() => this.dismiss(id), ttlMs);
    }
    return id;
  }

  error(message: string, ttlMs = 6000) {
    return this.#push("error", message, ttlMs);
  }
  warn(message: string, ttlMs = 5000) {
    return this.#push("warn", message, ttlMs);
  }
  info(message: string, ttlMs = 4000) {
    return this.#push("info", message, ttlMs);
  }
  ok(message: string, ttlMs = 4000) {
    return this.#push("ok", message, ttlMs);
  }

  dismiss(id: string) {
    this.list = this.list.filter((t) => t.id !== id);
  }
}

export const toasts = new ToastStore();
