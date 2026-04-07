#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Robot {
 private:
  bool moved = false;
  int idx = 0;
  vector<pair<int, int>> pos;
  vector<int> dir;
  unordered_map<int, string> to_dir = {{0, "East"}, {1, "North"}, {2, "West"}, {3, "South"}};

 public:
  Robot(int width, int height) {
    for (int i = 0; i < width; ++i) {
      pos.emplace_back(i, 0);
      dir.emplace_back(0);
    }
    for (int i = 1; i < height; ++i) {
      pos.emplace_back(width - 1, i);
      dir.emplace_back(1);
    }
    for (int i = width - 2; i >= 0; --i) {
      pos.emplace_back(i, height - 1);
      dir.emplace_back(2);
    }
    for (int i = height - 2; i > 0; --i) {
      pos.emplace_back(0, i);
      dir.emplace_back(3);
    }
    dir[0] = 3;
  }

  void step(int num) {
    moved = true;
    idx = (idx + num) % pos.size();
  }

  vector<int> getPos() { return {pos[idx].first, pos[idx].second}; }

  string getDir() {
    if (!moved) {
      return "East";
    }
    return to_dir[dir[idx]];
  }
};

int main() {
  Robot robot = Robot(6, 3);  // 初始化网格图，机器人在 (0, 0) ，朝东。
  robot.step(2);              // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
  robot.step(2);              // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
  robot.getPos();             // 返回 [4, 0]
  robot.getDir();             // 返回 "East"
  robot.step(2);              // 朝东移动 1 步到达 (5, 0) ，并朝东。
                              // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
                              // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
  robot.step(1);              // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
  robot.step(4);              // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
                              // 然后，移动 4 步到 (1, 2) ，并朝西。
  robot.getPos();             // 返回 [1, 2]
  robot.getDir();             // 返回 "West"
  return 0;
}
