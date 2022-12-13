from functools import cmp_to_key

def cmp(l1, l2):
    match l1, l2:
        case int(), int(): return l1-l2 #0 if equal, <0 if l1<l2
        case int(), list(): return cmp([l1], l2)
        case list(), int(): return cmp(l1, [l2])
        case list(), list():
            for l3 in map(cmp, l1, l2):
                if l3: return l3
            return cmp(len(l1), len(l2))

packets = []

for l in open("../input").read().split('\n\n'):
    p1, p2 = l.split()
    packets.append([eval(p1), eval(p2)])

print(sum([n+1 for n, pair in enumerate(packets) if cmp(*pair) < 0]))

packets2 = []
for p1, p2 in packets:
    packets2.append(p1)
    packets2.append(p2)
packets2.append([[2]])
packets2.append([[6]])
packets2 = sorted(packets2, key=cmp_to_key(cmp))
print( (packets2.index([[2]])+1) * (packets2.index([[6]]) +1))
