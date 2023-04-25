/*
 * @Date: 2023-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-25
 * @FilePath: /algorithm/cpp/2418_sort_people/sort_people.cpp
 */

#include <cassert>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
    int n = names.size();
    vector<pair<int, int>> arr;
    for (int i = 0; i < n; ++i) {
      arr.emplace_back(-heights[i], i);
    }
    sort(arr.begin(), arr.end());
    vector<string> ans;
    for (int i = 0; i < n; ++i) {
      ans.emplace_back(names[arr[i].second]);
    }
    return ans;
  }
};

int main() {
  {
    vector<string> names{"Mary", "John", "Emma"};
    vector<int> heights{180, 165, 170};
    vector<string> ans{"Mary", "Emma", "John"};
    assert(Solution().sortPeople(names, heights) == ans);
  }

  {
    vector<string> names{"Alice", "Bob", "Bob"};
    vector<int> heights{155, 185, 150};
    vector<string> ans{"Bob", "Alice", "Bob"};
    assert(Solution().sortPeople(names, heights) == ans);
  }
}