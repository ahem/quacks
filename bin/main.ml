open Base
open Stdio
open Quacks

let is_white = Chip.is_same_kind ~kind:White_snowberries

module Strategies = struct
  let prefer_to_buy : Chip.ingredient -> Game.state -> Chip.t list list -> int =
   fun kind _ lst ->
    List.findi lst ~f:(fun _ lst ->
        List.exists lst ~f:(Chip.is_same_kind ~kind))
    |> Option.value_map ~default:(Random.int (List.length lst)) ~f:fst

  let decide_blue_prefer :
      Chip.ingredient -> Game.state -> Cauldron.t -> Chip.t list -> int =
   fun kind _ _ chips ->
    printf "    draw blue [%s]: "
      (List.map chips ~f:Chip.show |> String.concat ~sep:", ");
    let is_wanted = Chip.is_same_kind ~kind in
    let choice_idx =
      List.mapi chips ~f:(fun i c -> (c, i))
      |> List.filter ~f:(fun (c, _) -> not (is_white c))
      |> List.sort ~compare:(fun a b ->
             let a = fst a and b = fst b in
             match (is_wanted a, is_wanted b) with
             | true, true -> snd b - snd a
             | false, true -> 1
             | true, false -> -1
             | false, false -> 0)
      |> List.hd
      |> Option.value_map ~default:(-1) ~f:snd
    in
    if choice_idx >= 0 then
      let choice = List.nth_exn chips choice_idx in
      printf "%s\n" (Chip.show choice)
    else printf "none\n";
    choice_idx
end

let () =
  let player : Game.player =
    {
      draw_again =
        (fun state cauldron ->
          let open Float in
          Cauldron.change_to_explode cauldron state.bag < 0.1);
      decide_blue = Strategies.decide_blue_prefer Chip.Blue_crow_skull;
      purchase_or_move = (fun _ _ -> `Purchase);
      spend_flask = (fun _ _ -> false);
      buy_chips = Strategies.prefer_to_buy Chip.Blue_crow_skull;
      on_chip_added = (fun c -> printf "  %s\n" (Chip.show c));
      (* on_chip_added = ignore; *)
      on_cauldron_full =
        (fun cauldron ->
          let score_field = Cauldron.score cauldron in
          printf "coins %d, points: %d%s%s\n" score_field.coins
            score_field.victory_points
            (if score_field.ruby then ", ruby" else "")
            (if Cauldron.is_exploded cauldron then " EXPLODED"
            else if Cauldron.is_full cauldron then " FULL"
            else ""));
    }
  in
  let state =
    List.range 0 9
    |> List.fold ~init:(Game.init ()) ~f:(fun state _ ->
           Game.do_round state player
             { chips = []; explosion_limit = 7; value = state.drop_position })
  in
  printf "FINAL SCORE: %d (%d rubies)\n" state.score state.rubies
