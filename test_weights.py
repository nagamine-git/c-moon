# 幾何平均の計算テスト

# ケース1: total_keystrokesとposition_costが優秀
tk1 = 0.98
pc1 = 0.98
sf1 = 0.85
rs1 = 0.85
alt1 = 0.75
hp1 = 0.65

product1 = (tk1**32.0) * (pc1**16.0) * (sf1**8.0) * (rs1**4.0) * (alt1**2.0) * (hp1**1.0)
total_weight = 32.0 + 16.0 + 8.0 + 4.0 + 2.0 + 1.0
core1 = (product1 ** (1.0 / total_weight)) * 100.0

print(f"ケース1 (tk=98%, pc=98%, sf=85%, rs=85%, alt=75%, hp=65%)")
print(f"  product = {product1:.6e}")
print(f"  core = {core1:.4f}")
print()

# ケース2: total_keystrokesとposition_costが低い
tk2 = 0.85
pc2 = 0.85
sf2 = 0.98
rs2 = 0.98
alt2 = 0.95
hp2 = 0.95

product2 = (tk2**32.0) * (pc2**16.0) * (sf2**8.0) * (rs2**4.0) * (alt2**2.0) * (hp2**1.0)
core2 = (product2 ** (1.0 / total_weight)) * 100.0

print(f"ケース2 (tk=85%, pc=85%, sf=98%, rs=98%, alt=95%, hp=95%)")
print(f"  product = {product2:.6e}")
print(f"  core = {core2:.4f}")
print()

print(f"差: {core1 - core2:.4f} (ケース1がケース2より{core1 - core2:.4f}点高い)")
print(f"ケース1の方が{((core1/core2 - 1) * 100):.2f}%高い")
