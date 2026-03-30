// Package main ...
package main

func checkStrings(s1 string, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}

	var counts [256]int

	for i := 0; i < len(s1); i++ {
		offset := (i & 1) << 7
		counts[offset+int(s1[i])]++
		counts[offset+int(s2[i])]--
	}

	for _, count := range counts {
		if count != 0 {
			return false
		}
	}

	return true
}

func main() {
	tests := []struct {
		s1  string
		s2  string
		ans bool
	}{
		{"abcdba", "cabdab", true},
		{"abe", "bea", false},
	}

	for _, test := range tests {
		if got := checkStrings(test.s1, test.s2); got != test.ans {
			panic(test)
		}
	}
}
