/*
 * @Date: 2023-11-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-03
 * @FilePath: /algorithm/golang/117_connect/connect.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Node Definition for a Node.
type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func connect(root *Node) *Node {
	if root == nil {
		return nil
	}
	q := []*Node{root}
	for len(q) > 0 {
		tmp := q
		q = nil
		for i, node := range tmp {
			if i+1 < len(tmp) {
				node.Next = tmp[i+1]
			}
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}
	return root
}

func main() {
	root := &Node{
		Val: 1,
		Left: &Node{
			Val:   2,
			Left:  &Node{Val: 4},
			Right: &Node{Val: 5},
		},
		Right: &Node{
			Val:   3,
			Right: &Node{Val: 7},
		},
		Next: nil,
	}

	ans := connect(root)

	t := &testing.T{}
	assert.Equal(t, 3, ans.Left.Next.Val)
	assert.Equal(t, 5, ans.Left.Left.Next.Val)
}
