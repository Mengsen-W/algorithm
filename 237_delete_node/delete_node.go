/*
 * @Date: 2021-11-02 01:11:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-02 01:33:17
 */

package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteNode(node *ListNode) {
	node.Val = node.Next.Val
	node.Next = node.Next.Next
}

func main() {
	node := &ListNode{4, &ListNode{5, &ListNode{1, &ListNode{9, nil}}}}
	deleteNode(node)
	if node.Next.Next.Val != 9 {
		panic("error")
	}
}
