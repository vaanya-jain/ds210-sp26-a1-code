from helper import address


v = [[1, 2], [3, 4]]
x = v[0]

print(address(v[0]))
print(address(x))






x.append(50)
print(v)



for i in range(5):
    v.append(i)

print(v)

print(x)
print(address(v[0]))
print(address(x))