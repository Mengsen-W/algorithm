/*
 * @Date: 2023-08-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-06
 * @FilePath: /algorithm/golang/24_swap_pairs/swap_pairs.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	dummyHead := &ListNode{0, head}
	temp := dummyHead
	for temp.Next != nil && temp.Next.Next != nil {
		node1 := temp.Next
		node2 := temp.Next.Next
		temp.Next = node2
		node1.Next = node2.Next
		node2.Next = node1
		temp = node1
	}
	return dummyHead.Next
}

func main() {
	tests := []struct {
		head *ListNode
		ans  *ListNode
	}{
		{
			&ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, nil}}}},
			&ListNode{2, &ListNode{1, &ListNode{4, &ListNode{3, nil}}}},
		},
		{
			nil,
			nil,
		},
		{
			&ListNode{1, nil},
			&ListNode{1, nil},
		},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, swapPairs(item.head), item.ans)
	}
}
