import type { PingStatusTexts } from "./ping-status";

export const pingStatusTexts = {
  de: {
    title: "Backend",
    version: (version) => `Backend-Version ${version}`,
    unreachable: "Das Backend ist gerade nicht erreichbar. Versuche es später noch einmal.",
  },
  en: {
    title: "Backend",
    version: (version) => `Backend version ${version}`,
    unreachable: "The backend can't be reached right now. Try again later.",
  },
} satisfies Record<"de" | "en", PingStatusTexts>;
