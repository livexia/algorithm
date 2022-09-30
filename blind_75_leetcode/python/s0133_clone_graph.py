from typing import Dict
from copy import copy

# Definition for a Node.
class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: "Node") -> "Node":
        if node is None:
            return None
        visited = {}
        # return Solution.dfs(node, visited)
        return Solution.bfs(node)

    def dfs(node: "Node", visited: Dict[int, "Node"]) -> "Node":
        if visited.get(node.val):
            return visited[node.val]
        val = copy(node.val)
        new_node = Node(val, None)
        visited[val] = new_node
        if node.neighbors is not None:
            neighbors = [Solution.bfs(neighbor, visited) for neighbor in node.neighbors]
            new_node.neighbors = neighbors
        return new_node

    def bfs(node: "Node") -> "Node":
        from collections import deque

        visited = {}
        queue = deque([node])
        visited[node.val] = Node(node.val, None)
        while queue:
            n = queue.popleft()
            for neighbor in n.neighbors:
                if neighbor.val not in visited:
                    visited[neighbor.val] = Node(neighbor.val, None)
                    queue.append(neighbor)
                visited[n.val].neighbors.append(visited[neighbor.val])
        return visited[node.val]
