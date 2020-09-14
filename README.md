# Brainfuck+

This is an interpreter for Brainfuck+ I made for fun. Brainfuck+ is very similar to the original Brainfuck, but with a few extra features.

### TL;DR

- `+10` is equivalent to `++++++++++`, the same is true for `<`, `>` and `-`.  
- Brainfuck+ has a register. `^` pushes data to register, `v` pulls data from register.  
- `#` prints decimal, `!` prints ASCII, `?` reads input.  
- Anything surrounded by two backticks (`` ` ``) is a comment.

That's it.

### Operation repetition

If any of the operators `<`, `>`, `+`, `-` is followed by a number `n`, that operator will execute `n` times.

For example: `>>>` can be written as `>3`, `+12` is equivalent to `++++++++++++`, you get the picture.

You can also use letters, their ASCII values will be used to repeat the operation. E.g: `+a` is equivalent to `+97`.

**Note:** "under the hood" `>3` is faster than `>>>`, they both move the memory pointer three times to the right, but the first takes 1 cycle to execute while the other takes 3 cycles.

### The register

Brainfuck+ has a "register" to which you can read and write a value. The operator `^` copies the data from the current memory cell into the register and the `v` operator copies the data from the register into the current memory cell.

Example:

Let's break down the following program: `+5^>3v`

(the snapshots between commands show the current state of the memory)

- **Snapshot:** strip: `[0, 0, 0, 0, 0]`, reg: `0`
- `+5`: increase current memory cell (which is `0`) five times.
- **Snapshot:** strip: `[5, 0, 0, 0, 0]`, reg `0`
- `^`: copy the value of the current cell (which is `5`) into the register.
- **Snapshot:** strip: `[5, 0, 0, 0, 0]`, reg `5`
- `>3`: Move right three times
- **Snapshot:** strip: `[5, 0, 0, 0, 0]`, reg `5`
- `v`: Copy contents of register into current cell.
- **Snapshot:** strip: `[5, 0, 0, 5, 0]`, reg `5`

### I/O

There are two write operators: `!`, which will print the current memory cell as an ASCII character, and `#`, which will print the raw value of the cell (as a decimal number).

The read operator is `?`.

### Comments

Since Brainfuck+ uses letters and numbers as operator repeaters, to add comments you need to surround them in two `` ` ``. Example:

```
>5      `moves pointer to 5`
[-]     `reset cell value to 0`
+15     `set cell value to 15`
#       `print decimal value of cell`
```

**Note:** Backticks do not need to be closed, ``>10`hello 123`` is valid Brainfuck+, it's the equivalent to `>10`.


### A few important notes

Memory does **not** wrap! If the size of the memory strip is 5 cells and you try to access the cell at positions `0` or `6`, the interpreter will panic.