import { describe, expect, test } from "bun:test";
import { buttonVariants } from "./button-variants";
import { cn } from "./cn";

const SIZES = ["default", "sm", "lg", "icon"] as const;

function classesOf(value: string): ReadonlyArray<string> {
  return value.split(/\s+/);
}

describe("buttonVariants", () => {
  for (const size of SIZES) {
    test(`keeps the ${size} size at least 44 px in both directions`, () => {
      const classes = classesOf(buttonVariants({ size }));
      expect(classes).toContain("min-h-11");
      expect(classes).toContain("min-w-11");
    });
  }

  test("keeps the minimum size when a caller adds its own classes", () => {
    const classes = classesOf(cn(buttonVariants(), "px-8 h-9"));
    expect(classes).toContain("min-h-11");
    expect(classes).toContain("min-w-11");
  });

  test("applies the default variant and size without props", () => {
    expect(buttonVariants()).toBe(buttonVariants({ variant: "default", size: "default" }));
    expect(classesOf(buttonVariants())).toContain("bg-primary");
    expect(classesOf(buttonVariants())).toContain("h-11");
  });
});
