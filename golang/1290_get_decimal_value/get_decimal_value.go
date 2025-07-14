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

func getDecimalValue(head *ListNode) (ans int) {
	for head != nil {
		ans = ans*2 + head.Val
		head = head.Next
	}
	return
}

func main() {
	tests := []struct {
		list *ListNode
		ans  int
	}{
		{&ListNode{1, &ListNode{0, &ListNode{1, nil}}}, 5},
		{&ListNode{0, nil}, 0},
		{&ListNode{1, nil}, 1},
		{&ListNode{1, &ListNode{0, &ListNode{0, &ListNode{1, &ListNode{0, &ListNode{0, &ListNode{1, &ListNode{1, &ListNode{1, &ListNode{0, &ListNode{0, &ListNode{0, &ListNode{0, &ListNode{0, &ListNode{0, nil}}}}}}}}}}}}}}}, 18880},
		{&ListNode{0, &ListNode{0, nil}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, getDecimalValue(test.list), index)
	}
}
