#include <iostream>
#include <string_view>
#include <vector>
#include <fstream>
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <string_view>
#include <map>

int main()
{
  std::ifstream f("input");
  std::vector<int> l;
  std::map<int, int> r;
  std::string line;
  int total = 0;
  while (std::getline(f, line))
  {
    int a = std::stoi(line.substr(0, 5).c_str());
    l.push_back(a);

    int b = std::stoi(line.substr(8, 5).c_str());
    if (!r[b]) {
      r[b] = 0;
    }
    r[b] += 1;
  }
  for (int i = 0; i < l.size(); i++) {
    if (r[l.at(i)]) {
      total += r[l.at(i)] * l.at(i);
    }
  }
  std::cout << total;
  return 0;
}


