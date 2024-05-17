let rec trimmer d = match d with
    | [] -> []
    | x :: y -> let length = String.length x in
        if String.ends_with ~suffix:"," x then
            String.sub x 0 (length-1) :: trimmer y
        else
            x :: trimmer y

let data =
    let ic = open_in "data/input" in
    try
        let line = input_line ic in
        close_in ic;
        trimmer (String.split_on_char ' ' line)
    with e ->
        close_in_noerr ic;
        raise e

let () = List.iter (Printf.printf "%s\n") data

let res = Day01.Walker.walker data Day01.Walker.North
let () = Printf.printf "%d\n" (Day01.Walker.distance res)
