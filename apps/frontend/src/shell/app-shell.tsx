import { A, type RouteSectionProps } from "@solidjs/router";
import { Suspense } from "solid-js";

/** The layout around every page: a sticky header and a centred, mobile-first main column. */
export function AppShell(props: RouteSectionProps) {
  return (
    <div class="flex min-h-dvh flex-col pr-[env(safe-area-inset-right)] pl-[env(safe-area-inset-left)]">
      <header class="sticky top-0 z-10 border-b bg-background/90 pt-[env(safe-area-inset-top)] backdrop-blur">
        <div class="mx-auto flex min-h-14 w-full max-w-screen-md items-center px-4 sm:px-6">
          <A
            href="/"
            class="inline-flex min-h-11 items-center rounded-md font-semibold text-primary text-xl tracking-tight focus-visible:ring-2 focus-visible:ring-ring"
          >
            evie
          </A>
        </div>
      </header>
      <main class="mx-auto w-full max-w-screen-md flex-1 px-4 py-6 pb-[calc(1.5rem+env(safe-area-inset-bottom))] sm:px-6 sm:py-10">
        <Suspense>{props.children}</Suspense>
      </main>
    </div>
  );
}
