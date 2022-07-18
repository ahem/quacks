open Base

module List = struct
  include List

  let remove_nth : 'a t -> int -> 'a option * 'a t =
   fun lst n ->
    if n < 0 then (None, lst)
    else
      let hd, tail = List.split_n lst n in
      match tail with
      | x :: tail -> (Some x, List.append hd tail)
      | [] -> (None, hd)
end
