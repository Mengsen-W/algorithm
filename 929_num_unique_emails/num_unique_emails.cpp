/*
 * @Date: 2022-06-04 09:06:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-04 09:11:17
 * @FilePath: /algorithm/929_num_unique_emails/num_unique_emails.cpp
 */

#include <cassert>
#include <string>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int numUniqueEmails(vector<string> &emails) {
    unordered_set<string> emailSet;
    for (auto &email : emails) {
      string local;
      for (char c : email) {
        if (c == '+' || c == '@') {
          break;
        }
        if (c != '.') {
          local += c;
        }
      }
      emailSet.emplace(local + email.substr(email.find('@')));
    }
    return emailSet.size();
  }
};

int main() {
  {
    vector<string> emails{"test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com",
                          "testemail+david@lee.tcode.com"};
    assert(Solution().numUniqueEmails(emails) == 2);
  }

  {
    vector<string> emails{"a@leetcode.com", "b@leetcode.com", "c@leetcode.com"};
    assert(Solution().numUniqueEmails(emails) == 3);
  }
}