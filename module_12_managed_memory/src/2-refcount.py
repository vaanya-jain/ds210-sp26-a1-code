import sys

from helper import address


x = [1, 2, 3]
y = x
x.append(4)
y.append(5)
print(x)
print(y)







print(address(x))
print(address(y))











print(sys.getrefcount(x))

del y
print(sys.getrefcount(x))
