#include <SDL2/SDL.h>
#include "game.hpp"
#include <iostream>

int main() {
  Game game;
  
  uint64_t frame_start;
  int frame_time;

  int output = game.init("Tetris", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, Window::WIDTH, Window::HEIGHT);
  if (output != 0) return 1;

  while (true) {
    frame_start = SDL_GetTicks64();

    game.handleEvents();
    if (!game.running()) break;

    game.update();
    game.render();

    frame_time = SDL_GetTicks64() - frame_start;

    if (FPS::FRAME_DELAY > frame_time) {
      SDL_Delay(FPS::FRAME_DELAY - frame_time);
    }
  }

  exit(0);
}