import { mount, signal, List, Image, Clipboard } from "nsh";
import * as rt from "nsh:rt";
import * as process from "process";
import { platform } from "os";
import * as base from "nsh/base";
import { Window, titleBar } from "nsh/desktop";
import icon from "./assets/icon.png";
import { app } from "./moonbit/_build/js/release/build/portfinder_ui.js";

mount(() =>
  app({ rt, signal, process, base, platform: platform(), Window, titleBar, List, Image, Clipboard, icon }),
);
