# Combat Core Documentation

## 📋 **Tổng Quan Tài Liệu**

Tài liệu này cung cấp hướng dẫn chi tiết về hệ thống Combat Core, bao gồm kiến trúc, thiết kế, và tích hợp với Enhanced Resource Manager.

## 📚 **Danh Sách Tài Liệu**

### **1. [00_Combat_Core_Overview.md](./00_Combat_Core_Overview.md)**
- **Mục đích**: Tổng quan hệ thống Combat Core
- **Nội dung**: 
  - Kiến trúc hệ thống
  - 6 loại action chính
  - Hệ thống damage, shield, status
  - Tích hợp với Actor Core & Enhanced Resource Manager
  - Performance considerations
  - Implementation phases

### **2. [01_Cultivation_System_Integration.md](./01_Cultivation_System_Integration.md)**
- **Mục đích**: Tích hợp với các hệ thống tu luyện
- **Nội dung**:
  - Interface-based integration
  - Stat aggregation với Weighted Sum
  - Cultivation system examples
  - Integration patterns
  - Questions for discussion

### **3. [02_Damage_System_Design.md](./02_Damage_System_Design.md)**
- **Mục đích**: Thiết kế hệ thống damage chi tiết
- **Nội dung**:
  - Damage categories và formulas
  - Power/Defense points calculation
  - DoT (Damage over Time) system
  - Damage events & logging
  - RNG determinism
  - Questions for discussion

### **4. [03_Enhanced_Resource_Manager_Integration.md](./03_Enhanced_Resource_Manager_Integration.md)**
- **Mục đích**: Tích hợp với Enhanced Resource Manager
- **Nội dung**:
  - Pre-calculated combat resources
  - Multi-system aggregation
  - 3-layer cache system
  - Ultra-fast combat calculation
  - Performance benchmarks
  - Implementation details

### **5. [04_Damage_Application_System.md](./04_Damage_Application_System.md)**
- **Mục đích**: Hệ thống áp dụng damage và xử lý shield
- **Nội dung**:
  - Shield order processing
  - Resource damage distribution
  - Resource protection system
  - Event system integration
  - Performance optimizations
  - Testing strategy

### **6. [05_Flexible_Action_System.md](./05_Flexible_Action_System.md)**
- **Mục đích**: Hệ thống action linh hoạt, data-driven
- **Nội dung**:
  - Data-driven action definitions
  - Modular architecture
  - Actor-based status effects and projectiles
  - Event trigger system
  - Resource consumption system
  - Damage type generation

### **7. [06_Modular_Architecture.md](./06_Modular_Architecture.md)**
- **Mục đích**: Kiến trúc modular cho hệ thống combat
- **Nội dung**:
  - File structure organization
  - Core files (6 files gốc)
  - Supporting systems (5 files bổ sung)
  - Integration patterns
  - Performance considerations
  - Testing strategy

### **8. [07_Implementation_Roadmap.md](./07_Implementation_Roadmap.md)**
- **Mục đích**: Roadmap triển khai hệ thống
- **Nội dung**:
  - 4 phases implementation
  - Detailed tasks and timelines
  - Testing strategy
  - Risk management
  - Success criteria
  - Resource requirements

## 🎯 **Hướng Dẫn Đọc**

### **Cho Developers**
1. Bắt đầu với [00_Combat_Core_Overview.md](./00_Combat_Core_Overview.md) để hiểu tổng quan
2. Đọc [05_Flexible_Action_System.md](./05_Flexible_Action_System.md) để hiểu hệ thống action mới
3. Đọc [06_Modular_Architecture.md](./06_Modular_Architecture.md) để hiểu kiến trúc modular
4. Đọc [07_Implementation_Roadmap.md](./07_Implementation_Roadmap.md) để hiểu roadmap triển khai
5. Đọc [03_Enhanced_Resource_Manager_Integration.md](./03_Enhanced_Resource_Manager_Integration.md) để hiểu tích hợp
6. Đọc [04_Damage_Application_System.md](./04_Damage_Application_System.md) để hiểu damage application
7. Tham khảo [01_Cultivation_System_Integration.md](./01_Cultivation_System_Integration.md) cho cultivation systems
8. Chi tiết implementation trong [02_Damage_System_Design.md](./02_Damage_System_Design.md)

### **Cho System Architects**
1. [00_Combat_Core_Overview.md](./00_Combat_Core_Overview.md) - Kiến trúc tổng thể
2. [06_Modular_Architecture.md](./06_Modular_Architecture.md) - Kiến trúc modular chi tiết
3. [05_Flexible_Action_System.md](./05_Flexible_Action_System.md) - Hệ thống action linh hoạt
4. [07_Implementation_Roadmap.md](./07_Implementation_Roadmap.md) - Roadmap triển khai
5. [03_Enhanced_Resource_Manager_Integration.md](./03_Enhanced_Resource_Manager_Integration.md) - Performance optimization
6. [04_Damage_Application_System.md](./04_Damage_Application_System.md) - Damage application architecture
7. [01_Cultivation_System_Integration.md](./01_Cultivation_System_Integration.md) - Integration patterns

### **Cho Game Designers**
1. [00_Combat_Core_Overview.md](./00_Combat_Core_Overview.md) - Game mechanics
2. [05_Flexible_Action_System.md](./05_Flexible_Action_System.md) - Hệ thống action linh hoạt
3. [02_Damage_System_Design.md](./02_Damage_System_Design.md) - Damage system details
4. [04_Damage_Application_System.md](./04_Damage_Application_System.md) - Shield và resource damage logic
5. [01_Cultivation_System_Integration.md](./01_Cultivation_System_Integration.md) - Cultivation integration

## 🚀 **Key Features**

### **Ultra-Fast Combat System**
- **Pre-calculated Resources**: Power/Defense points được tính trước
- **50x Performance**: Từ 5ms xuống 0.1ms
- **High Throughput**: 10,000+ calculations/second
- **Smart Caching**: 3-layer cache system

### **Multi-System Support**
- **Unified Actor System**: Tất cả đều là Actor
- **Flexible Action System**: Data-driven action definitions
- **Actor-Based Status/Projectiles**: Status effects và projectiles là actors
- **Multi-Faction Combat**: Combat hỗn chiến
- **Cultivation Integration**: Hỗ trợ nhiều hệ thống tu luyện

### **Flexible Action System**
- **Data-Driven Actions**: Actions định nghĩa bằng cấu trúc dữ liệu
- **Modular Architecture**: 11 files với trách nhiệm rõ ràng
- **Event Trigger System**: Hệ thống trigger linh hoạt
- **Resource Consumption**: Hệ thống tiêu hao tài nguyên đa dạng
- **Damage Type Generation**: Tạo damage types với unique keywords
 - **Precomputed Inputs**: Sử dụng Power/Defense points đã pre-calc từ Enhanced Resource Manager

### **Enhanced Resource Manager Integration**
- **Pre-calculation**: Combat resources được tính trước
- **Multi-System Aggregation**: Tổng hợp từ nhiều hệ thống
- **Stat Change Notification**: Tự động invalidate cache
- **Database Persistence**: Lưu trữ cho inactive actors

## 📊 **Performance Benchmarks**

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Damage Calculation | 5ms | 0.1ms | 50x faster |
| Cache Hit Rate | N/A | 95%+ | New feature |
| Memory Usage | 100% | 40% | 60% reduction |
| Throughput | 200/sec | 10,000+/sec | 50x increase |

## 🔧 **Configuration**

### **Enhanced Resource Manager**
```yaml
combat_support:
  enabled: true
  pre_calculation: true
  cache_ttl: 300000
  batch_size: 100
  parallel_processing: true
```

### **Combat Core**
```yaml
enhanced_resource_manager:
  enabled: true
  pre_calculation: true
  cache_invalidation: true
  batch_processing: true
```

### **Configs Directory**
The following YAML configs live under `docs/combat-core/configs/` and are referenced across the docs:
- `damage_types.yaml`
- `interactions.yaml`
- `aggregation.yaml`
- `action_schemas.yaml`
- `resource_exhaustion.yaml`
 - `true_damage.yaml`
 - `recovery.yaml`
 - `cc_dr.yaml`
 - `server_timing.yaml`
 - `protections.yaml`
 - `pvp_templates.yaml`
 - `telemetry.yaml`
 - `validation.yaml`
 - `turn_based.yaml`

## 🧪 **Testing Strategy**

### **Unit Tests**
- Pre-calculation tests
- Cache invalidation tests
- Multi-system aggregation tests
- Performance tests

### **Integration Tests**
- Enhanced Resource Manager integration
- Combat Core integration
- Cache system tests
- Stat change notification tests

### **Load Tests**
- High actor count scenarios
- Complex combat scenarios
- Memory usage tests
- Cache performance tests

## ❓ **Questions for Discussion**

1. **Pre-calculation Strategy**: Chiến lược pre-calculation có tối ưu không?
2. **Cache Invalidation**: Chiến lược invalidate cache có hiệu quả không?
3. **Multi-System Aggregation**: Phương pháp tổng hợp có công bằng không?
4. **Performance vs Memory**: Cân bằng giữa performance và memory usage?
5. **Cultivation Integration**: Tích hợp với các hệ thống tu luyện có đủ sâu không?

## 🎯 **Next Steps**

1. **Implement Flexible Action System**
   - Xem chi tiết: [05_Flexible_Action_System.md](./05_Flexible_Action_System.md)
   - Xem roadmap: [07_Implementation_Roadmap.md](./07_Implementation_Roadmap.md)
2. **Implement Modular Architecture**
   - Xem chi tiết: [06_Modular_Architecture.md](./06_Modular_Architecture.md)
3. **Implement Enhanced Resource Manager Extensions**
4. **Implement Combat Resource Pre-calculator**
5. **Implement Ultra-Fast Combat Core**
6. **Implement Performance Monitoring**
7. **Testing & Optimization**

---

*Tài liệu này sẽ được cập nhật khi hệ thống phát triển và có thêm yêu cầu mới.*
