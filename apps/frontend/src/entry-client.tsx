// @refresh reload
import { mount, StartClient } from "@solidjs/start/client";

const root = document.getElementById("app");
if (!root) {
  throw new Error("missing the #app element that entry-server.tsx renders");
}

mount(() => <StartClient />, root);
