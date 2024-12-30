// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// ListNode Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// TreeNode Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSubPath(head *ListNode, root *TreeNode) bool {
	if root == nil {
		return false
	}
	return dfs(root, head) || isSubPath(head, root.Left) || isSubPath(head, root.Right)
}

func dfs(rt *TreeNode, head *ListNode) bool {
	// 链表已经全部匹配完，匹配成功
	if head == nil {
		return true
	}
	// 二叉树访问到了空节点，匹配失败
	if rt == nil {
		return false
	}
	// 当前匹配的二叉树上节点的值与链表节点的值不相等，匹配失败
	if rt.Val != head.Val {
		return false
	}
	return dfs(rt.Left, head.Next) || dfs(rt.Right, head.Next)
}

func main() {
	tests := []struct {
		head *ListNode
		root *TreeNode
		ans  bool
	}{
		{
			&ListNode{4, &ListNode{2, &ListNode{8, nil}}},
			&TreeNode{
				1, &TreeNode{4, nil, &TreeNode{2, &TreeNode{1, nil, nil}, nil}},
				&TreeNode{
					4, &TreeNode{2, &TreeNode{6, nil, nil}, &TreeNode{8, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}},
					nil,
				},
			},
			true,
		},
		{
			&ListNode{1, &ListNode{4, &ListNode{2, &ListNode{6, nil}}}},
			&TreeNode{
				1, &TreeNode{4, nil, &TreeNode{2, &TreeNode{1, nil, nil}, nil}},
				&TreeNode{
					4, &TreeNode{2, &TreeNode{6, nil, nil}, &TreeNode{8, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}},
					nil,
				},
			},
			true,
		},
		{
			&ListNode{1, &ListNode{4, &ListNode{2, &ListNode{6, &ListNode{8, nil}}}}},
			&TreeNode{
				1, &TreeNode{4, nil, &TreeNode{2, &TreeNode{1, nil, nil}, nil}},
				&TreeNode{
					4, &TreeNode{2, &TreeNode{6, nil, nil}, &TreeNode{8, &TreeNode{1, nil, nil}, &TreeNode{3, nil, nil}}},
					nil,
				},
			},
			false,
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, isSubPath(test.head, test.root), index)
	}
}
