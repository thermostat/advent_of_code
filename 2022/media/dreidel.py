
d1 = """
  T
-----
| ג |
\   /
  v
"""

d2 = """
  T
-----
| ש |
\   /
  v
"""


d3 = """
  T
-----
| ׆ |
\   /
  v
"""

d4 = """
  T
-----
| ה |
\   /
  v
"""

trans = """
  T
-----
| | |
\ | /
  v
"""

import os, time, random

drawings = [d1, d2, d3, d4]

def prepend_space(drawing, i):
    dlines = drawing.split('\n')
    return "\n".join([ (" "*i)+x for x in dlines])

def update_indent(indent):
    r = random.randint(0,5)
    if r == 0:
        indent = min(60, indent+1)
    elif r == 5:
        indent = max(0, indent-1)
    return indent

i = 0
indent = 4

try:
    while True:
        os.system('clear')
        i = (i+1) % 4
        indent = update_indent(indent)
        print(prepend_space(drawings[i], indent))
        time.sleep(.4)
        os.system('clear')
        indent = update_indent(indent)        
        print(prepend_space(trans, indent))
        time.sleep(.4)
except:
    pass
