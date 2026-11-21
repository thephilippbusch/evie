import type { Component, ComponentProps } from "solid-js";
import { splitProps } from "solid-js";
import { Dynamic } from "solid-js/web";

import { cn } from "#utils/cn";

const Card: Component<ComponentProps<"div">> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <div
      class={cn("rounded-lg border bg-card text-card-foreground shadow-xs", local.class)}
      {...others}
    />
  );
};

const CardHeader: Component<ComponentProps<"div">> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <div class={cn("flex flex-col gap-1.5 p-4 sm:p-6", local.class)} {...others} />;
};

type CardTitleProps = ComponentProps<"h2"> & {
  /** The heading level, so the title fits the page's outline. */
  as?: "h1" | "h2" | "h3";
};

const CardTitle: Component<CardTitleProps> = (props) => {
  const [local, others] = splitProps(props, ["as", "class"]);
  return (
    <Dynamic
      component={local.as ?? "h2"}
      class={cn("text-lg font-semibold leading-none tracking-tight", local.class)}
      {...others}
    />
  );
};

const CardDescription: Component<ComponentProps<"p">> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <p class={cn("text-sm text-muted-foreground", local.class)} {...others} />;
};

const CardContent: Component<ComponentProps<"div">> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <div class={cn("p-4 pt-0 sm:p-6 sm:pt-0", local.class)} {...others} />;
};

const CardFooter: Component<ComponentProps<"div">> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return <div class={cn("flex items-center p-4 pt-0 sm:p-6 sm:pt-0", local.class)} {...others} />;
};

export type { CardTitleProps };
export { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle };
