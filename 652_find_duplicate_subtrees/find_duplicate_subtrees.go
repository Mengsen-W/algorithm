/*
 * @Date: 2022-09-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-05
 * @FilePath: /algorithm/652_find_duplicate_subtrees/find_duplicate_subtrees.go
 */

package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	type pair struct {
		node *TreeNode
		idx  int
	}
	repeat := map[*TreeNode]struct{}{}
	seen := map[[3]int]pair{}
	idx := 0
	var dfs func(*TreeNode) int
	dfs = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		tri := [3]int{node.Val, dfs(node.Left), dfs(node.Right)}
		if p, ok := seen[tri]; ok {
			repeat[p.node] = struct{}{}
			return p.idx
		}
		idx++
		seen[tri] = pair{node, idx}
		return idx
	}
	dfs(root)
	ans := make([]*TreeNode, 0, len(repeat))
	for node := range repeat {
		ans = append(ans, node)
	}
	return ans
}

func isSameTree(p, q *TreeNode) bool {
	if p == nil || q == nil {
		return p == q
	}

	if p.Val != q.Val {
		return false
	}

	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, nil, nil}, nil}, &TreeNode{3, &TreeNode{2, &TreeNode{4, nil, nil}, nil}, &TreeNode{4, nil, nil}}}
		ans := []*TreeNode{&TreeNode{2, &TreeNode{4, nil, nil}, nil}, &TreeNode{4, nil, nil}}
		output := findDuplicateSubtrees(root)
		for i := 0; i < len(ans); i++ {
			assert(isSameTree(ans[i], output[i]))
		}
	}

	{
		root := &TreeNode{2, &TreeNode{1, nil, nil}, &TreeNode{1, nil, nil}}
		ans := []*TreeNode{&TreeNode{1, nil, nil}}
		output := findDuplicateSubtrees(root)
		for i := 0; i < len(ans); i++ {
			assert(isSameTree(ans[i], output[i]))
		}
	}

	{
		root := &TreeNode{2, &TreeNode{2, &TreeNode{3, nil, nil}, nil}, &TreeNode{2, &TreeNode{3, nil, nil}, nil}}
		ans := []*TreeNode{&TreeNode{2, &TreeNode{3, nil, nil}, nil}, &TreeNode{3, nil, nil}}
		output := findDuplicateSubtrees(root)
		for i := 0; i < len(ans); i++ {
			assert(isSameTree(ans[i], output[i]))
		}
	}

}
