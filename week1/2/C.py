# https://www.acmicpc.net/problem/9012

import sys

input= sys.stdin.readline

T = int(input())

for i in range(T):
    stack= []
    
    brackets= input()
    for c in brackets:
        if c=='(':
            stack.append(c)
        elif c==')':
            if stack:
                stack.pop()
            else:
                print("NO")
                break
    else:
        if not stack:
            print("YES")
        else:
            print("NO")