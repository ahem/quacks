open Base
open Quacks

let () =
  let player : Game.player =
    {
      draw_again =
        (let open Float in
        fun state cauldron ->
          Cauldron.change_to_explode cauldron state.bag < 0.25);
      decide_blue = (fun _ _ chips -> Random.int (List.length chips));
      purchase_or_move = (fun _ _ -> `Purchase);
      spend_flask = (fun _ _ -> false);
      buy_chips =
        (fun _ lst ->
          List.findi lst ~f:(fun _ lst ->
              List.exists lst ~f:(Chip.is_same_kind ~kind:Blue_crow_skull))
          |> Option.value_map ~default:(Random.int (List.length lst)) ~f:fst);
    }
  in
  let cauldron : Cauldron.t = { chips = []; explosion_limit = 7; value = 0 } in
  let state = Game.init () in
  let state = Game.do_round state player cauldron in
  let _state = Game.do_round state player cauldron in
  ()
