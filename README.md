# Sudoku Solver

This project implements a simple backtracking Sudoku solver.
I created it mainly to learn Rust, so there's probably a lot of room for improvement.

Running the binary crate with `cargo run` will try to load a `puzzles.csv` file with pairs of games and solutions.

There's a great dataset [here](https://www.kaggle.com/datasets/bryanpark/sudoku).
If you want a quickstart, however, create the file with this content:
```
quizzes,solutions
004300209005009001070060043006002087190007400050083000600000105003508690042910300,864371259325849761971265843436192587198657432257483916689734125713528694542916378
040100050107003960520008000000000017000906800803050620090060543600080700250097100,346179258187523964529648371965832417472916835813754629798261543631485792254397186
600120384008459072000006005000264030070080006940003000310000050089700000502000190,695127384138459672724836915851264739273981546946573821317692458489715263562348197
```