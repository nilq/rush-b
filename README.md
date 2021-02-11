# Smith
## Match the world burn.

---

## Syntax

```java
public static if =
  | true body _  => body!
  | false _ body => body!

public static fib =
  | 0 => 0
  | 1 => 1
  | n => fib(n - 1) + fib(n - 2)

if fib(10) < 100
  print("what")
\else
  print("makes more sense")
```
