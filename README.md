# Quferek

> Lokalny menedżer haseł napisany w Ruscie i TypeScripcie. Szyfrowany, darmowy i dziecinnie prosty.

Stworzony dla nastolatków — bo większość z nich używa `imię + rok urodzenia` zamiast hasła.

## Dlaczego?

- młodzi ludzie używają tych samych słabych haseł wszędzie
- płatne menedżery są poza zasięgiem

## Co robi?

- przechowuje hasła zaszyfrowane lokalnie 
- generuje silne hasła — z **10³¹ możliwych kombinacji**

## Tech Stack:

- **Rust** — bezpieczeństwo pamięci bez GC
- **Tauri** — mały framework do tworzenia aplikacji mobilnych i desktopowych
- **SvelteKit** — framework pozwalający na wygodną pracę z JavaScript/TypeScript
