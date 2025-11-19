
# Type-Driven Development Code Generation Instructions

When building Rust applications, you MUST strictly follow the Type-Driven Development (TDD) architectural pattern described below.

This is a **specification-first** approach: design the complete application using types, verify correctness with tests, then implement real logic without changing the API.

## Core Principle

**Types describe intent, functions describe transformation.** Use Rust's type system to model application architecture through state machines with compile-time verification.

## Four-Phase Workflow

### Phase 1: Type-Only Design
Create the complete specification using only types. Use **empty structs** for data types and **execution types as function parameters**.

### Phase 2: Implement Robust Tests
Write comprehensive tests that verify all logic paths by directly passing execution types.

### Phase 3: Verification
Review the type specification and tests with stakeholders. Ensure all business logic is correctly modeled.

### Phase 4: Actual Implementation
Replace execution type parameters with real implementation code. Add actual fields to data structures. **DO NOT change state transitions, function signatures, or the API.**

## Mandatory Type Categories

Organize ALL types into exactly three categories:

### 1. Data Types
Represent domain entities. **In Phase 1**, use empty structs as placeholders. **In Phase 4**, add actual fields.

```rust
// PHASE 1: Design (empty placeholder)
struct User;
struct Order;

// PHASE 4: Implementation (add actual fields)
struct User {
    id: UserId,
    email: EmailAddress,
    name: String,
}

struct Order {
    id: OrderId,
    items: Vec<OrderItem>,
    total: Money,
}
```

### 2. Action Types
Represent operations WITHOUT implementation. Use empty structs or structs with parameters.

```rust
struct CreateUser;
struct DeleteUser;
struct SendEmail {
    to: EmailAddress,
    subject: String,
    body: String,
}
```

### 3. Execution Types
Describe possible outcomes/paths within functions. **Used only in Phases 1-3 for specification and testing.** Removed in Phase 4.

```rust
enum AuthenticationExecution {
    PasswordMatches,
    PasswordDoesNotMatch,
    UserNotFound,
    AccountLocked,
}

enum DatabaseExecution {
    Success(serde_json::Value),
    NotFound,
    ConnectionError,
}

enum PaymentExecution {
    Approved { transaction_id: String },
    Declined { reason: String },
    NetworkError,
}
```

## Function Pattern Evolution

### Phase 1-3: Specification Functions
Functions accept execution types as parameters and use pattern matching. NO real I/O.

```rust
// PHASE 1-3: Specification
fn authenticate_user(
    email: EmailAddress,
    password: String,
    execution: AuthenticationExecution,  // Specification parameter
) -> Result<AuthenticatedUser, AuthError> {
    match execution {  // Pattern match on all paths
        AuthenticationExecution::PasswordMatches => {
            Ok(AuthenticatedUser { email, authenticated_at: Utc::now() })
        }
        AuthenticationExecution::PasswordDoesNotMatch => {
            Err(AuthError::InvalidCredentials)
        }
        AuthenticationExecution::UserNotFound => {
            Err(AuthError::UserNotFound)
        }
        AuthenticationExecution::AccountLocked => {
            Err(AuthError::AccountLocked)
        }
    }
}
```

### Phase 4: Implementation Functions
Remove execution type parameter and replace pattern matching with actual implementation logic.

```rust
// PHASE 4: Implementation
async fn authenticate_user(
    email: EmailAddress,
    password: String,
    db: &Database,              // Real dependencies replace execution type
    hasher: &PasswordHasher,
) -> Result<AuthenticatedUser, AuthError> {
    // Actually query database
    let user = db.find_user_by_email(&email).await
        .map_err(|_| AuthError::UserNotFound)?;
    
    // Actually check if locked
    if user.is_locked {
        return Err(AuthError::AccountLocked);
    }
    
    // Actually verify password
    if hasher.verify(&password, &user.password_hash) {
        Ok(AuthenticatedUser { email, authenticated_at: Utc::now() })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}
```

**Critical**: Function signature (return type, state transitions) must remain identical. Only the parameters and internal implementation change.

## State Machine Implementation

For stateful entities, MUST use phantom types. State transitions remain identical across all phases.

```rust
// State markers (never change)
struct Draft;
struct Submitted;
struct Paid;

// PHASE 1: Generic over state (minimal fields)
struct Order<State> {
    _state: PhantomData<State>,
}

// PHASE 4: Add actual fields (state transitions unchanged)
struct Order<State> {
    id: OrderId,
    items: Vec<OrderItem>,
    total: Money,
    _state: PhantomData<State>,
}

// State-specific transitions (IDENTICAL in all phases)
impl Order<Draft> {
    fn submit(self) -> Order<Submitted> {
        Order {
            id: self.id,
            items: self.items,
            total: self.total,
            _state: PhantomData,
        }
    }
}

impl Order<Submitted> {
    fn pay(self, payment: PaymentInfo, execution: PaymentExecution) -> Result<Order<Paid>, PaymentError> {
        // PHASE 1-3: Pattern match on execution type
        // PHASE 4: Replace with real payment processing
    }
}
```

## Testing Pattern (Phase 2)

Tests MUST directly pass execution types to verify all business logic paths without mocks.

```rust
#[test]
fn test_authentication_success() {
    let email = EmailAddress::parse("test@example.com".to_string()).unwrap();
    
    // PHASE 2-3: Pass execution type directly
    let result = authenticate_user(
        email,
        "password".to_string(),
        AuthenticationExecution::PasswordMatches,
    );
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, email);
}

#[test]
fn test_authentication_locked_account() {
    let email = EmailAddress::parse("locked@example.com".to_string()).unwrap();
    
    let result = authenticate_user(
        email,
        "password".to_string(),
        AuthenticationExecution::AccountLocked,
    );
    
    assert!(matches!(result, Err(AuthError::AccountLocked)));
}

#[test]
fn test_authentication_wrong_password() {
    let email = EmailAddress::parse("user@example.com".to_string()).unwrap();
    
    let result = authenticate_user(
        email,
        "wrong".to_string(),
        AuthenticationExecution::PasswordDoesNotMatch,
    );
    
    assert!(matches!(result, Err(AuthError::InvalidCredentials)));
}

#[test]
fn test_authentication_user_not_found() {
    let email = EmailAddress::parse("unknown@example.com".to_string()).unwrap();
    
    let result = authenticate_user(
        email,
        "password".to_string(),
        AuthenticationExecution::UserNotFound,
    );
    
    assert!(matches!(result, Err(AuthError::UserNotFound)));
}
```

**Key Testing Principles:**
- Every execution variant MUST have at least one test
- Tests verify business logic decisions, not implementation details
- No mocks, databases, or external services needed
- Fast, deterministic, comprehensive coverage

## Mandatory Requirements Checklist

### Phase 1: Type-Only Design
- [ ] All types categorized as Data, Action, or Execution
- [ ] Data types use empty structs or minimal fields
- [ ] All execution types defined with every possible outcome
- [ ] State machines use phantom types
- [ ] Function signatures defined with execution type parameters
- [ ] Function bodies use pattern matching on execution types
- [ ] NO actual I/O, database, or external operations in functions
- [ ] Code compiles successfully

### Phase 2: Implement Robust Tests
- [ ] Every execution variant has at least one test
- [ ] Tests directly pass execution types to functions
- [ ] Tests verify all business logic paths
- [ ] No mocks or external dependencies in tests
- [ ] All tests pass

### Phase 3: Verification
- [ ] Review all types with stakeholders
- [ ] Verify state transitions are correct
- [ ] Confirm all execution paths are handled
- [ ] Validate function signatures match requirements
- [ ] Approval obtained before implementation

### Phase 4: Actual Implementation
- [ ] Add real fields to data types
- [ ] Remove execution type parameters from functions
- [ ] Replace pattern matching with real implementation logic
- [ ] State transitions remain IDENTICAL (no API changes)
- [ ] Function return types remain IDENTICAL
- [ ] Tests updated or adapted for real implementation
- [ ] Code compiles and all tests pass

## Key Benefits You Must Achieve

1. **Design Before Implementation**: Complete specification exists before writing any real code
2. **Compile-Time Correctness**: Invalid states/transitions = compile errors in all phases
3. **Verified Logic**: All business logic paths tested before implementation
4. **Stakeholder Approval**: Types serve as reviewable specification document
5. **Zero Mocks in Design Phase**: Tests control execution via execution types
6. **API Stability**: Implementation phase doesn't change public interfaces
7. **Self-Documentation**: Types describe intent, signatures describe transformations
8. **Risk Reduction**: Logic errors found in Phase 2-3, not during implementation

## Application Structure

### Phase 1-3 Structure
```
src/
├── types/
│   ├── data.rs       // Data types (empty structs initially)
│   ├── actions.rs    // Action types
│   └── execution.rs  // Execution types
├── logic/
│   └── *.rs          // Business logic (pattern match on execution types)
├── tests/
│   └── *.rs          // Tests using execution types
└── main.rs           // Minimal or placeholder
```

### Phase 4 Structure
```
src/
├── types/
│   ├── data.rs       // Data types (with real fields)
│   ├── actions.rs    // Action types (unchanged)
│   └── execution.rs  // Can be removed or kept for testing
├── logic/
│   └── *.rs          // Business logic (real implementations)
├── tests/
│   └── *.rs          // Tests (adapted for real code)
├── db/
│   └── *.rs          // Database layer
├── services/
│   └── *.rs          // External services
└── main.rs           // Full application wiring
```

## When Given an Application Spec

Follow this exact sequence:

### Phase 1: Type-Only Design
1. Identify domain entities → Create empty Data Types
2. Identify operations → Create Action Types  
3. Identify all possible outcomes for each operation → Create Execution Types
4. Define state machines using phantom types
5. Write function signatures that accept execution types as parameters
6. Implement function bodies using pattern matching on execution types
7. Ensure code compiles

### Phase 2: Implement Robust Tests
1. Write tests for every execution variant
2. Pass execution types directly to functions
3. Verify all business logic paths produce correct results
4. Ensure 100% coverage of execution paths
5. All tests must pass

### Phase 3: Verification
1. Review types, state transitions, and logic with stakeholders
2. Demonstrate tests showing all behavior paths
3. Obtain approval before proceeding to implementation
4. Document any required changes and return to Phase 1 if needed

### Phase 4: Actual Implementation
1. Add real fields to Data Types
2. Remove execution type parameters from functions
3. Replace pattern matching with actual implementation code (database calls, API calls, etc.)
4. **CRITICAL**: Do NOT change function return types or state transitions
5. Update or adapt tests for real implementation
6. Ensure all tests pass
7. Deploy

## Example: Complete Four-Phase Evolution

### Phase 1: Specification
```rust
// Data type - empty
struct User;

// Execution type
enum CreateUserExecution {
    EmailAvailable,
    EmailTaken,
    DatabaseError,
}

// Function with execution parameter
fn create_user(
    email: String,
    name: String,
    execution: CreateUserExecution,
) -> Result<User, CreateUserError> {
    // Validate email
    let email = EmailAddress::parse(email)?;
    
    // Pattern match on execution
    match execution {
        CreateUserExecution::EmailAvailable => Ok(User),
        CreateUserExecution::EmailTaken => Err(CreateUserError::EmailExists),
        CreateUserExecution::DatabaseError => Err(CreateUserError::DatabaseError),
    }
}
```

### Phase 2: Tests
```rust
#[test]
fn test_create_user_success() {
    let result = create_user(
        "test@example.com".to_string(),
        "Test User".to_string(),
        CreateUserExecution::EmailAvailable,
    );
    assert!(result.is_ok());
}

#[test]
fn test_create_user_email_taken() {
    let result = create_user(
        "taken@example.com".to_string(),
        "Test User".to_string(),
        CreateUserExecution::EmailTaken,
    );
    assert!(matches!(result, Err(CreateUserError::EmailExists)));
}
```

### Phase 3: Verification
✅ Stakeholders review and approve type design and logic paths

### Phase 4: Implementation
```rust
// Data type - with real fields
struct User {
    id: UserId,
    email: EmailAddress,
    name: String,
    created_at: DateTime<Utc>,
}

// Function with real implementation (execution parameter removed)
async fn create_user(
    email: String,
    name: String,
    db: &Database,
) -> Result<User, CreateUserError> {
    // Validate email (same as before)
    let email = EmailAddress::parse(email)?;
    
    // REAL database check (replaces execution type)
    let exists = db.email_exists(&email).await
        .map_err(|_| CreateUserError::DatabaseError)?;
    
    if exists {
        return Err(CreateUserError::EmailExists);
    }
    
    // REAL user creation
    let user = User {
        id: UserId::new(),
        email,
        name,
        created_at: Utc::now(),
    };
    
    db.save_user(&user).await
        .map_err(|_| CreateUserError::DatabaseError)?;
    
    Ok(user)
}
```

**Notice**: Return type remains `Result<User, CreateUserError>` - API unchanged!

**NEVER deviate from this four-phase pattern. The type system enforces correctness at every stage.**
