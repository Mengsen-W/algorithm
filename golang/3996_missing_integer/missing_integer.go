// Package main ...
package main

func missingInteger(nums []int) int {
	n := len(nums)
	numSet := make(map[int]bool, n)
	for _, num := range nums {
		numSet[num] = true
	}
	prefixLen := 1

	for i := 1; i < n; i++ {
		if nums[i] == nums[i-1]+1 {
			prefixLen += 1
		} else {
			break
		}
	}

	total := (nums[prefixLen-1] + nums[0]) * prefixLen / 2
	for numSet[total] {
		total += 1
	}

	return total
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 2, 5}, 6},
		{[]int{3, 4, 5, 1, 12, 14, 13}, 15},
	}

	for _, test := range tests {
		ans := missingInteger(test.nums)
		if ans != test.ans {
			panic("wrong answer")
		}
	}
}
