import { Card, CardDescription, CardHeader, CardTitle } from "@evie/ui/card";
import type { PingView } from "./ping-view";

/** The texts of the backend status, in the reader's language. */
export type PingStatusTexts = {
  readonly title: string;
  readonly version: (version: string) => string;
  readonly unreachable: string;
};

type PingStatusProps = {
  readonly view: PingView;
  readonly texts: PingStatusTexts;
};

/** Shows the backend's version, or that the backend can't be reached. */
export const PingStatus = (props: PingStatusProps) => {
  const description = () => {
    const view = props.view;
    switch (view._tag) {
      case "Reachable":
        return <CardDescription>{props.texts.version(view.version)}</CardDescription>;
      case "Unreachable":
        // A status role, so screen readers announce the outage.
        return <CardDescription role="status">{props.texts.unreachable}</CardDescription>;
      default:
        return view satisfies never;
    }
  };
  return (
    <Card>
      <CardHeader>
        <CardTitle as="h1">{props.texts.title}</CardTitle>
        {description()}
      </CardHeader>
    </Card>
  );
};
