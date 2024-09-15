#pragma once
#include <tuple>

constexpr int TILE_SIZE = 40;
constexpr int COLUMNS = 10;
constexpr int ROWS = 20;
constexpr int TOTAL_TILE_COUNT = COLUMNS * ROWS;

class Color {
public:
  int r, g, b;

  inline constexpr bool operator==(const Color& compare) const {
    return r == compare.r && g == compare.g && b == compare.b;
  }

  inline constexpr bool operator!=(const Color& compare) const {
    return r != compare.r || g != compare.g || b != compare.b;
  }
};

namespace Colors {
  constexpr Color red = { 255, 0, 0 };
  constexpr Color orange = { 255, 127, 0 };
  constexpr Color yellow = { 255, 255, 0 };
  constexpr Color green = { 0, 255, 0 };
  constexpr Color lightBlue = { 0, 144, 255 };
  constexpr Color darkBlue = { 0, 97, 171 };
  constexpr Color violet = { 127, 0, 255 };
  constexpr Color shadow = { 50, 50, 50 };
  constexpr Color empty = { 30, 30, 30 };
  constexpr Color dead = { 50, 30, 30 };
}

namespace Window {
  constexpr int HEIGHT = ROWS * TILE_SIZE;
  constexpr int WIDTH = COLUMNS * TILE_SIZE;
}

namespace FPS {
  constexpr int FPS = 30;
  constexpr int FRAME_DELAY = 1000 / FPS;
}

namespace Assets {
  namespace Fonts {
    constexpr char const *FONT = "./assets/fonts/font.ttf";
    constexpr int SIZE = 200;
  }
}