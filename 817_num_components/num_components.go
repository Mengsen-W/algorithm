/*
 * @Date: 2022-10-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-12
 * @FilePath: /algorithm/817_num_components/num_components.go
 */

package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func numComponents(head *ListNode, nums []int) (ans int) {
	set := make(map[int]struct{}, len(nums))
	for _, v := range nums {
		set[v] = struct{}{}
	}
	for inSet := false; head != nil; head = head.Next {
		if _, ok := set[head.Val]; !ok {
			inSet = false
		} else if !inSet {
			inSet = true
			ans++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		header := &ListNode{0, &ListNode{1, &ListNode{2, &ListNode{3, nil}}}}
		nums := []int{0, 1, 3}
		ans := 2
		assert(numComponents(header, nums) == ans)
	}

	{
		header := &ListNode{0, &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, nil}}}}}
		nums := []int{0, 3, 1, 4}
		ans := 2
		assert(numComponents(header, nums) == ans)
	}
}
