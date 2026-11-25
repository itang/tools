// The view tree. nsh's components run once: only expressions inside JSX bind
// to signals. So every branch is rendered and its visibility, text and props
// are bound in JSX — nothing is chosen in a function body.
import type { JSX } from "nsh";
import { Image, Text, View } from "nsh";
import { Button, DataTable, Input, NumberInput, Progress, type DataTableCell, type DataTableRef } from "nsh/base";
import type { Entry, ProcessRow } from "./types";
import {
  addrFamily, answerText, addressDetail, cpuLabel, memLabel, portsLabel,
  sinceLabel, visibleRows,
} from "./model";
import { formatTime, runningSeconds } from "./scan";
import type { Appearance, Store } from "./store";

const show = (on: boolean) => ({ display: on ? "flex" as const : "none" as const });

interface AnswerProps {
  status: number;
  entries: Entry[];
  port: number;
  error: string;
}

function AnswerStrip(props: AnswerProps): JSX.Element {
  return (
    <View style={show(props.status !== 0)} class="min-h-9 flex-row items-center gap-2 rounded-md bg-muted px-3 py-2">
      <Progress
        style={show(props.status === 1)}
        class="w-28 shrink-0" indeterminate
        trackClass="h-1.5 w-full rounded-full bg-border"
        indicatorClass="h-1.5 w-1/3 rounded-full bg-primary"
        accessibilityLabel="正在处理"
      />
      <View
        style={show(props.status !== 1)}
        class={props.status === 3
          ? "w-2 h-2 rounded-full shrink-0 bg-destructive"
          : props.entries.length === 0
            ? "w-2 h-2 rounded-full shrink-0 bg-muted-foreground"
            : "w-2 h-2 rounded-full shrink-0 bg-primary"}
      />
      <Text style={show(props.status === 1)} class="flex-1 text-sm text-muted-foreground">
        正在查询端口 {props.port} …
      </Text>
      <Text style={show(props.status === 3)} class="flex-1 text-sm text-destructive">
        查询失败：{props.error}
      </Text>
      <Text
        style={show(props.status !== 1 && props.status !== 3)}
        class={props.entries.length === 0
          ? "flex-1 truncate text-sm text-foreground"
          : "flex-1 truncate text-sm font-medium text-foreground"}
      >
        {answerText(props.entries, props.port)}
      </Text>
      <Text
        style={show(props.status !== 1 && props.status !== 3 && addressDetail(props.entries) !== "")}
        class="shrink-0 text-xs text-muted-foreground"
      >
        {addressDetail(props.entries)}
      </Text>
    </View>
  );
}

function AppearanceSwitch(props: { mode: Appearance; pick: (mode: Appearance) => void }): JSX.Element {
  const options: Appearance[] = ["system", "light", "dark"];
  const labels: Record<Appearance, string> = { system: "自动", light: "浅色", dark: "深色" };
  return (
    <View class="flex-row items-center gap-0.5 rounded-md border border-border bg-muted/60 p-0.5">
      {options.map((option) => (
        <Button
          class={option === props.mode
            ? "rounded bg-primary/15 px-2 py-0.5"
            : "rounded px-2 py-0.5 hover:bg-background/50 active:opacity-90"}
          onPress={() => props.pick(option)}
        >
          <Text class={option === props.mode ? "text-xs font-medium text-primary" : "text-xs text-muted-foreground"}>
            {labels[option]}
          </Text>
        </Button>
      ))}
    </View>
  );
}

function processCell(
  row: number, proc: ProcessRow, key: string, onKill: (proc: ProcessRow) => void,
): JSX.Element {
  switch (key) {
    case "seq":
      return <Text class="w-full text-right text-xs text-muted-foreground tabular-nums">{row + 1}</Text>;
    case "name":
      return <Text class="truncate text-sm font-medium text-foreground">{proc.name === "" ? "—" : proc.name}</Text>;
    case "pid":
      return <Text class="w-full text-right text-sm text-foreground tabular-nums">{proc.pid}</Text>;
    case "ports": {
      const ports = portsLabel(proc.tcp_ports, proc.udp_ports);
      return <Text class="truncate text-sm text-foreground tabular-nums">{ports === "" ? "—" : ports}</Text>;
    }
    case "cpu":
      return <Text class="w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums">{cpuLabel(proc.cpu_x10)}</Text>;
    case "mem":
      return <Text class="w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums">{memLabel(proc.mem_kb)}</Text>;
    case "uptime": {
      const since = sinceLabel(runningSeconds(proc.started_s));
      return <Text class="truncate whitespace-nowrap text-sm text-muted-foreground tabular-nums">{since === "" ? "—" : since}</Text>;
    }
    case "action":
      return (
        <View class="w-full flex-row items-center justify-center">
          <Button
            class="group shrink-0 rounded-md border border-border bg-background px-2 py-0.5 hover:border-destructive/50 hover:bg-destructive/10 active:opacity-90"
            onPress={() => onKill(proc)}
          >
            <Text class="whitespace-nowrap text-xs font-medium text-muted-foreground group-hover:text-destructive">结束</Text>
          </Button>
        </View>
      );
    default:
      return <Text class="text-sm text-foreground" />;
  }
}

interface TableProps {
  store: Store;
  rows: ProcessRow[];
  height: number;
  nameWidth: number;
  portsWidth: number;
  sortKey: string;
  sortAscending: boolean;
  selectedRow: number;
}

const COLUMNS = [
  { key: "seq", title: "序号", width: 56, fixed: true, align: "end" as const, cellClass: "text-right", headerClass: "text-right" },
  { key: "name", title: "进程", width: 176, fixed: true, sortable: true },
  { key: "pid", title: "PID", width: 76, align: "end" as const, sortable: true, cellClass: "text-right", headerClass: "text-right" },
  { key: "ports", title: "端口", width: 244, sortable: true },
  { key: "cpu", title: "CPU", width: 76, align: "end" as const, sortable: true, cellClass: "text-right", headerClass: "text-right" },
  { key: "mem", title: "内存", width: 92, align: "end" as const, sortable: true, cellClass: "text-right", headerClass: "text-right" },
  { key: "uptime", title: "运行时长", width: 98, sortable: true },
  { key: "action", title: "", width: 84, selectable: false, movable: false, resizable: false, cellClass: "px-2" },
];

function processTable(props: TableProps): JSX.Element {
  const { store, rows } = props;
  const columns = COLUMNS.map((column) => {
    if (column.key === "name") return { ...column, width: props.nameWidth };
    if (column.key === "ports") return { ...column, width: props.portsWidth };
    return column;
  });
  return (
    <DataTable
      ref={(ref: DataTableRef) => store.setTableRef(ref)}
      columns={columns}
      data={rows}
      rowHeight={32}
      headerHeight={30}
      rowHeader={false}
      columnSelectable={false}
      cellSelectable={false}
      stripe={false}
      sort={props.sortKey === "" ? null : { key: props.sortKey, direction: props.sortAscending ? "ascending" : "descending" }}
      selection={props.selectedRow >= 0 ? { row: props.selectedRow } : null}
      accessibilityLabel="占用端口的进程"
      class={`h-[${props.height}px] rounded-lg border border-border bg-background`}
      classes={{
        header: "border-b border-border bg-muted/40",
        headerCell: "px-3 text-xs font-medium text-muted-foreground",
        headerContent: "gap-1",
        sortIndicator: "text-muted-foreground",
        row: "h-8",
        rowHover: "bg-muted/50",
        rowSelected: "bg-primary/10",
        cell: "px-3 text-sm text-foreground",
      }}
      renderCell={(value: DataTableCell) => processCell(value.row, rows[value.row], value.key, (proc) => store.killProc(proc))}
      onSortChange={(e) => {
        const s = e.sort;
        store.applySort(s ? s.key : "", s && s.direction ? s.direction : "");
      }}
      onRowPress={(e) => {
        if (e.row >= 0 && e.column != null && columns[e.column]?.key === "action") {
          store.killProc(rows[e.row]);
        }
      }}
      onSelectionChange={(e) => {
        const sel = e.selection;
        if (sel && typeof sel.row === "number") store.selectRow(sel.row);
      }}
    />
  );
}

interface ListProps extends TableProps {
  status: number;
  error: string;
  filter: string;
}

function ListView(props: ListProps): JSX.Element {
  return (
    <View class="w-full flex-1 overflow-hidden">
      <View style={show(props.status === 1)} class="h-[140px] flex-col items-center justify-center gap-3 rounded-lg border border-border bg-background">
        <Progress
          class="w-56" indeterminate
          trackClass="h-1.5 w-full rounded-full bg-border"
          indicatorClass="h-1.5 w-1/3 rounded-full bg-primary"
          accessibilityLabel="正在处理"
        />
        <Text class="text-sm text-muted-foreground">正在读取系统进程表 …</Text>
      </View>
      <View style={show(props.status === 3)} class="h-[140px] items-center justify-center rounded-lg border border-border bg-background">
        <Text class="text-sm text-destructive">进程列表获取失败：{props.error}</Text>
      </View>
      <View style={show(props.status !== 1 && props.status !== 3 && props.rows.length === 0)} class="h-[140px] items-center justify-center rounded-lg border border-border bg-background">
        <Text class="text-sm text-muted-foreground">
          {props.filter.trim() === "" ? "没有发现监听端口的进程" : `没有名称匹配「${props.filter.trim()}」的进程`}
        </Text>
      </View>
      <View style={show(props.status !== 1 && props.status !== 3 && props.rows.length > 0)} class="w-full flex-1 overflow-hidden">
        {processTable(props)}
      </View>
    </View>
  );
}

function socketPairs(proc: ProcessRow, entries: Entry[]): [string, string][] {
  const pairs: [string, string][] = [];
  let ownsPort = false;
  for (const entry of entries) {
    if (entry.pid === proc.pid) {
      ownsPort = true;
      pairs.push([entry.proto, entry.addr]);
    }
  }
  if (ownsPort) return pairs;
  for (const raw of proc.listeners.split(";")) {
    const item = raw.trim();
    if (item === "") continue;
    const parts = item.split(" ");
    if (parts.length >= 2) pairs.push([parts[0], parts[1]]);
  }
  return pairs;
}

function InspectorField(props: { label: string; value: string }): JSX.Element {
  return (
    <View class="shrink-0 flex-row items-center gap-1">
      <Text class="text-xs text-muted-foreground">{props.label}</Text>
      <Text class="text-xs text-foreground tabular-nums">{props.value}</Text>
    </View>
  );
}

function SocketChip(props: { pair: [string, string] }): JSX.Element {
  const [proto, addr] = props.pair;
  const family = addrFamily(addr);
  return (
    <View class="flex-row items-center gap-1 overflow-hidden whitespace-nowrap rounded-md border border-border bg-background px-1.5 py-0.5">
      <Text class="shrink-0 text-xs text-muted-foreground">{proto} {proto === "TCP" ? "监听" : "绑定"}</Text>
      <Text class="truncate text-xs text-foreground tabular-nums">{addr}</Text>
      {family !== "" && <Text class="shrink-0 text-xs text-muted-foreground">{family}</Text>}
    </View>
  );
}

interface InspectorProps {
  store: Store;
  rows: ProcessRow[];
  selected: number;
  entries: Entry[];
  port: number;
  copied: boolean;
}

function InspectorView(props: InspectorProps): JSX.Element {
  const { store, rows, selected, entries } = props;
  const proc = selected >= 0 && selected < rows.length ? rows[selected] : null;
  const path = proc ? (proc.path === "" ? "可执行文件路径不可用" : proc.path) : "在列表中选择一行，查看父进程、启动时间、路径与监听地址";
  const pairs = proc ? socketPairs(proc, entries) : [];
  const label = proc && entries.some((e) => e.pid === proc.pid) ? `端口 ${props.port}` : "监听地址";
  return (
    <View class="shrink-0 flex-col gap-1 overflow-hidden rounded-lg bg-muted/40 px-3 py-1.5">
      <View class="h-5 flex-row items-center gap-1.5 overflow-hidden">
        <Text style={show(proc === null)} class="text-sm font-medium text-muted-foreground">未选择进程</Text>
        <Text style={show(proc !== null)} class="shrink-0 text-sm font-medium text-foreground">{proc ? (proc.name === "" ? "未知进程" : proc.name) : ""}</Text>
        <View style={show(proc !== null)} class="shrink-0 rounded-md border border-border bg-background px-1.5 py-0.5">
          <Text class="text-xs text-muted-foreground tabular-nums">PID {proc ? proc.pid : ""}</Text>
        </View>
        <Text style={show(proc !== null)} class="shrink-0 text-xs text-muted-foreground">·</Text>
        <InspectorField label="父进程" value={proc ? (proc.ppid > 0 ? String(proc.ppid) : "—") : "—"} />
        <Text style={show(proc !== null)} class="shrink-0 text-xs text-muted-foreground">·</Text>
        <InspectorField label="启动" value={proc ? (formatTime(proc.started_s) || "—") : "—"} />
      </View>
      <View class="w-full h-5 flex-row items-center gap-1.5 overflow-hidden">
        <Text class="shrink-0 text-xs text-muted-foreground">路径</Text>
        <Text class="flex-1 truncate text-xs text-muted-foreground">{path}</Text>
        <Button
          style={show(proc !== null && proc.path !== "")}
          class="shrink-0 rounded border border-border bg-background px-1.5 py-0.5 hover:bg-muted active:opacity-90"
          onPress={() => { if (proc) store.copyPath(proc.path); }}
        >
          <Text class="text-xs text-muted-foreground">{props.copied ? "已复制" : "复制"}</Text>
        </Button>
      </View>
      <View class="h-5 flex-row items-center gap-1.5 overflow-hidden">
        <Text style={show(proc !== null && pairs.length === 0)} class="text-xs text-muted-foreground">监听地址不可用</Text>
        <Text style={show(proc !== null && pairs.length > 0)} class="shrink-0 text-xs text-muted-foreground">{label}</Text>
        {pairs.slice(0, 3).map((pair, i) => <SocketChip pair={pair} />)}
        <Text style={show(pairs.length > 3)} class="shrink-0 text-xs text-muted-foreground">+{pairs.length - 3}</Text>
      </View>
    </View>
  );
}

function Footer(props: { count: number; filter: string; notice: string }): JSX.Element {
  return (
    <View class="h-5 flex-row items-center overflow-hidden">
      <Text style={show(props.notice !== "")} class="truncate text-xs text-destructive">{props.notice}</Text>
      <Text style={show(props.notice === "" && props.filter !== "")} class="truncate text-xs text-muted-foreground">
        共 {props.count} 个进程 · 名称匹配「{props.filter}」
      </Text>
      <Text style={show(props.notice === "" && props.filter === "")} class="text-xs text-muted-foreground">
        共 {props.count} 个进程 · 含 TCP 监听与 UDP 绑定
      </Text>
    </View>
  );
}

export function AppView(props: { store: Store; icon: unknown }): JSX.Element {
  const store = props.store;
  return (
    <View class="flex-1 bg-background flex flex-col overflow-hidden">
      <View
        class={`shrink-0 flex-row items-center justify-between border-b border-border bg-background ${store.titleStyle}`}
        desktop={{ windowDrag: true }}
      >
        <View class="flex-row items-center gap-2">
          <Image src={props.icon as never} class="w-[20px] h-[20px]" />
          <Text class="text-xs text-muted-foreground">端口雷达</Text>
        </View>
        <AppearanceSwitch mode={store.appearance()} pick={(mode) => store.applyAppearance(mode)} />
      </View>

      <View class="flex-1 flex-col p-4 overflow-hidden">
        <View class="w-full flex-1 flex flex-col gap-3 overflow-hidden">
          <View class="shrink-0 flex-row items-center gap-2">
            <Text class="text-xs text-muted-foreground">端口</Text>
            <NumberInput
              class="w-36 rounded-md border border-border bg-background focus-within:border-primary"
              inputClass="px-3 py-1.5 text-sm text-foreground"
              value={store.port() > 0 ? store.port() : null}
              min={1} max={65535} placeholder="端口号" accessibilityLabel="端口号"
              onChange={(e) => { store.port.set(e.value && e.value > 0 ? e.value : 0); }}
              onSubmit={(e) => { store.port.set(e.value ?? 0); store.query(); }}
            />
            <Text class="text-xs text-muted-foreground">名称</Text>
            <Input
              class="w-40 rounded-md border border-border bg-background px-3 py-1.5 text-sm text-foreground focus:border-primary"
              value={store.nameDraft()}
              placeholder="进程名" accessibilityLabel="进程名称"
              onChange={(e) => store.nameDraft.set(e.value)}
              onSubmit={() => store.query()}
            />
            <Button
              class="rounded-md bg-primary px-4 py-1.5 hover:bg-primary/90 active:opacity-90"
              onPress={() => store.query()}
            >
              <Text class="text-sm font-medium text-primary-foreground">查询</Text>
            </Button>
            <Button
              class="rounded-md border border-border px-3 py-1.5 hover:bg-muted active:opacity-90"
              onPress={() => store.clearQuery()}
            >
              <Text class="text-sm font-medium text-muted-foreground">清空</Text>
            </Button>
          </View>

          <View class="shrink-0">
            <AnswerStrip status={store.status()} entries={store.entries()} port={store.port()} error={store.error()} />
          </View>

          <View class="shrink-0 flex-row items-center justify-between">
            <Text class="text-sm font-semibold text-foreground">开放端口的进程</Text>
            <Button
              class="rounded-md border border-border px-2.5 py-1 hover:bg-muted active:opacity-90"
              onPress={() => store.refreshAll()}
            >
              <Text class="text-xs text-muted-foreground">刷新</Text>
            </Button>
          </View>

          <View
            class="w-full flex-1 overflow-hidden"
            onLayout={(e) => store.onTableLayout(Math.round(e.bounds.size.width), Math.round(e.bounds.size.height))}
          >
            <ListView
              store={store}
              rows={visibleRows(store.procs(), store.nameFilter())}
              height={store.tableHeight()}
              nameWidth={store.tableNameWidth()}
              portsWidth={store.tablePortsWidth()}
              sortKey={store.sortKey()}
              sortAscending={store.sortAscending()}
              selectedRow={store.selectedRow()}
              status={store.procStatus()}
              error={store.procError()}
              filter={store.nameFilter()}
            />
          </View>

          <InspectorView
            store={store}
            rows={visibleRows(store.procs(), store.nameFilter())}
            selected={store.selectedRow()}
            entries={store.entries()}
            port={store.port()}
            copied={store.copied()}
          />
          <Footer
            count={visibleRows(store.procs(), store.nameFilter()).length}
            filter={store.nameFilter()}
            notice={store.notice()}
          />
        </View>
      </View>
    </View>
  );
}
