# LLM Guidelines: Python-to-Rust Code Conversion

## Conversion Context and Purpose

When asked to convert Python code to Rust, follow these guidelines to create idiomatic, functional Rust code while maintaining the original program's semantics. Focus on producing correct, readable code that balances type safety with reasonable verbosity.

## Core Conversion Rules

1. **Type Annotation Conversion**
   - Convert all Python type annotations to Rust types using the mapping table below
   - Add type annotations to variables in Rust when the type isn't immediately obvious
   - Represent Python's duck typing with appropriate trait bounds when necessary
   - Implement `From` traits for necessary type conversions

2. **Data Structure Conversion**
   - Convert Python classes to Rust structs with associated methods
   - Convert Python dataclasses to Rust structs with derived traits
   - Convert Python Enums to Rust enums with appropriate variants
   - Ensure immutable Python structures become immutable in Rust

3. **Error Handling Translation**
   - Convert Python exceptions to Rust's `Result` type with appropriate error types
   - Translate `try/except` blocks to `match` expressions or the `?` operator
   - Create custom error types when Python code uses multiple exception types
   - Use `Option<T>` for Python functions that might return `None`

4. **Ownership and Borrowing Implementation**
   - Add lifetime parameters where necessary
   - Use borrowing (`&T` and `&mut T`) instead of cloning when appropriate
   - Implement `Clone` for types that need to be duplicated
   - Use `Rc` or `Arc` only when shared ownership is truly needed

## Python-to-Rust Type Mapping

```
Python                       Rust
--------------------------------------------
bool                        bool
int                         i32 or i64 (default to i32)
float                       f64
str                         String or &str
bytes                       Vec<u8> or &[u8]
list[T]                     Vec<T>
tuple[T, U]                 (T, U)
dict[K, V]                  HashMap<K, V>
set[T]                      HashSet<T>
Optional[T]                 Option<T>
Union[T, U]                 enum Either { Left(T), Right(U) }
Callable[[A, B], R]         fn(A, B) -> R
dataclass                   struct with #[derive(...)]
Enum                        enum
Exception                   Error trait
```

## Specific Conversion Patterns

### 1. Class Conversion Pattern

For Python classes with methods:

```python
# Python
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
    
    def greet(self) -> str:
        return f"Hello, my name is {self.name} and I am {self.age} years old."
    
    def have_birthday(self) -> None:
        self.age += 1
```

Convert to:

```rust
// Rust
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }
    
    fn have_birthday(&mut self) {
        self.age += 1;
    }
}
```

### 2. Error Handling Pattern

For Python functions with exceptions:

```python
# Python
def divide(a: int, b: int) -> float:
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b
```

Convert to:

```rust
// Rust
fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a as f64 / b as f64)
}
```

### 3. Collection Processing Pattern

For Python code using list comprehensions:

```python
# Python
def square_even_numbers(numbers: list[int]) -> list[int]:
    return [x * x for x in numbers if x % 2 == 0]
```

Convert to:

```rust
// Rust
fn square_even_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect()
}
```

### 4. Dataclass Conversion Pattern

For Python dataclasses:

```python
# Python
from dataclasses import dataclass
from typing import List, Optional

@dataclass(frozen=True)
class Product:
    id: int
    name: str
    price: float
    tags: List[str]
    description: Optional[str] = None
```

Convert to:

```rust
// Rust
#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: i32,
    name: String,
    price: f64,
    tags: Vec<String>,
    description: Option<String>,
}

impl Product {
    fn new(id: i32, name: String, price: f64, tags: Vec<String>, description: Option<String>) -> Self {
        Self { id, name, price, tags, description }
    }
}
```

## Function Transformation Steps

When converting each Python function to Rust:

1. **Convert signature**
   - Translate parameter types and return type
   - Add lifetime parameters if needed
   - Change return type to Result/Option if function can fail

2. **Convert body**
   - Translate Python control structures to Rust equivalents
   - Replace list/dict comprehensions with iterator chains
   - Modify variable declarations to include types
   - Add explicit return statements where needed

3. **Handle errors appropriately**
   - Use `?` operator for propagating errors in Rust
   - Replace exception handling with Result pattern matching
   - Create appropriate custom error types as needed

4. **Optimize borrowing**
   - Use references instead of value ownership when appropriate
   - Add `mut` to parameters that are modified
   - Use `&str` instead of `String` for string parameters that aren't modified

## Common Conversion Challenges

### Object-Oriented Patterns

For Python code using inheritance:

```python
# Python
class Animal:
    def speak(self) -> str:
        return "Some sound"

class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"
```

Convert to a trait-based approach:

```rust
// Rust
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}
```

### Dynamic Typing Challenges

For Python functions using dynamic types:

```python
# Python
def process(obj):
    if hasattr(obj, "process"):
        return obj.process()
    elif callable(obj):
        return obj()
    else:
        return str(obj)
```

Convert to enum-based pattern matching:

```rust
// Rust
enum Processable {
    WithProcess(Box<dyn Process>),
    Callable(Box<dyn Fn() -> String>),
    Other(String),
}

trait Process {
    fn process(&self) -> String;
}

fn process(obj: Processable) -> String {
    match obj {
        Processable::WithProcess(p) => p.process(),
        Processable::Callable(f) => f(),
        Processable::Other(s) => s,
    }
}
```

## Code Style Guidelines for Generated Rust

1. **Naming Conventions**
   - Use `snake_case` for functions, methods, variables
   - Use `CamelCase` for types, traits, and enums
   - Use `SCREAMING_SNAKE_CASE` for constants

2. **Structure Organization**
   - Group imports at the top
   - Define structs before their implementations
   - Define constants before functions

3. **Idiomatic Patterns**
   - Use `.unwrap_or_else()` instead of match for simple Option/Result handling
   - Use `.map()` and `.and_then()` for Option/Result chaining
   - Use tuple structs for simple wrappers

4. **Documentation**
   - Add doc comments (`///`) for public functions and types
   - Include examples in documentation when appropriate
   - Document panicking conditions with `# Panics` sections

## Evaluation Criteria

After generating Rust code, verify:

1. **Functional Equivalence**: Does the Rust code do exactly what the Python code did?
2. **Type Safety**: Have all dynamic Python features been properly translated to static Rust?
3. **Memory Safety**: Are there any potential memory leaks or unsafe operations?
4. **Idiomaticity**: Does the Rust code follow Rust conventions and best practices?
5. **Error Handling**: Are all error cases properly handled with appropriate Result types?

## Example Analysis

```python
# Python
def find_items(items: List[Dict[str, Any]], key: str, value: Any) -> List[Dict[str, Any]]:
    try:
        return [item for item in items if item.get(key) == value]
    except AttributeError:
        return []
```

Step-by-step conversion:

1. **Analyze Python code**:
   - Input: List of dictionaries, string key, any value
   - Output: Filtered list of dictionaries
   - Error handling: Returns empty list on AttributeError

2. **Design Rust signature**:
   ```rust
   fn find_items(items: &[HashMap<String, Value>], key: &str, value: &Value) -> Vec<HashMap<String, Value>>
   ```

3. **Convert algorithm**:
   ```rust
   fn find_items(items: &[HashMap<String, Value>], key: &str, value: &Value) -> Vec<HashMap<String, Value>> {
       items.iter()
           .filter(|item| item.get(key).map_or(false, |v| v == value))
           .cloned()
           .collect()
   }
   ```

4. **Handle errors**:
   - Python returns empty list on error, so we handle None with `map_or`
   - No need for explicit Result type since we match Python's behavior

Final implementation:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    None,
}

fn find_items(items: &[HashMap<String, Value>], key: &str, value: &Value) -> Vec<HashMap<String, Value>> {
    items.iter()
        .filter(|item| item.get(key).map_or(false, |v| v == value))
        .cloned()
        .collect()
}
```

## Final Checklist

When generating the final Rust code:

✓ All type conversions are explicit and sound
✓ Error handling is comprehensive
✓ Ownership and borrowing are properly implemented
✓ Idiomatic Rust patterns are used
✓ Code is well-structured and follows Rust conventions
✓ No unsafe blocks unless absolutely necessary
✓ Documentation is clear and helpful
