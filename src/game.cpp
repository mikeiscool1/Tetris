#include "game.hpp"
#include "kick_map.hpp"
#include <iostream>

Game::Game(): 
  isRunning(false),
  screen(Screen::PLAYING),
  level(1),
  score(0),
  framesForGravity(FPS::FPS),
  linesCleared(0),
  frameCount(0),
  furthestDown(0),
  timeReset(false),
  timeResets(0),
  timer(~0),
  hold(BlockType::None),
  holdLocked(false)
 {}

Game::~Game() { clean(); }

int Game::init(const char* title, int x, int y, int w, int h) {
  srand(time(0));
  rand();

  if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
    SDL_Log("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
    
    return 1;
  }

  window = SDL_CreateWindow(title, x, y, w, h, SDL_WINDOW_SHOWN);

  if (!window) {
    SDL_Log("Window creation failed! SDL_Error: %s\n", SDL_GetError());
    SDL_Quit();

    return 1;
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
  if (!renderer) {
    SDL_Log("Renderer creation failed! SDL_Error: %s\n", SDL_GetError());
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 1;
  }

  int ttf = TTF_Init();
  if (ttf != 0) {
    SDL_Log("Failed to initialize TrueType Format! SDL_Error: %s\n", SDL_GetError());
    SDL_DestroyWindow(window);
    SDL_DestroyRenderer(renderer);
    SDL_Quit();

    return 1;
  }

  font = TTF_OpenFont(Assets::Fonts::FONT, Assets::Fonts::SIZE);
  if (!font) {
    SDL_Log("Failed to open font! SDL_Error: %s\n", SDL_GetError());
    SDL_DestroyWindow(window);
    SDL_DestroyRenderer(renderer);
    SDL_Quit();

    return 1;
  }

  SDL_ShowCursor(SDL_DISABLE);

  for (Color &tileColor : tileColors)
    tileColor = Colors::empty;

  spawnBlock();

  isRunning = true;
  return 0;
}

inline constexpr Coords toWindowCoords(int x, int y) {
  return { x * TILE_SIZE, (Window::HEIGHT - y * TILE_SIZE) - TILE_SIZE };
}

inline constexpr Coords toTileCoords(int x, int y) {
  return { x / TILE_SIZE, (ROWS - 1) - y / TILE_SIZE };
}

void Game::update() {
  if (screen == Screen::AWAIT_BEGIN) return;

  frameCount++;

  if (!blockCanDrop() && SDL_GetTicks64() - timer >= 500) {
    if (timeReset) timeResets++;

    if (!timeReset || timeResets == 4) {
      if (!place()) {
        screen = Screen::AWAIT_BEGIN;
        return;
      };

      spawnBlock();
      resetTimers();
    } else timer = SDL_GetTicks64();

    timeReset = false;
  }
  
  if (frameCount == framesForGravity || framesForGravity == 0) {
    if (framesForGravity == 0 && level > 30) {
      for (int i = 0; i < level - 30; i++) moveDown();
    }

    else moveDown();

    if (activeBlock.structure.back().y > furthestDown) {
      furthestDown = activeBlock.structure.back().y;
      timer = SDL_GetTicks64();
    }

    frameCount = 0;
  }
}

void Game::render() {
  SDL_RenderClear(renderer);

  renderBackground();
  renderShadow();
  renderBlocks();
  renderScore();

  SDL_RenderPresent(renderer);
}

void Game::clean() {
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);
  TTF_Quit();
  SDL_Quit();
}

void Game::renderBackground() {
  SDL_SetRenderDrawColor(renderer, Colors::empty.r, Colors::empty.g, Colors::empty.b, 255);
  SDL_RenderClear(renderer);

  SDL_SetRenderDrawColor(renderer, 20, 20, 20, 255);

  // draw horizontal first
  for (int y = 0; y <= Window::HEIGHT; y += TILE_SIZE)
    SDL_RenderDrawLine(renderer, 0, y, Window::WIDTH, y);

  // then vertical
  for (int x = 0; x <= Window::WIDTH; x += TILE_SIZE)
    SDL_RenderDrawLine(renderer, x, 0, x, Window::HEIGHT);
}

void Game::renderBlocks() {
  // begin with set blocks
  for (int i = 0; i < TOTAL_TILE_COUNT; i++) {
    Coords tile = toWindowCoords(i % COLUMNS, i / COLUMNS);
    Color color = tileColors.flat_index(i);

    if (color == Colors::empty) continue;

    SDL_Rect coloredTile = { tile.x + 1, tile.y + 1, TILE_SIZE - 1, TILE_SIZE - 1 };
    SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, 255);
    SDL_RenderFillRect(renderer, &coloredTile);
  }

  // then the dropping block
  for (const Coords& coord : activeBlock.structure) {
    SDL_Rect coloredTile = { coord.x + 1, coord.y + 1, TILE_SIZE - 1, TILE_SIZE - 1 };
    SDL_SetRenderDrawColor(renderer, activeBlock.color.r, activeBlock.color.g, activeBlock.color.b, 255);
    SDL_RenderFillRect(renderer, &coloredTile);
  }
}

void Game::renderShadow() {
  Coords end = endLocation();
  Coords& ref = activeBlock.structure.back();

  for (const Coords& coord : activeBlock.structure) {
    Coords offset = { coord.x - ref.x, coord.y - ref.y };

    SDL_Rect coloredTile = { end.x + offset.x + 1, end.y + offset.y + 1, TILE_SIZE - 1, TILE_SIZE - 1 };
    SDL_SetRenderDrawColor(renderer, Colors::shadow.r, Colors::shadow.g, Colors::shadow.b, 255);
    SDL_RenderFillRect(renderer, &coloredTile);
  }
}

void Game::renderScore() {
  constexpr SDL_Color white = { 255, 255, 255, 100 };
  constexpr int pixelsPerChar = 10;

  static SDL_Texture* texture = nullptr;

  std::string textString = "Score: " + std::to_string(score) + " | Level: " + std::to_string(level);
  const char* text = textString.c_str();
  int text_size = strlen(text);

  SDL_Surface *surface = TTF_RenderText_Solid(font, text, white);
  if (texture) SDL_DestroyTexture(texture);
  texture = SDL_CreateTextureFromSurface(renderer, surface);
  SDL_FreeSurface(surface);

  SDL_Rect rect = { 
    5,
    TILE_SIZE / 4, 
    text_size * pixelsPerChar,
    TILE_SIZE / 2
    };

  SDL_RenderCopy(renderer, texture, NULL, &rect);
}

bool Game::blockCanDrop() {
  auto it = std::find_if(activeBlock.structure.begin(), activeBlock.structure.end(), [this](const Coords& coord) {
    if (coord.y + TILE_SIZE >= Window::HEIGHT) return true;
    if (coord.y + TILE_SIZE < 0) return false;

    Coords tileCoords = toTileCoords(coord.x, coord.y);

    if (tileColors[tileCoords.y - 1][tileCoords.x] != Colors::empty) return true;

    return false;
  });

  if (it != activeBlock.structure.end()) return false;
  else return true;
}

bool Game::moveDown() {
  if (!blockCanDrop()) return false;

  for (Coords& coord : activeBlock.structure)
    coord.y += TILE_SIZE;

  return true;
}

bool Game::moveHorizontal(int direction) {
  auto it = std::find_if(activeBlock.structure.begin(), activeBlock.structure.end(), [this, &direction](const Coords& coord) {
    int sum = coord.x + direction * TILE_SIZE;
    if (sum < 0 || sum >= Window::WIDTH) return true;

    if (coord.y < 0) return false;

    Coords tileCoords = toTileCoords(coord.x, coord.y);

    if (tileColors[tileCoords.y][tileCoords.x + direction] != Colors::empty) return true;

    return false;
  });

  if (it != activeBlock.structure.end()) return false;

  for (Coords& coord : activeBlock.structure)
    coord.x += direction * TILE_SIZE;

  timeReset = true;
  return true;
}

bool Game::rotate(int direction) {
  if (activeBlock.type == BlockType::O) return false;

  int newRotationState = activeBlock.rotationState + direction;
  if (activeBlock.rotationState == 3) newRotationState = 0;
  else if (activeBlock.rotationState == 0) newRotationState = 3;

  std::array<std::pair<int, int>, 5> testCases = 
  activeBlock.type == BlockType::I 
    ? kickMapI.find({ activeBlock.rotationState, newRotationState })->second
    : kickMap.find({ activeBlock.rotationState, newRotationState })->second;

  Coords point = activeBlock.structure.back();

  static std::array<Coords, 4> tempStructure;
  
  for (std::pair<int, int> vector : testCases) {
    for (int i = 0; i < activeBlock.structure.size(); i++) {
      const Coords& coord = activeBlock.structure[i];

      Coords relativePos = { coord.x - point.x, coord.y - point.y };
      Coords newRelativePos = { -direction * relativePos.y + vector.first * TILE_SIZE, direction * relativePos.x + vector.second * TILE_SIZE };

      tempStructure[i] = { point.x + newRelativePos.x, point.y + newRelativePos.y };
    }

    // validate rotation
    bool bad = false;
    for (const Coords& coord : tempStructure) {
      if (coord.x < 0 || coord.x >= Window::WIDTH || coord.y > Window::HEIGHT) {
        bad = true;
        break;
      }

      if (coord.y > 0) {
        Coords tileCoords = toTileCoords(coord.x, coord.y);

        if (tileColors[tileCoords.y][tileCoords.x] != Colors::empty) {
          bad = true;
          break;
        }
      }
    }

    if (bad) continue;

    activeBlock.structure = tempStructure;
    break;
  }

  activeBlock.rotationState = newRotationState;
  timeReset = true;

  return true;
}

Coords Game::endLocation() {
  auto copy = activeBlock.structure;

  while (moveDown());
  Coords coord = activeBlock.structure.back();
  activeBlock.structure = copy;

  return coord;
}

bool Game::place() {
  for (const Coords& coord : activeBlock.structure) {
    if (coord.y < 0 - TILE_SIZE) return false;

    Coords tileCoords = toTileCoords(coord.x, coord.y);
    tileColors[tileCoords.y][tileCoords.x] = activeBlock.color;
  }

  // clear lines

  int cleared = 0;

  for (int row = ROWS - 1; row >= 0; row--) {
    bool clear = true;
    for (int col = 0; col < COLUMNS; col++)
      if (tileColors[row][col] == Colors::empty) clear = false;

    if (clear) {
      for (int col = 0; col < COLUMNS; col++)
        tileColors[row][col] = Colors::empty;

      Color* it = tileColors.begin() + row * COLUMNS;
      std::rotate(it, it + COLUMNS, tileColors.end());

      cleared++;
      linesCleared++;
    }
  }

  switch (cleared) {
    case 1: score += 100; break;
    case 2: score += 300; break;
    case 3: score += 500; break;
    case 4: score += 800; break;
    default: break;
  }

  int levelBefore = level;
  level = linesCleared / 5 + 1;
  if (level != levelBefore)
    framesForGravity = framesForGravity * (1.0f - 0.1f);

  frameCount = 0;
  holdLocked = false;
  return true;
}

void Game::spawnBlock(BlockType type) {
  // crazy rng algorithm
  if (type != BlockType::None) activeBlock.type = type;
  else {
    static BlockType lastBlock = BlockType::None;
    BlockType blockType = static_cast<BlockType>(rand() % 7);
    if (blockType == lastBlock) blockType = static_cast<BlockType>(rand() % 7);
    lastBlock = blockType;

    activeBlock.type = blockType;
  }

  int height = 0;
  for (int i = tileColors.size() - 1; i >= 0; i--) {
    if (tileColors.flat_index(i) != Colors::empty) {
      height = i / COLUMNS;
      break;
    }
  }

  int heightOffset = 0;
  if (height != tileColors.size()) {
    if (height >= 19) heightOffset = 2;
    else if (height >= 16) heightOffset = 1;
  }

  Coords refTile = toWindowCoords(COLUMNS / 2 - 1, ROWS - 2 + heightOffset);

  // THE LAST VALUE IS THE POINT IN WHICH THE BLOCK ROTATES AROUND
  
  switch (activeBlock.type) {
    case BlockType::I:
      activeBlock.structure = { refTile.moveX(-1), refTile.moveX(1), refTile.moveX(2), refTile };
      activeBlock.color = Colors::lightBlue;
      break;
    case BlockType::O:
      activeBlock.structure = { refTile.moveY(1), refTile.move(1, 1), refTile.moveX(1), refTile };
      activeBlock.color = Colors::yellow;
      break;
    case BlockType::T:
      activeBlock.structure = { refTile.moveX(-1), refTile.moveX(1), refTile.moveY(1), refTile };
      activeBlock.color = Colors::violet;
      break;
    case BlockType::L:
      activeBlock.structure = { refTile, refTile.moveX(2), refTile.move(2, 1), refTile.moveX(1) };
      activeBlock.color = Colors::orange;
      break;
    case BlockType::J:
      activeBlock.structure = { refTile.moveY(1), refTile, refTile.moveX(2), refTile.moveX(1) };
      activeBlock.color = Colors::red;
      break;
    case BlockType::S:
      activeBlock.structure = { refTile.moveY(1), refTile.move(1, 1), refTile.moveX(2), refTile.moveX(1) };
      activeBlock.color = Colors::green;
      break;
    case BlockType::Z:
      activeBlock.structure = { refTile.move(2, 1), refTile.move(1, 1), refTile, refTile.moveX(1) };
      activeBlock.color = Colors::darkBlue;
      break;
    case BlockType::None:
      break;
  }
}

void Game::handleEvents() {
  SDL_Event event;

  while (SDL_PollEvent(&event)) {
    switch (event.type) {
      case SDL_QUIT:
        isRunning = false;
        return;
      case SDL_KEYDOWN:
        if (screen == Screen::AWAIT_BEGIN && event.key.keysym.sym == SDLK_SPACE) {
          screen = Screen::PLAYING;
          level = 1;
          score = 0;
          framesForGravity = FPS::FPS;
          linesCleared = 0;
          frameCount = 0;
          resetTimers();
          hold = BlockType::None;
          holdLocked = false;

          for (Color &tileColor : tileColors)
            tileColor = Colors::empty;

          spawnBlock();

          return;
        } else if (screen == Screen::AWAIT_BEGIN) return;

        switch (event.key.keysym.sym) {
          case SDLK_SPACE:
            while (moveDown());
            if (!place()) {
              screen = Screen::AWAIT_BEGIN;
              return;
            }

            resetTimers();
            spawnBlock();
            
            break;
          case SDLK_UP:
            rotate(1);
            break;
          case SDLK_z:
            rotate(-1);
            break;
          case SDLK_c:
            if (holdLocked) break;

            if (hold != BlockType::None) {
              BlockType before = activeBlock.type;
              spawnBlock(hold);
              hold = before;
            } else {
              hold = activeBlock.type;
              spawnBlock();
            }

            holdLocked = true;
            break;
        }
    }
  }

  if (screen == Screen::AWAIT_BEGIN) return;

  static int downWait = 0;
  static int leftWait = 0;
  static int rightWait = 0;
  constexpr int delay = 4;
  constexpr int beforeContinuous = 6;

  SDL_PumpEvents();
  const Uint8* keystate = SDL_GetKeyboardState(NULL);

  if (keystate[SDL_SCANCODE_DOWN]) {
    if (downWait % delay == 0) {
      if (moveDown()) frameCount = 0;
    }

    downWait++;
  } else downWait = 0;

  if (keystate[SDL_SCANCODE_LEFT] && keystate[SDL_SCANCODE_RIGHT]) return;

  if (keystate[SDL_SCANCODE_LEFT]) {
    if (leftWait == 0 || (leftWait > beforeContinuous && leftWait % delay == 0))
      moveHorizontal(-1);

    leftWait++;
  } else leftWait = 0;

  if (keystate[SDL_SCANCODE_RIGHT]) {
    if (rightWait == 0 || (rightWait > beforeContinuous && rightWait % delay == 0))
      moveHorizontal(1);

    rightWait++;
  } else rightWait = 0;
}