export type Item = {
    body: string,
    vote_count: number,
    sort_order: number,
};

export type Lane = {
    title: string,
    theme: "went-well" | "to-improve" | "action-items";
    items: { [k: string]: Item };
};

export type Board = {
    title: string,
    lanes: { [k: string]: Lane };
}

export type ActionAddLane = {
    title: string,
};

export type ActionAddItem = {
    type: "AddItem",
    lane_id: string,
    body: string,
};

export type ActionRemoveItem = {
    lane_id: string,
    id: string,
};

export type ActionUpvoteItem = {
    type: "UpvoteItem",
    lane_id: string,
    id: string,
};

export type AllActions = ActionAddItem | ActionAddLane | ActionRemoveItem | ActionUpvoteItem;

export type SendActionFunc = (action: AllActions) => void;


export type WebsocketState = {
    state: Board | undefined,
    connected: boolean,
}
