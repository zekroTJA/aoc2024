# Advent of Code Rust Template

You can use this template repository to set up a workspace to solve Advent of Code challenges using Rust with a lot of handy tooling.

## Usage

First of all, you should install the `task` tool. [Here you can find out how.](https://taskfile.dev/installation/)

Now, use `task --list` to see which tasks are available.

```
task: Available tasks for this project:
* commit:        Commit the latest day solution.      (aliases: c)
* init:          Initialize your workspace.           (aliases: i)
* new:           Create a new day project.            (aliases: n)
* run:           Run solution with user input.        (aliases: r)
* runtest:       Run solution with test input.        (aliases: rt)
* test:          Run all unit tests.                  (aliases: t)
```

### Setup

To setup your workspace, first use `task init`. After that, a `.env` file is created with a variable `SESSION_TOKEN`. Now, log in to [adventofcode.com](https://adventofcode.com), open your browser's developer tools and copy the `session` cookie value to the variable.

### Workflow

Each day, you can now run `task new` to create a new directory for the next day. This directory contains your user input (`input.txt`) as well as a file `test_input.txt`, which you can use to run your code on the test input declared in the challenge.

When you have developed your solution, you can run the current day with `task run`. Alternatively, you can also select the day to be run with `task run -- 12`.

After that, you can use `task commit` to automatically commit and push your solution. This also checks that all tests and that the code compiles and runs before committing.

Also, it checks that the `challenge.txt` is not empty. It should contain the text of the entire challenge from the Advent of Code website. This is currently not automated, so you need to copy it manually before committing.