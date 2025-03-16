# TypeScript → Rust Translation Rules

## Core Patterns

1. **Ownership model** - Each value has exactly one owner; use explicit cloning
2. **Immutability by default** - Use `readonly` for everything unless explicitly needed
3. **No null values** - Use `Option<T>` instead of null/undefined
4. **No exceptions** - Use `Result<T, E>` for all operations that can fail
5. **Trait-based design** - Use interfaces for behavior contracts

## Type Definitions

### Use Algebraic Data Types
```typescript
// ✅ Do this - models Rust's struct
interface Point {
  readonly x: number;
  readonly y: number;
}

// ✅ Do this - models Rust's enum
enum ShapeKind { Circle, Rectangle }

interface Circle {
  readonly kind: ShapeKind.Circle;
  readonly radius: number;
}

interface Rectangle {
  readonly kind: ShapeKind.Rectangle;
  readonly width: number;
  readonly height: number;
}

type Shape = Circle | Rectangle;
```

### Implement Option<T>
```typescript
// Directly models Rust's Option enum
enum OptionKind { Some, None }

interface Some<T> {
  readonly kind: OptionKind.Some;
  readonly value: T;
}

interface None {
  readonly kind: OptionKind.None;
}

type Option<T> = Some<T> | None;

// Factory functions (like Rust's constructors)
function Some<T>(value: T): Option<T> {
  return { kind: OptionKind.Some, value };
}

function None<T>(): Option<T> {
  return { kind: OptionKind.None };
}
```

### Implement Result<T,E>
```typescript
// Directly models Rust's Result enum
enum ResultKind { Ok, Err }

interface Ok<T> {
  readonly kind: ResultKind.Ok;
  readonly value: T;
}

interface Err<E> {
  readonly kind: ResultKind.Err;
  readonly error: E;
}

type Result<T, E> = Ok<T> | Err<E>;

// Factory functions
function Ok<T, E>(value: T): Result<T, E> {
  return { kind: ResultKind.Ok, value };
}

function Err<T, E>(error: E): Result<T, E> {
  return { kind: ResultKind.Err, error };
}
```

## Traits and Implementations

### Model Traits with Interfaces
```typescript
// ✅ Do this - similar to Rust trait
interface Display {
  toString(): string;
}

// ✅ Do this - trait implementation with namespace
interface User {
  readonly id: string;
  readonly name: string;
}

// Similar to Rust's impl block
namespace User {
  // Associated function (static method in Rust)
  export function create(id: string, name: string): User {
    return { id, name };
  }
  
  // Implementation of Display trait
  export function toString(user: User): string {
    return `User(${user.id}): ${user.name}`;
  }
}
```

### Use Generics with Trait Bounds
```typescript
// ✅ Do this - models Rust's trait bounds
interface Comparable<T> {
  compare(other: T): number;
}

// Generic function with trait bound
function max<T extends Comparable<T>>(a: T, b: T): T {
  return a.compare(b) >= 0 ? a : b;
}
```

## Ownership and Borrowing

### Model Ownership Explicitly
```typescript
// ✅ Do this - explicit ownership transfer
function takeOwnership<T>(value: T): void {
  // Value is consumed here
}

// ✅ Do this - explicit borrowing (read-only reference)
function borrow<T>(value: readonly T): void {
  // Can read but not modify
}

// ✅ Do this - clone instead of sharing mutable data
function process(data: readonly number[]): number[] {
  const copy = [...data]; // Explicit clone
  copy.sort(); // Modify the copy
  return copy; // Return new data
}
```

### Handle Interior Mutability
```typescript
// ✅ Do this - similar to Rust's Cell/RefCell
class Cell<T> {
  private value: T;
  
  constructor(initialValue: T) {
    this.value = initialValue;
  }
  
  get(): T {
    return this.value;
  }
  
  set(newValue: T): void {
    this.value = newValue;
  }
}

// Usage
const counter = new Cell(0);
counter.set(counter.get() + 1);
```

## Pattern Matching

### Use Exhaustive Matching
```typescript
// ✅ Do this - exhaustive pattern matching
function processShape(shape: Shape): number {
  switch (shape.kind) {
    case ShapeKind.Circle:
      return Math.PI * shape.radius ** 2;
    case ShapeKind.Rectangle:
      return shape.width * shape.height;
  }
  
  // TypeScript will error if a case is missing - similar to Rust's exhaustive match
  const _exhaustiveCheck: never = shape;
  return _exhaustiveCheck;
}
```

### Chain Result Operations
```typescript
// ✅ Do this - similar to Rust's ? operator
function validateUserInput(input: UserInput): Result<User, string> {
  // Validate email
  const emailResult = validateEmail(input.email);
  if (emailResult.kind === ResultKind.Err) {
    return emailResult;
  }
  
  // Validate name
  const nameResult = validateName(input.name);
  if (nameResult.kind === ResultKind.Err) {
    return nameResult;
  }
  
  // Create user with validated data
  return Ok({
    id: generateId(),
    email: emailResult.value,
    name: nameResult.value
  });
}
```

## Iterators and Functional Patterns

### Use Iterator Chains
```typescript
// ✅ Do this - similar to Rust's iterator chains
function processItems(items: readonly number[]): number {
  return items
    .filter(item => item > 0)
    .map(item => item * 2)
    .reduce((sum, item) => sum + item, 0);
}
```

### Implement Builder Pattern
```typescript
// ✅ Do this - similar to Rust builder pattern
interface UserBuilder {
  readonly id?: string;
  readonly name?: string;
  readonly email?: string;
}

// Builder implementation
namespace UserBuilder {
  export function create(): UserBuilder {
    return {};
  }
  
  export function withId(builder: UserBuilder, id: string): UserBuilder {
    return { ...builder, id };
  }
  
  export function withName(builder: UserBuilder, name: string): UserBuilder {
    return { ...builder, name };
  }
  
  export function withEmail(builder: UserBuilder, email: string): UserBuilder {
    return { ...builder, email };
  }
  
  export function build(builder: UserBuilder): Result<User, string> {
    if (!builder.id) return Err("Missing id");
    if (!builder.name) return Err("Missing name");
    if (!builder.email) return Err("Missing email");
    
    return Ok({
      id: builder.id,
      name: builder.name,
      email: builder.email
    });
  }
}

// Usage
const userResult = UserBuilder.create()
  |> UserBuilder.withId(#, "u1")
  |> UserBuilder.withName(#, "Alice")
  |> UserBuilder.withEmail(#, "alice@example.com")
  |> UserBuilder.build(#);
```

## Module Organization

### Structure Files Like Rust Modules
```typescript
// types.ts - similar to a Rust module
export interface User {
  readonly id: string;
  readonly name: string;
}

export namespace User {
  export function create(id: string, name: string): User {
    return { id, name };
  }
}

// user_service.ts - importing from other modules
import { User } from "./types";
import { Option, Some, None } from "./option";

export function findUser(id: string): Option<User> {
  // Implementation
}
```

## Complete Example: Data Processing Pipeline

```typescript
// --- Core types ---
enum OptionKind { Some, None }
interface Some<T> { readonly kind: OptionKind.Some; readonly value: T; }
interface None { readonly kind: OptionKind.None; }
type Option<T> = Some<T> | None;

function Some<T>(value: T): Option<T> { return { kind: OptionKind.Some, value }; }
function None<T>(): Option<T> { return { kind: OptionKind.None }; }

enum ResultKind { Ok, Err }
interface Ok<T> { readonly kind: ResultKind.Ok; readonly value: T; }
interface Err<E> { readonly kind: ResultKind.Err; readonly error: E; }
type Result<T, E> = Ok<T> | Err<E>;

function Ok<T, E>(value: T): Result<T, E> { return { kind: ResultKind.Ok, value }; }
function Err<T, E>(error: E): Result<T, E> { return { kind: ResultKind.Err, error }; }

// --- Domain types ---
interface DataPoint {
  readonly timestamp: number;
  readonly value: number;
}

enum DataErrorKind { InvalidFormat, OutOfRange, MissingValue }
interface DataError {
  readonly kind: DataErrorKind;
  readonly message: string;
}

// --- Trait interfaces ---
interface Parser<T, E> {
  parse(input: string): Result<T, E>;
}

// --- Implementation ---
const DataPointParser: Parser<DataPoint, DataError> = {
  parse(input: string): Result<DataPoint, DataError> {
    const parts = input.split(',');
    
    if (parts.length !== 2) {
      return Err({
        kind: DataErrorKind.InvalidFormat,
        message: `Expected 2 values, got ${parts.length}`
      });
    }
    
    const timestamp = parseInt(parts[0], 10);
    if (isNaN(timestamp)) {
      return Err({
        kind: DataErrorKind.InvalidFormat,
        message: "Invalid timestamp format"
      });
    }
    
    const value = parseFloat(parts[1]);
    if (isNaN(value)) {
      return Err({
        kind: DataErrorKind.InvalidFormat,
        message: "Invalid value format"
      });
    }
    
    return Ok({ timestamp, value });
  }
};

// --- Data processing pipeline ---
function processData(lines: readonly string[]): Result<readonly DataPoint[], DataError> {
  const dataPoints: DataPoint[] = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    if (line === '') continue;
    
    const result = DataPointParser.parse(line);
    if (result.kind === ResultKind.Err) {
      return Err({
        kind: result.error.kind,
        message: `Line ${i+1}: ${result.error.message}`
      });
    }
    
    dataPoints.push(result.value);
  }
  
  return Ok(dataPoints);
}

function analyzeData(dataPoints: readonly DataPoint[]): Result<number, DataError> {
  if (dataPoints.length === 0) {
    return Err({
      kind: DataErrorKind.MissingValue,
      message: "No data points to analyze"
    });
  }
  
  // Calculate average
  const sum = dataPoints.reduce((acc, point) => acc + point.value, 0);
  const average = sum / dataPoints.length;
  
  return Ok(average);
}

// --- Usage example ---
const rawData = [
  "1621234567,23.5",
  "1621234667,24.1",
  "1621234767,22.8"
];

// Process the pipeline
const dataResult = processData(rawData);
if (dataResult.kind === ResultKind.Err) {
  console.error(`Error: ${dataResult.error.message}`);
} else {
  const analysisResult = analyzeData(dataResult.value);
  if (analysisResult.kind === ResultKind.Ok) {
    console.log(`Average value: ${analysisResult.value.toFixed(2)}`);
  } else {
    console.error(`Analysis error: ${analysisResult.error.message}`);
  }
}
```
