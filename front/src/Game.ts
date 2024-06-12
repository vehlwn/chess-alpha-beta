import {
    Chessboard,
    INPUT_EVENT_TYPE,
    COLOR,
    PIECE
} from "cm-chessboard/src/Chessboard.js";
import { MARKER_TYPE } from "cm-chessboard/src/extensions/markers/Markers.js";
import { PROMOTION_DIALOG_RESULT_TYPE } from "cm-chessboard/src/extensions/promotion-dialog/PromotionDialog.js";
import { Chess, Square, Move } from "chess.js";

import { CONFIG } from "./config";

export enum GameMode {
    CC,
    WUBC,
    BUWC
}

export class LogMessageEvent extends Event {
    message: string;
    constructor(msg: string) {
        super("log_message");
        this.message = msg;
    }
}

export class TurnChangedEvent extends Event {
    color: COLOR;
    constructor(color: COLOR) {
        super("turn_changed");
        this.color = color;
    }
}
export class CheckEvent extends Event {
    color: COLOR;
    constructor(color: COLOR) {
        super("check");
        this.color = color;
    }
}

export class GameOverEvent extends Event {
    message: string;
    constructor(message: string) {
        super("game_over");
        this.message = message;
    }
}

export class RequestStartedEvent extends Event {
    constructor() {
        super("request_started");
    }
}

export class RequestDoneEvent extends Event {
    constructor() {
        super("request_done");
    }
}

const MarkerHighlight = { class: "marker_highlight", slice: "marker_highlight" };
// Marker highlighting valid moves
const AutoMarker = MARKER_TYPE.square;

interface MoveInputEvent {
    chessboard: Chessboard;
    type: INPUT_EVENT_TYPE;
    squareFrom: Square;
}

// https://github.com/shaack/cm-chessboard/blob/7918204d91aa6c1916f1ed1ca804524bca43b718/src/view/ChessboardView.js#L362
interface MoveInputStartedEvent extends MoveInputEvent {
    piece: PIECE;
}
interface ValidateMoveInputEvent extends MoveInputEvent {
    squareTo: string;
    piece: PIECE;
}
// https://github.com/shaack/cm-chessboard/blob/7918204d91aa6c1916f1ed1ca804524bca43b718/src/view/VisualMoveInput.js#L348
interface MovingOverSquareEvent extends MoveInputEvent {
    squareTo: string | null;
    piece: PIECE;
}
interface MoveInputFinishedEvent extends MoveInputEvent {
    squareTo: string;
    legalMove: boolean;
}

// https://github.com/shaack/cm-chessboard/blob/7918204d91aa6c1916f1ed1ca804524bca43b718/src/extensions/promotion-dialog/PromotionDialog.js#L148
interface PromotionDialogResult {
    type: PROMOTION_DIALOG_RESULT_TYPE;
}
interface PromotionDialogPieceSelected extends PromotionDialogResult {
    square: Square;
    piece: PIECE;
}

interface GetBestMoveResponse {
    m: string;
    value: number;
}

export class Game extends EventTarget {
    private board: Chessboard;
    private chess: Chess;
    private game_mode: GameMode;
    private search_depth: number;

    private last_valid_moves: Map<string, Move>;

    constructor(board: Chessboard, game_mode: GameMode, search_depth: number) {
        super();

        this.board = board;
        this.chess = new Chess();
        this.game_mode = game_mode;
        this.set_search_depth(search_depth);

        this.last_valid_moves = new Map();

        if (this.game_mode === GameMode.WUBC) {
            this.board.enableMoveInput(
                (e: MoveInputEvent) => this.inputHandler(e),
                COLOR.white
            );
        }
    }

    set_search_depth(n: number) {
        if (n <= 0 || !Number.isInteger(n)) {
            throw new Error(`search_depth must be positive integer (${n})`);
        }
        this.search_depth = n;
    }

    switch_orientation() {
        this.board.setOrientation(
            this.board.getOrientation() === COLOR.white ? COLOR.black : COLOR.white
        );
    }

    private update_view(last_move: Move) {
        this.board.setPosition(this.chess.fen(), true);
        this.dispatchEvent(new TurnChangedEvent(this.chess.turn()));
        this.dispatchEvent(new LogMessageEvent(last_move.san));
        if (last_move.captured !== undefined) {
            this.dispatchEvent(
                new LogMessageEvent("captured " + last_move.captured)
            );
        }
        this.check_game_over();
    }

    private highlight_move(m: Move) {
        this.board.removeMarkers(MarkerHighlight);
        this.board.addMarker(MarkerHighlight, m.from);
        this.board.addMarker(MarkerHighlight, m.to);
    }

    private _do_random_move() {
        const valid_moves = this.chess.moves({ verbose: true });
        if (valid_moves.length > 0) {
            const i = Math.floor(Math.random() * valid_moves.length);
            const random_move = valid_moves[i];
            this.chess.move(random_move);
            this.update_view(random_move);
            this.highlight_move(random_move);
            if (this.game_mode !== GameMode.CC) {
                this.enable_user_input();
            }
        }
    }

    private async do_server_request(): Promise<GetBestMoveResponse> {
        this.dispatchEvent(new RequestStartedEvent());
        const search_depth = this.search_depth;
        const fen = this.chess.fen();
        const data = { search_depth, fen };
        const resp = await fetch(`${CONFIG.api_prefix}/api/get_best_move`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(data)
        });
        if (resp.status !== 200) {
            const text = await resp.text();
            let msg = `server returned bad status ${resp.status}`;
            if (text.length !== 0) {
                msg += ": " + text;
            }
            throw new Error(msg);
        }
        const best_move = (await resp.json()) as GetBestMoveResponse;
        return best_move;
    }

    do_computer_move() {
        // this.do_random_move();
        this.do_server_request()
            .then((best_move: GetBestMoveResponse) => {
                console.debug("best_move =", best_move);
                const last_move = this.chess.move(best_move.m);
                if (this.game_mode !== GameMode.CC) {
                    this.enable_user_input();
                }
                this.update_view(last_move);
                this.highlight_move(last_move);
            })
            .catch((e) => {
                this.dispatchEvent(new LogMessageEvent("request failed: " + e));
            })
            .finally(() => {
                this.dispatchEvent(new RequestDoneEvent());
            });
    }

    private inputHandler(event: MoveInputEvent) {
        switch (event.type) {
            case INPUT_EVENT_TYPE.moveInputStarted: {
                const e = event as MoveInputStartedEvent;
                return this.handle_move_input_started(e);
            }
            case INPUT_EVENT_TYPE.movingOverSquare: {
                const e = event as MovingOverSquareEvent;
                return this.handle_moving_over_square(e);
            }
            case INPUT_EVENT_TYPE.validateMoveInput: {
                const e = event as ValidateMoveInputEvent;
                return this.handle_validate_move_input(e);
            }
            case INPUT_EVENT_TYPE.moveInputFinished: {
                const e = event as MoveInputFinishedEvent;
                if (e.legalMove) {
                    event.chessboard.disableMoveInput();
                }
                event.chessboard.removeLegalMovesMarkers();
                event.chessboard.removeMarkers(AutoMarker);
            }
        }
    }

    private check_game_over() {
        if (this.chess.isGameOver()) {
            let msg = "";
            if (this.chess.isCheckmate()) {
                msg = "checkmate";
            } else if (this.chess.isStalemate()) {
                msg = "stalemate";
            } else if (this.chess.isDraw()) {
                msg = "draw";
            }
            this.board.disableMoveInput();
            this.dispatchEvent(new GameOverEvent(msg));
        }
    }

    private enable_user_input() {
        if (this.game_mode === GameMode.WUBC && this.chess.turn() === COLOR.white) {
            this.board.enableMoveInput(
                (e: MoveInputEvent) => this.inputHandler(e),
                COLOR.white
            );
        } else if (
            this.game_mode === GameMode.BUWC &&
            this.chess.turn() === COLOR.black
        ) {
            this.board.enableMoveInput(
                (e: MoveInputEvent) => this.inputHandler(e),
                COLOR.black
            );
        }
    }

    private handle_move_input_started(e: MoveInputStartedEvent): boolean {
        const moves = this.chess.moves({
            square: e.squareFrom,
            verbose: true
        });
        this.last_valid_moves = new Map(moves.map((m) => [m.lan, m]));
        e.chessboard.removeMarkers(AutoMarker);
        e.chessboard.removeLegalMovesMarkers();
        if (
            this.chess.get(e.squareFrom).color === this.chess.turn() &&
            moves.length > 0
        ) {
            e.chessboard.addMarker(AutoMarker, e.squareFrom);
        }

        e.chessboard.addLegalMovesMarkers(moves);
        return moves.length > 0;
    }

    private handle_moving_over_square(e: MovingOverSquareEvent) {
        this.board.removeMarkers(AutoMarker);
        this.board.addMarker(AutoMarker, e.squareFrom);
        if (e.squareTo === null) {
            return;
        }
        if (this.last_valid_moves.has(e.squareFrom + e.squareTo)) {
            this.board.addMarker(AutoMarker, e.squareTo);
        }
    }

    private handle_validate_move_input(event: ValidateMoveInputEvent): boolean {
        const move = {
            from: event.squareFrom,
            to: event.squareTo
        };
        try {
            const move_result = this.chess.move(move);
            event.chessboard.state.moveInputProcess.then(() => {
                // update position, maybe castled
                this.update_view(move_result);
                this.do_computer_move();
            });
            return true;
        } catch (er) {
            // maybe promotion
            return this.handle_promotion(event);
        }
    }

    private handle_promotion(event: ValidateMoveInputEvent): boolean {
        for (const [_lan, m] of this.last_valid_moves) {
            if (m.promotion && m.to === event.squareTo) {
                event.chessboard.showPromotionDialog(
                    event.squareTo,
                    COLOR.white,
                    (result: PromotionDialogResult) => {
                        console.debug("promotion result", result);
                        if (
                            result.type ===
                            PROMOTION_DIALOG_RESULT_TYPE.pieceSelected
                        ) {
                            const selected = result as PromotionDialogPieceSelected;
                            const move_result = this.chess.move({
                                from: event.squareFrom,
                                to: event.squareTo,
                                promotion: selected.piece.charAt(1)
                            });
                            this.update_view(move_result);
                            this.do_computer_move();
                        } else {
                            // promotion canceled
                            this.enable_user_input();
                            // undo pawn move
                            event.chessboard.setPosition(this.chess.fen(), true);
                        }
                    }
                );
                return true;
            }
        }
        return false;
    }
}
