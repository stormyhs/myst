### Variables
```ts
let age: i32 = 69;
let inferredAge = 420;
const name: String = "Andrew";
```

### Loops
```ts

```

### Functions
```ts
fn giveMeAString() {
    return "I am a string!";
}

fn addTwoNumbers(a: i32, b: i32) -> i32 {
    const result = a + b;
    return result;

    // OR

    return a + b;
}
```

### Type Definitions
```ts
type Unit { "Metric" | "Imperial" } // Variables of this type must be a string of "Metric" or "Imperial"
```

```ts
type Person {
    name: String;
    age: i32;
    getLocation: fn(unitType: Unit) -> String; // `getLocation` is a function that takes in `unitType` as an argument of type `Unit` and returns a `String`
}
```

