// Package main ...
package main

func getCommon(nums1 []int, nums2 []int) int {
	i, j := 0, 0
	for i < len(nums1) && j < len(nums2) {
		if nums1[i] == nums2[j] {
			return nums1[i]
		}
		if nums1[i] < nums2[j] {
			i++
		} else {
			j++
		}
	}
	return -1
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{1, 2, 3}, []int{2, 4}, 2},
		{[]int{1, 2, 3, 6}, []int{2, 3, 4, 5}, 2},
	}

	for _, test := range tests {
		ans := getCommon(test.nums1, test.nums2)
		if ans != test.ans {
			panic("error")
		}
	}
}
