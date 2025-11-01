// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// ListNode ...
type ListNode struct {
	Val  int
	Next *ListNode
}

func modifiedList(nums []int, head *ListNode) *ListNode {
	isExist := make(map[int]bool)
	for _, num := range nums {
		isExist[num] = true
	}

	sentry := &ListNode{Next: head}
	p := sentry
	for p.Next != nil {
		if isExist[p.Next.Val] {
			p.Next = p.Next.Next
		} else {
			p = p.Next
		}
	}
	return sentry.Next
}

func isSameList(l1, l2 *ListNode) bool {
	for l1 != nil && l2 != nil {
		if l1.Val != l2.Val {
			return false
		}
		l1 = l1.Next
		l2 = l2.Next
	}
	if l1 != nil || l2 != nil {
		return false
	}
	return true
}

func main() {
	tests := []struct {
		nums   []int
		head   *ListNode
		expect *ListNode
	}{
		{
			[]int{1, 2, 3},
			&ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, nil}}}}},
			&ListNode{4, &ListNode{5, nil}},
		},
		{
			[]int{1},
			&ListNode{1, &ListNode{2, &ListNode{1, &ListNode{2, &ListNode{1, &ListNode{2, nil}}}}}},
			&ListNode{2, &ListNode{2, &ListNode{2, nil}}},
		},
		{
			[]int{5},
			&ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, nil}}}},
			&ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, nil}}}},
		},
	}

	for index, test := range tests {
		assert.True(&testing.T{}, isSameList(test.expect, modifiedList(test.nums, test.head)), index)
	}
}
