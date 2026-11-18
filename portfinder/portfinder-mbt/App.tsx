import { mount, signal, List, Image, Clipboard, Storage, setColorScheme } from "nsh";
import * as rt from "nsh:rt";
import * as process from "process";
import { platform } from "os";
import * as base from "nsh/base";
import { titleBar } from "nsh/desktop";
import icon from "./assets/icon.png";
import { app } from "./moonbit/_build/js/release/build/portfinder_ui.js";
import { Scanner } from "portradar/native";

mount(() =>
  app({
    rt, signal, process, base, platform: platform(), titleBar, List, Image, Clipboard, Storage, setColorScheme, icon,
    scan: () => Scanner.scanProcesses(),
    scanPort: (port: number) => Scanner.scanPort(port),
    killProcess: (pid: number) => Scanner.killProcess(pid),
  }),
);
