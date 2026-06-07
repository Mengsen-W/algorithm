// Package main ...
package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func createBinaryTree(descriptions [][]int) *TreeNode {
	isRoot := make(map[int]bool)     // 数值对应的节点是否为根节点的哈希表
	nodes := make(map[int]*TreeNode) // 数值与对应节点的哈希表

	for _, d := range descriptions {
		p := d[0]
		c := d[1]
		left := d[2] == 1

		if _, exists := isRoot[p]; !exists {
			isRoot[p] = true
		}
		isRoot[c] = false

		// 创建或更新节点
		if _, exists := nodes[p]; !exists {
			nodes[p] = &TreeNode{Val: p}
		}
		if _, exists := nodes[c]; !exists {
			nodes[c] = &TreeNode{Val: c}
		}

		if left {
			nodes[p].Left = nodes[c]
		} else {
			nodes[p].Right = nodes[c]
		}
	}

	// 寻找根节点
	root := -1
	for val, isRootNode := range isRoot {
		if isRootNode {
			root = val
			break
		}
	}

	return nodes[root]
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
		return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	tests := []struct {
		descriptions [][]int
		expected     *TreeNode
	}{
		{
			[][]int{{20, 15, 1}, {20, 17, 0}, {50, 20, 1}, {50, 80, 0}, {80, 19, 1}},
			&TreeNode{
				50, &TreeNode{20, &TreeNode{15, nil, nil}, &TreeNode{17, nil, nil}},
				&TreeNode{80, &TreeNode{19, nil, nil}, nil},
			},
		},
		{
			[][]int{{1, 2, 1}, {2, 3, 0}, {3, 4, 1}},
			&TreeNode{1, &TreeNode{2, nil, &TreeNode{3, &TreeNode{4, nil, nil}, nil}}, nil},
		},
	}

	for _, test := range tests {
		result := createBinaryTree(test.descriptions)
		if !isSameTree(result, test.expected) {
			panic("test failed")
		}
	}
}
