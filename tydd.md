# Type Driven Development

## Instructions for AI Agents

When implementing features using Type Driven Development, follow these rules strictly.

## Rule 1: States Are Always Enums

States must always be enums. State variants can hold data.

Data containers are either:
- Empty structs
- Enums (which may hold other data)

```rust
enum User {
    Guest(GuestData),
    Registered(RegisteredData),
    Verified(VerifiedData),
    Suspended,
}

struct GuestData;
struct RegisteredData;
enum VerifiedData {
    Email,
    Phone,
}
```

## Rule 2: Actions Are Enums

Actions are enums with empty variants that describe transformations.

```rust
enum UserAction {
    Register,
    Verify,
    Suspend,
    Reactivate,
}

enum OrderAction {
    Confirm,
    Ship,
    Deliver,
    Cancel,
}
```

## Rule 3: State Transition Functions

State transition functions must:

1. Use the prefix `stf_`
2. Take ownership of state
3. Borrow action with `&`
4. Always return a new state (never Result or Option)
5. Use pattern matching on `(state, action)` tuple

```rust
fn stf_process_user(user: User, action: &UserAction) -> User {
    match (user, action) {
        (User::Guest(data), UserAction::Register) => User::Registered(RegisteredData),
        (User::Registered(data), UserAction::Verify) => User::Verified(VerifiedData::Email),
        (User::Verified(data), UserAction::Suspend) => User::Suspended,
        (state, _) => state,
    }
}

fn stf_process_order(order: Order, action: &OrderAction) -> Order {
    match (order, action) {
        (Order::Pending(data), OrderAction::Confirm) => Order::Confirmed(ConfirmedData),
        (Order::Confirmed(data), OrderAction::Ship) => Order::Shipped(ShippingData),
        (state, _) => state,
    }
}
```

## Rule 4: No Methods, No Impl Blocks

Do not use:
- Impl blocks
- Trait implementations (except derives)
- Methods on types
- Comments

Only use:
- Type definitions (enum, struct)
- Function definitions
- Pattern matching

## Rule 5: Execution Types (Optional)

When an action could lead to multiple states depending on how it resolves, use an execution parameter.

Execution types are paired with specific stf_ functions - they represent the typed context of what will eventually happen inside that function's implementation.

Execution enums:
- Describe how a transition resolves without implementation details
- Use `Execution` suffix
- Variants don't need suffix
- Pattern match on `(state, action, execution)` tuple
- Each execution type is specific to its stf_ function

```rust
enum Player {
    Alive(HealthData),
    Dead(DeathData),
}

struct HealthData;
struct DeathData;

enum PlayerAction {
    TakeDamage,
    Heal,
    Revive,
}

enum DamageExecution {
    Kills,
    DoesNotKill,
}

fn stf_player(player: Player, action: &PlayerAction, execution: &DamageExecution) -> Player {
    match (player, action, execution) {
        (Player::Alive(data), PlayerAction::TakeDamage, DamageExecution::Kills) => Player::Dead(DeathData),
        (Player::Alive(data), PlayerAction::TakeDamage, DamageExecution::DoesNotKill) => Player::Alive(data),
        (Player::Dead(data), PlayerAction::Revive, _) => Player::Alive(HealthData),
        (state, _, _) => state,
    }
}
```

**When to use execution:**
- Action could lead to multiple different states based on how it resolves
- Need to represent "outcome context" without implementation
- The execution type models what would be decided inside the stf_ implementation

**When NOT to use execution:**
- Action always leads to same state (e.g., VisitAction::Shop action always results in LocationState::Shop state)
- Keep it simple - only add execution when multiple outcomes exist

**Key distinction:**
- Action = what the player/system intends to do
- Execution = how that action resolves (the typed context of the stf_ implementation)

Use one execution type per stf_ function when needed.

## Rule 6: Architecture Before Implementation

**Architecture Phase:**
1. Define state enums with named variants
2. Define empty data container structs/enums
3. Define action enums
4. Define stf_ function signatures with pattern matching

**Implementation Phase:**
1. Add fields to data containers
2. Implement stf_ function logic

## Code Organization

One state machine per file. Each file contains:

1. State enum
2. Data container structs/enums
3. Action enum
4. Execution enums (paired with specific stf_ functions)
5. stf_ functions

```
src/
├── user.rs          // Complete User state machine
├── order.rs         // Complete Order state machine
├── payment.rs       // Complete Payment state machine
├── common.rs        // Shared data containers (if needed)
└── lib.rs
```

### File Structure Example

```rust
// State
enum Player { /* ... */ }

// Data containers
struct HealthData;
struct DeadData;

// Actions
enum PlayerAction { /* ... */ }

// Execution (paired with stf_player)
enum DamageExecution {
    Kills,
    DoesNotKill,
}

// State transition function
fn stf_player(player: Player, action: &PlayerAction, execution: &DamageExecution) -> Player
```

Always split files into modules only when they exceed 500 lines.
Organize modules with state, data, and action types in state.rs and execution types and stf's in stfs.rs 

## Complete Example

```rust
enum Article {
    Draft(DraftData),
    UnderReview(ReviewData),
    Published(PublishedData),
    Archived,
}

struct DraftData;
struct ReviewData;
struct PublishedData;

enum ArticleAction {
    Submit,
    Approve,
    Reject,
    Publish,
    Archive,
}

fn stf_transition_article(article: Article, action: &ArticleAction) -> Article {
    match (article, action) {
        (Article::Draft(data), ArticleAction::Submit) => Article::UnderReview(ReviewData),
        (Article::UnderReview(data), ArticleAction::Approve) => Article::Published(PublishedData),
        (Article::UnderReview(data), ArticleAction::Reject) => Article::Draft(DraftData),
        (Article::Published(data), ArticleAction::Archive) => Article::Archived,
        (state, _) => state,
    }
}
```

## Using TyDD as Specification Documents

Type Driven Development serves as an executable specification format. Instead of traditional design documents with prose descriptions, use TyDD architecture as the specification itself.

### Specification = Architecture Phase

The architecture phase output IS the design document:

```rust
enum Quest {
    Available(QuestDetails),
    Active(QuestProgress),
    Completed(QuestRewards),
    Failed(FailureReason),
}

struct QuestDetails;
struct QuestProgress;
struct QuestRewards;
struct FailureReason;

enum QuestAction {
    Accept,
    Progress,
    Complete,
    Fail,
    Abandon,
}

fn stf_quest(quest: Quest, action: &QuestAction) -> Quest
```

This specification:
- Shows all possible quest states
- Lists all actions that can occur
- Defines state transitions
- Type-checks before implementation
- Requires no interpretation

### Benefits Over Traditional Specs

**Traditional specification:**
- Written in prose
- Ambiguous interpretation
- Becomes outdated
- Separate from code
- Requires translation to implementation

**TyDD specification:**
- Written in types
- Unambiguous and precise
- Always reflects current design
- IS the code structure
- Direct implementation path

### Workflow with TyDD Specs

1. **Design phase**: Designer creates state machines (enums, actions, function signatures)
2. **Review phase**: Team reviews the state machine for completeness
3. **Implementation phase**: Developer adds fields and implements stf_ logic
4. **Result**: Spec and code are unified

### Example: Game Design Spec

Instead of writing "The player can craft items by combining materials. Each recipe requires specific ingredients. Crafting can succeed or fail based on skill level..."

Write:

```rust
enum CraftingState {
    Idle,
    SelectingRecipe(AvailableRecipes),
    GatheringMaterials(RequiredMaterials),
    Crafting(CraftingAttempt),
    Success(CraftedItem),
    Failure,
}

struct AvailableRecipes;
struct RequiredMaterials;
struct CraftingAttempt;
struct CraftedItem;

enum CraftingAction {
    SelectRecipe,
    AddMaterial,
    StartCraft,
    CompleteCraft,
    CancelCraft,
}

fn stf_crafting(state: CraftingState, action: &CraftingAction) -> CraftingState
```

The state machine communicates the entire crafting system design without ambiguity.

## Key Points

- States are always enums
- Data containers are empty structs or enums
- stf_ functions always pattern match on (state, action) or (state, action, execution)
- Use execution parameter only when action could lead to multiple states
- Execution enums have Execution suffix, variants don't
- stf_ functions always return new state, never Result or Option
- No impl blocks, no methods, no comments
- One state machine per file
- Architecture phase: define types and signatures
- Implementation phase: add fields and logic
- TyDD architecture serves as executable specification
