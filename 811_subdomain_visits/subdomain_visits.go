/*
 * @Date: 2022-10-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-05
 * @FilePath: /algorithm/811_subdomain_visits/subdomain_visits.go
 */

package main

import (
	"reflect"
	"strconv"
	"strings"
)

func subdomainVisits(cpdomains []string) []string {
	cnt := map[string]int{}
	for _, s := range cpdomains {
		i := strings.IndexByte(s, ' ')
		c, _ := strconv.Atoi(s[:i])
		s = s[i+1:]
		cnt[s] += c
		for {
			i := strings.IndexByte(s, '.')
			if i < 0 {
				break
			}
			s = s[i+1:]
			cnt[s] += c
		}
	}
	ans := make([]string, 0, len(cnt))
	for s, c := range cnt {
		ans = append(ans, strconv.Itoa(c)+" "+s)
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		cpdomains := []string{"9001 discuss.leetcode.com"}
		ans := []string{"9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"}
		assert(subdomainVisits(cpdomains), ans)
	}

	{
		cpdomains := []string{"900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"}
		ans := []string{"901 mail.com", "50 yahoo.com", "900 google.mail.com", "5 wiki.org", "5 org",
			"1 intel.mail.com", "951 com"}
		assert(subdomainVisits(cpdomains), ans)
	}
}
