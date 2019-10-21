# todo-macro

A macro in Rust that stops compiling after a user-specified deadline. Like TODO comments but meaner.

This is just a dumb (but fun) toy, don't use it in a real project unless you're *really* comfortable with non-deterministic builds.

## Example

It's January 1, 2020. I'm working on some Rust code that compiles, but it's not quite ready to ship. 

I want to take a break, but I know myself – I'll probably forget about the deficiency. I could add a TODO comment, but that depends on me actively searching for TODO comments next time I open the project.

To save me from myself, I add a quick `todo` macro with a deadline of January 2 (in ISO 8601 format):

```rust
// Implement the timeout handling
todo!("2020-01-02")
```

That compiles for now, but as soon as the deadline is passed (i.e. our system clock returns Jan 3), builds start failing:

```
error: proc macro panicked
 --> src/main.rs:5:5
  |
5 |     todo!("2020-01-02");
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: Tsk tsk. You missed your deadline.
```

Think of `todo` like a reminder that actively forces your future self to deal with a problem.

## TODO for todo-macro

It would be cool if users didn't even have to specify a date. Like maybe they call `todo_in_x_days!(3)` and that magically stops compiling after 3 days. This might be doable with some extremely hacky local state, but I'd definitely go to compiler hell for that.

I wonder whether this could function like Scala's [`???`](https://alvinalexander.com/scala/what-does-three-question-marks-in-scala-mean) (not implemented) expression, so it replaces entire expressions instead of being used like a statement.

## Prerequisites

You'll need to add `#![feature(proc_macro_hygiene)]` to your code that uses `todo`, at least until [this Rust issue](https://github.com/rust-lang/rust/issues/54727) is sorted out.