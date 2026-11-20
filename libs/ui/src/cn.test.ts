import { describe, expect, test } from "bun:test";
import { cn } from "./cn";

describe("cn", () => {
  test("joins class names with a space", () => {
    expect(cn("flex", "items-center")).toBe("flex items-center");
  });

  test("drops falsy values", () => {
    expect(cn("flex", false, null, undefined, "", 0, "gap-2")).toBe("flex gap-2");
  });

  test("keeps the later of two conflicting Tailwind classes", () => {
    expect(cn("px-2 py-1", "px-4")).toBe("py-1 px-4");
  });
});
