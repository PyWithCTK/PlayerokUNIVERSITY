# PlayerokUniversity ⚡

Десктопное приложение для учёта сделок на Playerok: считает сумму «на карту», чистый профит и общую выручку с учётом комиссий площадки (10% / 20%) и вывода (6% / 10%).

Написано на Rust (egui) — один нативный `.exe`, без Python и сторонних библиотек.

## Установка

Скачай со страницы [Releases](../../releases):

- Windows: `PlayerokUniversity-Setup-X.Y.Z.exe` (установщик) или `PlayerokUniversity-portable.exe`;
- Linux: `.deb` (`sudo apt install ./файл.deb`) или `.tar.gz`;
- macOS: `.dmg` (Apple Silicon — `arm64`, Intel — `x86_64`; приложение без подписи: правый клик → «Открыть»).

## Где данные

`data.json` лежит в `%APPDATA%\PlayerokUniversity\`. Если положить свой `data.json` рядом с exe (в том числе из старой Python-версии — формат тот же), программа возьмёт его.

## Сборка

```bash
cargo run --release
cargo test
```

Релиз: пушишь тег `vX.Y.Z` — GitHub Actions соберёт exe и установщик (Inno Setup) и выложит в Releases.

Программа не является официальным приложением площадки Playerok.
