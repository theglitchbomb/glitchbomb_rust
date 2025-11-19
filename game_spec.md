# Glitch Bomb - Game Specification

## Core Concept
Glitch Bomb is a bag-building, luck-based game where players pull random orbs from their bag to accumulate points while managing health and resources across 7 progressive levels.

## Game Phases

### Phase Flow
1. **menu** → Start game
2. **level** → Play level (pull orbs)
3. **confirmation** → Level complete decision point
4. **marketplace** → Purchase orbs/items with chips
5. **gameover** | **victory** → End states
6. Return to **menu**

### Phase Transitions
- **menu → level**: Pay glitchbytes (level entry cost) to start
- **level → confirmation**: Reach point milestone OR lose (health=0 or no orbs)
- **level → gameover**: Health reaches 0 or run out of orbs before milestone
- **level → victory**: Complete level 7 (final level)
- **confirmation → marketplace**: Commit to next level (converts points to chips)
- **marketplace → level**: Proceed to next level (costs glitchbytes)

## Currencies & Resources

### Glitchbytes (Persistent)
- Starting amount: 1,000
- **Persists across sessions** (saved to localStorage)
- Used to enter levels (entry costs)
- Earned by: Converting points (1:1), special orbs, shop items, victory rewards
- Free 1,000 glitchbytes available if balance < 100

### Chips (Session-based)
- Does NOT persist across game sessions
- Earned by: Converting points to chips when advancing levels, chip orbs
- Used in marketplace to purchase orbs and shop items
- Resets to 0 when starting new game (level 1)

### Points (Level-based)
- Reset to 0 when entering each level
- Earned during level by pulling point orbs (affected by multiplier)
- Converted to chips (1:1) when advancing to marketplace
- Converted to glitchbytes (1:1) on cash-out or victory
- Used to reach level milestones

### Health
- Max: 5 HP
- Starting: 5 HP (full)
- Reduced by bomb orbs
- Game over when health = 0 (unless milestone reached)
- Resets to 5 when entering new level

## Levels

### Level Progression
| Level | Milestone | Entry Cost (GB) |
|-------|-----------|-----------------|
| 1     | 12 pts    | 10              |
| 2     | 18 pts    | 1               |
| 3     | 28 pts    | 2               |
| 4     | 44 pts    | 4               |
| 5     | 70 pts    | 6               |
| 6     | 100 pts   | 9               |
| 7     | 150 pts   | 13              |

### Level Mechanics
- **Level Complete**: Points ≥ milestone for current level
- **Game Over Conditions**: (Health = 0 OR no orbs remaining) AND points < milestone
- **Victory**: Complete level 7 (final level)
- **Victory Reward**: All remaining points converted to glitchbytes (1:1)

## Orb Bag System

### Starting Orbs (Level 1)
- 3x Point Orbs (5 pts each)
- 4x Bomb Orbs (1, 1, 2, 3 damage)
- 1x Health Orb (1 HP)
- 1x Combo Orb (1 pt per remaining orb)
- 1x Danger Orb (4 pts per bomb pulled)
- 1x Multiplier Orb (1.0x boost)

### Orb Mechanics
- **Available vs Total**: 
  - `available[]` = orbs that can currently be pulled
  - `total[]` = all owned orbs (including pulled ones)
- **Pulling**: Random selection from available orbs
- **Consuming**: Orb removed from available array when pulled
- **Reset**: When entering marketplace, available = total (orbs replenished)

### Orb Types

#### 1. Health Orb
- Default amount: +1 HP
- Max health cap: 5 HP
- Cannot exceed max health

#### 2. Point Orb
- Default amount: +5 points
- **Affected by level multiplier**
- Shop variants: 5, 7, 8, 9 points

#### 3. Bomb Orb
- Default amount: 2 damage
- Reduces health by orb amount
- Increments `bombsPulledThisLevel` counter
- Triggers danger orb calculations
- Starting bombs: 1, 1, 2, 3 damage
- **Special**: Matrix Disarray bomb (2 damage) appears in levels 4-7 if triggered

#### 4. Combo Orb (points_per_anyorb)
- Default amount: 2 pts per orb
- **Calculation**: `remainingOrbs × orbAmount`
- Counts all orbs AFTER this orb is pulled
- **Affected by level multiplier**
- Starting: 1 pt per orb

#### 5. Danger Orb (points_per_bombpulled)
- Default amount: 1 pt per bomb
- **Calculation**: `bombsPulledThisLevel × orbAmount`
- Tracks bombs pulled during current level
- **Affected by level multiplier**
- Starting: 4 pts per bomb

#### 6. Multiplier Orb
- Default amount: +0.5x
- **Cumulative**: Adds to `levelMultiplier` (starts at 1.0)
- Affects all point-generating orbs pulled after it
- Persists for entire level (resets between levels)
- Starting: 1.0x boost
- Shop variants: 0.5x, 1.0x, 1.5x

#### 7. Chips Orb (bits)
- Default amount: 10 chips
- Directly adds to chip currency
- Used for marketplace purchases
- Shop variants: 15 chips

#### 8. Glitchbytes Orb
- Default amount: 5 glitchbytes
- Directly adds to persistent glitchbytes
- Can be used for level entry
- Shop variants: 15, 40 glitchbytes

## Marketplace System

### Entry
- Accessed after level completion via confirmation phase
- Requires committing to next level (irreversible)
- Points converted to chips (1:1 ratio)
- Available orbs replenished (available = total)

### Shop Deck System
- **Persistent Pricing**: Item prices persist throughout game session
- **Price Escalation**: Each purchase increases that item's price by 20%
- **Price Formula**: `baseCost × (1.2 ^ purchaseCount)`
- **Per-Item Tracking**: Each item tracks its own purchase count
- Deck resets when starting new game (level 1)

### Shop Tiers
All tiers available at all levels (1-7)

#### Common Items (3 random items shown)
- Point Orb +5 (5 chips)
- Point Orb +7 (8 chips)
- Chips +15 (5 chips)
- Glitchbytes +15 (8 chips)
- Danger Orb +4 per bomb (6 chips)
- Health Orb +1 HP (9 chips)
- Multiplier +0.5x (9 chips)

#### Rare Items (2 random items shown)
- Point Orb +8 (11 chips)
- Point Orb +9 (13 chips)
- Multiplier +1.0x (14 chips)
- Multiplier +1.5x (16 chips)

#### Cosmic Items (1 random item shown)
- Health Orb +3 HP (21 chips)
- Glitchbytes +40 (23 chips)

### Basic Orb Purchases
- Health Orb: 2 chips (default 1 HP)
- Point Orb: 2 chips (default 5 pts)
- Bomb orbs cannot be purchased

## Special Mechanics

### Level Multiplier
- Starts at 1.0 each level
- Increased by multiplier orbs
- Affects: point orbs, combo orbs, danger orbs
- Formula: `points = basePoints × levelMultiplier` (floored)
- Resets to 1.0 when entering new level

### Cash Out System
- **Mid-Level**: Points → Glitchbytes (1:1), return to menu
- **Post-Level**: Points → Glitchbytes (1:1), return to menu
- Resets entire game session (clears chips, orbs, progress)

### Matrix Disarray (Special Event)
- Status flag: `matrixDisarrayActive`
- Adds extra 2-damage bomb to bag in levels 4-7
- Persists across levels 4-7
- Resets on victory or return to menu

### Point History Tracking
- Tracks all point changes with timestamps
- Records action descriptions
- Stores cumulative level costs
- Max 100 entries (auto-prunes oldest)
- Used for profit/loss analysis

### Game Log
- Structured logging system with typed entries
- Log types: orb_pulled, shop_purchase, level_change, points_conversion, game_event, system
- Max 30 entries (auto-prunes oldest)
- Includes detailed effect data and resulting stats

## Victory & Loss Conditions

### Victory
- Complete level 7 (reach 150 points)
- Points converted to glitchbytes (1:1)
- Matrix disarray reset
- Return to menu with updated glitchbytes

### Game Over
- Health = 0 before reaching milestone
- No orbs remaining before reaching milestone
- All progress lost (except glitchbytes earned)
- Return to menu

### Restart Options
- **Return to Menu**: Keep current glitchbytes, reset session
- **Restart Game**: Reset to level 1 with current glitchbytes
- **Cash Out**: Convert points to glitchbytes, return to menu

## Economic Balance

### Level Investment Strategy
- Cumulative costs to reach each level:
  - Level 1: 10 GB
  - Level 2: 11 GB
  - Level 3: 13 GB
  - Level 4: 17 GB
  - Level 5: 23 GB
  - Level 6: 32 GB
  - Level 7: 45 GB

### Risk/Reward
- Higher levels = higher milestones = more chips = better shop items
- Entry costs increase, requiring successful runs to profit
- Points convert 1:1 to glitchbytes, incentivizing milestone completion
- Shop price escalation discourages repeated purchases of same item

## Implementation Notes

### State Persistence
- **Persists**: Glitchbytes only (localStorage)
- **Session**: Chips, shop deck prices, orb bag, level progress, matrix disarray
- **Level**: Points, health, multiplier, bombs pulled counter

### Reactivity Patterns
- Uses Svelte 5 runes ($state, $derived)
- Immutable state updates for predictable behavior
- Sound effects triggered on stat changes

### Validation & Safety
- Level bounds checking (1-7)
- Affordability checks before purchases
- Health capping at max (5 HP)
- Orb availability validation before pulls

### Audio Feedback
- Point gains: pointsbar sound
- Health loss: bomb1 or endgame sound
- Multiplier: multiplier sound
- Chips/GB: specialpull sound
- Level complete: levelup sound
