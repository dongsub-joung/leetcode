import sys

input= sys.stdin.readline

dictionary= {}
answers= []

n,m =map(int, input().split())

for _ in range(n):
    site, pw= input().split(' ')
    dictionary[site]= pw
for _ in range(m):
    site= input()
    answers.append(dictionary.get(site))

for answer in answers:
    print(answer)