type direction = North | South | East | West
type rotation = L | R

let rotate d f = match d, f with
    | L, North -> East
    | L, East -> South
    | L, South -> West
    | L, West -> North
    | R, North -> West
    | R, West -> South
    | R, South -> East
    | R, East -> North

let get_steps x =
    let length = String.length x in
        Stdlib.int_of_string (String.sub x 1 (length-1))

let rec walker d f = match d with
    | [] -> (0, 0)
    | head :: rest -> let dir = if String.starts_with ~prefix:"L" head then L else R in
        let face = rotate dir f in
            match face with
            | North -> let (x, y) = walker rest face in
                (x + get_steps head, y)
            | South -> let (x_offset, y) = walker rest face in
                (x_offset - get_steps head, y)
            | East -> let (x, y) = walker rest face in
                (x, y + get_steps head)
            | West -> let (x, y) = walker rest face in
                (x, y - get_steps head)

let distance (x, y) = Int.abs(x) + Int.abs(y)
