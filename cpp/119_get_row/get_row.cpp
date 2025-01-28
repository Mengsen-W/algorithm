#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> getRow(int rowIndex) {
    vector<int> row(rowIndex + 1);
    row[0] = 1;
    for (int i = 1; i <= rowIndex; ++i) {
      row[i] = 1LL * row[i - 1] * (rowIndex - i + 1) / i;
    }
    return row;
  }
};

int main() {
  vector<tuple<int, vector<int>>> tests{
      {3, {1, 3, 3, 1}},
      {0, {1}},
      {1, {1, 1}},
  };

  for (auto &[rowIndex, ans] : tests) {
    assert(Solution().getRow(rowIndex) == ans);
  }
}