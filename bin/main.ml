open Base
open Stdio
open Quacks

let () =
  let player : Game.player =
    {
      draw_again =
        (fun state cauldron ->
          let open Float in
          Cauldron.change_to_explode cauldron state.bag < 0.25);
      decide_blue =
        (fun _ _ chips ->
          (*
          printf "    draw blue [%s]: "
            (List.map chips ~f:Chip.show |> String.concat ~sep:", ");
          *)
          List.findi chips ~f:(fun _ c ->
              not (Chip.is_same_kind ~kind:White_snowberries c))
          |> function
          | Some (choice_idx, choice) ->
              printf "%s\n" (Chip.show choice);
              choice_idx
          | None ->
              printf "none\n";
              -1);
      purchase_or_move = (fun _ _ -> `Purchase);
      spend_flask = (fun _ _ -> false);
      buy_chips =
        (fun _ lst ->
          (*
          List.iter lst ~f:(fun chips ->
              List.map chips ~f:(fun c -> Chip.show c)
              |> String.concat ~sep:", " |> printf "  %s\n");
              *)
          List.findi lst ~f:(fun _ lst ->
              List.exists lst ~f:(Chip.is_same_kind ~kind:Blue_crow_skull))
          |> Option.value_map ~default:(Random.int (List.length lst)) ~f:fst);
      (* on_chip_added = (fun c -> printf "x  %s\n" (Chip.show c)); *)
      on_chip_added = ignore;
      on_cauldron_full =
        (fun cauldron ->
          let score_field = Cauldron.score cauldron in
          printf "coins %d, points: %d%s%s\n" score_field.coins
            score_field.victory_points
            (if score_field.ruby then ", ruby" else "")
            (if Cauldron.is_exploded cauldron then " EXPLODED" else ""));
    }
  in
  let cauldron : Cauldron.t = { chips = []; explosion_limit = 7; value = 0 } in
  let state = Game.init () in
  let state = Game.do_round state player cauldron in
  let _state = Game.do_round state player cauldron in
  ()
