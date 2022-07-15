open Base
open List_extension

type state = {
  flask : bool;
  score : int;
  rubies : int;
  bag : Bag.t;
  drop_position : int;
}

let init () =
  { flask = true; score = 0; rubies = 0; bag = Bag.init (); drop_position = 0 }

let ingredient_list : (Chip.t * int) list =
  [
    ((Orange_pumpkin, 1), 3);
    ((Blue_crow_skull, 1), 5);
    ((Blue_crow_skull, 2), 10);
    ((Blue_crow_skull, 4), 19);
    ((Red_toadstool, 1), 6);
    ((Red_toadstool, 2), 10);
    ((Red_toadstool, 4), 16);
    ((Yellow_mandrake, 1), 8);
    ((Yellow_mandrake, 2), 12);
    ((Yellow_mandrake, 4), 18);
    ((Green_garden_spider, 1), 4);
    ((Green_garden_spider, 2), 8);
    ((Green_garden_spider, 4), 14);
    ((Purple_ghosts_breath, 1), 9);
    ((Black_deaths_head_hawkmoth, 1), 10);
  ]

type player = {
  spend_flask : state -> Cauldron.t -> bool;
  (* true means draw another chip, false means stop *)
  draw_again : state -> Cauldron.t -> bool;
  (* given a list of chips, return an index in the list. Non-existing means put all back *)
  decide_blue : state -> Cauldron.t -> Chip.t list -> int;
  (* after the pot have exploded, player must select purchase or move *)
  purchase_or_move : state -> Cauldron.t -> [ `Move | `Purchase ];
  (* given a list of possible choices (lists with one or two elements), return an index in the list with the wanted chips *)
  buy_chips : state -> Chip.t list list -> int;
}

let rec add_chip : player -> state -> Cauldron.t -> Chip.t -> state * Cauldron.t
    =
 fun player state cauldron chip ->
  match fst chip with
  | Blue_crow_skull -> (
      let cauldron = Cauldron.add cauldron chip in
      (* draw some chips, player may select 1 *)
      let drawn, bag = Bag.draw_n state.bag (snd chip) in
      let drawn, selected =
        List.pick_nth drawn (player.decide_blue state cauldron drawn)
      in
      (* return the rest *)
      let state = { state with bag = Bag.add_chips bag drawn } in
      (* if a chip was selected, add that *)
      match selected with
      | Some selected_chip -> add_chip player state cauldron selected_chip
      | None -> (state, cauldron))
  | Red_toadstool ->
      let cauldron = Cauldron.add cauldron chip in
      let orange_cnt = Cauldron.count cauldron Orange_pumpkin in
      let bonus =
        if orange_cnt = 0 then 0 else if orange_cnt < 3 then 1 else 2
      in
      let cauldron = Cauldron.incr cauldron bonus in
      (state, cauldron)
  | Yellow_mandrake ->
      let cauldron =
        match Cauldron.last cauldron with
        | Some (White_snowberries, value) ->
            Cauldron.incr (Cauldron.drop_last cauldron) value
        | _ -> cauldron
      in
      let cauldron = Cauldron.add cauldron chip in
      (state, cauldron)
  | Black_deaths_head_hawkmoth | Green_garden_spider | Orange_pumpkin
  | Purple_ghosts_breath | White_snowberries ->
      let cauldron = Cauldron.add cauldron chip in
      (state, cauldron)

let eval_green : state -> player -> Cauldron.t -> state * Cauldron.t =
 fun state _player cauldron ->
  let is_green = function
    | None -> false
    | Some c -> Chip.is_same_kind ~kind:Green_garden_spider c
  in
  if is_green (Cauldron.last cauldron) || is_green (Cauldron.next_last cauldron)
  then ({ state with rubies = state.rubies + 1 }, cauldron)
  else (state, cauldron)

let rec fill_cauldron : state -> player -> Cauldron.t -> state * Cauldron.t =
 fun state player cauldron ->
  let chip, bag = Bag.draw state.bag in
  let state, cauldron = add_chip player { state with bag } cauldron chip in
  let state = { state with bag } in

  Out_channel.output_string Out_channel.stdout
  @@ Printf.sprintf "  %s\n" (Chip.show chip);

  if Cauldron.is_exploded cauldron then (state, cauldron)
  else
    let state, cauldron =
      match Cauldron.last cauldron with
      | Some (White_snowberries, _) ->
          if state.flask && player.spend_flask state cauldron then
            ({ state with flask = false }, cauldron)
          else (state, cauldron)
      | _ -> (state, cauldron)
    in
    if player.draw_again state cauldron then fill_cauldron state player cauldron
    else (state, cauldron)

let price_of : Chip.t -> int =
 fun chip ->
  let _, price =
    List.find_exn ingredient_list ~f:(fun (c, _) -> Poly.equal chip c)
  in
  price

let purchase_chips : state -> player -> int -> state =
 fun state player coins ->
  (* find available choices *)
  let choices =
    List.filter ingredient_list ~f:(fun (_, price) -> price <= coins)
    |> List.map ~f:(fun (chip, price) ->
           let is_same_color = Chip.is_same_kind ~kind:(fst chip) in
           let coins_left = coins - price in
           let second_choice =
             List.filter ingredient_list ~f:(fun (c, price) ->
                 (not (is_same_color c)) && price <= coins_left)
           in
           match second_choice with
           | [] -> [ [ chip ] ]
           | _ ->
               List.map second_choice ~f:(fun (c, p) ->
                   if price > p then [ chip; c ] else [ c; chip ]))
    |> List.concat
  in

  (* remove duplicates *)
  let choices =
    List.sort choices ~compare:Poly.compare
    |> List.remove_consecutive_duplicates ~equal:Poly.equal
  in

  (* sort by price *)
  let choices =
    List.sort choices ~compare:(fun a b ->
        let a = List.map a ~f:price_of |> List.fold ~init:0 ~f:( + ) in
        let b = List.map b ~f:price_of |> List.fold ~init:0 ~f:( + ) in
        Int.compare b a)
  in

  (* log *)
  List.iter choices ~f:(fun chips ->
      List.map chips ~f:(fun c -> Chip.show c)
      |> String.concat ~sep:", " |> Printf.sprintf "  %s\n"
      |> Out_channel.output_string Out_channel.stdout);

  (* ask player and perform purchase *)
  match List.nth choices (player.buy_chips state choices) with
  | None -> state
  | Some chips ->
      List.map chips ~f:(fun c -> Chip.show c)
      |> String.concat ~sep:", "
      |> Printf.sprintf "bought: %s\n"
      |> Out_channel.output_string Out_channel.stdout;
      { state with bag = Bag.add_chips state.bag chips }

let spend_rubies : state -> player -> state =
 fun state _player ->
  (* TODO *)
  state

let do_round : state -> player -> Cauldron.t -> state =
 fun state player cauldron ->
  let bag = state.bag in
  let state, cauldron = fill_cauldron state player cauldron in

  (* refill bag *)
  let state = { state with bag } in

  (* TODO: roll bonus die *)
  (* TODO: eval black *)
  (* eval green *)
  let state, cauldron = eval_green state player cauldron in
  (* TODO: eval purple *)
  let score_field = Cauldron.score cauldron in

  Out_channel.output_string Out_channel.stdout
    (Printf.sprintf "coins %d, points: %d%s%s\n" score_field.coins
       score_field.victory_points
       (if score_field.ruby then ", ruby" else "")
       (if Cauldron.is_exploded cauldron then " EXPLODED" else ""));

  (* award ruby *)
  let state =
    if score_field.ruby then { state with rubies = state.rubies + 1 } else state
  in
  let state =
    if Cauldron.is_exploded cauldron then
      (* award score or purchase_chips *)
      match player.purchase_or_move state cauldron with
      | `Move -> { state with score = state.score + score_field.victory_points }
      | `Purchase -> purchase_chips state player score_field.coins
    else
      (* awards score and purchase chips *)
      purchase_chips
        { state with score = state.score + score_field.victory_points }
        player score_field.coins
  in
  (* spend rubies *)
  let state = spend_rubies state player in
  state
