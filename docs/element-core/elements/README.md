# Element Types Detailed Documentation

## 📋 **Tổng Quan**

Thư mục này chứa tài liệu chi tiết cho từng loại element trong Element Core, bao gồm:

- **Element Properties**: Thuộc tính cơ bản của element
- **Status Effects**: Các trạng thái mà element có thể gây ra
- **Derived Stats**: Các derived stats mà element hỗ trợ
- **Interactions**: Tương tác với các element khác
- **Game Mechanics**: Cơ chế game cụ thể
- **Examples**: Ví dụ sử dụng trong game

## 📚 **Danh Sách Elements**

### **🌟 Universal Elements**
- [Omni](./omni_element.md) - Toàn năng, thuộc tính cơ bản cho tất cả nhân vật

### **🌿 Ngũ Hành (Five Elements)**
- [Kim (Metal)](./kim_metal.md) - Kim loại, sắc bén, cứng rắn
- [Mộc (Wood)](./moc_wood.md) - Gỗ, sinh trưởng, mềm mại
- [Thủy (Water)](./thuy_water.md) - Nước, linh hoạt, thấm nhuần
- [Fire (Hỏa)](./fire_element.md) - Lửa, nóng bỏng, phá hủy
- [Thổ (Earth)](./tho_earth.md) - Đất, ổn định, nuôi dưỡng

### **☯️ Âm Dương (Yin-Yang)**
- [Âm (Yin)](./am_yin.md) - Tối, lạnh, thụ động
- [Dương (Yang)](./duong_yang.md) - Sáng, nóng, chủ động

### **🌟 Light & Dark**
- [Light](./light.md) - Ánh sáng, thánh thiện, chữa lành
- [Dark](./dark.md) - Bóng tối, tà ác, phá hủy

### **💚 Life & Death**
- [Life](./life.md) - Sự sống, hồi phục, tăng trưởng
- [Death](./death.md) - Cái chết, phá hủy, suy tàn

### **⏰ Time & Space**
- [Time](./time.md) - Thời gian, tốc độ, thời lượng
- [Space](./space.md) - Không gian, vị trí, khoảng cách

### **🧠 Mental & Psychic**
- [Mental](./mental.md) - Tâm trí, tấn công tinh thần
- [Psychic](./psychic.md) - Tâm linh, năng lượng tâm linh

### **🌌 Advanced Elements**
- [Void](./void.md) - Hư không, trống rỗng, hấp thụ
- [Chaos](./chaos.md) - Hỗn mang, ngẫu nhiên, không kiểm soát
- [Reality](./reality.md) - Thực tại, thực tế, hiện thực
- [Conceptual](./conceptual.md) - Khái niệm, trừu tượng

### **⚔️ Physical Elements**
- [Physical](./physical.md) - Vật lý, tấn công cơ bản
- [Slashing](./slashing.md) - Chém, kiếm, dao
- [Piercing](./piercing.md) - Đâm, thương, mũi tên
- [Blunt](./blunt.md) - Đập, búa, gậy
- [Crushing](./crushing.md) - Nghiền, sức mạnh lớn

### **🔮 Magical Elements**
- [Arcane](./arcane.md) - Huyền bí, ma thuật thuần túy
- [Mystical](./mystical.md) - Thần bí, ma thuật bí ẩn
- [Spiritual](./spiritual.md) - Tinh thần, năng lượng tinh thần

### **🌱 Cultivation Elements**
- [Qi](./qi.md) - Khí, năng lượng tu luyện cơ bản
- [Dao](./dao.md) - Đạo, con đường tu luyện
- [Profound](./profound.md) - Áo nghĩa, ý nghĩa sâu sắc
- [Karma](./karma.md) - Nghiệp, nghiệp lực
- [Fate](./fate.md) - Số mệnh, định mệnh

## 🎯 **Cấu Trúc Tài Liệu**

Mỗi file element sẽ chứa:

### **1. Element Overview**
- Tên và mô tả element
- Category và classification
- Visual representation
- Lore và background

### **2. Element Properties**
- Base stats và characteristics
- Scaling factors
- Special properties
- Unique mechanics

### **3. Status Effects**
- Status effects mà element có thể gây ra
- Cơ chế hoạt động của status effects
- Duration và intensity calculations
- Stacking rules

### **4. Derived Stats**
- Các derived stats mà element hỗ trợ
- Stat weights và priorities
- Scaling formulas
- Cap values

### **5. Element Interactions**
- Tương sinh (generating) relationships
- Tương khắc (overcoming) relationships
- Special interactions
- Damage multipliers

### **6. Game Mechanics**
- Combat applications
- Shield interactions
- Item attribute effects
- Race talent bonuses

### **7. Configuration Examples**
- YAML configuration examples
- JSON schema definitions
- Usage examples
- Best practices

### **8. Testing & Validation**
- Unit test examples
- Integration test scenarios
- Performance benchmarks
- Balance considerations

## 🚀 **Usage Guidelines**

### **For Developers**
1. **Read Element Overview**: Hiểu element cơ bản
2. **Study Properties**: Nắm vững thuộc tính
3. **Understand Status Effects**: Hiểu cơ chế status
4. **Review Interactions**: Nắm vững tương tác
5. **Check Examples**: Xem ví dụ sử dụng

### **For Game Designers**
1. **Element Balance**: Cân bằng giữa các elements
2. **Status Effects**: Thiết kế status effects hấp dẫn
3. **Interactions**: Tạo tương tác thú vị
4. **Progression**: Thiết kế progression system
5. **Player Experience**: Tối ưu trải nghiệm người chơi

### **For System Architects**
1. **Architecture**: Hiểu kiến trúc element system
2. **Performance**: Tối ưu performance
3. **Scalability**: Đảm bảo khả năng mở rộng
4. **Integration**: Tích hợp với các systems khác
5. **Maintenance**: Dễ dàng bảo trì và cập nhật

## 📊 **Element Summary Matrix**

| Element | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----------|---------------|-------------|--------------|---------|
| Omni | Universal | None | Universal | Balanced | Baseline |
| Kim | Ngũ Hành | Bleeding | Physical | High Defense | Penetration |
| Mộc | Ngũ Hành | Poison | Nature | Medium Defense | Growth |
| Thủy | Ngũ Hành | Slow | Water | Medium Defense | Flexibility |
| Hỏa | Ngũ Hành | Burning | Fire | Low Defense | Destruction |
| Thổ | Ngũ Hành | Petrification | Earth | High Defense | Stability |
| Light | Light/Dark | Purification | Holy | High Defense | Healing |
| Dark | Light/Dark | Corruption | Shadow | Low Defense | Destruction |
| Life | Life/Death | Regeneration | Nature | Medium Defense | Healing |
| Death | Life/Death | Decay | Shadow | Low Defense | Destruction |
| Time | Time/Space | Temporal Distortion | Arcane | Medium Defense | Speed |
| Space | Time/Space | Spatial Lock | Arcane | High Defense | Control |

## 🎯 **Next Steps**

1. **Create Individual Element Docs**: Tạo tài liệu chi tiết cho từng element
2. **Add Status Effect Details**: Thêm chi tiết status effects
3. **Create Interaction Matrix**: Tạo ma trận tương tác
4. **Add Game Examples**: Thêm ví dụ game cụ thể
5. **Create Test Vectors**: Tạo test vectors cho testing

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: In Progress  
**Maintainer**: Chaos World Team
