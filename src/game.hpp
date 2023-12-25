#pragma once

#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>
#include "constants.hpp"
#include <array>
#include <vector>

enum Screen {
  AWAIT_BEGIN,
  PLAYING
};

enum BlockType {
  None, I, O, T, L, J, S, Z
};

struct Coords {
  int x, y;

  // move functions automatically convert tile coords to window coords.
  inline Coords move(int x, int y) const {
    return { this->x + x * TILE_SIZE, this->y - y * TILE_SIZE };
  }

  inline Coords moveX(int x) const {
    return { this->x + x * TILE_SIZE, this->y };
  }

  inline Coords moveY(int y) const {
    return { this->x, this->y - y * TILE_SIZE };
  }
};

struct Block {
  std::array<Coords, 4> structure;
  Color color;
  BlockType type;

  // amount of times rotated clockwise
  int rotationState = 0;
};

class Game {
public:
  Game();
  ~Game();

  int init(const char* title, int x, int y, int w, int h);
  void handleEvents();
  void clean();

  void update();
  void render();

  void renderBackground();
  void renderBlocks();
  void renderShadow();
  void renderScore();
  
  bool blockCanDrop();
  bool moveDown();
  // 1 = right, -1 = left
  bool moveHorizontal(int direction);
  // 1 = clockwise, -1 = counterclockwise
  bool rotate(int direction);

  Coords endLocation();

  bool place();

  void spawnBlock(BlockType type = BlockType::None);

  inline bool running() const { return isRunning; };
  inline bool getScreen() const { return screen; }
private:
  bool isRunning;
  Screen screen;
  SDL_Window *window;
  SDL_Renderer *renderer;
  TTF_Font *font;

  Block activeBlock;
  std::array<Color, TOTAL_TILE_COUNT> tileColors;
  int level;
  int score;
  int framesForGravity;
  int linesCleared;

  int frameCount;
  int furthestDown;
  bool timeReset;
  int timeResets;
  uint64_t timer;
  inline void resetTimers() {
    furthestDown = 0;
    timeReset = false;
    timeResets = 0;
    timer = ~0;
  };

  BlockType hold;
  bool holdLocked;
};