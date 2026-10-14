// Package main ...
package main

func maxSumOfSquares(num int, sum int) string {
	if sum > 9*num {
		return ""
	}
	r := sum % 9
	ans := make([]byte, 0, num)
	for i := 0; i < sum/9; i++ {
		ans = append(ans, '9')
	}
	if r > 0 {
		ans = append(ans, byte('0'+r))
	}
	zeros := num - len(ans)
	for i := 0; i < zeros; i++ {
		ans = append(ans, '0')
	}
	return string(ans)
}

func main() {
	tests := []struct {
		num int
		sum int
		ans string
	}{
		{2, 3, "30"},
		{2, 17, "98"},
		{1, 10, ""},
	}

	for _, test := range tests {
		if maxSumOfSquares(test.num, test.sum) != test.ans {
			panic("error")
		}
	}
}
