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

func mergeNodes(head *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	total := 0
	for cur := head.Next; cur != nil; cur = cur.Next {
		if cur.Val == 0 {
			node := &ListNode{Val: total}
			tail.Next = node
			tail = tail.Next
			total = 0
		} else {
			total += cur.Val
		}
	}
	return dummy.Next
}

func listToVec(head *ListNode) []int {
	step := head
	ans := []int{}
	for step != nil {
		ans = append(ans, step.Val)
		step = step.Next
	}
	return ans
}

func main() {
	tests := []struct {
		head *ListNode
		ans  *ListNode
	}{
		{
			&ListNode{0, &ListNode{3, &ListNode{1, &ListNode{0, &ListNode{4, &ListNode{5, &ListNode{2, &ListNode{0, nil}}}}}}}},
			&ListNode{4, &ListNode{11, nil}},
		},

		{
			&ListNode{0, &ListNode{1, &ListNode{0, &ListNode{3, &ListNode{0, &ListNode{2, &ListNode{2, &ListNode{0, nil}}}}}}}},
			&ListNode{1, &ListNode{3, &ListNode{4, nil}}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, listToVec(test.ans), listToVec(mergeNodes(test.head)), index)
	}
}
