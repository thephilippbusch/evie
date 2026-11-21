import "@evie/ui/styles.css";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { AppShell } from "~/shell/app-shell";

export default function App() {
  return (
    <Router root={AppShell}>
      <FileRoutes />
    </Router>
  );
}
