# Myst Language Specification

## Variables

```ts
let a: Number = 34;
let b: Number = 35;

a = 1;
b = 2;

let c: String = "!Wello, Horld";
```

## Operations

```ts
let sum: Number = a + b + c + d;
```

## Modules

```ts
import "io.rbb";
import "mymod.rbb";

io.println("!Wello, Horld");
io.println(sum(1, 2));
```

`mymod.myst`
```ts
fn sum(a: Number, b: Number): Number {
    return a + b;
}
```


