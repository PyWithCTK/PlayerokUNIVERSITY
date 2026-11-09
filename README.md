# PlayerokUniversity ⚡

Десктопное приложение для учёта сделок на Playerok: считает сумму «на карту», чистый профит и общую выручку с учётом комиссий площадки (10% / 20%) и вывода (6% / 10%).

Написано на Rust (egui) — один нативный `.exe`, без Python и сторонних библиотек.

## Установка

Скачай со страницы [Releases](../../releases):

- `PlayerokUniversity-Setup-X.Y.Z.exe` — установщик (ярлыки, удаление через «Приложения»);
- `PlayerokUniversity-portable.exe` — без установки, просто запусти.

## Где данные

`data.json` лежит в `%APPDATA%\PlayerokUniversity\`. Если положить свой `data.json` рядом с exe (в том числе из старой Python-версии — формат тот же), программа возьмёт его.

## Сборка

```bash
cargo run --release
cargo test
```

Релиз: пушишь тег `vX.Y.Z` — GitHub Actions соберёт exe и установщик (Inno Setup) и выложит в Releases.

Программа не является официальным приложением площадки Playerok.
