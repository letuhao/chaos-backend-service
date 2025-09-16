# Ngũ Hành Diễn Sinh (Extended Elements) – Danh sách & tương tác ý tưởng

> Mục tiêu: Liệt kê các nguyên tố diễn sinh (ngoài 5 hệ gốc) và các tương tác/synergy/counter khả dĩ để thảo luận chi tiết sau. Giữ nguyên tắc no hard caps + yin–yang counterbalance; mọi hiệu ứng dùng dynamics + refractory; triển khai dùng `status_pool.yaml` + `interaction_config.yaml`.

## Nguyên tắc tổ chức
- five_elements: 5 hệ gốc (fire, water, wood, metal, earth)
- extended_elements (natural_phenomena/fusion): các diễn sinh dưới đây
- Mỗi diễn sinh có:
  - affinities (liên hệ tới 5 gốc, dùng làm bias, không phải cap)
  - core statuses (nền) + same/neutral by element
  - tương tác với 5 gốc (pairs) và một số tương tác chéo với diễn sinh khác

## Danh sách diễn sinh đề xuất (draft)

| ID | Tên | Affinities (fire/water/wood/metal/earth) | Core Status(es) | Vai trò | Ghi chú |
|---|---|---|---|---|---|
| wind | Phong | 0.15/0.10/0.50/0.20/0.05 | Shear (accuracy/defense↓), Turbulence (miss/cooldown di chuyển) | control/mobility | Displacement nhẹ (an toàn, có refractory) |
| lightning | Lôi | 0.40/0.10/0.30/0.20/0.00 | Shocked/Paralyze, Overcharge | burst/chain | Paralyze dùng refractory, không ICD cứng |
| ice | Băng | 0.10/0.70/0.00/0.20/0.00 | Chill/Freeze/Frostbite | slow/immobilize | Freeze theo ngưỡng intensity/stack |
| steam_smoke | Hơi nước/Khói | 0.40/0.50/0.10/0.00/0.00 | Obscured/Choked | zone/vision denial | Khuếch tán theo gió |
| sand_dust | Cát/Bụi | 0.00/0.00/0.00/0.10/0.90 | Blind-at-range, Mud Drag nhẹ | anti-range/terrain | Hình thành sandstorm với Wind |
| thorns_bramble | Gai/Dây leo | 0.00/0.00/0.70/0.00/0.30 | Bleed nhẹ, Entangle+ | attrition/anti-dash | Reflect mỏng, di chuyển gây bleed |
| spores_pollen | Bào tử/Phấn hoa | 0.00/0.10/0.80/0.00/0.10 | Haze (accuracy↓), Susceptibility↑ | debuff support | Khuếch tán tốt trong Smoke/Wind |
| crystal_geomancy | Tinh thể/Địa thuật | 0.00/0.00/0.00/0.50/0.50 | Crystal Guard (fortify), Shatter | fortify/reactive | Vỡ khi nhận hit lớn (nổ mảnh) |
| magnetism | Từ trường | 0.00/0.00/0.00/0.60/0.40 | Magnetized (pull/đạn lệch), Disrupt | control/disruption | Synergy Lightning; Earth neo |
| lava | Dung nham | 0.60/0.00/0.00/0.00/0.40 | Melt Armor (defense↓ theo thời gian), Magma Ground (DoT vùng) | siege/zone | Water/Ice giảm hiệu lực |
| boil_scald | Sôi/Phỏng | 0.40/0.60/0.00/0.00/0.00 | Scald (DoT + healing penalty), Steam Veil | hybrid DoT/control | Earth hút ẩm giảm hiệu lực |
| acid_corrosion | A-xít/Ăn mòn | 0.00/0.50/0.00/0.50/0.00 | Corroded (armor melt) | armor break | Earth trung hòa một phần |
| poison | Độc | 0.00/0.40/0.60/0.00/0.00 | Poisoned (DoT + heal penalty), Toxin Pressure | attrition | Light purify, Dark cộng hưởng |
| storm | Bão tố | 0.00/0.00/0.00/0.00/0.00 | Chain Shock + Turbulence | hybrid chain/control | Wind+Lightning hợp lực |
| sandstorm | Bão cát | 0.00/0.00/0.00/0.00/0.00 | Blind-at-range + Mud Drag | anti-range/zone | Wind+Sand hợp lực |

Gợi ý: các hệ “hợp lực” (storm, sandstorm) có thể là effects/patterns thay vì element độc lập.

## Tương tác với 5 gốc (ý tưởng)

- wind:
  - vs fire: quạt lửa (damage↑ nhưng self-risk↑) hoặc làm lệch đường bay dựa tình huống
  - vs water: tạo sóng/đẩy (displace nhẹ), tăng spread Soaked
  - vs wood: khuếch tán spores/pollen; thorns bị thổi lùi (reflect↓)
  - vs metal: làm lệch đạn, giảm parry nhỏ (conductivity tùy vào lightning)
  - vs earth: bị “grounding” giảm displacement; tạo dust nếu nền đất khô
- lightning:
  - vs water: Conduction (lan truyền), Shock↑
  - vs wood: Overload (cây dẫn điện) nhưng có self-short nếu intensity quá cao
  - vs metal: Overload/crit_damage↑, risk self-short
  - vs earth: Grounding giảm hiệu lực; với crystal có nguy cơ shatter
  - vs fire: Overheat (risk-reward)
- ice:
  - vs water: Freeze nhanh hơn (nhiệt độ↓)
  - vs fire: khắc chế (chill mạnh), nhưng fire có thể “defrost”
  - vs earth: làm cứng bùn (mire→freeze root)
  - vs metal: giòn hóa (brittle↑)
  - vs wood: làm nặng dây leo, di chuyển gây bleed mạnh hơn (thorns synergy)
- steam_smoke:
  - vs fire: bùng hơi (steam veil) – mù nhẹ
  - vs water: tăng mù/khuếch tán
  - vs wind: khuếch tán xa hơn
  - vs light: bị purify giảm hiệu lực
  - vs earth: hấp thụ/đọng giọt (giảm thời gian)
- sand_dust:
  - vs wind: thành sandstorm (blind mạnh tầm xa)
  - vs water: rửa trôi (hiệu lực↓)
  - vs ice: đóng băng hạt cát (tăng sát thương khi va chạm?)
  - vs fire: tạo khói bụi nóng (vision denial)
  - vs metal: mài mòn nhẹ (accuracy↓)
- thorns_bramble:
  - vs fire: cháy lan (risk) – tradeoff reflect/bleed
  - vs water: root dính hơn (soaked giúp bám)
  - vs metal: chống cận chiến tốt (bleed khi dash)
  - vs earth: bám nền tốt (entangle hiệu quả)
- spores_pollen:
  - vs wind/smoke: khuếch tán khắp vùng (haze↑)
  - vs light: bị purify giảm tác dụng
  - vs fire: cháy làm giảm haze nhưng gây burst nhỏ
- crystal_geomancy:
  - vs ice: shatter burst khi nhận damage lớn
  - vs acid: ăn mòn tinh thể (giảm guard)
  - vs lightning: gia tăng nguy cơ vỡ do dao động
- magnetism:
  - vs metal: control tốt (magnetized)
  - vs earth: neo giảm hiệu lực kéo
  - vs lightning: overload (paralyze↑ nhưng self-short risk)
- lava:
  - vs water/ice: nguội nhanh (intensity_gain↓)
  - vs earth: mở đường (magma ground)
  - vs metal: làm mềm (defense↓)
- boil_scald:
  - vs earth: hút ẩm giảm hiệu lực
  - vs wind: khuếch tán hơi nóng
- acid_corrosion:
  - vs metal: corrode mạnh
  - vs earth: trung hòa bớt
- poison:
  - vs light: purify giảm DoT
  - vs dark: cộng hưởng (drain↑)

## Tương tác chéo giữa diễn sinh (mở rộng)

- Wind ↔ Lightning: Storm Charge (chain↑), Thunderclap (displace trên shocked gây nổ nhỏ)
- Wind ↔ Ice: Whiteout/Blizzard (accuracy↓ mạnh + slow)
- Wind ↔ Steam/Smoke: Plume (khuếch tán khói/steam ra xa hơn)
- Wind ↔ Sand/Dust: Sandstorm (blind-at-range, mud drag nhẹ)
- Wind ↔ Spores/Pollen: Haze Bloom (haze↑, susceptibility↑)

- Lightning ↔ Water: Conduction (lan theo Soaked)
- Lightning ↔ Metal: Overload (crit_damage↑, risk self-short); Magnetized đạn lệch
- Lightning ↔ Earth/Crystal: Resonance (shatter risk), Grounding (giảm hiệu lực)
- Lightning ↔ Ice: Superconduct (defense↓ tạm thời)

- Ice ↔ Water: Quick Freeze (freeze ngưỡng thấp hơn)
- Ice ↔ Fire: Defrost contest (bên nào intensity cao hơn thắng theo thời điểm)
- Ice ↔ Crystal: Shatter burst (nổ mảnh khi nhận hit lớn)
- Ice ↔ Sand/Dust: Sleet/Sand-ice mix (va chạm đau hơn, accuracy↓)

- Steam/Smoke ↔ Fire: Steam Veil (mù nhẹ), Heat Mirage (ảo ảnh giảm accuracy)
- Steam/Smoke ↔ Light: Purify contest (giảm hiệu lực khói/steam)
- Steam/Smoke ↔ Spores: Toxic Haze (vision↓, susceptibility↑)

- Sand/Dust ↔ Fire: Ash Veil (vision denial)
- Sand/Dust ↔ Water: Washout (hiệu lực↓)
- Sand/Dust ↔ Magnetism: Iron Sand (điều hướng hạt kim loại mịn)

- Thorns/Bramble ↔ Ice: Blood-on-move (di chuyển gây bleed mạnh)
- Thorns/Bramble ↔ Lava/Fire: Burn Bleed (attrition↑ nhưng self-risk)

- Spores/Pollen ↔ Light: Purify giảm haze/susceptibility
- Spores/Pollen ↔ Steam/Smoke: Haze mạnh (stack tốt)

- Crystal/Geomancy ↔ Acid/Corrosion: Crystal Melt (giảm guard)
- Crystal/Geomancy ↔ Lightning: Vibration crack (dễ vỡ)

- Magnetism ↔ Metal: Pull/đạn lệch, disrupt thiết bị
- Magnetism ↔ Lightning: Overload + Paralyze risk

- Lava ↔ Water: Quench (intensity_gain↓)
- Lava ↔ Metal: Heat Soften (defense↓)

- Boil/Scald ↔ Earth: Desiccate (giảm hiệu lực)
- Acid/Corrosion ↔ Metal: Corrode mạnh
- Poison ↔ Light: Purify giảm DoT; ↔ Dark: Drain↑

## Ứng viên status_pool (tên gợi ý)
- storm_charge, thunderclap, whiteout, superconduct, conduction, grounding, overload
- obscured, choked, sand_blind, mud_drag_plus, bramble_thorns, spore_haze
- crystal_guard, shatter_burst, magnetized, melt_armor, scald, corroded, poisoned

## Ghi chú triển khai
- Không cap cứng; dùng dynamics `{intensity_gain, intensity_damping, decay_rate, refractory_gain, refractory_decay}`
- Cross-element effects: định nghĩa tại `interaction_config.yaml` và tham chiếu `status_pool.yaml` bằng `pool_id`
- Element-owned: same/neutral by element trong `elements/configs/<id>_element.yaml`
- Thiết lập golden vectors cơ bản cho sanity ranges

## Next steps (đề xuất)
- Chốt n diễn sinh cần triển khai trước (6 hoặc 8)
- Tạo pool effects → pairs → YAML/MD → golden vectors cho từng element
