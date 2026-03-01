export type Item = {
  body: string;
  vote_count: number;
  sort_order: number;
};

export type LaneThemeKey = "went-well" | "to-improve" | "action-items";

export type Lane = {
  title: string;
  theme: LaneThemeKey;
  items: { [k: string]: Item };
};

export type Board = {
  title: string;
  lanes: { [k: string]: Lane };
};

export type ActionAddLane = {
  type: "AddLane";
  title: string;
};

export type ActionAddItem = {
  type: "AddItem";
  lane_id: string;
  body: string;
};

export type ActionRemoveItem = {
  type: "RemoveItem";
  lane_id: string;
  id: string;
};

export type ActionUpvoteItem = {
  type: "UpvoteItem";
  lane_id: string;
  id: string;
};

export type ActionMoveItem = {
  type: "MoveItem";
  from_lane_id: string;
  to_lane_id: string;
  item_id: string;
};

export type ActionReorderItem = {
  type: "ReorderItem";
  lane_id: string;
  item_id: string;
  new_position: number;
};

export type ActionEditItem = {
  type: "EditItem";
  lane_id: string;
  id: string;
  body: string;
};

export type ActionMergeItems = {
  type: "MergeItems";
  lane_id: string;
  source_id: string;
  target_id: string;
  merged_body: string;
};

export type AllActions =
  | ActionAddItem
  | ActionAddLane
  | ActionRemoveItem
  | ActionUpvoteItem
  | ActionMoveItem
  | ActionReorderItem
  | ActionEditItem
  | ActionMergeItems;

export type SendActionFunc = (action: AllActions) => void;

/** Merge source state shared across the board */
export type MergeSource = {
  laneId: string;
  itemId: string;
  body: string;
  vote_count: number;
} | null;

/** DnD item shape used by svelte-dnd-action */
export type DndItem = {
  id: string;
  body: string;
  vote_count: number;
  sort_order: number;
  isDndShadowItem?: boolean;
};

