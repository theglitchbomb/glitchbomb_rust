# Glitch Bomb - Game Essentials

## Core Loop
Pull random orbs from your bag to score points while managing health across 7 levels. Reach the milestone to advance and buy better orbs.

## Currencies
- **Glitchbytes**: Persistent currency, pay to enter levels (starts at 1,000)
- **Chips**: Session currency, earned from points, spend in shop
- **Points**: Level currency, converted to chips/glitchbytes when advancing/cashing out
- **Health**: 5 max, reset each level, game over at 0

## Game Flow
menu → level → (milestone reached) → shop → next level → ... → victory
         ↓ (health=0 or no orbs)
      gameover

## Levels
| Level | Milestone | Entry Cost |
|-------|-----------|------------|
| 1     | 12        | 10 GB      |
| 2     | 18        | 1 GB       |
| 3     | 28        | 2 GB       |
| 4     | 44        | 4 GB       |
| 5     | 70        | 6 GB       |
| 6     | 100       | 9 GB       |
| 7     | 150       | 13 GB      |

**Win**: Reach milestone → advance
**Lose**: Health=0 or no orbs before milestone
**Victory**: Complete level 7

## Orb System

**Starting Bag**: 3 Point(5), 4 Bombs(1,1,2,3), 1 Health(1), 1 Combo(1/orb), 1 Danger(4/bomb), 1 Multiplier(1.0x)

**Mechanics**: Pull random orb → apply effect → orb consumed. Shop refills all orbs between levels.

### Orb Types
1. **Health** - Restore HP (max 5)
2. **Point** - Add points (×multiplier)
3. **Bomb** - Take damage, increment bomb counter
4. **Combo** - Points = remaining orbs × amount (×multiplier)
5. **Danger** - Points = bombs pulled this level × amount (×multiplier)
6. **Multiplier** - Boost point orbs for rest of level (cumulative)
7. **Chips** - Gain chips directly
8. **Glitchbytes** - Gain glitchbytes directly

## Shop

**Access**: After reaching milestone, points → chips (1:1)

**Pricing**: Each purchase increases that item's price by 20% (baseCost × 1.2^purchases)

**Available**: 6 random items from tier pools + 2 basic orbs
- 3 Common (5-9 chips): Point orbs, chips, glitchbytes, danger, health, multiplier
- 2 Rare (11-16 chips): Bigger point orbs, bigger multipliers
- 1 Cosmic (21-23 chips): +3 HP, +40 GB
- Basic: Health(2), Point(2)

## Key Mechanics

**Multiplier**: Starts at 1.0 each level, increased by multiplier orbs, affects point/combo/danger orbs

**Cash Out**: Convert points → glitchbytes (1:1), return to menu, session resets

**Matrix Disarray**: Special event adds extra 2-damage bomb in levels 4-7

## Win/Lose

**Victory**: Beat level 7, points → glitchbytes
**Game Over**: Health=0 or no orbs before milestone
**Profit**: Glitchbytes persist, everything else resets on new game
