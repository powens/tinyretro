import type { LaneThemeKey } from "$lib/BoardState.svelte";

export type LaneTheme = {
  /** Header background tint */
  headerBg: string;
  /** Header title text color */
  headerText: string;
  /** Badge preset class */
  badge: string;
  /** Emoji icon */
  icon: string;
  /** Card background (items) */
  cardBg: string;
  /** Card border (items) */
  cardBorder: string;
  /** Lane column background tint */
  laneBg: string;
  /** Lane column border color */
  laneBorder: string;
  /** Top accent bar color */
  accent: string;
};

export const laneThemeMap: Record<LaneThemeKey, LaneTheme> = {
  "went-well": {
    headerBg: "bg-green-500/10",
    headerText: "text-green-700 dark:text-green-400",
    badge: "preset-filled-success-200-800",
    icon: "🎉",
    cardBg: "bg-green-500/5 dark:bg-green-500/10",
    cardBorder: "border-green-400/40 dark:border-green-500/30",
    laneBg: "bg-green-500/[0.03] dark:bg-green-500/[0.06]",
    laneBorder: "border-green-400/30 dark:border-green-500/20",
    accent: "bg-green-500",
  },
  "to-improve": {
    headerBg: "bg-amber-500/10",
    headerText: "text-amber-700 dark:text-amber-400",
    badge: "preset-filled-warning-200-800",
    icon: "🔧",
    cardBg: "bg-amber-500/5 dark:bg-amber-500/10",
    cardBorder: "border-amber-400/40 dark:border-amber-500/30",
    laneBg: "bg-amber-500/[0.03] dark:bg-amber-500/[0.06]",
    laneBorder: "border-amber-400/30 dark:border-amber-500/20",
    accent: "bg-amber-500",
  },
  "action-items": {
    headerBg: "bg-blue-500/10",
    headerText: "text-blue-700 dark:text-blue-400",
    badge: "preset-filled-primary-200-800",
    icon: "🚀",
    cardBg: "bg-blue-500/5 dark:bg-blue-500/10",
    cardBorder: "border-blue-400/40 dark:border-blue-500/30",
    laneBg: "bg-blue-500/[0.03] dark:bg-blue-500/[0.06]",
    laneBorder: "border-blue-400/30 dark:border-blue-500/20",
    accent: "bg-blue-500",
  },
};

/** Get theme for a lane, falling back to went-well */
export function getLaneTheme(themeKey: LaneThemeKey): LaneTheme {
  return laneThemeMap[themeKey] ?? laneThemeMap["went-well"];
}
