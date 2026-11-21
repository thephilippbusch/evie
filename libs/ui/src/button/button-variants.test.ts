import { describe, expect, test } from "bun:test";
import { cn } from "#utils/cn";
import { buttonVariants } from "./button-variants";

const SIZES = ["default", "sm", "lg", "icon"] as const;

const classesOf = (value: string): ReadonlyArray<string> => value.split(/\s+/);

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
    expect(classesOf(buttonVariants())).toContain("bg-secondary");
    expect(classesOf(buttonVariants())).toContain("h-11");
  });

  test("gives the brand variant the primary colour", () => {
    expect(classesOf(buttonVariants({ variant: "brand" }))).toContain("bg-primary");
  });
});
