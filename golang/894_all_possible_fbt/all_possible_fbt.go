/*
 * @Date: 2024-04-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-02
 * @FilePath: /algorithm/golang/894_all_possible_fbt/all_possible_fbt.go
 */

// Package main ...
package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func allPossibleFBT(n int) []*TreeNode {
	if n%2 == 0 {
		return []*TreeNode{}
	}

	dp := make([][]*TreeNode, n+1)
	dp[1] = []*TreeNode{{Val: 0}}
	for i := 3; i <= n; i += 2 {
		for j := 1; j < i; j += 2 {
			for _, leftSubtree := range dp[j] {
				for _, rightSubtree := range dp[i-1-j] {
					root := &TreeNode{Val: 0, Left: leftSubtree, Right: rightSubtree}
					dp[i] = append(dp[i], root)
				}
			}
		}
	}
	return dp[n]
}
