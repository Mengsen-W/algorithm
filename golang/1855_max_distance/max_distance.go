// Package main ...
package main

func maxDistance(nums1 []int, nums2 []int) int {
	n1 := len(nums1)
	n2 := len(nums2)
	i := 0
	res := 0

	for j := 0; j < n2; j++ {
		for i < n1 && nums1[i] > nums2[j] {
			i++
		}
		if i < n1 {
			if j-i > res {
				res = j - i
			}
		}
	}

	return res
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{55, 30, 5, 4, 2}, []int{100, 20, 10, 10, 5}, 2},
		{[]int{2, 2, 2}, []int{10, 10, 1}, 1},
		{[]int{30, 29, 19, 5}, []int{25, 25, 25, 25, 25}, 2},
	}

	for _, test := range tests {
		ans := maxDistance(test.nums1, test.nums2)
		if ans != test.ans {
			panic("wrong")
		}
	}
}
