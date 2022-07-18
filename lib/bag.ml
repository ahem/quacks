open Base
open List_extension

type t = Chip.t list

let init () =
  let open Chip in
  [
    (White_snowberries, 1);
    (White_snowberries, 1);
    (White_snowberries, 1);
    (White_snowberries, 1);
    (White_snowberries, 2);
    (White_snowberries, 2);
    (White_snowberries, 3);
    (Orange_pumpkin, 1);
    (Green_garden_spider, 1);
  ]

let add bag chip = chip :: bag
let add_chips = List.append
let shuffle = List.permute ?random_state:None

let draw bag =
  let len = List.length bag in
  if len > 0 then List.remove_nth bag (Random.int len) else (None, [])

let draw_n : t -> int -> Chip.t list * t =
 fun bag n ->
  if n > 0 then
    List.fold ~init:([], bag) (List.range 0 n) ~f:(fun (lst, bag) _ ->
        match draw bag with
        | Some c, bag -> (c :: lst, bag)
        | None, bag -> (lst, bag))
  else ([], bag)

let count ?kind ?min bag =
  let lst =
    match kind with
    | Some kind -> List.filter bag ~f:(Chip.is_same_kind ~kind)
    | None -> bag
  in
  let lst =
    match min with
    | Some min -> List.filter lst ~f:(fun (_, n) -> min >= n)
    | None -> lst
  in
  List.length lst
