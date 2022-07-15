open Base
open List_extension

type t = Chip.t list

let init : unit -> Chip.t list =
 fun () ->
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

let draw : t -> Chip.t * t =
 fun bag ->
  let n = Random.int (List.length bag) in
  let bag, picked = List.pick_nth bag n in
  (Option.value_exn ~message:"attempt to draw from empty bag" picked, bag)

let draw_n : t -> int -> Chip.t list * t =
 fun bag n ->
  List.fold ~init:(bag, []) (List.range 0 n) ~f:(fun (bag, lst) _ ->
      let c, bag = draw bag in
      (c :: lst, bag))

let add : t -> Chip.t -> t = fun bag chip -> chip :: bag
let add_chips : t -> Chip.t list -> t = List.append

let rec shuffle : t -> t =
 fun bag ->
  let cnt = List.length bag in
  if cnt < 1 then []
  else
    let rest, picked = List.pick_nth bag (Random.int cnt) in
    match picked with Some c -> c :: shuffle rest | None -> []
