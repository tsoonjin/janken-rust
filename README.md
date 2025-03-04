# Jankenryusagi 🎮

Jankenryusagi is an exciting twist on the classic Rock, Paper, Scissors game, featuring dragons, rabbits, and unique character classes that add strategic depth to the gameplay.

## Game Overview

Jankenryusagi is a 2-player card battle game that combines the familiar Rock-Paper-Scissors mechanics with additional elements:
- 5 card types: Rock, Paper, Scissors, Dragon, and Rabbit
- 4 unique character classes with special abilities
- 5 rounds of strategic gameplay
- Exciting quick-draw tiebreaker system

## Card Mechanics

### Basic Rules
- **Standard Rules Apply**: Rock beats Scissors, Scissors beats Paper, Paper beats Rock
- **Dragon**: Beats everything except Rabbit
- **Rabbit**: Loses to everything except Dragon

### Card Distribution
- Each player receives 5 cards
- Every hand guaranteed to contain:
  - 1 Dragon card
  - 1 Rabbit card
  - 3 randomly selected cards (Rock/Paper/Scissors)

## Gameplay Flow

### 1. Game Room Setup
- Players can create or join an existing game room
- Game starts when 2 players join the room

### 2. Class Selection
- Players have 1 minute to choose one of 4 classes
- If no selection is made, a random class is assigned

### 3. Main Game (5 Rounds)
Each round consists of:
1. Card Selection Phase
2. Ability Usage Phase
3. Resolution Phase
   - Winner gets 1 point
   - Abilities may modify point distribution

### 4. Tiebreaker (If Needed)
- Triggered when players are tied after 5 rounds
- Quick-draw mechanic:
  - Signal appears randomly within 1 minute
  - Fastest player to press the button wins

## Character Classes

### 🛡️ Royalguard
- **Uses**: 3 times per game
- **Ability**: Defensive Stance
  - In case of a draw: Gain 1 point
  - When losing: Opponent gains no points

### ⚔️ Swordmaster
- **Uses**: 3 times per game
- **Ability**: Power Strike
  - When winning: Gain 2 points (instead of 1)
  - When losing: Opponent gains 2 points
  - In case of a draw: Opponent gains 1 point

### 🔫 Gunslinger
- **Uses**: 2 times per game
- **Ability**: Card Snipe
  - When winning: Remove one card from opponent's hand

### 🎭 Trickster
- **Uses**: 2 abilities (1 use each)
- **Abilities**:
  1. Card Peek: See opponent's selected card
  2. Card Swap: Exchange one of your cards with opponent

## Victory Conditions
- Player with the most points after 5 rounds wins
- In case of a tie, winner is determined by quick-draw tiebreaker

## Technical Implementation Notes

### Game States
1. Room Creation/Joining
2. Player Class Selection (60-second timer)
3. Round Phases:
   - Card Selection
   - Ability Activation
   - Resolution
4. Tiebreaker (if needed)
5. Game Conclusion

### Critical Features
- Real-time player interaction
- Timer management
- Card distribution system
- Class ability effects
- Point calculation
- Quick-draw mechanism

---

## Development

This project is implemented in Rust. For detailed technical documentation and setup instructions, please refer to the development documentation.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Getting Started

### Running with Docker Compose

1. **Set Environment Variables:**
   - Create a `.env` file in the project root with the following content:
   ```plaintext
   DB_USER=postgres
   DB_PASSWORD=your_password
```

1. **Install Docker and Docker Compose:**
   - Ensure Docker and Docker Compose are installed on your system. You can download them from [Docker's official website](https://www.docker.com/products/docker-desktop).

2. **Clone the Repository:**
   - Open a terminal and run the following command to clone the repository:
   ```bash
   git clone https://github.com/tsoonjin/jankenryusagi.git
```
3. **Running docker-compose**
```
docker-compose up --build` to build and start all services
docker-compose down` to stop and remove all containers
```

These commands should be run from the project root directory where the `docker-compose.yml` file is located.

## Frontend Features

The Jankenryusagi game includes the following frontend features:

- **Landing Page:**
  - Shows a list of game rooms and allows users to create a new room with a unique name.
  - Provides a search functionality for existing rooms. If a room doesn't exist, the button changes from "Join" to "Create".

- **Leaderboard:**
  - Displays an infinite scrolling list of players with their win, draw, loss, and match statistics.

- **Player Detail Page:**
  - Shows detailed player information, including match history and head-to-head records with other players.