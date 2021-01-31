#include "Menu.h"
#include "MainMenu.h"

void Menu::render(int x, int y) const {
  for (int i = 0; i < this->itemCount(); i++) {
    this->renderItem(i, x, ITEM_HEIGHT * 2 + y + i * ITEM_HEIGHT);
  }
}

Menu *Menu::tick(CFinalInput *inputs) {
  if (this->scrollTimer > 0) {
    this->scrollTimer--;
  }
  if (inputs->PDPDown() || inputs->PLADown() || inputs->PRADown()) {
    this->scrollBy(1);
    this->scrollTimer = 15;
  }
  if (inputs->PDPUp() || inputs->PLAUp() || inputs->PRAUp()) {
    this->scrollBy(-1);
    this->scrollTimer = 15;
  }
  if (inputs->PDPLeft() || inputs->PLALeft() || inputs->PRALeft()) {
    this->scrollBy(-5);
    this->scrollTimer = 15;
  }
  if (inputs->PDPRight() || inputs->PLARight() || inputs->PRARight()) {
    this->scrollBy(-5);
    this->scrollTimer = 15;
  }
  if (inputs->PLTrigger()) {
    this->cursor = 0;
    this->scrollTimer = 15;
  }
  if (inputs->PRTrigger()) {
    this->cursor = this->itemCount();
    this->scrollTimer = 15;
  }
  if (inputs->DDPDown() || inputs->DLADown() || inputs->DRADown()) {
    if (this->scrollTimer <= 0) {
      this->scrollBy(1);
      this->scrollTimer = 4;
    }
  }
  if (inputs->DDPUp() || inputs->DLAUp() || inputs->DRAUp()) {
    if (this->scrollTimer <= 0) {
      this->scrollBy(-1);
      this->scrollTimer = 4;
    }
  }
  if (inputs->PB()) {
    return this->backMenu();
  }
  if (inputs->PA()) {
    return this->clickItem(this->cursor);
  }
  return nullptr;
}

void Menu::scrollTo(int index) {
  if (index < 0) {
    this->cursor = 0;
  } else if (index > itemCount()) {
    this->cursor = this->itemCount() - 1;
  } else {
    this->cursor = index;
  }
}

void Menu::scrollBy(int delta) {
  int index = (this->cursor + delta) % this->itemCount();
  if (index < 0) {
    index += this->itemCount();
  }
  this->scrollTo(index);
}

int Menu::currentCursor() const {
  return this->cursor;
}
