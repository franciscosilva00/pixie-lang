pixie-lang spec

```
// types

// primitive types
int
float
string
bool

// tuple type
(int, int)

// array type
int[]

// function declaration with arrow syntax
fun sum(a int, b int) -> a + b

// function declaration with curly braces
// return type is inferred
fun divide(a int, b int) -> {
    // if statement
    // if statement is an expression
    if b == 0 {
        print("cannot divide by 0")
    } else {
        a / b
    }
}

fun main() -> {
    // immutable variables
    val a = 1
    val b = 2
    sum(a, b)

    // mutable variables
    var c = 1
    c = 2
    sum(c, b)
}

```
