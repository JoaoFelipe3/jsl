This is just a toy language, not for serious use. :)

# Syntax

You can write number literals by just writing the number, with an optimal decimal part (with `.`).

String literals are written with `"` enclosing the text, and the following escape sequences are supported:
* `\r` for Carriage Return
* `\n` for Line Feed (even though multiline strings are supported)
* `\t` for Tab
* `\"` to escape a double quote
* `\\` to escape a backslash
Any other escape seqeuence is *invalid*, and will error.

Code can be wrapped in `{}` to turn it into a *function*. This treats the code as a value in the stack, and the code can be called with `!` (more about later).

There are also literals for empty arrays and null values: `□` and `∅`, respectively.

Anything else other than primitives (more about later) is treated as an identifier. Identifiers can also be a sequence of letters (from any alphabet!).

You can bind values on the stack to those identifiers with the syntax `→<identifier>`.

# Primitives

This section documents *every* primitive in detail.

## Pop: `.`
Pops a value from the stack. Note that this does nothing if the stack is empty.

## Duplicate: `:`
Duplicates a value from the stack. Note that this pushes `∅` if the stack is empty.

## Flip: `⭥`
Swaps the top two values from the stack. Note that this does nothing if there are not enough values.

## Call: `!`
Calls a function (errors if the value is not a function).

All bindings made inside functions are local, which means they will not persist once the function ends.

## Join: `”`
This function behaves differently depending on the types of the inputs:
* Two strings: will get concatenated.
* A string and a number: the number gets joined to either the start or end of the string, depending on the stack order.
* Two lists: will get joined.
* A list and a value: the value gets joined to either the start or end of the list, depending on the stack order.
* Two functions: the functions will get composed. (yes i know you can build arbitrary code with this)
* Anything else, or insufficient values: this will error.

## Pair: `,`
Takes two values from the stack (uses `∅` if insufficient) and puts them into a list of two elements.
You can use this to make an "if-statement":
```
{"true"}{"false"}⭥,⭥⤉!
```

## Index: `⤉`
Uses a number to index a list or string below it. This will error if the number is not an integer.

If the index is out of bounds, this will return `∅`.

## Print: `↗`
Prints a value. Lists are printed with spaces, functions display as `{…}` and null values display as `∅`.

Note that this does not print a newline! If you want that, make a function:
```
{ ↗ "\n"↗ } →println
```

## Arithmetic operations: `+-×÷`
These work like you would expect them to.

They error if the input is not a number, or if there are not enough stack values.

## Equals: `=`
Outputs `1` if the inputs equal eachother, otherwise outputs `0`.

Lists will check if each element is equal; functions do not equal anything.
