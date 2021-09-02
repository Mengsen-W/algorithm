/*
 * @Date: 2021-09-02 13:02:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-02 13:11:30
 */

package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getKthFromEnd(head *ListNode, k int) *ListNode {
	fast, slow := head, head
	for fast != nil && k > 0 {
		fast = fast.Next
		k--
	}
	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}
	return slow
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	root := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, nil}}}}}
	k := 2
	res := getKthFromEnd(root, k)
	assert(res.Val == 4)
	assert(res.Next.Val == 5)
}
