# Functional specification for v0.1

## 1. Introduction

Being the first playable version, the scope of v0.1 is very limited and heavily focused on core gameplay and networking. It must serve as a solid base for future updates.

## 2. Game mechanics

### 2.1. Core gameplay

The basic gameplay foundation of Ulneron is movement, automatic combat and stat progression. These three elements are represented in v0.1 in their most basic form. Players are able to play missions and fight ever increasing hordes of Nerons. Each player controls a single character, gaining Nerite, upgrading their stats and surviving for as long as they can.

### 2.2. Agents

  - Human soldiers: one for each player.
  - Nerons: controlled by the AI.

### 2.3. Gameplay elements

  - Health: the amount of damage a unit can take before dying.
  - Nerite: the vital energy of Ulneron, used by humans and nerons to enhance their capabilities.
  - Stats: numeric representation of the capabilities of a unit. These are:
      - Maximum health: limit of the health value.
      - Health regeneration: health regenerated per second.
      - Movement speed: distance traveled per second.
      - Attack damage: damage dealt per attack.
      - Attack rate: attacks performed per second.
  - Stat upgrades: enhancements associated to an specific stat and Nerite cost.

### 2.4. Player actions

  - Moving in their desired direction.
  - Acquiring stat upgrades with Nerite.

### 2.5. Gameplay systems

  - Units die when their health reaches zero.
  - Still players attack enemies in their range.
  - Enemies chase their closest player.
  - Enemies attack players in their range.
  - Points are awarded to players for killing enemies.
  - Enemies spawn at an ever increasing rate.

### 2.6. Multiplayer

  - Players join games by either:
      - Hosting a game.
      - Connecting to an existing game.
  - Hosts control when the game starts.
  - Players can't join a game once it has started.

## 3. User interface

### 3.1. In lobby

  - Players see their game connection status.
  - Players see the number of players in the lobby.

### 3.2. In game

  - Players see their health.
  - Players see their stats.
  - Players see their Nerite.
  - Players see their available upgrades.
  - Players can acquire upgrades.

## 4. Art

Art will be minimal in this first version and primitive models will be used as placeholders.

## 5. Story

Even though lore elements are used in this document for names and context, the story is not presented to the player in any way in v0.1.
