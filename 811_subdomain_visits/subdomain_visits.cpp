/*
 * @Date: 2022-10-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-05
 * @FilePath: /algorithm/811_subdomain_visits/subdomain_visits.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> subdomainVisits(const vector<string> &cpdomains) {
    vector<string> ans;
    unordered_map<string, int> counts;
    for (auto &&cpdomain : cpdomains) {
      const int space = cpdomain.find(' ');
      const int count = stoi(cpdomain.substr(0, space));
      string domain = cpdomain.substr(space + 1);
      counts[domain] += count;
      for (int i = 0; i < domain.size(); i++) {
        if (domain[i] == '.') {
          const string subdomain = domain.substr(i + 1);
          counts[subdomain] += count;
        }
      }
    }
    ans.reserve(counts.size());
    for (auto &&[subdomain, count] : counts) {
      ans.emplace_back(to_string(count) + " " + subdomain);
    }
    return ans;
  }
};

int main() {
  {
    const vector<string> cpdomains{"9001 discuss.leetcode.com"};
    const vector<string> ans{"9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"};
    assert(Solution().subdomainVisits(cpdomains) == ans);
  }

  {
    const vector<string> cpdomains{"900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"};
    const vector<string> ans{"901 mail.com",     "50 yahoo.com", "900 google.mail.com", "5 wiki.org", "5 org",
                             "1 intel.mail.com", "951 com"};
    assert(Solution().subdomainVisits(cpdomains) == ans);
  }
}