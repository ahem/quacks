open Base

type t = { value : int; chips : Chip.t list; explosion_limit : int }
type cauldron_field = { coins : int; victory_points : int; ruby : bool }

let playing_field =
  [
    { coins = 0; victory_points = 0; ruby = false };
    { coins = 1; victory_points = 0; ruby = false };
    { coins = 2; victory_points = 0; ruby = false };
    { coins = 3; victory_points = 0; ruby = false };
    { coins = 4; victory_points = 0; ruby = false };
    { coins = 5; victory_points = 0; ruby = true };
    { coins = 6; victory_points = 1; ruby = false };
    { coins = 7; victory_points = 1; ruby = false };
    { coins = 8; victory_points = 1; ruby = false };
    { coins = 9; victory_points = 1; ruby = true };
    { coins = 10; victory_points = 2; ruby = false };
    { coins = 11; victory_points = 2; ruby = false };
    { coins = 12; victory_points = 2; ruby = false };
    { coins = 13; victory_points = 2; ruby = true };
    { coins = 14; victory_points = 3; ruby = false };
    { coins = 15; victory_points = 3; ruby = false };
    { coins = 16; victory_points = 3; ruby = false };
    { coins = 16; victory_points = 4; ruby = false };
    { coins = 17; victory_points = 4; ruby = false };
    { coins = 17; victory_points = 4; ruby = true };
    { coins = 18; victory_points = 4; ruby = false };
    { coins = 18; victory_points = 5; ruby = false };
    { coins = 19; victory_points = 5; ruby = false };
    { coins = 19; victory_points = 5; ruby = true };
    { coins = 20; victory_points = 5; ruby = false };
    { coins = 20; victory_points = 6; ruby = false };
    { coins = 21; victory_points = 6; ruby = false };
    { coins = 21; victory_points = 6; ruby = true };
    { coins = 22; victory_points = 7; ruby = false };
    { coins = 22; victory_points = 7; ruby = true };
    { coins = 23; victory_points = 7; ruby = false };
    { coins = 23; victory_points = 8; ruby = false };
    { coins = 24; victory_points = 8; ruby = false };
    { coins = 24; victory_points = 8; ruby = true };
    { coins = 25; victory_points = 9; ruby = false };
    { coins = 25; victory_points = 9; ruby = true };
    { coins = 26; victory_points = 9; ruby = false };
    { coins = 26; victory_points = 10; ruby = false };
    { coins = 27; victory_points = 10; ruby = false };
    { coins = 27; victory_points = 10; ruby = true };
    { coins = 28; victory_points = 11; ruby = false };
    { coins = 29; victory_points = 11; ruby = false };
    { coins = 29; victory_points = 12; ruby = false };
    { coins = 30; victory_points = 12; ruby = false };
    { coins = 30; victory_points = 12; ruby = true };
    { coins = 31; victory_points = 12; ruby = false };
    { coins = 31; victory_points = 13; ruby = false };
    { coins = 32; victory_points = 13; ruby = false };
    { coins = 32; victory_points = 13; ruby = true };
    { coins = 33; victory_points = 14; ruby = false };
    { coins = 33; victory_points = 14; ruby = true };
    { coins = 35; victory_points = 15; ruby = false };
  ]

let add : t -> Chip.t -> t =
 fun cauldron chip ->
  {
    cauldron with
    value = cauldron.value + snd chip;
    chips = chip :: cauldron.chips;
  }

let contains : t -> Chip.ingredient -> bool =
 fun cauldron kind -> List.exists cauldron.chips ~f:(Chip.is_same_kind ~kind)

(* coundt the number of chips of a given color *)
let count : t -> Chip.ingredient -> int =
 fun cauldron kind ->
  List.count cauldron.chips ~f:(function chip -> Poly.equal (fst chip) kind)

(* total sum of the vaulues of all chips in the cauldron with the given color *)
let total : t -> Chip.ingredient -> int =
 fun cauldron kind ->
  List.filter cauldron.chips ~f:(function chip -> Poly.equal (fst chip) kind)
  |> List.fold ~init:0 ~f:(fun acc (_, n) -> acc + n)

let incr : t -> int -> t =
 fun cauldron n -> { cauldron with value = cauldron.value + n }

let drop_last : t -> t =
 fun cauldron ->
  match cauldron.chips with
  | (_, n) :: tail -> { cauldron with value = cauldron.value - n; chips = tail }
  | [] -> cauldron

let is_exploded : t -> bool =
 fun cauldron -> total cauldron White_snowberries > cauldron.explosion_limit

let last : t -> Chip.t option = fun cauldron -> List.hd cauldron.chips
let next_last : t -> Chip.t option = fun cauldron -> List.nth cauldron.chips 1

let score : t -> cauldron_field =
 fun cauldron -> List.nth playing_field cauldron.value |> Option.value_exn

let change_to_explode : t -> Bag.t -> float =
 fun cauldron bag ->
  let limit = cauldron.explosion_limit - total cauldron White_snowberries in
  let total_chip_count = List.length bag in
  let danger_chip_count =
    List.count bag ~f:(fun chip ->
        Chip.is_same_kind ~kind:White_snowberries chip && snd chip > limit)
  in
  Float.of_int danger_chip_count /. Float.of_int total_chip_count
