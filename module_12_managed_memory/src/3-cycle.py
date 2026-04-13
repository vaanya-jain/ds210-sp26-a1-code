from helper import address

import sys
import gc


x = [1, 2, 3]
x.append(x)
x.append(x)





print(address(x))
print(sys.getrefcount(x))














#del x

#garbage_items = gc.collect()
#print(garbage_items)
