open Alcotest

let run_test data expected ()=
    let res = Day01.Walker.walker data Day01.Walker.North in
    let dist = Day01.Walker.distance res in
        check int "same int" expected dist

let suite = [
    "", `Quick, run_test ["R2"; "L3"] 5;
    "", `Quick, run_test ["R2"; "R2"; "R2"] 2;
    "", `Quick, run_test ["R5"; "L5"; "R5"; "R3"] 12;
]

let () = Alcotest.run "Dummy" [ "Walk", suite ]
