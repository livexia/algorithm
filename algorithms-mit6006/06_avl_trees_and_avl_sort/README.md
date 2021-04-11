## 5. Binary Search Trees, BST Sort

时间：20210410

记录：

1. height of a tree: length of longest path from root down to leaf
2. balanced: height = Θ(log2(n))
3. height of a node: length of longest path from it down to leaf
    - Max(height(left child), height(right child)) + 1
3. AVL Trees
    - require height of left and right children of every node differ by at most 1
    - h < 1.44*log2(n)
    - Rotation
        - Left-Rotate(x)
    - AVL insert
        - simple BST insert
        - fix AVL property
            - suppose x id the lowest node violating AVL
            - assume x.right higher
            - if x.right child is right-heavy or balanced
                - Left-Rotate(x) 
            - else if x'right child is left-heavy or balanced
                - Right-Rotate(x.right)
                - Left-Rotate(x)
    - AVL Sort
        - Insert n items -> Θ(nlogn)
        - in-order traversal -> Θ(n)
