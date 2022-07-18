type t

val init : unit -> t
val add : t -> Chip.t -> t
val add_chips : t -> Chip.t list -> t
val shuffle : t -> t
val draw : t -> Chip.t option * t
val draw_n : t -> int -> Chip.t list * t
val count : ?kind:Chip.ingredient -> ?min:int -> t -> int
