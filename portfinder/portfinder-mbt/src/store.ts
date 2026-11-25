// The app's state and actions. Every dynamic value is a signal: reading it in
// JSX creates a binding, so the views never need manual rebuild slots or
// version counters. Async scans capture this same store and write through it.
import { signal, type Signal } from "nsh";
import { platform } from "os";
import { titleBar } from "nsh/desktop";
import { AlertDialog, type DataTableRef } from "nsh/base";
import { Clipboard, Storage, setColorScheme } from "nsh";
import type { Entry, ProcessRow, ScanStatus } from "./types";
import { indexOfPid, sortProcesses, visibleRows } from "./model";
import {
  Scanner, killProcess, parseProcesses, scanPort, scanProcesses,
} from "./scan";

export type Appearance = "system" | "light" | "dark";

// Geometry of the hidden title bar, in points: [height, left, right].
function titleBarMetrics(): [number, number, number] {
  try {
    const area = titleBar();
    return [Math.round(area.height), Math.round(area.left), Math.round(area.right)];
  } catch {
    return [0, 0, 0];
  }
}

export class Store {
  readonly port = signal<number>(8080);
  readonly status = signal<ScanStatus>(1);
  readonly entries = signal<Entry[]>([]);
  readonly error = signal("");
  readonly procs = signal<ProcessRow[]>([]);
  readonly procStatus = signal<ScanStatus>(1);
  readonly procError = signal("");
  readonly notice = signal("");
  readonly sortKey = signal("");
  readonly sortAscending = signal(true);
  readonly nameFilter = signal("");
  readonly nameDraft = signal("");
  readonly selectedRow = signal(-1);
  readonly selectedPid = signal(-1);
  readonly tableHeight = signal(30 + 9 * 32 + 2);
  readonly tableNameWidth = signal(176);
  readonly tablePortsWidth = signal(244);
  readonly appearance = signal<Appearance>("system");
  readonly copiedKey = signal("");

  // Non-reactive coordination: a newer query wins over an older reply, and a
  // table rebuild re-applies the selection the user last saw.
  private sequence = 0;
  private procSequence = 0;
  private pendingRevealPid = 0;
  private tableRef: DataTableRef | null = null;
  private readonly barHeight: number;
  private readonly padLeft: number;
  private readonly padRight: number;

  constructor() {
    const [height, left, right] = titleBarMetrics();
    this.barHeight = height > 0 ? height : 44;
    const os = platform();
    this.padLeft = left > 0 ? left + 12 : os === "macos" ? 88 : 16;
    this.padRight = right > 0 ? right + 12 : os === "macos" ? 16 : 150;
  }

  get titleStyle(): string {
    return `h-[${this.barHeight}px] pl-[${this.padLeft}px] pr-[${this.padRight}px]`;
  }

  setTableRef(ref: DataTableRef | null): void {
    this.tableRef = ref;
  }

  revealRow(row: number): void {
    if (row >= 0 && this.tableRef) {
      try {
        this.tableRef.select({ row });
        this.tableRef.scrollToRow(row, false);
      } catch {
        // The table may be rebuilding; the selection signal still holds.
      }
    }
  }

  private clearNotice(): void {
    if (this.notice() !== "") this.notice.set("");
  }

  private copyFailed(message: string): void {
    this.notice.set(`复制路径失败：${message}`);
  }

  applyAppearance(mode: Appearance): void {
    if (mode === this.appearance()) return;
    this.appearance.set(mode);
    setColorScheme(mode);
    Storage.setItem("appearance", mode).catch(() => {});
  }

  loadAppearance(): void {
    Storage.getItem("appearance").then((value) => {
      if (value === "light" || value === "dark" || value === "system") {
        if (value !== this.appearance()) {
          this.appearance.set(value);
          setColorScheme(value);
        }
      }
    }).catch(() => {});
  }

  async refreshProcesses(): Promise<void> {
    const current = ++this.procSequence;
    this.procStatus.set(1);
    this.procError.set("");
    try {
      const text = await scanProcesses(Scanner);
      if (current !== this.procSequence) return;
      const wanted = this.pendingRevealPid || this.selectedPid();
      this.pendingRevealPid = 0;
      const procs = parseProcesses(text);
      sortProcesses(procs, this.sortKey(), this.sortAscending());
      this.procs.set(procs);
      this.procStatus.set(2);
      this.procError.set("");
      const visible = visibleRows(procs, this.nameFilter());
      const target = indexOfPid(visible, wanted);
      this.selectedRow.set(target);
      this.selectedPid.set(target >= 0 ? visible[target].pid : -1);
      this.revealRow(target);
    } catch (error) {
      if (current !== this.procSequence) return;
      this.pendingRevealPid = 0;
      const message = error instanceof Error ? error.message : String(error);
      this.procError.set(message);
      if (this.procs().length === 0) {
        this.procStatus.set(3);
      } else {
        // Keep the rows in view and report the failed refresh beside them.
        this.procStatus.set(2);
        this.notice.set(`进程列表刷新失败：${message}`);
      }
    }
  }

  async search(): Promise<void> {
    const current = ++this.sequence;
    this.pendingRevealPid = 0;
    this.status.set(1);
    const port = this.port();
    try {
      const found = await scanPort(port, Scanner);
      if (current !== this.sequence) return;
      this.entries.set(found);
      this.status.set(2);
      let reveal = -1;
      if (found.length === 0) {
        this.selectedRow.set(-1);
        this.selectedPid.set(-1);
      } else {
        const pid = found[0].pid;
        const visible = visibleRows(this.procs(), this.nameFilter());
        const target = indexOfPid(visible, pid);
        this.selectedRow.set(target);
        this.selectedPid.set(target >= 0 ? pid : -1);
        reveal = target;
        // A row hidden by the name filter stays hidden; only a pid missing
        // from the list is worth a refresh.
        if (target < 0 && indexOfPid(this.procs(), pid) < 0) {
          this.pendingRevealPid = pid;
          this.refreshProcesses();
        }
      }
      this.revealRow(reveal);
    } catch (error) {
      if (current !== this.sequence) return;
      this.error.set(error instanceof Error ? error.message : String(error));
      this.status.set(3);
      this.selectedRow.set(-1);
      this.selectedPid.set(-1);
      this.pendingRevealPid = 0;
      this.revealRow(-1);
    }
  }

  query(): void {
    const nextFilter = this.nameDraft().trim();
    if (nextFilter !== this.nameFilter()) {
      this.nameFilter.set(nextFilter);
      this.selectedRow.set(-1);
      this.selectedPid.set(-1);
      this.pendingRevealPid = 0;
      this.revealRow(-1);
    }
    this.clearNotice();
    if (this.port() <= 0) {
      // No port asked: the strip stays empty, the name condition still applies.
      this.sequence++;
      this.entries.set([]);
      this.error.set("");
      this.status.set(0);
      return;
    }
    this.search();
  }

  refreshAll(): void {
    this.clearNotice();
    if (this.port() > 0) this.search();
    this.refreshProcesses();
  }

  clearQuery(): void {
    this.sequence++;
    this.port.set(0);
    this.nameDraft.set("");
    this.nameFilter.set("");
    this.entries.set([]);
    this.error.set("");
    this.status.set(0);
    this.selectedRow.set(-1);
    this.selectedPid.set(-1);
    this.pendingRevealPid = 0;
    this.clearNotice();
    this.revealRow(-1);
    this.refreshProcesses();
  }

  applySort(key: string, direction: string): void {
    const ascending = direction !== "descending";
    if (key === this.sortKey() && ascending === this.sortAscending()) return;
    this.sortKey.set(key);
    this.sortAscending.set(ascending);
    const procs = this.procs();
    sortProcesses(procs, key, ascending);
    this.procs.set([...procs]);
    this.selectedRow.set(-1);
    this.selectedPid.set(-1);
    this.revealRow(-1);
  }

  selectRow(row: number): void {
    const visible = visibleRows(this.procs(), this.nameFilter());
    const pid = row >= 0 && row < visible.length ? visible[row].pid : -1;
    if (row === this.selectedRow() && pid === this.selectedPid()) return;
    this.selectedRow.set(row);
    this.selectedPid.set(pid);
    this.revealRow(row);
  }

  async killProc(proc: ProcessRow): Promise<void> {
    const name = proc.name === "" ? "该进程" : proc.name;
    const result = await AlertDialog.show({
      title: "结束进程",
      description: `确定结束 ${name}（PID ${proc.pid}）？它占用的端口会随之释放。`,
      actions: [
        { id: "cancel", label: "取消", role: "cancel" },
        { id: "ok", label: "结束进程", role: "destructive" },
      ],
      closeOnEscape: true,
    });
    if (result.action !== "ok") return;
    try {
      await killProcess(proc.pid, Scanner);
      this.refreshProcesses();
      this.search();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      this.notice.set(`结束 PID ${proc.pid} 失败：${message}`);
    }
  }

  // Copies one field of the selected process; `key` drives the per-field
  // "已复制" feedback so each value can be copied independently.
  copyText(text: string, key: string): void {
    Clipboard.writeText(text).then(() => {
      this.copiedKey.set(key);
    }, (error) => {
      this.copyFailed(error instanceof Error ? error.message : String(error));
    });
  }

  onTableLayout(width: number, height: number): void {
    if (width <= 0 || height <= 0) return;
    // Let the table use all the room the slot gives it, minus a small gap.
    const nextHeight = Math.max(3 * 32 + 32, height - 8);
    const extra = width - 16 - 902;
    const ports = Math.max(140, extra > 0 ? 244 + Math.trunc(extra * 45 / 100) : 244 + extra);
    const name = Math.max(120, 176 + extra - (ports - 244));
    if (nextHeight !== this.tableHeight() ||
        name !== this.tableNameWidth() || ports !== this.tablePortsWidth()) {
      this.tableHeight.set(nextHeight);
      this.tableNameWidth.set(name);
      this.tablePortsWidth.set(ports);
    }
  }
}
