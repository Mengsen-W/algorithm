/*
 * @Date: 2023-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-03
 * @FilePath: /algorithm/cpp/1487_get_folder_names/get_folder_names.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string addSuffix(string name, int k) { return name + "(" + to_string(k) + ")"; }

  vector<string> getFolderNames(vector<string>& names) {
    unordered_map<string, int> index;
    vector<string> res;
    for (const auto& name : names) {
      if (!index.count(name)) {
        res.push_back(name);
        index[name] = 1;
      } else {
        int k = index[name];
        while (index.count(addSuffix(name, k))) {
          k++;
        }
        res.push_back(addSuffix(name, k));
        index[name] = k + 1;
        index[addSuffix(name, k)] = 1;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<string> names{"pes", "fifa", "gta", "pes(2019)"};
    vector<string> ans{"pes", "fifa", "gta", "pes(2019)"};
    assert(Solution().getFolderNames(names) == ans);
  }

  {
    vector<string> names{"gta", "gta(1)", "gta", "avalon"};
    vector<string> ans{"gta", "gta(1)", "gta(2)", "avalon"};
    assert(Solution().getFolderNames(names) == ans);
  }

  {
    vector<string> names{"onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece"};
    vector<string> ans{"onepiece", "onepiece(1)", "onepiece(2)", "onepiece(3)", "onepiece(4)"};
    assert(Solution().getFolderNames(names) == ans);
  }

  {
    vector<string> names{"wano", "wano", "wano", "wano"};
    vector<string> ans{"wano", "wano(1)", "wano(2)", "wano(3)"};
    assert(Solution().getFolderNames(names) == ans);
  }

  {
    vector<string> names{"kaido", "kaido(1)", "kaido", "kaido(1)"};
    vector<string> ans{"kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"};
    assert(Solution().getFolderNames(names) == ans);
  }
}
