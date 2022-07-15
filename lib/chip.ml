open Base

type ingredient =
  | Black_deaths_head_hawkmoth
  | Blue_crow_skull
  | Green_garden_spider
  | Orange_pumpkin
  | Purple_ghosts_breath
  | Red_toadstool
  | White_snowberries
  | Yellow_mandrake

type t = ingredient * int

let show : t -> string = function
  | Black_deaths_head_hawkmoth, n -> Printf.sprintf "Black %d" n
  | Blue_crow_skull, n -> Printf.sprintf "Blue %d" n
  | Green_garden_spider, n -> Printf.sprintf "Green %d" n
  | Orange_pumpkin, n -> Printf.sprintf "Orange %d" n
  | Purple_ghosts_breath, n -> Printf.sprintf "Purple %d" n
  | Red_toadstool, n -> Printf.sprintf "Red %d" n
  | White_snowberries, n -> Printf.sprintf "White %d" n
  | Yellow_mandrake, n -> Printf.sprintf "Yellow %d" n

let is_same_kind : kind:ingredient -> t -> bool =
 fun ~kind chip -> phys_equal (fst chip) kind
