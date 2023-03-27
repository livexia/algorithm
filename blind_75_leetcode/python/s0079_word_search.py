from typing import List


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m = len(board)
        n = len(board[0])
        visited = [[False for _ in range(n)] for _ in range(m)]

        def dfs(x: int, y: int, matched: int) -> bool:
            if board[x][y] != word[matched]:
                return False
            if matched == len(word) - 1:
                return True
            visited[x][y] = True
            resullt = (
                (x > 0 and not visited[x - 1][y] and dfs(x - 1, y, matched + 1))
                or (x < m - 1 and not visited[x + 1][y] and dfs(x + 1, y, matched + 1))
                or (y > 0 and not visited[x][y - 1] and dfs(x, y - 1, matched + 1))
                or (y < n - 1 and not visited[x][y + 1] and dfs(x, y + 1, matched + 1))
            )
            visited[x][y] = False
            return resullt

        for x in range(m):
            for y in range(n):
                if dfs(x, y, 0):
                    return True

        return False
