# Database Schema Diagrams

## 🗄️ Core Database Schema

```mermaid
erDiagram
    USERS ||--o{ ACTORS : "has"
    USERS ||--o{ USER_PREFERENCES : "has"
    USERS ||--o{ PAYMENT_TRANSACTIONS : "makes"
    USERS ||--o{ CHAT_MESSAGES : "sends"
    USERS ||--o{ EVENT_PARTICIPATIONS : "participates"
    
    ACTORS ||--o{ ACTOR_RESOURCES : "has"
    ACTORS ||--o{ ACTOR_SKILLS : "learns"
    ACTORS ||--o{ ACTOR_EQUIPMENT : "wears"
    ACTORS ||--o{ INVENTORY_ITEMS : "owns"
    ACTORS ||--o{ GUILD_MEMBERS : "member_of"
    
    GUILDS ||--o{ GUILD_MEMBERS : "contains"
    GUILDS ||--o{ GUILD_STORAGE : "owns"
    GUILDS ||--o{ TERRITORIES : "controls"
    GUILDS ||--o{ GUILD_ACTIVITIES : "hosts"
    
    ITEMS ||--o{ INVENTORY_ITEMS : "instance_of"
    ITEMS ||--o{ CRAFTING_RECIPES : "used_in"
    ITEMS ||--o{ EVENT_REWARDS : "rewarded_as"
    
    EVENTS ||--o{ EVENT_PARTICIPATIONS : "has"
    EVENTS ||--o{ EVENT_REWARDS : "gives"
    EVENTS ||--o{ EVENT_SCHEDULES : "scheduled_by"
    
    CHANNELS ||--o{ CHAT_MESSAGES : "contains"
    CHANNELS ||--o{ CHANNEL_MEMBERS : "has"
    
    MATCHES ||--o{ MATCH_PARTICIPANTS : "has"
    MATCHES ||--o{ MATCH_RESULTS : "produces"
    
    USERS {
        uuid id PK
        string username UK
        string email UK
        string password_hash
        string status
        timestamp created_at
        timestamp last_login
        json preferences
    }
    
    ACTORS {
        uuid id PK
        uuid user_id FK
        string name
        int level
        bigint experience
        json stats
        json position
        timestamp created_at
        timestamp last_active
    }
    
    GUILDS {
        uuid id PK
        string guild_id UK
        string name
        text description
        string emblem_url
        int level
        bigint experience
        int max_members
        bigint treasury
        string status
        timestamp created_at
        timestamp updated_at
    }
    
    ITEMS {
        uuid id PK
        string item_id UK
        string name
        text description
        string item_type
        string rarity
        int level_requirement
        int value
        int stack_size
        json properties
        json requirements
        json effects
        boolean is_active
        int version
        timestamp created_at
        timestamp updated_at
    }
    
    EVENTS {
        uuid id PK
        string event_id UK
        string name
        text description
        string event_type
        string category
        string status
        timestamp start_time
        timestamp end_time
        int duration
        int max_participants
        int current_participants
        json requirements
        json rewards
        json settings
        uuid created_by
        timestamp created_at
        timestamp updated_at
    }
    
    CHANNELS {
        uuid id PK
        string channel_id UK
        string name
        text description
        string channel_type
        uuid owner_id
        uuid guild_id
        boolean is_active
        int max_members
        json permissions
        timestamp created_at
        timestamp updated_at
    }
    
    MATCHES {
        uuid id PK
        string match_id UK
        string game_mode
        string match_type
        json participants
        json match_data
        string status
        timestamp started_at
        timestamp ended_at
        int duration
        int winner_team
        timestamp created_at
    }
```

## 🎮 Game Data Schema

```mermaid
erDiagram
    ACTOR_RESOURCES ||--|| RESOURCE_DEFINITIONS : "defined_by"
    ACTOR_SKILLS ||--|| SKILL_DEFINITIONS : "defined_by"
    ACTOR_EQUIPMENT ||--|| ITEMS : "equipped_item"
    INVENTORY_ITEMS ||--|| ITEMS : "item_definition"
    CRAFTING_RECIPES ||--|| ITEMS : "result_item"
    
    RESOURCE_DEFINITIONS {
        uuid id PK
        string resource_id UK
        string name
        text description
        string category
        string resource_type
        decimal base_value
        decimal min_value
        decimal max_value
        decimal regen_rate
        string regen_type
        json properties
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    SKILL_DEFINITIONS {
        uuid id PK
        string skill_id UK
        string name
        text description
        string skill_type
        string element
        int level_requirement
        int mana_cost
        int cooldown
        decimal damage
        decimal range
        json effects
        json requirements
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    ACTOR_RESOURCES {
        uuid id PK
        uuid actor_id FK
        string resource_id FK
        decimal current_value
        decimal max_value
        decimal regen_rate
        timestamp last_regen
        timestamp created_at
        timestamp updated_at
    }
    
    ACTOR_SKILLS {
        uuid id PK
        uuid actor_id FK
        string skill_id FK
        int level
        int experience
        boolean is_learned
        timestamp learned_at
        timestamp last_used
    }
    
    ACTOR_EQUIPMENT {
        uuid id PK
        uuid actor_id FK
        string slot_type
        uuid item_id FK
        int durability
        int enhancement_level
        json properties
        timestamp equipped_at
    }
    
    INVENTORY_ITEMS {
        uuid id PK
        uuid actor_id FK
        int slot_position
        uuid item_id FK
        int quantity
        int durability
        int enhancement_level
        json properties
        timestamp acquired_at
        timestamp updated_at
    }
    
    CRAFTING_RECIPES {
        uuid id PK
        string recipe_id UK
        uuid result_item_id FK
        int result_quantity
        json required_items
        int required_level
        string required_skill
        int crafting_time
        decimal success_rate
        timestamp created_at
        timestamp updated_at
    }
```

## 💬 Communication Schema

```mermaid
erDiagram
    CHANNELS ||--o{ CHAT_MESSAGES : "contains"
    CHANNELS ||--o{ CHANNEL_MEMBERS : "has"
    CHAT_MESSAGES ||--o{ MESSAGE_REACTIONS : "has"
    CHAT_MESSAGES ||--o{ MESSAGE_REPLIES : "replied_to"
    
    CHANNEL_MEMBERS {
        uuid id PK
        uuid channel_id FK
        uuid user_id FK
        string role
        json permissions
        timestamp joined_at
        timestamp last_read_at
        boolean is_muted
        boolean is_banned
    }
    
    CHAT_MESSAGES {
        uuid id PK
        string message_id UK
        uuid channel_id FK
        uuid user_id FK
        text content
        string message_type
        string reply_to
        json metadata
        boolean is_edited
        timestamp edited_at
        boolean is_deleted
        timestamp deleted_at
        timestamp created_at
    }
    
    MESSAGE_REACTIONS {
        uuid id PK
        uuid message_id FK
        uuid user_id FK
        string emoji
        timestamp created_at
    }
    
    MESSAGE_REPLIES {
        uuid id PK
        uuid parent_message_id FK
        uuid reply_message_id FK
        timestamp created_at
    }
    
    NOTIFICATIONS {
        uuid id PK
        string notification_id UK
        uuid user_id FK
        string notification_type
        string channel
        string title
        text message
        json data
        string status
        string priority
        timestamp scheduled_at
        timestamp sent_at
        timestamp delivered_at
        text failure_reason
        int retry_count
        timestamp created_at
        timestamp updated_at
    }
```

## 🏆 Progression Schema

```mermaid
erDiagram
    ACTORS ||--o{ ACTOR_LEVELS : "has"
    ACTORS ||--o{ ACHIEVEMENTS : "earns"
    ACTORS ||--o{ QUEST_PROGRESS : "tracks"
    ACTORS ||--o{ RATINGS : "has"
    
    ACTOR_LEVELS {
        uuid id PK
        uuid actor_id FK
        int level
        bigint experience_required
        bigint experience_current
        json stat_bonuses
        json skill_points
        timestamp achieved_at
    }
    
    ACHIEVEMENTS {
        uuid id PK
        string achievement_id UK
        string name
        text description
        string category
        int points
        json requirements
        json rewards
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    ACTOR_ACHIEVEMENTS {
        uuid id PK
        uuid actor_id FK
        uuid achievement_id FK
        timestamp earned_at
        json progress_data
    }
    
    QUESTS {
        uuid id PK
        string quest_id UK
        string title
        text description
        string quest_type
        int level_requirement
        json prerequisites
        json objectives
        json rewards
        uuid npc_id
        string zone_id
        boolean is_repeatable
        int cooldown
        boolean is_active
        int version
        timestamp created_at
        timestamp updated_at
    }
    
    QUEST_PROGRESS {
        uuid id PK
        uuid actor_id FK
        uuid quest_id FK
        string status
        json progress
        timestamp started_at
        timestamp completed_at
    }
    
    RATINGS {
        uuid id PK
        uuid actor_id FK
        string game_mode
        int rating
        int wins
        int losses
        int draws
        int win_streak
        int loss_streak
        timestamp last_played
        timestamp created_at
        timestamp updated_at
    }
```

## 🎯 Matchmaking Schema

```mermaid
erDiagram
    MATCHES ||--o{ MATCH_PARTICIPANTS : "has"
    MATCHES ||--o{ MATCH_RESULTS : "produces"
    MATCH_QUEUES ||--o{ MATCHES : "creates"
    
    MATCH_QUEUES {
        uuid id PK
        string queue_id UK
        uuid user_id FK
        string game_mode
        string queue_type
        int priority
        json preferences
        string status
        timestamp queued_at
        timestamp matched_at
        timestamp expires_at
    }
    
    MATCH_PARTICIPANTS {
        uuid id PK
        uuid match_id FK
        uuid user_id FK
        int team_id
        string role
        json match_data
        timestamp joined_at
    }
    
    MATCH_RESULTS {
        uuid id PK
        uuid match_id FK
        uuid user_id FK
        int team_id
        string result
        int rating_change
        int new_rating
        decimal performance_score
        json rewards
        timestamp created_at
    }
    
    TEAMS {
        uuid id PK
        string team_id UK
        string team_name
        string team_type
        uuid leader_id FK
        json members
        int team_rating
        int wins
        int losses
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    TEAM_MEMBERS {
        uuid id PK
        uuid team_id FK
        uuid user_id FK
        string role
        timestamp joined_at
        timestamp last_active
    }
```

## 🌍 World Schema

```mermaid
erDiagram
    WORLD_ZONES ||--o{ TERRITORIES : "contains"
    WORLD_ZONES ||--o{ SPAWN_POINTS : "has"
    WORLD_ZONES ||--o{ WORLD_EVENTS : "hosts"
    TERRITORIES ||--o{ GUILDS : "owned_by"
    
    WORLD_ZONES {
        uuid id PK
        string zone_id UK
        string name
        text description
        string zone_type
        int level_range_min
        int level_range_max
        json coordinates
        json spawn_points
        json safe_zones
        json pvp_zones
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    TERRITORIES {
        uuid id PK
        string territory_id UK
        string name
        string zone_id FK
        uuid owner_guild_id FK
        string territory_type
        json coordinates
        json resources
        json buildings
        json defenses
        int level
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    SPAWN_POINTS {
        uuid id PK
        string spawn_id UK
        string zone_id FK
        string spawn_type
        string entity_id
        json coordinates
        int spawn_radius
        int spawn_count
        int respawn_time
        decimal spawn_chance
        json level_range
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    WORLD_EVENTS {
        uuid id PK
        string event_id UK
        string event_type
        string title
        text description
        string zone_id FK
        timestamp start_time
        timestamp end_time
        int duration
        int max_participants
        int current_participants
        string status
        json rewards
        json requirements
        timestamp created_at
    }
    
    WEATHER_DATA {
        uuid id PK
        string zone_id FK
        string weather_type
        decimal intensity
        decimal temperature
        decimal humidity
        decimal wind_speed
        int wind_direction
        json effects
        timestamp timestamp
    }
```

## 📊 Analytics Schema

```mermaid
erDiagram
    EVENT_ANALYTICS ||--|| EVENTS : "tracks"
    USER_ANALYTICS ||--|| USERS : "tracks"
    GAME_ANALYTICS ||--|| ACTORS : "tracks"
    
    EVENT_ANALYTICS {
        uuid id PK
        string event_id FK
        string metric_name
        decimal metric_value
        string metric_type
        json tags
        timestamp timestamp
    }
    
    USER_ANALYTICS {
        uuid id PK
        uuid user_id FK
        string metric_name
        decimal metric_value
        string metric_type
        json tags
        timestamp timestamp
    }
    
    GAME_ANALYTICS {
        uuid id PK
        uuid actor_id FK
        string metric_name
        decimal metric_value
        string metric_type
        json tags
        timestamp timestamp
    }
    
    SESSION_DATA {
        uuid id PK
        uuid user_id FK
        string session_id
        timestamp start_time
        timestamp end_time
        int duration
        json actions
        json performance
        timestamp created_at
    }
    
    PERFORMANCE_METRICS {
        uuid id PK
        string service_name
        string metric_name
        decimal metric_value
        string metric_type
        json tags
        timestamp timestamp
    }
    
    BUSINESS_METRICS {
        uuid id PK
        string metric_name
        decimal metric_value
        string metric_type
        json dimensions
        timestamp timestamp
    }
```

## 🔐 Security Schema

```mermaid
erDiagram
    USERS ||--o{ USER_SESSIONS : "has"
    USERS ||--o{ AUDIT_LOGS : "generates"
    USERS ||--o{ SECURITY_EVENTS : "triggers"
    
    USER_SESSIONS {
        uuid id PK
        uuid user_id FK
        string session_token
        string ip_address
        string user_agent
        timestamp created_at
        timestamp expires_at
        timestamp last_activity
        boolean is_active
    }
    
    AUDIT_LOGS {
        uuid id PK
        uuid user_id FK
        string action
        string resource
        json details
        string ip_address
        string user_agent
        timestamp timestamp
    }
    
    SECURITY_EVENTS {
        uuid id PK
        uuid user_id FK
        string event_type
        string severity
        text description
        json details
        string ip_address
        boolean is_resolved
        timestamp created_at
        timestamp resolved_at
    }
    
    MODERATION_ACTIONS {
        uuid id PK
        string action_id UK
        uuid moderator_id FK
        uuid target_user_id FK
        string action_type
        text reason
        int duration
        string channel_id
        string message_id
        timestamp created_at
        timestamp expires_at
    }
    
    FRAUD_DETECTION {
        uuid id PK
        uuid user_id FK
        string fraud_type
        decimal risk_score
        json indicators
        boolean is_confirmed
        text resolution
        timestamp detected_at
        timestamp resolved_at
    }
```
