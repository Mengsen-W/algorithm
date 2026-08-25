// Package main ...
package main

func missingMultiple(nums []int, k int) int {
	seen := make(map[int]bool)
	for _, num := range nums {
		seen[num] = true
	}
	ans := k
	for seen[ans] {
		ans += k
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{8, 2, 3, 4, 6}, 2, 10},
		{[]int{1, 4, 7, 10, 15}, 5, 5},
	}

	for index, test := range tests {
		if ans := missingMultiple(test.nums, test.k); ans != test.ans {
			println(index, ans, test.ans)
		}
	}
}
