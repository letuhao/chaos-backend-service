# Hybrid Subsystem (Hệ Lai)

Mục tiêu: Hỗ trợ các nguyên tố lai (kết hợp hai hay nhiều yếu tố/khía cạnh), ví dụ: "Mộc Thần Lôi" (Lightning + Wood + Holy/Exorcism). Giữ nguyên tắc no hard caps + yin–yang counterbalance; dùng dynamics + refractory.

## Nguyên tắc
- Hybrid KHÔNG phải five_elements gốc; thuộc nhóm `hybrid`.
- Mỗi hybrid có:
  - parents: các element/aspect gốc (vd: lightning, wood, holy)
  - tags/aspects: holy, exorcism, etc.
  - core statuses và modifiers vs target tags (undead/ghost/corpse…)
  - pairs tối giản trong `interaction_config.yaml` (tránh nổ tổ hợp)
  - golden vectors riêng

## Cấu trúc đề xuất
- `hybrid/`
  - `README.md` (tài liệu này)
  - `configs/` (YAML từng hybrid)
  - `effects/` (ghi chú effect ids cần ở status_pool)
  - `golden_vectors/` (sanity data)

## Tích hợp
- Item/cultivation có thể “kích hoạt” hybrid cho kỹ năng/đòn đánh (on/off), không thay đổi element gốc.
- Engine route hiệu ứng sang hybrid khi thỏa điều kiện (item/talent).

## Next
- Thêm `lightning_divine_wood` (Mộc Thần Lôi): parents [lightning, wood], tags [holy, exorcism].
