import { describe, test, expect } from "vitest";
import { getLaneTheme, laneThemeMap } from "./v2-theme";
import type { LaneThemeKey } from "$lib/BoardState.svelte";

describe("v2-theme", () => {
  test("returns went-well theme", () => {
    const theme = getLaneTheme("went-well");
    expect(theme.icon).toBe("🎉");
    expect(theme.accent).toContain("green");
  });

  test("returns to-improve theme", () => {
    const theme = getLaneTheme("to-improve");
    expect(theme.icon).toBe("🔧");
    expect(theme.accent).toContain("amber");
  });

  test("returns action-items theme", () => {
    const theme = getLaneTheme("action-items");
    expect(theme.icon).toBe("🚀");
    expect(theme.accent).toContain("blue");
  });

  test("falls back to went-well for unknown key", () => {
    const theme = getLaneTheme("nonexistent" as LaneThemeKey);
    expect(theme).toEqual(laneThemeMap["went-well"]);
  });

  test("all themes have required properties", () => {
    const requiredKeys = [
      "headerBg",
      "headerText",
      "badge",
      "icon",
      "cardBg",
      "cardBorder",
      "laneBg",
      "laneBorder",
      "accent",
    ];

    for (const [key, theme] of Object.entries(laneThemeMap)) {
      for (const prop of requiredKeys) {
        expect(theme).toHaveProperty(prop);
        expect((theme as Record<string, string>)[prop]).toBeTruthy();
      }
    }
  });

  test("laneThemeMap has exactly 3 themes", () => {
    expect(Object.keys(laneThemeMap)).toHaveLength(3);
    expect(Object.keys(laneThemeMap)).toEqual(
      expect.arrayContaining(["went-well", "to-improve", "action-items"]),
    );
  });
});
