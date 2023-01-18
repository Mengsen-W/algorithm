/*
 * @Date: 2023-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-12
 * @FilePath: /algorithm/1807_evaluate/evaluate.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string evaluate(string s, vector<vector<string>>& knowledge) {
    unordered_map<string, string> dict;
    for (auto& kd : knowledge) {
      dict[kd[0]] = kd[1];
    }
    bool addKey = false;
    string key, res;
    for (char c : s) {
      if (c == '(') {
        addKey = true;
      } else if (c == ')') {
        if (dict.count(key) > 0) {
          res += dict[key];
        } else {
          res.push_back('?');
        }
        addKey = false;
        key.clear();
      } else {
        if (addKey) {
          key.push_back(c);
        } else {
          res.push_back(c);
        }
      }
    }
    return res;
  }
};

int main() {
  {
    string s{"(name)is(age)yearsold"};
    vector<vector<string>> knowledge{{"name", "bob"}, {"age", "two"}};
    string ans{"bobistwoyearsold"};
    assert(Solution().evaluate(s, knowledge) == ans);
  }

  {
    string s{"hi(name)"};
    vector<vector<string>> knowledge{{"a", "b"}};
    string ans{"hi?"};
    assert(Solution().evaluate(s, knowledge) == ans);
  }

  {
    string s{"(a)(a)(a)aaa"};
    vector<vector<string>> knowledge{{"a","yes"}};
    string ans{"yesyesyesaaa"};
    assert(Solution().evaluate(s, knowledge) == ans);
  }
}
