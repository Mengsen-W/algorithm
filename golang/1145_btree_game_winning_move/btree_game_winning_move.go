/*
 * @Date: 2023-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-03
 * @FilePath: /algorithm/golang/1145_btree_game_winning_move/btree_game_winning_move.go
 */

package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func btreeGameWinningMove(root *TreeNode, n int, x int) bool {
	var xNode *TreeNode

	var getSubtreeSize func(*TreeNode) int
	getSubtreeSize = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		if node.Val == x {
			xNode = node
		}
		return 1 + getSubtreeSize(node.Left) + getSubtreeSize(node.Right)
	}

	getSubtreeSize(root)
	leftSize := getSubtreeSize(xNode.Left)
	if leftSize >= (n+1)/2 {
		return true
	}
	rightSize := getSubtreeSize(xNode.Right)
	if rightSize >= (n+1)/2 {
		return true
	}
	remain := n - leftSize - rightSize - 1
	return remain >= (n+1)/2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{1, &TreeNode{2, &TreeNode{4, &TreeNode{8, nil, nil}, &TreeNode{9, nil, nil}}, &TreeNode{5, &TreeNode{10, nil, nil}, &TreeNode{11, nil, nil}}}, &TreeNode{3, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}}}
		n := 11
		x := 3
		ans := true
		assert(btreeGameWinningMove(root, n, x) == ans)
	}

	{
		root := &TreeNode{1, &TreeNode{2, nil, nil}, &TreeNode{3, nil, nil}}
		n := 3
		x := 1
		ans := false
		assert(btreeGameWinningMove(root, n, x) == ans)
	}
}
