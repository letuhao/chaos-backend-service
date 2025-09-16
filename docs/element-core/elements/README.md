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
- [Metal (Kim)](./metal_element.md) - Kim loại, sắc bén, cứng rắn
- [Wood (Mộc)](./wood_element.md) - Gỗ, sinh trưởng, mềm mại
- [Water (Thủy)](./water_element.md) - Nước, linh hoạt, thấm nhuần
- [Fire (Hỏa)](./fire_element.md) - Lửa, nóng bỏng, phá hủy
- [Earth (Thổ)](./earth_element.md) - Đất, ổn định, nuôi dưỡng

### **☯️ Âm Dương (Yin-Yang)**
- [Yin (Âm)](./yin_element.md) - Tối, lạnh, thụ động
- [Yang (Dương)](./yang_element.md) - Sáng, nóng, chủ động

### **🌟 Light & Dark**
- [Light](./light_element.md) - Ánh sáng, thánh thiện, chữa lành
- [Dark](./dark_element.md) - Bóng tối, tà ác, phá hủy

### **💚 Life & Death**
- [Life](./life_element.md) - Sự sống, hồi phục, tăng trưởng
- [Death](./death_element.md) - Cái chết, phá hủy, suy tàn

### **⏰ Time & Space**
- [Time](./time_element.md) - Thời gian, tốc độ, thời lượng
- [Space](./space_element.md) - Không gian, vị trí, khoảng cách

### **🧠 Mental & Psychic**
- [Mental](./mental_element.md) - Tâm trí, tấn công tinh thần
- [Psychic](./psychic_element.md) - Tâm linh, năng lượng tâm linh

### **🌌 Advanced Elements**
- [Void](./void_element.md) - Hư không, trống rỗng, hấp thụ
- [Chaos](./chaos_element.md) - Hỗn mang, ngẫu nhiên, không kiểm soát
- [Reality](./reality_element.md) - Thực tại, thực tế, hiện thực
- [Conceptual](./conceptual_element.md) - Khái niệm, trừu tượng

### **⚔️ Physical Elements**
- [Physical](./physical_element.md) - Vật lý, tấn công cơ bản
- [Slashing](./slashing_element.md) - Chém, kiếm, dao
- [Piercing](./piercing_element.md) - Đâm, thương, mũi tên
- [Blunt](./blunt_element.md) - Đập, búa, gậy
- [Crushing](./crushing_element.md) - Nghiền, sức mạnh lớn

### **🔮 Magical Elements**
- [Arcane](./arcane_element.md) - Huyền bí, ma thuật thuần túy
- [Mystical](./mystical_element.md) - Thần bí, ma thuật bí ẩn
- [Spiritual](./spiritual_element.md) - Tinh thần, năng lượng tinh thần

### **🌱 Cultivation Elements**
- [Qi](./qi_element.md) - Khí, năng lượng tu luyện cơ bản
- [Dao](./dao_element.md) - Đạo, con đường tu luyện
- [Profound](./profound_element.md) - Áo nghĩa, ý nghĩa sâu sắc
- [Karma](./karma_element.md) - Nghiệp, nghiệp lực
- [Fate](./fate_element.md) - Số mệnh, định mệnh

## 🔗 **Element Interaction System**

### **Tương Sinh Tương Khắc (Element Interactions)**
- [Element Interaction System Design](../10_Element_Interaction_System_Design.md) - Hệ thống tương sinh tương khắc với Elemental Mastery integration
- **Bảng Overview**: Chi tiết trigger probability và buff/debuff effects cho tất cả element combinations
- **Mastery-Based**: Trigger dựa trên mastery difference giữa attacker và defender
- **Strategic Depth**: Tạo ra meta game cân bằng và thú vị

### **Elemental Mastery Integration**
- [Elemental Mastery System Design](../08_Elemental_Mastery_System_Design.md) - Plugin-based cultivation system
- [Actor Core Integration Guide](../09_Actor_Core_Integration_Guide.md) - Hướng dẫn tích hợp vào Actor Core
- **Mastery Progression**: Experience-based progression với decay system
- **Plugin Architecture**: Mỗi element là một plugin riêng biệt

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

| Element | Category | Status Effect | Damage Type | Defense Type | Special | Mastery Integration |
|---------|----------|---------------|-------------|--------------|---------|-------------------|
| **Omni** | Universal | None | Universal | Balanced | Baseline | ✅ Yes |
| **Metal** | Ngũ Hành | Bleeding | Physical | High Defense | Penetration | ✅ Yes |
| **Wood** | Ngũ Hành | Poison | Nature | Medium Defense | Growth | ✅ Yes |
| **Water** | Ngũ Hành | Slow | Water | Medium Defense | Flexibility | ✅ Yes |
| **Fire** | Ngũ Hành | Burning | Fire | Low Defense | Destruction | ✅ Yes |
| **Earth** | Ngũ Hành | Petrification | Earth | High Defense | Stability | ✅ Yes |
| **Yin** | Âm Dương | Chill | Cold | Medium Defense | Passive | ✅ Yes |
| **Yang** | Âm Dương | Heat | Fire | Medium Defense | Active | ✅ Yes |
| **Light** | Light/Dark | Purification | Holy | High Defense | Healing | ✅ Yes |
| **Dark** | Light/Dark | Corruption | Shadow | Low Defense | Destruction | ✅ Yes |
| **Life** | Life/Death | Regeneration | Nature | Medium Defense | Healing | ✅ Yes |
| **Death** | Life/Death | Decay | Shadow | Low Defense | Destruction | ✅ Yes |
| **Time** | Time/Space | Temporal Distortion | Arcane | Medium Defense | Speed | ✅ Yes |
| **Space** | Time/Space | Spatial Lock | Arcane | High Defense | Control | ✅ Yes |
| **Mental** | Mental/Psychic | Confusion | Psychic | Low Defense | Mind Control | ✅ Yes |
| **Psychic** | Mental/Psychic | Telepathy | Psychic | Medium Defense | ESP | ✅ Yes |
| **Void** | Advanced | Absorption | Void | High Defense | Nullify | ✅ Yes |
| **Chaos** | Advanced | Random | Chaos | Low Defense | Unpredictable | ✅ Yes |
| **Reality** | Advanced | Manifestation | Reality | High Defense | Materialize | ✅ Yes |
| **Conceptual** | Advanced | Conceptual | Abstract | Medium Defense | Abstract | ✅ Yes |
| **Physical** | Physical | Stun | Physical | High Defense | Raw Power | ✅ Yes |
| **Slashing** | Physical | Bleeding | Physical | Medium Defense | Sharp | ✅ Yes |
| **Piercing** | Physical | Penetration | Physical | Low Defense | Precise | ✅ Yes |
| **Blunt** | Physical | Concussion | Physical | High Defense | Crushing | ✅ Yes |
| **Crushing** | Physical | Shatter | Physical | High Defense | Overwhelming | ✅ Yes |
| **Arcane** | Magical | Mana Burn | Arcane | Medium Defense | Pure Magic | ✅ Yes |
| **Mystical** | Magical | Mystification | Mystical | Medium Defense | Enigmatic | ✅ Yes |
| **Spiritual** | Magical | Soul Drain | Spiritual | Medium Defense | Ethereal | ✅ Yes |
| **Qi** | Cultivation | Qi Disruption | Qi | High Defense | Energy | ✅ Yes |
| **Dao** | Cultivation | Dao Insight | Dao | High Defense | Enlightenment | ✅ Yes |
| **Profound** | Cultivation | Profound Understanding | Profound | High Defense | Wisdom | ✅ Yes |
| **Karma** | Cultivation | Karmic Debt | Karma | Medium Defense | Fate | ✅ Yes |
| **Fate** | Cultivation | Destiny | Fate | High Defense | Predetermined | ✅ Yes |

## 🎯 **Next Steps**

1. **Create Individual Element Docs**: Tạo tài liệu chi tiết cho từng element với tên file chuẩn hóa
2. **Implement Element Interaction System**: Implement hệ thống tương sinh tương khắc
3. **Add Elemental Mastery Integration**: Tích hợp Elemental Mastery System cho tất cả elements
4. **Create Interaction Matrix**: Tạo ma trận tương tác chi tiết cho tất cả element combinations
5. **Add Game Examples**: Thêm ví dụ game cụ thể với mastery calculations
6. **Create Test Vectors**: Tạo test vectors cho testing với mastery scenarios
7. **Performance Optimization**: Tối ưu performance cho large-scale element interactions

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: In Progress  
**Maintainer**: Chaos World Team
