/*
 * @Date: 2021-06-04 19:17:13
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-06-04 20:11:13
 */

package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func getIntersectionNode_hashset(headA, headB *ListNode) *ListNode {
	vis := map[*ListNode]bool{}
	for tmp := headA; tmp != nil; tmp = tmp.Next {
		vis[tmp] = true
	}
	for tmp := headB; tmp != nil; tmp = tmp.Next {
		if vis[tmp] {
			return tmp
		}
	}
	return nil
}

func getIntersectionNode_doublepointer(headA, headB *ListNode) *ListNode {
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
			panic("Not Passed!")
		}
	}
	{
		intersection := &ListNode{8, &ListNode{4, &ListNode{5, nil}}}
		a := &ListNode{4, &ListNode{1, intersection}}
		b := &ListNode{5, &ListNode{0, &ListNode{1, intersection}}}
		assert(getIntersectionNode_doublepointer(a, b).Val == 8)
		assert(getIntersectionNode_hashset(a, b).Val == 8)
	}
	{
		intersection := &ListNode{2, &ListNode{4, nil}}
		a := &ListNode{0, &ListNode{9, &ListNode{1, intersection}}}
		b := &ListNode{3, intersection}
		assert(getIntersectionNode_doublepointer(a, b).Val == 2)
		assert(getIntersectionNode_hashset(a, b).Val == 2)
	}
	{
		a := &ListNode{2, &ListNode{4, &ListNode{4, nil}}}
		b := &ListNode{1, &ListNode{5, nil}}
		assert(getIntersectionNode_doublepointer(a, b) == nil)
		assert(getIntersectionNode_hashset(a, b) == nil)
	}
}
