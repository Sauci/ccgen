def st2a(spd, ti): 
    return ti * 360.0 * spd / 60.0

def sa2t(spd, ag):
    return ag / (360.0*spd/60.0)

print(st2a(3000, 170.6e-6))
print(sa2t(3000, 1.5))