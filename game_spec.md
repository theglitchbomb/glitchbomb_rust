# Glitch Bomb - Game Essentials

## Game State

### Persistent Data
- **glitchbytes**: integer (starts at 1000)

### Session Data
- **phase**: menu | level | confirmation | marketplace | gameover | victory
- **currentLevel**: 1-7
- **health**: 0-5
- **points**: integer
- **chips**: integer
- **multiplier**: float (resets to 1.0 each level)
- **bombsPulled**: integer (resets each level)
- **orbBag**: collection of orbs (available + total)
- **shopDeck**: collection of shop items with purchase counts
- **matrixDisarray**: boolean (active in levels 4-7)

## State Machine

### States & Transitions

**MENU**
- Actions: start_game
- Transitions:
  - start_game → LEVEL (level=1, deduct entry cost from glitchbytes)

**LEVEL**
- Actions: pull_orb, cash_out
- State conditions:
  - points < milestone AND health > 0 AND orbs available
- Transitions:
  - points ≥ milestone AND level < 7 → CONFIRMATION
  - points ≥ milestone AND level = 7 → VICTORY
  - health = 0 OR no orbs available → GAMEOVER
  - cash_out → MENU (convert points to glitchbytes)

**CONFIRMATION**
- Actions: continue_to_marketplace, cash_out
- Transitions:
  - continue_to_marketplace → MARKETPLACE (convert points to chips)
  - cash_out → MENU (convert points to glitchbytes)

**MARKETPLACE**
- Actions: purchase_item, advance_level
- Transitions:
  - advance_level → LEVEL (level += 1, deduct entry cost)

**GAMEOVER**
- Actions: return_to_menu
- Transitions:
  - return_to_menu → MENU (reset session data)

**VICTORY**
- Actions: return_to_menu
- Entry effects: convert points to glitchbytes, deactivate matrix disarray
- Transitions:
  - return_to_menu → MENU (reset session data)

## Level Configuration
| Level | Milestone | Entry Cost |
|-------|-----------|------------|
| 1     | 12        | 10         |
| 2     | 18        | 1          |
| 3     | 28        | 2          |
| 4     | 44        | 4          |
| 5     | 70        | 6          |
| 6     | 100       | 9          |
| 7     | 150       | 13         |

## Orb Data Model

### Initial Bag State
```
Point(5) × 3
Bomb(1) × 2, Bomb(2) × 1, Bomb(3) × 1
Health(1) × 1
PointsPerAnyOrb(1) × 1
PointsPerBombPulled(4) × 1
Multiplier(1.0) × 1
```

### Orb Types & Effects
| Type | Effect Formula | State Changes |
|------|----------------|---------------|
| health | n/a | health = min(health + amount, 5) |
| point | points + (amount × multiplier) | points += calculated |
| bomb | n/a | health -= amount, bombsPulled += 1 |
| points_per_anyorb | (remaining_orbs × amount) × multiplier | points += calculated |
| points_per_bombpulled | (bombsPulled × amount) × multiplier | points += calculated |
| multiplier | n/a | multiplier += amount |
| bits | n/a | chips += amount |
| glitchbytes | n/a | glitchbytes += amount |

## Shop Data Model

### Item Pools
**Common** (3 selected randomly per level):
| Orb Type | Amount | Base Cost |
|----------|--------|-----------|
| point | 5 | 5 |
| point | 7 | 8 |
| bits | 15 | 5 |
| glitchbytes | 15 | 8 |
| points_per_bombpulled | 4 | 6 |
| health | 1 | 9 |
| multiplier | 0.5 | 9 |

**Rare** (2 selected randomly per level):
| Orb Type | Amount | Base Cost |
|----------|--------|-----------|
| point | 8 | 11 |
| point | 9 | 13 |
| multiplier | 1.0 | 14 |
| multiplier | 1.5 | 16 |

**Cosmic** (1 selected randomly per level):
| Orb Type | Amount | Base Cost |
|----------|--------|-----------|
| health | 3 | 21 |
| glitchbytes | 40 | 23 |

**Basic** (always available):
| Orb Type | Amount | Cost |
|----------|--------|------|
| health | 1 | 2 |
| point | 5 | 2 |

### Pricing Formula
```
current_cost = ceil(base_cost × 1.2^purchase_count)
```
Purchase count persists across levels within same session.

## Special Rules

### Matrix Disarray
**Trigger condition**: `currentLevel = 4 AND matrixDisarray = false`
**Effect**: Add `Bomb(2) × 1` to orbBag.total, set `matrixDisarray = true`
**Duration**: Persists while `currentLevel ≥ 4`
**Reset**: Set `matrixDisarray = false` when entering MENU state

### State Reset Rules
**On enter MENU from any state:**
- Reset: phase, currentLevel, health, points, chips, multiplier, bombsPulled, orbBag, shopDeck, matrixDisarray
- Preserve: glitchbytes

**On enter LEVEL:**
- Reset: health = 5, points = 0, multiplier = 1.0, bombsPulled = 0
- If level = 1: Reset chips = 0, restore orbBag.available from orbBag.total

**On enter MARKETPLACE from CONFIRMATION:**
- Restore orbBag.available from orbBag.total (refill consumed orbs)

### Conversion Rules
| From State | Action | Conversion |
|------------|--------|------------|
| LEVEL | cash_out | points → glitchbytes (1:1) |
| CONFIRMATION | continue | points → chips (1:1) |
| CONFIRMATION | cash_out | points → glitchbytes (1:1) |
| VICTORY | (automatic) | points → glitchbytes (1:1) |
