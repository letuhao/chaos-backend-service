# CMS Service Data Models

## Overview

This document describes the data models used by the CMS service for managing game content. All models are stored in MongoDB and follow a consistent structure with versioning, timestamps, and metadata.

## Common Fields

All models include these common fields:

```json
{
  "id": "string",           // Unique identifier
  "created_at": "datetime", // Creation timestamp
  "updated_at": "datetime", // Last update timestamp
  "version": "number",      // Version number for optimistic locking
  "status": "string",       // Status (draft, active, archived)
  "metadata": {             // Additional metadata
    "author": "string",
    "tags": ["string"],
    "notes": "string"
  }
}
```

## Quest Model

### Schema
```json
{
  "id": "quest_001",
  "title": "The Lost Artifact",
  "description": "Find the ancient artifact hidden in the ruins",
  "type": "main",                    // main, side, daily, weekly
  "status": "active",                // draft, active, archived
  "level_required": 10,
  "level_recommended": 12,
  "prerequisites": ["quest_000"],    // Array of quest IDs that must be completed first
  "objectives": [
    {
      "id": "obj_001",
      "description": "Find the artifact",
      "type": "collect",             // collect, kill, reach, talk, use
      "target": "ancient_artifact",  // Item/monster/location ID
      "quantity": 1,
      "optional": false,
      "hidden": false,
      "conditions": {                // Optional conditions
        "time_limit": 3600,          // Time limit in seconds
        "location": "ruins_ancient"  // Must be in specific location
      }
    }
  ],
  "rewards": {
    "experience": 1000,
    "gold": 500,
    "items": [
      {
        "id": "sword_001",
        "quantity": 1,
        "bound": true                // Item is bound to player
      }
    ],
    "reputation": {
      "faction": "village_guard",
      "amount": 50
    }
  },
  "location": "ruins_ancient",       // Primary location
  "npc_giver": "npc_001",           // NPC who gives the quest
  "npc_turn_in": "npc_002",         // NPC who receives completion
  "time_limit": 3600,               // Quest time limit in seconds
  "repeatable": false,              // Can quest be repeated
  "cooldown": 86400,                // Cooldown between repeats in seconds
  "difficulty": "normal",           // easy, normal, hard, expert
  "category": "story",              // story, combat, exploration, crafting
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["artifact", "ruins", "main_story"],
    "notes": "First quest in the artifact storyline"
  }
}
```

### Objective Types
- `collect`: Collect items
- `kill`: Defeat monsters
- `reach`: Reach a location
- `talk`: Talk to an NPC
- `use`: Use an item
- `craft`: Craft an item
- `explore`: Explore an area
- `survive`: Survive for a duration

### Quest Types
- `main`: Main storyline quests
- `side`: Optional side quests
- `daily`: Daily repeatable quests
- `weekly`: Weekly repeatable quests
- `event`: Special event quests

## NPC Model

### Schema
```json
{
  "id": "npc_001",
  "name": "Elder Sage",
  "type": "quest_giver",            // quest_giver, merchant, guard, citizen, enemy
  "subtype": "elder",               // elder, blacksmith, guard_captain, etc.
  "location": "village_center",
  "description": "A wise elder who gives quests to adventurers",
  "level": 50,
  "faction": "village_guard",       // Faction affiliation
  "stats": {
    "health": 1000,
    "mana": 500,
    "defense": 50,
    "magic_resistance": 75,
    "attack_power": 100,
    "magic_power": 200
  },
  "appearance": {
    "model": "elder_male_01",
    "texture": "elder_sage_texture",
    "animations": ["idle", "talk", "wave"],
    "scale": 1.0,
    "color": "#8B4513"
  },
  "dialogue_tree": {
    "greeting": "Welcome, traveler. I have a task for you.",
    "farewell": "Safe travels, adventurer.",
    "branches": [
      {
        "id": "branch_001",
        "condition": "quest_001_completed",
        "response": "Thank you for completing the quest! Here is your reward.",
        "actions": ["give_reward", "start_quest_002"],
        "next_branches": ["branch_002"]
      },
      {
        "id": "branch_002",
        "condition": "level_10_plus",
        "response": "You look strong enough for this challenge.",
        "actions": ["offer_quest_001"],
        "next_branches": []
      }
    ]
  },
  "inventory": [                    // Items NPC can sell/trade
    {
      "item_id": "item_001",
      "quantity": 5,
      "price": 100,
      "stock": 10,                  // Maximum stock
      "restock_time": 3600          // Restock time in seconds
    }
  ],
  "quests": ["quest_001", "quest_002"], // Quests this NPC can give
  "services": ["heal", "repair", "identify"], // Services NPC provides
  "schedule": {                     // NPC daily schedule
    "monday": {
      "start": "06:00",
      "end": "22:00",
      "location": "village_center"
    },
    "tuesday": {
      "start": "06:00",
      "end": "22:00",
      "location": "village_center"
    }
  },
  "ai_behavior": {
    "patrol_route": ["point_001", "point_002", "point_003"],
    "patrol_speed": 1.0,
    "reaction_distance": 10.0,
    "flee_health_threshold": 0.2
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["elder", "quest_giver", "village"],
    "notes": "Main quest giver for the village"
  }
}
```

### NPC Types
- `quest_giver`: Gives quests to players
- `merchant`: Sells items to players
- `guard`: Protects areas
- `citizen`: Regular NPCs with dialogue
- `enemy`: Hostile NPCs

### Dialogue Conditions
- `quest_completed`: Quest must be completed
- `level_10_plus`: Player level must be 10 or higher
- `item_in_inventory`: Player must have specific item
- `faction_reputation`: Faction reputation requirement
- `time_of_day`: Specific time of day
- `weather`: Specific weather condition

## Item Model

### Schema
```json
{
  "id": "item_001",
  "name": "Ancient Sword",
  "type": "weapon",                 // weapon, armor, consumable, misc, quest
  "subtype": "sword",               // sword, bow, staff, etc.
  "rarity": "legendary",            // common, uncommon, rare, epic, legendary
  "level_required": 20,
  "description": "A sword forged in ancient times with magical properties",
  "flavor_text": "The blade hums with ancient power...",
  "icon": "sword_ancient.png",
  "model": "sword_ancient_3d.obj",
  "texture": "sword_ancient_texture.png",
  "stats": {
    "damage": 150,
    "durability": 100,
    "critical_chance": 0.15,
    "critical_damage": 2.0,
    "attack_speed": 1.2,
    "range": 1.5
  },
  "requirements": {
    "strength": 25,
    "dexterity": 15,
    "class": ["warrior", "paladin"]  // Required classes
  },
  "effects": [
    {
      "type": "passive",
      "name": "Ancient Power",
      "description": "Increases damage by 10% against undead",
      "value": 0.1,
      "target": "undead"
    },
    {
      "type": "on_hit",
      "name": "Life Steal",
      "description": "Heals for 5% of damage dealt",
      "value": 0.05,
      "chance": 0.1
    }
  ],
  "crafting": {
    "materials": [
      {
        "item_id": "iron_ingot",
        "quantity": 5
      },
      {
        "item_id": "magic_crystal",
        "quantity": 2
      }
    ],
    "skill_required": "blacksmithing",
    "skill_level": 50,
    "crafting_time": 3600,          // Time in seconds
    "success_rate": 0.8
  },
  "enchanting": {
    "max_enchant_level": 10,
    "enchant_materials": [
      {
        "item_id": "enchant_essence",
        "quantity": 1
      }
    ]
  },
  "stackable": false,               // Can items be stacked
  "stack_size": 1,                  // Maximum stack size
  "bound": false,                   // Item is bound to player
  "soulbound": false,               // Item is soulbound (cannot be traded)
  "droppable": true,                // Can item be dropped
  "tradable": true,                 // Can item be traded
  "sellable": true,                 // Can item be sold to NPCs
  "value": 1000,                    // Base value in gold
  "weight": 2.5,                    // Weight in game units
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["sword", "ancient", "legendary"],
    "notes": "Legendary sword from the ancient civilization"
  }
}
```

### Item Types
- `weapon`: Weapons for combat
- `armor`: Protective equipment
- `consumable`: Items that are consumed when used
- `misc`: Miscellaneous items
- `quest`: Quest-specific items

### Rarity Levels
- `common`: Basic items (white)
- `uncommon`: Slightly better items (green)
- `rare`: Good items (blue)
- `epic`: Very good items (purple)
- `legendary`: Best items (orange)

## Location Model

### Schema
```json
{
  "id": "location_001",
  "name": "Village Center",
  "type": "city",                   // city, dungeon, wilderness, instance
  "subtype": "village",             // village, town, city, ruins, cave, forest
  "region": "north",                // Geographic region
  "level_range": "1-10",            // Recommended level range
  "description": "The bustling center of the village where adventurers gather",
  "coordinates": {
    "x": 100,
    "y": 200,
    "z": 0,
    "world": "overworld"            // World/dimension
  },
  "bounds": {                       // Area boundaries
    "min_x": 50,
    "max_x": 150,
    "min_y": 150,
    "max_y": 250,
    "min_z": -10,
    "max_z": 10
  },
  "map": "village_center.png",      // Map image
  "minimap": "village_center_mini.png", // Minimap icon
  "npcs": ["npc_001", "npc_002", "npc_003"],
  "shops": ["shop_001", "shop_002"],
  "quests": ["quest_001", "quest_002"],
  "connections": [                  // Connected locations
    {
      "location_id": "location_002",
      "direction": "north",
      "distance": 100,
      "travel_time": 300,           // Travel time in seconds
      "requirements": {
        "level": 5,
        "quests": ["quest_001"]
      }
    }
  ],
  "spawns": [                       // Monster spawns
    {
      "monster_id": "monster_001",
      "level": 5,
      "spawn_rate": 0.1,            // Spawns per second
      "max_count": 10,
      "respawn_time": 300,          // Respawn time in seconds
      "spawn_area": {
        "min_x": 60,
        "max_x": 140,
        "min_y": 160,
        "max_y": 240
      }
    }
  ],
  "resources": [                    // Harvestable resources
    {
      "resource_id": "iron_ore",
      "spawn_rate": 0.05,
      "max_count": 5,
      "respawn_time": 1800,
      "skill_required": "mining",
      "skill_level": 10
    }
  ],
  "weather": {
    "type": "clear",                // clear, rain, snow, fog
    "intensity": 0.5,               // 0.0 to 1.0
    "duration": 3600,               // Duration in seconds
    "effects": ["visibility_reduced"]
  },
  "ambient_sounds": [
    {
      "sound_id": "village_ambient",
      "volume": 0.7,
      "loop": true
    }
  ],
  "lighting": {
    "ambient": 0.8,                 // Ambient light level
    "directional": 0.6,             // Directional light level
    "color": "#FFE4B5"              // Light color
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["village", "safe", "hub"],
    "notes": "Main hub for new players"
  }
}
```

### Location Types
- `city`: Safe areas with NPCs and services
- `dungeon`: Dangerous areas with monsters
- `wilderness`: Open world areas
- `instance`: Instanced areas for specific content

## Dialogue Model

### Schema
```json
{
  "id": "dialogue_001",
  "npc_id": "npc_001",
  "title": "Quest Introduction",
  "branches": [
    {
      "id": "branch_001",
      "text": "Welcome, traveler. I have a task for you.",
      "speaker": "npc",             // npc, player, narrator
      "emotion": "friendly",        // friendly, angry, sad, excited
      "conditions": {
        "quest_completed": "quest_000",
        "level_min": 10
      },
      "responses": [
        {
          "id": "response_001",
          "text": "I'm ready to help!",
          "action": "start_quest_001",
          "next_branch": "branch_002"
        },
        {
          "id": "response_002",
          "text": "I need to prepare first.",
          "action": "none",
          "next_branch": "branch_003"
        }
      ]
    }
  ],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["quest", "introduction"],
    "notes": "Main quest introduction dialogue"
  }
}
```

## Event Model

### Schema
```json
{
  "id": "event_001",
  "name": "Dragon Attack",
  "type": "world_event",            // world_event, server_event, player_event
  "description": "A dragon attacks the village",
  "status": "scheduled",            // scheduled, active, completed, cancelled
  "start_time": "2024-01-01T12:00:00Z",
  "end_time": "2024-01-01T14:00:00Z",
  "duration": 7200,                 // Duration in seconds
  "location": "village_center",
  "participants": {
    "min_players": 5,
    "max_players": 50,
    "level_required": 20
  },
  "objectives": [
    {
      "id": "obj_001",
      "description": "Defeat the dragon",
      "type": "kill",
      "target": "dragon_boss",
      "quantity": 1
    }
  ],
  "rewards": {
    "experience": 5000,
    "gold": 2000,
    "items": ["dragon_scale", "legendary_sword"],
    "title": "Dragon Slayer"
  },
  "conditions": {
    "weather": "clear",
    "time_of_day": "day",
    "server_population": 100
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1,
  "metadata": {
    "author": "admin",
    "tags": ["dragon", "world_event", "boss"],
    "notes": "Major world event for high-level players"
  }
}
```

## File Model

### Schema
```json
{
  "id": "file_001",
  "filename": "sword_ancient.png",
  "original_name": "ancient_sword.png",
  "type": "image",                  // image, audio, video, document
  "category": "item_icons",         // item_icons, npc_models, sounds, etc.
  "size": 1024000,                  // File size in bytes
  "mime_type": "image/png",
  "url": "/files/file_001",
  "thumbnail_url": "/files/file_001/thumbnail",
  "dimensions": {
    "width": 64,
    "height": 64
  },
  "metadata": {
    "author": "artist_001",
    "description": "Icon for ancient sword",
    "tags": ["sword", "ancient", "weapon"]
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "version": 1
}
```

## Version Model

### Schema
```json
{
  "id": "version_001",
  "content_type": "quest",          // quest, npc, item, location, etc.
  "content_id": "quest_001",
  "version": 2,
  "changes": "Updated objectives and rewards",
  "author": "admin",
  "data": {                         // Full content data at this version
    "id": "quest_001",
    "title": "The Lost Artifact",
    "description": "Find the ancient artifact hidden in the ruins",
    "objectives": [
      {
        "id": "obj_001",
        "description": "Find the artifact",
        "type": "collect",
        "target": "ancient_artifact",
        "quantity": 1
      }
    ]
  },
  "created_at": "2024-01-01T00:00:00Z",
  "metadata": {
    "reason": "Updated quest balance",
    "tags": ["balance", "objectives"],
    "notes": "Reduced difficulty and increased rewards"
  }
}
```

## Indexes

### Quest Indexes
```javascript
// MongoDB indexes for optimal query performance
db.quests.createIndex({ "status": 1, "type": 1 })
db.quests.createIndex({ "level_required": 1 })
db.quests.createIndex({ "location": 1 })
db.quests.createIndex({ "npc_giver": 1 })
db.quests.createIndex({ "title": "text", "description": "text" })
```

### NPC Indexes
```javascript
db.npcs.createIndex({ "type": 1, "location": 1 })
db.npcs.createIndex({ "level": 1 })
db.npcs.createIndex({ "faction": 1 })
db.npcs.createIndex({ "name": "text", "description": "text" })
```

### Item Indexes
```javascript
db.items.createIndex({ "type": 1, "rarity": 1 })
db.items.createIndex({ "level_required": 1 })
db.items.createIndex({ "name": "text", "description": "text" })
```

### Location Indexes
```javascript
db.locations.createIndex({ "type": 1, "region": 1 })
db.locations.createIndex({ "level_range": 1 })
db.locations.createIndex({ "name": "text", "description": "text" })
```

## Data Validation

### Quest Validation Rules
- `title`: Required, max 100 characters
- `description`: Required, max 1000 characters
- `type`: Must be one of: main, side, daily, weekly, event
- `status`: Must be one of: draft, active, archived
- `level_required`: Must be between 1 and 100
- `objectives`: Must have at least one objective
- `rewards`: Must have at least one reward type

### NPC Validation Rules
- `name`: Required, max 50 characters
- `type`: Must be one of: quest_giver, merchant, guard, citizen, enemy
- `location`: Must reference valid location
- `level`: Must be between 1 and 100
- `stats`: All stat values must be non-negative

### Item Validation Rules
- `name`: Required, max 50 characters
- `type`: Must be one of: weapon, armor, consumable, misc, quest
- `rarity`: Must be one of: common, uncommon, rare, epic, legendary
- `level_required`: Must be between 1 and 100
- `stats`: All stat values must be non-negative

### Location Validation Rules
- `name`: Required, max 50 characters
- `type`: Must be one of: city, dungeon, wilderness, instance
- `level_range`: Must be valid range format (e.g., "1-10")
- `coordinates`: Must have valid x, y, z values

## Relationships

### Quest Relationships
- `prerequisites`: References other quests
- `location`: References location
- `npc_giver`: References NPC
- `npc_turn_in`: References NPC
- `objectives.target`: References items, monsters, locations, NPCs

### NPC Relationships
- `location`: References location
- `inventory.item_id`: References items
- `quests`: References quests
- `faction`: References faction

### Item Relationships
- `crafting.materials.item_id`: References items
- `requirements.class`: References character classes

### Location Relationships
- `npcs`: References NPCs
- `shops`: References shops
- `quests`: References quests
- `connections.location_id`: References other locations
- `spawns.monster_id`: References monsters
- `resources.resource_id`: References resources
