import time as t
import fileinput


while 1:
    f = open("test.txt", "a")
    f.write(input())
    fileinput.input()
    f.close()
    t.sleep(3)