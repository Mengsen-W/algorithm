/*
 * @Date: 2022-06-04 09:06:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-04 09:13:24
 * @FilePath: /algorithm/929_num_unique_emails/num_unique_emails.go
 */

package main

import "strings"

func numUniqueEmails(emails []string) int {
	emailSet := map[string]struct{}{}
	for _, email := range emails {
		i := strings.IndexByte(email, '@')
		local := strings.SplitN(email[:i], "+", 2)[0] // 去掉本地名第一个加号之后的部分
		local = strings.ReplaceAll(local, ".", "")    // 去掉本地名中所有的句点
		emailSet[local+email[i:]] = struct{}{}
	}
	return len(emailSet)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(numUniqueEmails([]string{"test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"}) == 2)
	assert(numUniqueEmails([]string{"a@leetcode.com", "b@leetcode.com", "c@leetcode.com"}) == 3)
}
