# Jankenryusagi - System Architecture

This document outlines the technical architecture and implementation guidelines for the Jankenryusagi game system. The architecture is designed to be language-agnostic, allowing developers to implement the system using their preferred technology stack.

## System Overview

### High-Level Architecture

```
┌─────────────┐     WebSocket     ┌──────────────┐
│   Client    │<----------------->│  Game Server │
└─────────────┘                   └──────────────┘
                                        │
                                        │
                                  ┌──────────────┐
                                  │   Database   │
                                  └──────────────┘
```

## Deployment Architecture

### Docker Compose Setup

```yaml
services:
  frontend:
    # Frontend service container
    ports:
      - "3000:3000"
    depends_on:
      - backend

  backend:
    # Backend service container
    ports:
      - "8080:8080"
    depends_on:
      - db

  db:
    image: postgres:latest
    environment:
      POSTGRES_DB: jankenryusagi
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## Database Schema

### Tables

```sql
-- Players table
CREATE TABLE players (
    id UUID PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Game Rooms table
CREATE TABLE game_rooms (
    id UUID PRIMARY KEY,
    status VARCHAR(20) NOT NULL, -- 'waiting', 'in_progress', 'completed', 'paused'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Game Sessions table
CREATE TABLE game_sessions (
    id UUID PRIMARY KEY,
    room_id UUID REFERENCES game_rooms(id),
    player_id UUID REFERENCES players(id),
    character_class VARCHAR(20) NOT NULL,
    points INTEGER DEFAULT 0,
    status VARCHAR(20) NOT NULL, -- 'active', 'disconnected'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Game Rounds table
CREATE TABLE game_rounds (
    id UUID PRIMARY KEY,
    room_id UUID REFERENCES game_rooms(id),
    round_number INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL, -- 'pending', 'in_progress', 'completed'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## WebSocket Communication Protocol

### Message Format

All WebSocket messages follow this JSON format:

```json
{
    "type": "string",      // Message type
    "payload": {},        // Message data
    "timestamp": "string" // ISO 8601 timestamp
}
```

### Message Types

#### Client -> Server Messages

1. Room Management
```json
{
    "type": "create_room",
    "payload": {
        "player_id": "uuid"
    }
}

{
    "type": "join_room",
    "payload": {
        "room_id": "uuid",
        "player_id": "uuid"
    }
}
```

2. Game Actions
```json
{
    "type": "select_class",
    "payload": {
        "class_name": "string",
        "room_id": "uuid",
        "player_id": "uuid"
    }
}

{
    "type": "play_card",
    "payload": {
        "card_type": "string",
        "room_id": "uuid",
        "player_id": "uuid",
        "round_number": "integer"
    }
}

{
    "type": "use_ability",
    "payload": {
        "ability_name": "string",
        "room_id": "uuid",
        "player_id": "uuid",
        "target_player_id": "uuid"
    }
}

{
    "type": "quick_draw_action",
    "payload": {
        "room_id": "uuid",
        "player_id": "uuid",
        "timestamp": "string"
    }
}
```

#### Server -> Client Messages

1. Room Updates
```json
{
    "type": "room_status",
    "payload": {
        "room_id": "uuid",
        "status": "string",
        "players": [
            {
                "player_id": "uuid",
                "username": "string",
                "status": "string"
            }
        ]
    }
}
```

2. Game State Updates
```json
{
    "type": "game_state",
    "payload": {
        "room_id": "uuid",
        "round_number": "integer",
        "phase": "string",
        "players": [
            {
                "player_id": "uuid",
                "points": "integer",
                "character_class": "string",
                "abilities_remaining": {}
            }
        ],
        "timer": "integer"
    }
}
```

3. Round Results
```json
{
    "type": "round_result",
    "payload": {
        "room_id": "uuid",
        "round_number": "integer",
        "players": [
            {
                "player_id": "uuid",
                "card_played": "string",
                "ability_used": "string",
                "points_gained": "integer"
            }
        ]
    }
}
```

## Error Handling

### Disconnect Handling

1. When a player disconnects:
   - Server marks player's session as 'disconnected'
   - Game room status changes to 'paused'
   - Remaining player receives disconnect notification
   - Server starts a reconnection timer (e.g., 2 minutes)

2. Reconnection Process:
   - Player can rejoin using the same room_id and player_id
   - Game state is restored from database
   - Both players receive updated game state
   - Game resumes from the last valid state

3. Timeout Handling:
   - If disconnected player doesn't reconnect within the time limit:
     - Game is marked as completed
     - Connected player wins by default
     - Room is closed

### Error Response Format

```json
{
    "type": "error",
    "payload": {
        "code": "string",
        "message": "string",
        "details": {}
    }
}
```

## System Interfaces

### Game Room Interface
```typescript
interface GameRoom {
    id: string;
    status: 'waiting' | 'in_progress' | 'completed' | 'paused';
    players: Player[];
    currentRound: number;
    gameState: GameState;
}
```

### Player Interface
```typescript
interface Player {
    id: string;
    username: string;
    characterClass?: CharacterClass;
    status: 'active' | 'disconnected';
    points: number;
    hand: Card[];
    abilities: Ability[];
}
```

### Game State Interface
```typescript
interface GameState {
    phase: 'class_selection' | 'card_selection' | 'ability_usage' | 'resolution' | 'tiebreaker';
    roundNumber: number;
    timer: number;
    players: Player[];
    roundHistory: RoundResult[];
}
```

### Card Interface
```typescript
interface Card {
    type: 'rock' | 'paper' | 'scissors' | 'dragon' | 'rabbit';
    status: 'in_hand' | 'played' | 'removed';
}
```

### Ability Interface
```typescript
interface Ability {
    name: string;
    uses: number;
    effect: AbilityEffect;
}
```

## Implementation Guidelines

1. **State Management**
   - Implement robust state machine for game phases
   - Use database transactions for critical state changes
   - Cache frequently accessed game states

2. **Concurrency Handling**
   - Implement locks for critical game operations
   - Use optimistic locking for game state updates
   - Handle race conditions in ability usage

3. **Performance Considerations**
   - Implement connection pooling for database
   - Use appropriate indexes on frequently queried fields
   - Consider caching for game room states

4. **Security Measures**
   - Implement player authentication
   - Validate all WebSocket messages
   - Rate limit client messages
   - Sanitize all user inputs

5. **Monitoring and Logging**
   - Log all game state changes
   - Monitor WebSocket connection health
   - Track game metrics (rounds played, disconnects, etc.)
   - Implement error tracking and reporting