# Difference of Squares

Welcome to Difference of Squares on Exercism's MIPS Assembly Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Instructions

Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.

The square of the sum of the first ten natural numbers is
(1 + 2 + ... + 10)² = 55² = 3025.

The sum of the squares of the first ten natural numbers is
1² + 2² + ... + 10² = 385.

Hence the difference between the square of the sum of the first ten natural numbers and the sum of the squares of the first ten natural numbers is 3025 - 385 = 2640.

You are not expected to discover an efficient solution to this yourself from first principles; research is allowed, indeed, encouraged.
Finding the best algorithm for the problem is a key skill in software engineering.

## Registers

| Register | Usage     | Type    | Purpose                    |
| -------- | --------- | ------- | -------------------------- |
| `$a0`    | input     | integer | given value                |
| `$v0`    | output    | integer | difference of squares      |
| `$t0-9`  | temporary | any     | used for temporary storage |

## Source

### Created by

- @mpriestman

### Contributed to by

- @ozan

### Based on

Problem 6 at Project Euler - https://projecteuler.net/problem=6