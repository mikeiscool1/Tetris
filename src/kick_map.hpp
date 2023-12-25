// see: https://tetris.wiki/Super_Rotation_System

#pragma once
#include <map>
#include <array>

// first pair is based off rotation state, the second pair is based off the vector.
// this one is for all tetrominos besides O and I. O has none, but I has a special pair.
using std::make_pair;
std::map<std::pair<int, int>, std::array<std::pair<int, int>, 5>> kickMap = {
  { { 0, 1 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(-1, 1), make_pair(0, -2), make_pair(-1, -2) } },
  { { 1, 0 }, { make_pair(0, 0), make_pair(1, 0), make_pair(1, -1), make_pair(0, 2), make_pair(1, 2) } },
  { { 1, 2 }, { make_pair(0, 0), make_pair(1, 0), make_pair(-1, 1), make_pair(0, -2), make_pair(-1, -2) } },
  { { 2, 1 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(-1, 1), make_pair(0, -2), make_pair(-1, -2) } },
  { { 2, 3 }, { make_pair(0, 0), make_pair(1, 0), make_pair(1, 1), make_pair(0, -2), make_pair(1, -2) } },
  { { 3, 2 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(-1, -1), make_pair(0, 2), make_pair(-1, 2) } },
  { { 3, 0 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(-1, -1), make_pair(0, 2), make_pair(-1, 2) } },
  { { 0, 3 }, { make_pair(0, 0), make_pair(1, 0), make_pair(1, 1), make_pair(0, -2), make_pair(1, -2) } },
};

std::map<std::pair<int, int>, std::array<std::pair<int, int>, 5>> kickMapI = {
  { { 0, 1 }, { make_pair(0, 0), make_pair(-2, 0), make_pair(1, 0), make_pair(-2, -1), make_pair(1, 2) } },
  { { 1, 0 }, { make_pair(0, 0), make_pair(2, 0), make_pair(-1, 0), make_pair(2, 1), make_pair(-1, -2) } },
  { { 1, 2 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(2, 0), make_pair(-1, 2), make_pair(2, -1) } },
  { { 2, 1 }, { make_pair(0, 0), make_pair(1, 0), make_pair(-2, 0), make_pair(1, -2), make_pair(-2, 1) } },
  { { 2, 3 }, { make_pair(0, 0), make_pair(2, 0), make_pair(-1, 0), make_pair(2, 1), make_pair(-1, -2) } },
  { { 3, 2 }, { make_pair(0, 0), make_pair(-2, 0), make_pair(1, 0), make_pair(-2, -1), make_pair(1, 2) } },
  { { 3, 0 }, { make_pair(0, 0), make_pair(1, 0), make_pair(-2, 0), make_pair(1, -2), make_pair(-2, 1) } },
  { { 0, 3 }, { make_pair(0, 0), make_pair(-1, 0), make_pair(2, 0), make_pair(-1, 2), make_pair(2, -1) } },
};