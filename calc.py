def st2a(spd, ti): 
    return ti * 360.0 * spd / 60.0

def sa2t(spd, ag):
    return ag / (360.0*spd/60.0)

print(st2a(500, .2e-6))
print(sa2t(500, 1.5))

print(2890 + 1000 + 8000 + 1000 + 2000 + 1000 + 5000 + 1000 + 5000 + 1000 + 11000 + 1000 + 11000 + 1000 + 5000 + 1000 + 5000 + 1000 + 2000 + 1000 + 5110)