# Theseus and the Minotaur

## Read Before You Start

1. When you are finished, submit this repository on Gradescope. You can
   resubmit as many times as you like until the deadline.

2. Submissions should not have compilation warnings or errors.

3. See the course syllabus for the collaboration policy. All code
   submitted should be your own.

4. For an easy and feature-complete Rust editing experience, consider using
   VSCode with the
   [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
   extension. You are, of course, welcome to use any editor of your choice, but
   we can only guarantee the ability to help you debug editor configuration
   issues for VSCode.

5. Code you submit should be formatted with `rustfmt`, the standard Rust
   formatter. If you are using the editor setup described in (4), using
   VSCode's built-in formatting functionality will take care of this. If you
   submit code with a significant amount of formatting issues, we may deduct
   points.

6. As always, questions about the assignment of any form are welcome on EdStem.
   If you are posting a question with code you've written for an assignment,
   make sure to make it private.

## Introduction

[Theseus and the
Minotaur](https://en.wikipedia.org/wiki/Theseus_and_the_Minotaur) is a
pen-and-paper maze game first released in a puzzle book in 1990. A puzzle might look like the following:

![](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/Theseus_and_the_Minotaur_puzzle.png/330px-Theseus_and_the_Minotaur_puzzle.png)

Where the blue dot is Theseus (the player) and the red dot is the Minotaur (the
enemy). The goal of the game is for the Theseus to escape the maze while
avoiding the Minotaur. Unfortunately for Theseus, the Minotaur is able to move
two tiles for every one tile Theseus is able to move. Luckily, the Minotaur is
not very smart and will always move predictably:

1. If the Minotaur can move horizontally to draw closer to Theseus, it will.
2. If (1) isn't applicable but the Minotaur can move vertically to draw closer to Theseus, it will.
3. If neither (1) nor (2) is applicable, the Minotaur stays in place.

By taking advantage of these simple rules, Theseus can trick the Minotaur and
successfully escape the maze.

In this project, you will implement Theseus and the Minotaur in Rust to gain
experience with defining your own types, handling errors, and thinking about
ownership.

## Setup

As with previous projects, you will need to copy this template repository and
clone your copy to your local machine.

## Part 0: Understanding the Code

The code for this project is split into two files: `lib.rs` and `main.rs`. All
the code you write to implement the game logic will be in `lib.rs`. `main.rs`
contains a simple driver program that uses your code to play the game.

As you complete the assignment, you will add members to the `Game` struct and
implement the functions in the `Game` impl block. Part of the challenge of this
assignment is to determine what members you need to add to the `Game` struct
and what helper functions will be useful. Make sure you don't change any of the
provided function signatures, as this will cause the autograder to fail.


## Part 1: Loading Boards

The first step in implementing Theseus and the Minotaur is to load a maze. Our
mazes are stored in `.txt` files where `X` represents a wall, `T` represents
Theseus, `M` represents the Minotaur, and `G` represents the goal. An example maze might look like this:

```
XXXXXXXXX
X  T  XXX
X XXX XXX
X   X  GX
X XXX XXX
X  M  XXX
XXXXXXXXX
```

You can find a few example mazes in the files `board0.txt`, `board1.txt`, and `board2.txt`.

**Your first task is to decide what fields to add to your `Game` struct and fill in
the `Game::from_board` function, which takes a Board represented as a string
and returns a `Game` struct.**

Keep in mind the code you write doesn't need to be limited to the provided
structs and functions. You are welcome to define additional structs and enums
to help you structure your code better. To get you started, we have provided
the `Grid` struct which might be a helpful abstraction to keep inside your game
struct to represent the current game board. If you don't find this helpful,
feel free to ignore it.

**Once you have written the `Game::from_board` function, fill in the provided
functions  `is_theseus`, `is_minotaur`, `is_wall`, `is_goal`, and `is_empty`.**
These functions are used by the autograder and should be simple to write once
you decide on the fields of your `Game` struct.

To run the tests for this section, run
```
cargo test load_board
```

### Tips:

You will find the `Vec<T>` type very useful for both parsing and storing the
maze. This is a resizable list type (analogus to ArrayList in Java, list in
Python, or std::vector in C++).
* You can create a new `Vec` with `let v = Vec::new()`.
* You can add elements with `v.push(element)`.
* You can access elements with `v[index]` to panic on out-of-bounds errors or access elements with `v.get(index)` to return an `Option<T>`.
* You can get the length of the vector with `v.len()`.
* You can iterate over the elements with `for element in v.iter() { ... }` or `for (i, element) in v.iter().enumerate() { ... }`.

For more info, see [The Rust
Book](https://doc.rust-lang.org/book/ch08-01-vectors.html)

You may need to iterate over the characters or lines in a string. You can do
this with `for line in s.lines() { ... }` and `for c in line.chars() { ... }`.
You can type a character literal in Rust using single quotes, like `'X'`.

`Game::from_board` returns a Result type as discussed in lecture 3. If the
board is invalid for any reason, you should return the appropriate error. For a
list of possible errors, see the `GameError` enum.

## Part 2: Printing Boards

Now that you can load a board, you need to be able to print it. Implement the
`Game::show` function, which should print the current board. There are
no hard requirements for how you do this, but a reasonable start would be to
print something similar to the board input format. Make sure the player is able
to clearly distinguish where Theseus is, where the Minotaur is, where the walls
are, and where the exit is. You may find the "block" character `█` useful for
printing walls.

There are no tests for this section, but you should test your implementation
by running the game, which can be done as
```
cargo run -- board0.txt
```
If you see a maze printed to the console, you have successfully implemented part 2!

## Part 3: Handling Input and Moving Theseus

Now that you can load and print boards, you need to be able to take user input
to determine where Theseus should move.

**Fill in the `input` function in `lib.rs` to take user input and return an
element of the `Command` struct.**

Taking input on the command line can be tricky, so read the comments attached
to the `input` function to guide your implementation.

Once you're able to take input, you can use that input to move Theseus.

**Fill in the `Game::move_theseus` function which takes a `Command` and updates
the game state to reflect that command.**

Once you've completed this section, run the tests with

```
cargo test theseus
```

Additionally, to make sure your input code is working run

```
cargo run -- board0.txt
```

and ensure that you are able to move Theseus.

### Notes:

* If the user attempts to make an invalid move (like into a wall), Theseus should
not move and the turn should end.

**Fill in the `input` function and `Game::move_theseus` function.**

## Part 4: Moving the Minotaur

Almost done! The final step is moving the minotaur. We can start by making the
logic for the Minotaur's movement more precise. For each call to
`minotaur_move`, the Minotaur follows the following logic to pick a move, where
the valid Minotaur movements are one tile to the left, one tile to the right,
one tile up, one tile down, or no movement. The Minotaur can't make a move into a wall.

_Suppose the player is at position (tx,ty) and the Minotaur is at position (mx,my)._

1. If the Minotaur can make a move which decreases the distance `abs(tx - mx)`, it will make that move.
2. If not, if the minotaur can make a move which decreases the distance `abs(ty - my)`, it will make that move.
3. Otherwise, the Minotaur will not move.

To understand this behavior better, you can try [playing the game
online](https://estivalet.github.io/theseus-minotaur/) or looking at the test
case `test_minotaur_move_basic` in the file `tests/test.rs`.

**Fill in the function `Game::minotaur_move` which takes a game state and
updates it to reflect one minotaur move.**

With that done, you should be able to run the Minotaur tests
```
cargo test minotaur
```

Additionally, you should be able to play the game while running away from the Minotaur.

```
cargo run -- board0.txt
```

## Part 5: Finishing Up

If you played through a game after completing the previous part, you'll have
noticed that the game goes on forever even if the Minotaur catches Theseus or
Theseus reaches the goal. We need to implement one final function to check for
any game over condition.

**Fill in the `Game::status` function which determines whether the game is in a
finish state or should keep going.**

You should now be able to run the final tests

```
cargo test status
```

And play the game to completion

```
cargo run -- board.txt
```

## All done

To double check, you can run all tests with

```
cargo test
```

When you are finished, submit your repository to Gradescope and ensure that the
autograder runs properly. Additionally, ensure your code compiles without any
warnings and that you are able to successfully play the game, as some
functionality is not captures by the tests and will be graded manually.
