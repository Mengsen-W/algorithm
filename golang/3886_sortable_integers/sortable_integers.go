// Package main ...
package main

import "sort"

func sortableIntegers(nums []int) int {
	n := len(nums)
	var fac []int

	for x := 1; x*x <= n; x++ {
		if n%x == 0 {
			fac = append(fac, x)
			if n/x != x {
				fac = append(fac, n/x)
			}
		}
	}

	target := make([]int, n)
	copy(target, nums)
	sort.Ints(target)

	// 检查能否通过对 nums1 进行若干次循环移动操作得到 nums2
	check := func(nums1, nums2 []int) bool {
		L := len(nums1)

		for i := 0; i < L-1; i++ {
			if nums1[i] > nums1[i+1] {
				rotated := make([]int, 0, L)
				rotated = append(rotated, nums1[i+1:]...)
				rotated = append(rotated, nums1[:i+1]...)

				if len(rotated) != len(nums2) {
					return false
				}
				for j := range rotated {
					if rotated[j] != nums2[j] {
						return false
					}
				}
				return true
			}
		}

		if len(nums1) != len(nums2) {
			return false
		}
		for i := range nums1 {
			if nums1[i] != nums2[i] {
				return false
			}
		}
		return true
	}

	ans := 0

	for _, d := range fac {
		allGood := true
		for left := 0; left < n; left += d {
			right := left + d - 1

			if !check(nums[left:right+1], target[left:right+1]) {
				allGood = false
				break
			}
		}
		if allGood {
			ans += d
		}
	}

	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 1, 2}, 3},
		{[]int{7, 6, 5}, 0},
		{[]int{5, 8}, 3},
	}

	for _, test := range tests {
		if sortableIntegers(test.nums) != test.ans {
			panic("wrong")
		}
	}
}
