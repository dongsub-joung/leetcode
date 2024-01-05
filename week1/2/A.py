# https://leetcode.com/problems/implement-stack-using-queues/
# https://zhenyu0519.github.io/2020/07/03/lc255/#methodology

import collections

class MyStack:

    def __init__(self):
        self._queue= collections.deque()

    def push(self, x: int) -> None:
        self._queue.append(x)
        for _ in range(len(self._queue) -1):
            self._queue.append(self._queue.popleft())

    def pop(self) -> int:
            return self._queue.popleft()

    def top(self) -> int:
        return self._queue[0]

    def empty(self) -> bool:
        return len(self._queue) == 0