import "@evie/ui/styles.css";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { AppShell } from "~/shell/app-shell";

const App = () => (
  <Router root={AppShell}>
    <FileRoutes />
  </Router>
);

export default App;
