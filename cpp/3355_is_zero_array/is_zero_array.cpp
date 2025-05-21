#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    bool isZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
        vector<int> deltaArray(nums.size() + 1, 0);
        for (const auto& query : queries) {
            int left = query[0];
            int right = query[1];
            deltaArray[left] += 1;
            deltaArray[right + 1] -= 1;
        }
        vector<int> operationCounts;
        int currentOperations = 0;
        for (int delta : deltaArray) {
            currentOperations += delta;
            operationCounts.push_back(currentOperations);
        }
        for (int i = 0; i < nums.size(); ++i) {
            if (operationCounts[i] < nums[i]) {
                return false;
            }
        }
        return true;
    }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, bool>> tests{
    {{1,0,1}, {{0,2}}, true},
    {{4,3,2,1}, {{1,3},{0,2}}, false},
  };

  for (auto &[nums, queries, ans] : tests) {
    assert(Solution().isZeroArray(nums, queries) == ans);
  }
}