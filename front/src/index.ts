import "./styles.css";

import { Chessboard, FEN, COLOR } from "cm-chessboard/src/Chessboard.js";
import { Markers } from "cm-chessboard/src/extensions/markers/Markers.js";
import { PromotionDialog } from "cm-chessboard/src/extensions/promotion-dialog/PromotionDialog.js";

import {
    GameMode,
    Game,
    TurnChangedEvent,
    LogMessageEvent,
    GameOverEvent
} from "./Game";

let game_mode_selector: HTMLDivElement;
let cc_mode_btn: HTMLButtonElement;
let wubc_mode_btn: HTMLButtonElement;
let buwc_mode_btn: HTMLButtonElement;
let game_container_div: HTMLDivElement;
let board_div: HTMLDivElement;
let turn_lbl: HTMLLabelElement;
let depth_input: HTMLInputElement;
let switch_orientation_btn: HTMLButtonElement;
let next_move_btn: HTMLButtonElement;
let messages_div: HTMLDivElement;

let game: Game;

let game_mode: GameMode;

const DEFAULT_SEARCH_DEPTH = 5;

function show_message(text: string) {
    const node = document.createElement("div");
    node.innerText = text;
    messages_div.appendChild(node);
    const children = messages_div.childNodes;
    if (children.length > 20) {
        messages_div.removeChild(children[0]);
    }
}

function start_game() {
    const board = new Chessboard(board_div, {
        position: FEN.start,
        assetsUrl: "./assets/",
        extensions: [
            { class: Markers, props: { autoMarkers: null } },
            { class: PromotionDialog }
        ],
        style: { animationDuration: 200 }
    });
    const depth = Number.parseInt(depth_input.value);
    if (Number.isNaN(depth)) {
        show_message("Depth value is not a number!");
        return;
    }

    game = new Game(board, game_mode, depth);
    game.addEventListener("turn_changed", (e: TurnChangedEvent) => {
        set_turn_text(e.color);
    });
    game.addEventListener("log_message", (e: LogMessageEvent) => {
        show_message(e.message);
    });
    game.addEventListener("game_over", (e: GameOverEvent) => {
        show_message(`Game over: ${e.message}`);
        next_move_btn.classList.add("hidden");
    });
    game.addEventListener("request_started", () => {
        next_move_btn.disabled = true;
    });
    game.addEventListener("request_done", () => {
        next_move_btn.disabled = false;
    });

    set_turn_text(COLOR.white);
    if (game_mode === GameMode.BUWC) {
        game.do_computer_move();
    }

    if (game_mode === GameMode.CC) {
        next_move_btn.onclick = () => game.do_computer_move();
    }
}

function set_game_mode(mode: GameMode) {
    game_mode = mode;
    game_mode_selector.classList.add("hidden");
    game_container_div.classList.remove("hidden");
    if (game_mode === GameMode.CC) {
        next_move_btn.classList.remove("hidden");
    }
    start_game();
}

function set_turn_text(color: COLOR) {
    if (color === COLOR.white) {
        turn_lbl.innerText = "White turn";
    } else {
        turn_lbl.innerText = "Black turn";
    }
}

function validate_search_depth(s: string): number {
    if (s.length === 0) {
        return DEFAULT_SEARCH_DEPTH;
    }
    const i = Number.parseInt(s);
    const min = Number.parseInt(depth_input.min);
    const max = Number.parseInt(depth_input.max);
    if (Number.isNaN(i) || i < min || i > max) {
        console.warn("depth number is invalid");
        return DEFAULT_SEARCH_DEPTH;
    }
    return i;
}

function add_event_listeners() {
    wubc_mode_btn.onclick = () => set_game_mode(GameMode.WUBC);
    buwc_mode_btn.onclick = () => set_game_mode(GameMode.BUWC);
    cc_mode_btn.onclick = () => set_game_mode(GameMode.CC);
    switch_orientation_btn.onclick = () => game.switch_orientation();
    depth_input.onchange = () => {
        const s = depth_input.value;
        const new_value = validate_search_depth(s);
        depth_input.value = new_value.toString();
        game.set_search_depth(new_value);
    };
}

window.onload = () => {
    game_mode_selector = document.getElementById(
        "game_mode_selector"
    ) as HTMLDivElement;
    cc_mode_btn = document.getElementById("cc") as HTMLButtonElement;
    wubc_mode_btn = document.getElementById("wubc") as HTMLButtonElement;
    buwc_mode_btn = document.getElementById("buwc") as HTMLButtonElement;
    switch_orientation_btn = document.getElementById(
        "switch_orientation"
    ) as HTMLButtonElement;
    game_container_div = document.getElementById("game_container") as HTMLDivElement;
    depth_input = document.getElementById("depth") as HTMLInputElement;
    board_div = document.getElementById("board") as HTMLDivElement;
    turn_lbl = document.getElementById("turn") as HTMLLabelElement;
    next_move_btn = document.getElementById("next_move") as HTMLButtonElement;
    messages_div = document.getElementById("messages") as HTMLDivElement;

    depth_input.value = DEFAULT_SEARCH_DEPTH.toString();
    add_event_listeners();
};
