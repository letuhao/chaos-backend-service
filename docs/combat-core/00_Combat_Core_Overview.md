# Combat Core System Overview

## 📋 **Tổng Quan Hệ Thống**

Combat Core là hệ thống chiến đấu linh hoạt và mở rộng được thiết kế cho các game online phức tạp với nhiều hệ thống tu luyện khác nhau. Hệ thống này được xây dựng trên nguyên tắc **"Tất cả đều là Actor"** và hỗ trợ combat đa mục tiêu với hệ thống faction linh hoạt.

## 🎯 **Nguyên Tắc Thiết Kế Chính**

### **1. Unified Actor System**
- **Tất cả đều là Actor**: Không phân biệt Player, Monster, NPC, Summon, Equipment
- **Xử lý thống nhất**: Tất cả actors đều được xử lý logic giống nhau
- **Linh hoạt cao**: Dễ dàng thêm mới loại actor từ các hệ thống tu luyện khác

### **2. Flexible Action System**
- **6 Loại Action Chính**: Tấn công, Phòng thủ, Di chuyển, Sử dụng vật phẩm, Kỹ năng phụ trợ, Triệu hồi
- **Multi-Category Support**: Mỗi action có thể thuộc nhiều category
- **Resource-Based**: Tất cả actions đều tốn tài nguyên (linh lực, mana, sinh mệnh, thọ nguyên, ...)

### **3. Multi-Faction Combat**
- **Không phân biệt địch/ta cố định**: Combat hỗn chiến giữa nhiều bên
- **Relationship System**: Quan hệ linh hoạt giữa các actors
- **AI Targeting**: Dựa trên sức chiến đấu, độ thù hận, mối quan hệ

## 🏗️ **Kiến Trúc Hệ Thống**

### **Core Components**

```
Combat Core System
├── Actor Management
│   ├── Unified Actor System
│   ├── Actor Stats Integration
│   └── Actor State Management
├── Action System
│   ├── Action Categories (6 loại)
│   ├── Resource Management
│   └── Action Validation
├── Damage System
│   ├── Power/Defense Points
│   ├── Damage Categories
│   └── Damage Calculation
├── Shield System
│   ├── Multi-Layer Shields
│   ├── Shield Stacking
│   └── Shield Penetration
├── Status System
│   ├── Buff/Debuff Management
│   ├── Status Categories
│   └── Status Interactions
├── Event System
│   ├── Event Handler
│   ├── Butterfly Effect
│   └── Event Queue
├── Passive System
│   ├── Triggered Passives
│   └── Continuous Passives
└── Multi-Target Combat
    ├── Faction System
    ├── Relationship Management
    └── AI Targeting
```

## ⚔️ **Hệ Thống Action (6 Loại)**

### **1. Tấn Công (Attack)**
- **Tấn công vật lý**: Luyện thể hệ thống, tốn thể lực, thần lực, sinh mệnh, thọ nguyên
- **Tấn công ma pháp**: Ma pháp hệ thống, tốn mana, sinh mệnh, thọ nguyên
- **Pháp thuật/Tiên thuật/Thần thông**: Luyện khí hệ thống, tốn linh lực, niệm lực, thần thức, tiên lực
- **Tùy chỉnh**: Các hệ thống tu luyện khác (Succubus, Mị Ma, ...)

### **2. Phòng Thủ (Defense)**
- **Kỹ năng phòng thủ**: Tương tự tấn công, tốn tài nguyên tương đương
- **Shield activation**: Kích hoạt shield, tốn tài nguyên
- **Dodge/Parry**: Né tránh, phản đòn

### **3. Di Chuyển/Bỏ Trốn (Movement)**
- **Hệ thống khoảng cách**: Các đòn tấn công cần trong phạm vi
- **Di chuyển thường**: Tốn thể lực
- **Kỹ năng di chuyển**: Ngũ hành độn thuật, huyệt độn thuật, tốn linh khí/mana/thọ nguyên

### **4. Sử Dụng Vật Phẩm (Item Usage)**
- **Vật phẩm tiêu hao**: Tốn vật phẩm
- **Pháp khí/Pháp bảo**: Tốn mana/thần thức/niệm lực/sinh mệnh/thọ nguyên
- **Thông thiên linh bảo/Tiên bảo/Thánh bảo/Để bảo**: Tùy theo hệ thống tu luyện

### **5. Kỹ Năng Phụ Trợ (Support)**
- **Heal/Buff/Debuff**: Hồi máu, tăng cường, giảm sát thương
- **Bày trận/Cấm chế**: Trận pháp, cấm chế
- **Tất cả đều tốn tài nguyên**

### **6. Triệu Hồi (Summon)**
- **Kỹ năng triệu hồi**: Tốn tài nguyên
- **Vật phẩm triệu hồi**: Tốn vật phẩm tiêu hao
- **Bổ sung actor vào combat**

## 💥 **Hệ Thống Tổn Thương**

### **Power/Defense Points**
```go
type DamageSystem struct {
    PowerPoints  map[string]float64  // Sức mạnh tấn công
    DefensePoints map[string]float64 // Sức mạnh phòng thủ
    Categories   map[string]DamageCategory
    Formula      DamageFormula
}
```

### **Damage Categories**
- **Flexible Categories**: Định nghĩa bởi combat-core, mở rộng được
- **Cultivation Integration**: Mỗi hệ thống tu luyện tự định nghĩa chi tiết
- **Scale by Primary Stats**: Tỷ lệ với primary stats của từng hệ thống

### **Damage Formula**
```
Final Damage = (Power Point - Defense Point) × Multipliers × Other Factors
```

## 🛡️ **Hệ Thống Shield**

### **Multi-Layer Shield Support**
- **Shield Types**: Định nghĩa bởi từng hệ thống tu luyện
- **Stacking Rules**: Có thể stack cùng loại hoặc khác loại
- **Resource Cost**: Kích hoạt shield tốn tài nguyên

### **Shield Mechanics**
- **Priority**: Shield được tính trước khi tính vào sinh mệnh/thọ nguyên
- **Penetration**: Một số kỹ năng có thể bỏ qua shield
- **Shield Breaking**: Kỹ năng chuyên biệt nhằm vào shield

## 🎭 **Hệ Thống Trạng Thái**

### **Status Categories**
- **Flexible Categories**: Định nghĩa bởi combat-core
- **Cultivation Specific**: Chi tiết do từng hệ thống tu luyện
- **Stacking Support**: Có thể stack cùng loại hoặc khác loại

### **Status Types**
- **Buffs**: Trạng thái tích cực
- **Debuffs**: Trạng thái tiêu cực
- **Neutral**: Trạng thái trung tính

## 📡 **Hệ Thống Event**

### **Event Types**
- **Attack Events**: Ai đánh ai, chuỗi phản ứng
- **Movement Events**: Di chuyển, chạy trốn
- **Summon Events**: Triệu hồi actor mới
- **Item Usage Events**: Sử dụng vật phẩm

### **Event Processing**
- **Event Queue**: Hàng đợi sự kiện
- **Butterfly Effect**: Chuỗi phản ứng từ sự kiện
- **World Integration**: Tích hợp với world system

## 🔄 **Hệ Thống Bị Động**

### **Triggered Passives**
- **Condition-Based**: Kích hoạt bởi điều kiện cụ thể
- **Event-Driven**: Kích hoạt bởi event handler
- **Examples**: Chết rồi dục hỏa trùng sinh, tấn công có xác suất kích hoạt hiệu ứng

### **Continuous Passives**
- **Enableable**: Actor phải kích hoạt
- **Resource Cost**: Tốn tài nguyên để duy trì
- **Examples**: Buff liên tục, aura effects

## 🎯 **Combat Đa Mục Tiêu**

### **Faction System**
- **No Fixed Sides**: Không có 2 phe cố định
- **Flexible Relationships**: Quan hệ linh hoạt giữa actors
- **Complex Scenarios**: 5 actors, 2 đồng minh, 2 kẻ thù, nhưng không chắc 2 đồng minh là kẻ thù của 2 kẻ thù

### **AI Targeting**
- **Weight-Based**: Dựa trên trọng số
- **Factors**: Sức chiến đấu, độ thù hận, mối quan hệ
- **Random Selection**: Chọn đối tượng tấn công ngẫu nhiên theo weight

## 🔗 **Tích Hợp Với Actor Core**

### **Stats Integration (Actor Core v3, Rust)**
- **Primary/Derived**: Combat đọc `Snapshot` từ Actor Core Aggregator (Rust); không lưu state trong Combat Core
- **Determinism**: Kết quả độc lập thứ tự input theo pipeline buckets và clamp precedence
- **Caps precedence**: EffectiveCaps → Combiner `clamp_default` → constants clamp ranges

```rust
use actor_core::{RegistryFactory, ServiceFactory, CacheFactory};
let plugin = RegistryFactory::create_plugin_registry();
let combiner = RegistryFactory::create_combiner_registry();
let cap_layers = RegistryFactory::create_cap_layer_registry();
let caps = ServiceFactory::create_caps_provider(cap_layers);
let cache = CacheFactory::create_in_memory_cache(10_000, 600);
let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);
let rt = tokio::runtime::Runtime::new().unwrap();
let actor = actor_core::types::Actor::new("Player".into(), "Human".into());
let snapshot = rt.block_on(aggregator.resolve(&actor)).unwrap();
let hp = snapshot.primary.get("hp_current").copied().unwrap_or(0.0);
```

### **Cultivation Systems**
- **Jindan System**: Tích hợp với luyện khí hệ thống
- **Other Systems**: Hỗ trợ các hệ thống tu luyện khác
- **Resource Management**: Quản lý tài nguyên đa dạng

## 📊 **Performance Considerations**

### **Optimization Strategies**
- **Object Pooling**: Tái sử dụng objects
- **Event Batching**: Xử lý events theo batch
- **Caching**: Cache calculations phức tạp
- **Memory Management**: Quản lý memory hiệu quả

### **Scalability**
- **Horizontal Scaling**: Hỗ trợ multiple combat instances
- **Load Balancing**: Phân tải combat load
- **Network Optimization**: Tối ưu network communication

## 🧪 **Testing Strategy**

### **Unit Tests**
- **Action System Tests**: Test từng loại action
- **Damage System Tests**: Test damage calculations
- **Status System Tests**: Test status effects
- **Event System Tests**: Test event handling

### **Integration Tests**
- **Actor Core Integration**: Test tích hợp với Actor Core
- **Multi-System Tests**: Test với nhiều hệ thống tu luyện
- **Performance Tests**: Test performance under load

### **Load Tests**
- **High Actor Count**: Test với nhiều actors
- **Complex Scenarios**: Test scenarios phức tạp
- **Memory Usage**: Test memory consumption

## 🚀 **Implementation Phases**

### **Phase 1: Core System**
1. **Actor Management**: Unified actor system
2. **Action System**: 6 loại action cơ bản
3. **Damage System**: Power/Defense points
4. **Event System**: Basic event handling

### **Phase 2: Advanced Features**
1. **Shield System**: Multi-layer shield support
2. **Status System**: Buff/Debuff management
3. **Passive System**: Triggered và continuous passives
4. **Multi-Target Combat**: Faction system

### **Phase 3: Polish & Optimization**
1. **Performance Optimization**: Caching, pooling
2. **Network Layer**: Client-server synchronization
3. **AI System**: Smart targeting
4. **Testing & Documentation**: Comprehensive test suite

## ❓ **Questions for Discussion**

1. **Action Complexity**: Độ phức tạp của action system có phù hợp không?
2. **Resource Management**: Hệ thống tài nguyên có đủ linh hoạt không?
3. **Faction System**: Hệ thống faction có phù hợp với game design không?
4. **Performance vs Flexibility**: Cân bằng giữa performance và flexibility như thế nào?
5. **Cultivation Integration**: Tích hợp với các hệ thống tu luyện có đủ sâu không?

## 🎯 **Next Steps**

1. **Detailed Design**: Thiết kế chi tiết từng component
2. **API Design**: Thiết kế interfaces và APIs
3. **Data Structures**: Định nghĩa data structures
4. **Implementation Plan**: Lập kế hoạch implement chi tiết

---

*Tài liệu này sẽ được cập nhật khi hệ thống phát triển và có thêm yêu cầu mới.*

## 🛠️ **Configuration Examples (Rust)**

- Thư mục cấu hình: `docs/combat-core/configs/`
- Env override: `COMBAT_CORE_CONFIG_DIR`
- Ví dụ `damage_types.yaml` (rút gọn):
```yaml
version: 1
categories:
  - id: physical
  - id: magical
  - id: elemental
  - id: true
interactions:
  - pair: [fire, ice]
    modifier: 1.10
```
- Ví dụ `multipliers.yaml` (rút gọn):
```yaml
version: 1
multipliers:
  - id: weapon_multiplier
    value: 1.20
  - id: strength_multiplier
    value: 1.10
```

## 🔧 **Production Readiness (Rust)**

- Readiness: `actor_core::production::check_readiness`
- Metrics: p95 resolve, crit rate, block rate; log or Prometheus
- RNG determinism: `StdRng` + seed cho golden vectors/tests
- Config hot-reload: versioned configs; reload-safe

## 🔒 **Determinism & RNG Policy**

- RNG seed per-hit/tick cho test tái lập
- Không phụ thuộc thứ tự systems; sắp xếp deterministic trước khi aggregate
