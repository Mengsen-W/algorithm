/*
 * @Date: 2022-11-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-03
 * @FilePath: /algorithm/1668_max_repeating/max_repeating.go
 */

package main

func maxRepeating(sequence, word string) (ans int) {
	n, m := len(sequence), len(word)
	if n < m {
		return
	}
	fail := make([]int, m)
	for i := range fail {
		fail[i] = -1
	}
	for i := 1; i < m; i++ {
		j := fail[i-1]
		for j != -1 && word[j+1] != word[i] {
			j = fail[j]
		}
		if word[j+1] == word[i] {
			fail[i] = j + 1
		}
	}

	f := make([]int, n)
	j := -1
	for i := 0; i < n; i++ {
		for j != -1 && word[j+1] != sequence[i] {
			j = fail[j]
		}
		if word[j+1] == sequence[i] {
			j++
			if j == m-1 {
				if i < m {
					f[i] = 1
				} else {
					f[i] = f[i-m] + 1
				}
				if f[i] > ans {
					ans = f[i]
				}
				j = fail[j]
			}
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
	assert(maxRepeating("ababc", "ab") == 2)
	assert(maxRepeating("ababc", "bc") == 1)
	assert(maxRepeating("ababc", "ac") == 0)
}
