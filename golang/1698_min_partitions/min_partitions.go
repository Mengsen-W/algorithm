// Package main ...
package main

func minPartitions(n string) int {
	res := 0
	for _, c := range n {
		res = max(res, int(c-'0'))
	}
	return res
}

func main() {
	tests := []struct {
		n   string
		ans int
	}{
		{"32", 3},
		{"82734", 8},
		{"27346209830709182346", 9},
	}

	for index, test := range tests {
		if ans := minPartitions(test.n); ans != test.ans {
			println(index, test.n, ans, test.ans)
		}
	}
}
