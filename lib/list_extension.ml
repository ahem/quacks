open Base

module List = struct
  include List

  let pick_nth lst n =
    if n < 0 then (lst, None)
    else
      let hd, tail = List.split_n lst n in
      match tail with
      | x :: tail -> (List.append hd tail, Some x)
      | [] -> (hd, None)
end
