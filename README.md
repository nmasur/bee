# Bee

Simple program to solve the [NYTimes Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee) using a dictionary of every English word, taken from [here](https://raw.githubusercontent.com/elasticdog/yawl/master/yawl-0.3.2.03/word.list).

This is a really rough solution, so the output will not necessarily contain the same word list that NYT uses. Some of their words may not be in this list, and many in this list will not count. If you want the actual answers, it should be pretty easy to look them up. This is just a fun experiment.

## Installation

See [releases](https://github.com/nmasur/bee/releases) page for binaries.

On MacOS, you can also install from Homebrew:

```
brew tap nmasur/repo
brew install nmasur/repo/bee
```

Alternatively, build from source using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```
git clone git://github.com/nmasur/bee
cd bee
cargo build --release
```

## Quick Start

Run `bee`.

First, input all of the letters in the puzzle, including the required letter.

```
Letters: abcdefg
```

Then, add just the required (center) letter.

```
Required letters: a
```

The program will output the full list of possible words.


## Rules

1. Each word must be more than 4 characters long.
2. You can only use the letters in the puzzle, but they can be used as many times as you want.
3. The required letter (center letter) must be used at least once.
4. Every puzzle contains at least one "pangram," in which every letter is used at least once.
