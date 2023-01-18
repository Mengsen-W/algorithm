/*
 * @Date: 2021-07-21 18:18:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-21 19:47:34
 */

package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	if headA == nil || headB == nil {
		return nil
	}
	pa, pb := headA, headB
	for pa != pb {
		if pa == nil {
			pa = headB
		} else {
			pa = pa.Next
		}
		if pb == nil {
			pb = headA
		} else {
			pb = pb.Next
		}
	}
	return pa
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		intersection := &ListNode{8, &ListNode{4, &ListNode{5, nil}}}
		headA := &ListNode{4, &ListNode{1, intersection}}
		headB := &ListNode{5, &ListNode{0, &ListNode{1, intersection}}}
		assert(getIntersectionNode(headA, headB).Val == 8)
	}
	{
		intersection := &ListNode{2, &ListNode{4, nil}}
		headA := &ListNode{3, intersection}
		headB := &ListNode{0, &ListNode{9, &ListNode{1, intersection}}}
		assert(getIntersectionNode(headA, headB).Val == 2)
	}
	{
		headA := &ListNode{1, &ListNode{5, nil}}
		headB := &ListNode{2, &ListNode{6, &ListNode{4, nil}}}
		assert(getIntersectionNode(headA, headB) == nil)
	}
}
