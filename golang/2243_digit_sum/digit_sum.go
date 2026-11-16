// Package main ...
package main

import "strconv"

func digitSum(str string, k int) string {
	for len(str) > k {
		tmp := "" // 每次操作结束的字符串
		n := len(str)
		for i := 0; i < n; i += k {
			val := 0
			end := i + k
			if end > n {
				end = n
			}
			for j := i; j < end; j++ {
				val += int(str[j] - '0')
			}
			tmp += strconv.Itoa(val)
		}
		str = tmp
	}
	return str
}

func main() {
	tests := []struct {
		str string
		k   int
		ans string
	}{
		{"11111222223", 3, "135"},
		{"00000000", 3, "000"},
	}

	for _, test := range tests {
		ans := digitSum(test.str, test.k)
		if ans != test.ans {
			panic("wrong")
		}
	}
}
