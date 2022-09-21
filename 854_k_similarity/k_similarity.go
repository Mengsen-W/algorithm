/*
 * @Date: 2022-09-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-21
 * @FilePath: /algorithm/854_k_similarity/k_similarity.go
 */

package main

func kSimilarity(s1, s2 string) (step int) {
	type pair struct {
		s string
		i int
	}
	q := []pair{{s1, 0}}
	vis := map[string]bool{s1: true}
	for n := len(s1); ; step++ {
		tmp := q
		q = nil
		for _, p := range tmp {
			s, i := p.s, p.i
			if s == s2 {
				return
			}
			for i < n && s[i] == s2[i] {
				i++
			}
			t := []byte(s)
			for j := i + 1; j < n; j++ {
				if s[j] == s2[i] && s[j] != s2[j] {
					t[i], t[j] = t[j], t[i]
					if t := string(t); !vis[t] {
						vis[t] = true
						q = append(q, pair{t, i + 1})
					}
					t[i], t[j] = t[j], t[i]
				}
			}
		}
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s1 := "ab"
		s2 := "ba"
		ans := 1
		assert(kSimilarity(s1, s2) == ans)
	}

	{
		s1 := "abc"
		s2 := "bca"
		ans := 2
		assert(kSimilarity(s1, s2) == ans)
	}
}
