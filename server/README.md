# Guessing Game Server

## Layers

### Infrastructure

- Network
- Database
- Infrastructure as Implementation
  - Inject implementation into Services layer
  - `ConcreteDatabase.write(data);`

### Services

- World
  - Create new games
  - Apply simulation updates to games
  - Convert infrastructure events to domain events
- Players
  - Manage player sessions/connections
  - Manage player data
- Infrastructure as Abstractions
  - `PlayerStore.save(player);`

### Domain

- Accept domain event inputs
- Simulate game with domain I/O
- Output domain events
