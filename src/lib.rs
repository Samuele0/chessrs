#![allow(clippy::wildcard_imports)]
#![allow(unused_imports)] // TODO: Remove
mod board;
mod minimax;
use board::*;
use minimax::*;
use seed::{prelude::*, *};
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board: Default::default(),
        selected: None,
    }
}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    board: ChessBoard,
    selected: Option<(usize, usize)>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Select(usize, usize),
    EnemyMove(usize, usize, usize, usize),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, o: &mut impl Orders<Msg>) {
    match msg {
        Msg::Select(x, y) => {
            if let Some((x1, y1)) = model.selected {
                if (x1, y1) == (x, y) {
                    model.selected = None;
                    return;
                }
                let piece = model.board.get(x1, y1);
                if let Some(_p) = piece {
                    /* if p.can_move(x, y) {
                        model.board.make_move((x1, y1), (x, y));
                        model.selected = None;
                        return;
                    } */
                    if model.board.can_move(x1, y1, x, y) {
                        model.board.make_move((x1, y1), (x, y));
                        model.selected = None;
                        o.perform_cmd({
                            let clonedb= model.board.clone();
                            async {
                                let mov = maximize(clonedb, 0);
                                Msg::EnemyMove(mov.start.0, mov.start.1, mov.end.0, mov.end.1)
                            }
                        });
                        return;
                    }
                }
            }
            let piece = model.board.get(x, y);
            if let Some(ChessPiece {
                piece_color: PieceColor::White,
                ..
            }) = piece
            {
                model.selected = Some((x, y));
            }
        }
        Msg::EnemyMove(x0, y0, x1, y1) => {
            model.board.make_move((x0, y0), (x1, y1));
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container"],
        table![
            C!["chessboard"],
            // Draw checkboard
            (0..8).map(|i| {
                tr![(0..8).map(|j| {
                    td![
                        C![if (i + j) % 2 == 0 { "white" } else { "black" }],
                        C![if let Some((x, y)) = model.selected {
                            if x == j && y == i {
                                "selected"
                            } else {
                                ""
                            }
                        } else {
                            ""
                        },],
                        ev(Ev::Click, move |_| Msg::Select(j, i))
                    ]
                })]
            }),
        ],
        model.board.pieces.iter().map(|p| {
            if p.position.is_some() {
                div![
                    C!["piece"],
                    img![attrs! {
                        At::Src => format!("./imgs/{}.svg",p)
                    }],
                    style![
                        St::Position => "absolute",
                        St::Top => format!("{}rem", p.position.unwrap().1*5),
                        St::Left => format!("{}rem", p.position.unwrap().0*5),
                    ]
                ]
            } else {
                empty!()
            }
        })
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
