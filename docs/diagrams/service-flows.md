# Service Flow Diagrams

## 🎮 Player Action Flow - Fireball Spell

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant UserMgmt as User Management
    participant ChaosBackend as Chaos Backend
    participant World as World Service
    participant Inventory as Inventory Service
    participant Chat as Chat Service
    participant Notification as Notification Service
    participant Analytics as Analytics Service
    
    Note over Player, Analytics: Player casts Fireball spell
    
    Player->>Frontend: Click Fireball Button
    Frontend->>Gateway: POST /skills/use
    Note right of Gateway: {skill_id: "fireball", target: {x: 100, y: 200}}
    
    Gateway->>UserMgmt: Validate Token
    UserMgmt-->>Gateway: User Valid
    
    Gateway->>ChaosBackend: Use Skill Request
    Note right of ChaosBackend: Check cooldown, mana cost, range
    
    ChaosBackend->>Inventory: Check Mana
    Inventory-->>ChaosBackend: Mana Available: 50/100
    
    ChaosBackend->>World: Check Line of Sight
    World-->>ChaosBackend: Clear Path
    
    ChaosBackend->>Inventory: Consume Mana (30)
    Inventory-->>ChaosBackend: Mana Consumed
    
    ChaosBackend->>World: Apply Damage
    Note right of World: Calculate damage, apply to target
    
    World->>Chat: Broadcast Combat Message
    Chat->>Notification: Notify Nearby Players
    Notification-->>Chat: Notifications Sent
    Chat-->>World: Message Broadcasted
    
    World-->>ChaosBackend: Damage Applied
    ChaosBackend->>Analytics: Log Combat Action
    Analytics-->>ChaosBackend: Action Logged
    
    ChaosBackend-->>Gateway: Skill Executed
    Gateway-->>Frontend: Success Response
    Frontend-->>Player: Fireball Animation + Damage
```

## 🛒 Trading Flow

```mermaid
sequenceDiagram
    participant Player1
    participant Player2
    participant Frontend1
    participant Frontend2
    participant Gateway as API Gateway
    participant Inventory as Inventory Service
    participant Chat as Chat Service
    participant Notification as Notification Service
    participant Payment as Payment Service
    
    Note over Player1, Payment: Player-to-Player Trading
    
    Player1->>Frontend1: Initiate Trade with Player2
    Frontend1->>Gateway: POST /trades/create
    Note right of Gateway: {target_user: "player2", items: [sword], gold: 1000}
    
    Gateway->>Inventory: Create Trade
    Inventory->>Notification: Send Trade Invitation
    Notification-->>Inventory: Invitation Sent
    Inventory-->>Gateway: Trade Created
    
    Gateway-->>Frontend1: Trade Invitation Sent
    Frontend1-->>Player1: Waiting for Response
    
    Player2->>Frontend2: Receive Trade Notification
    Frontend2->>Gateway: GET /trades/pending
    Gateway-->>Frontend2: Pending Trades
    Frontend2-->>Player2: Show Trade Offer
    
    Player2->>Frontend2: Accept Trade + Add Items
    Frontend2->>Gateway: POST /trades/{id}/accept
    Note right of Gateway: {items: [shield], gold: 500}
    
    Gateway->>Inventory: Process Trade
    Inventory->>Inventory: Validate Items
    Inventory->>Inventory: Transfer Items
    Inventory->>Payment: Process Gold Transfer
    Payment-->>Inventory: Gold Transferred
    
    Inventory->>Chat: Broadcast Trade Completion
    Chat->>Notification: Notify Both Players
    Notification-->>Chat: Notifications Sent
    
    Inventory-->>Gateway: Trade Completed
    Gateway-->>Frontend1: Trade Success
    Gateway-->>Frontend2: Trade Success
    
    Frontend1-->>Player1: Trade Completed
    Frontend2-->>Player2: Trade Completed
```

## 👥 Guild Creation Flow

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant Guild as Guild Service
    participant Inventory as Inventory Service
    participant Chat as Chat Service
    participant Notification as Notification Service
    participant World as World Service
    
    Note over Player, World: Guild Creation Process
    
    Player->>Frontend: Create Guild
    Frontend->>Gateway: POST /guilds/create
    Note right of Gateway: {name: "Dragon Slayers", description: "Elite guild"}
    
    Gateway->>Guild: Create Guild Request
    Guild->>Guild: Validate Guild Name
    Guild->>Inventory: Check Creation Cost
    Inventory-->>Guild: Cost: 1000 gold
    
    Guild->>Inventory: Deduct Gold
    Inventory-->>Guild: Gold Deducted
    
    Guild->>Guild: Create Guild Record
    Guild->>Guild: Add Player as Master
    
    Guild->>Chat: Create Guild Channel
    Chat-->>Guild: Channel Created
    
    Guild->>World: Reserve Guild Territory
    World-->>Guild: Territory Reserved
    
    Guild->>Notification: Send Guild Created
    Notification-->>Guild: Notification Sent
    
    Guild-->>Gateway: Guild Created
    Gateway-->>Frontend: Guild Success
    Frontend-->>Player: Guild Created + Access Granted
```

## 🎯 Matchmaking Flow

```mermaid
sequenceDiagram
    participant Player1
    participant Player2
    participant Frontend1
    participant Frontend2
    participant Gateway as API Gateway
    participant Matchmaking as Matchmaking Service
    participant ChaosBackend as Chaos Backend
    participant Notification as Notification Service
    
    Note over Player1, Notification: PvP Matchmaking Process
    
    Player1->>Frontend1: Join PvP Queue
    Frontend1->>Gateway: POST /queues/join
    Note right of Gateway: {game_mode: "pvp_1v1", preferences: {...}}
    
    Gateway->>Matchmaking: Join Queue
    Matchmaking->>Matchmaking: Add to Queue
    Matchmaking-->>Gateway: Queue Position: 1
    Gateway-->>Frontend1: In Queue
    
    Player2->>Frontend2: Join PvP Queue
    Frontend2->>Gateway: POST /queues/join
    Gateway->>Matchmaking: Join Queue
    Matchmaking->>Matchmaking: Find Match
    Matchmaking->>Matchmaking: Calculate Skill Match
    
    Matchmaking->>ChaosBackend: Create Match Instance
    ChaosBackend-->>Matchmaking: Match Created
    
    Matchmaking->>Notification: Notify Both Players
    Notification-->>Matchmaking: Notifications Sent
    
    Matchmaking-->>Gateway: Match Found
    Gateway-->>Frontend1: Match Ready
    Gateway-->>Frontend2: Match Ready
    
    Frontend1-->>Player1: Match Found - Accept?
    Frontend2-->>Player2: Match Found - Accept?
    
    Player1->>Frontend1: Accept Match
    Player2->>Frontend2: Accept Match
    
    Frontend1->>Gateway: POST /matches/{id}/accept
    Frontend2->>Gateway: POST /matches/{id}/accept
    
    Gateway->>Matchmaking: Both Accepted
    Matchmaking->>ChaosBackend: Start Match
    ChaosBackend-->>Matchmaking: Match Started
    
    Matchmaking-->>Gateway: Match Started
    Gateway-->>Frontend1: Match Started
    Gateway-->>Frontend2: Match Started
    
    Frontend1-->>Player1: Enter Battle Arena
    Frontend2-->>Player2: Enter Battle Arena
```

## 📱 Notification Flow

```mermaid
sequenceDiagram
    participant System
    participant Notification as Notification Service
    participant Gateway as API Gateway
    participant Player
    participant Frontend
    participant PushService as Push Service
    participant EmailService as Email Service
    participant Chat as Chat Service
    
    Note over System, Chat: Multi-channel Notification System
    
    System->>Notification: Send Notification
    Note right of Notification: {user_id: "player123", type: "guild_invite", channels: ["push", "in_game"]}
    
    Notification->>Notification: Check User Preferences
    Notification->>Notification: Determine Channels
    
    par Push Notification
        Notification->>PushService: Send Push
        PushService->>Player: Mobile Push
    and In-Game Notification
        Notification->>Chat: Send In-Game
        Chat->>Frontend: WebSocket Message
        Frontend->>Player: In-Game Popup
    and Email Notification
        Notification->>EmailService: Send Email
        EmailService->>Player: Email Notification
    end
    
    Notification->>Notification: Log Delivery Status
    Notification-->>System: Notification Sent
```

## 🎉 Event Participation Flow

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant Event as Event Service
    participant ChaosBackend as Chaos Backend
    participant Inventory as Inventory Service
    participant Notification as Notification Service
    participant Analytics as Analytics Service
    
    Note over Player, Analytics: Event Participation Process
    
    Player->>Frontend: View Available Events
    Frontend->>Gateway: GET /events
    Gateway->>Event: Get Active Events
    Event-->>Gateway: Events List
    Gateway-->>Frontend: Events Data
    Frontend-->>Player: Show Events
    
    Player->>Frontend: Join Event
    Frontend->>Gateway: POST /events/{id}/join
    Gateway->>Event: Join Event Request
    Event->>Event: Check Requirements
    Event->>Event: Add Participant
    
    Event->>ChaosBackend: Apply Event Buffs
    ChaosBackend-->>Event: Buffs Applied
    
    Event->>Notification: Send Event Start
    Notification-->>Event: Notification Sent
    
    Event-->>Gateway: Joined Successfully
    Gateway-->>Frontend: Join Success
    Frontend-->>Player: Event Started
    
    Note over Player, Analytics: Event Progress
    Player->>Frontend: Complete Event Objective
    Frontend->>Gateway: POST /events/{id}/complete
    Gateway->>Event: Complete Objective
    Event->>Event: Update Progress
    Event->>Inventory: Grant Rewards
    Inventory-->>Event: Rewards Granted
    
    Event->>Analytics: Log Participation
    Analytics-->>Event: Data Logged
    
    Event-->>Gateway: Objective Completed
    Gateway-->>Frontend: Success
    Frontend-->>Player: Rewards Received
```

## 🔍 Anti-Cheat Detection Flow

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant ChaosBackend as Chaos Backend
    participant AntiCheat as Anti-Cheat Service
    participant Analytics as Analytics Service
    participant Notification as Notification Service
    participant Admin as Admin Panel
    
    Note over Player, Admin: Anti-Cheat Detection Process
    
    Player->>Frontend: Suspicious Action
    Frontend->>Gateway: Action Request
    Gateway->>ChaosBackend: Process Action
    ChaosBackend->>AntiCheat: Analyze Action
    Note right of AntiCheat: Check patterns, speed, consistency
    
    AntiCheat->>AntiCheat: ML Analysis
    AntiCheat->>AntiCheat: Behavioral Analysis
    AntiCheat->>Analytics: Get Historical Data
    Analytics-->>AntiCheat: Player History
    
    alt Suspicious Activity Detected
        AntiCheat->>AntiCheat: Calculate Risk Score
        AntiCheat->>Notification: Send Warning
        Notification-->>Player: Warning Message
        
        AntiCheat->>Admin: Flag for Review
        Admin->>AntiCheat: Review Decision
        
        alt Confirmed Cheating
            AntiCheat->>ChaosBackend: Apply Penalty
            ChaosBackend->>Player: Account Suspended
            AntiCheat->>Analytics: Log Violation
        else False Positive
            AntiCheat->>AntiCheat: Update ML Model
            AntiCheat->>Player: Apology Message
        end
    else Normal Activity
        AntiCheat-->>ChaosBackend: Action Approved
        ChaosBackend-->>Gateway: Action Processed
        Gateway-->>Frontend: Success
        Frontend-->>Player: Action Completed
    end
```

## 💰 Payment Processing Flow

```mermaid
sequenceDiagram
    participant Player
    participant Frontend
    participant Gateway as API Gateway
    participant Payment as Payment Service
    participant Inventory as Inventory Service
    participant Notification as Notification Service
    participant AntiCheat as Anti-Cheat Service
    participant External as External Payment
    
    Note over Player, External: Payment Processing Flow
    
    Player->>Frontend: Purchase Item
    Frontend->>Gateway: POST /payments/purchase
    Note right of Gateway: {item_id: "premium_sword", amount: 9.99, currency: "USD"}
    
    Gateway->>Payment: Process Payment
    Payment->>AntiCheat: Fraud Check
    AntiCheat-->>Payment: Risk Assessment: Low
    
    Payment->>External: Charge Card
    External-->>Payment: Payment Authorized
    
    Payment->>Payment: Create Transaction Record
    Payment->>Inventory: Grant Item
    Inventory-->>Payment: Item Granted
    
    Payment->>Notification: Send Receipt
    Notification-->>Player: Purchase Confirmation
    
    Payment-->>Gateway: Payment Success
    Gateway-->>Frontend: Purchase Complete
    Frontend-->>Player: Item Received
    
    Note over Payment, External: Refund Process (if needed)
    Player->>Frontend: Request Refund
    Frontend->>Gateway: POST /payments/refund
    Gateway->>Payment: Process Refund
    Payment->>External: Refund Card
    External-->>Payment: Refund Processed
    Payment->>Inventory: Remove Item
    Payment-->>Gateway: Refund Complete
    Gateway-->>Frontend: Refund Success
    Frontend-->>Player: Refund Processed
```
