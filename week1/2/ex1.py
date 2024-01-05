from collections import deque

n, m = map(int, input().split())
orders = list(map(int, input().split()))
deck = deque(range(1, n + 1))
ans = 0
for order in orders:
    moved_right = 0
    while deck[0] != order:
        deck.append(deck.popleft())
        moved_right += 1
    deck.popleft()
    moved_left = n - moved_right
    ans += min(moved_left, moved_right)
    n -= 1
print(ans)