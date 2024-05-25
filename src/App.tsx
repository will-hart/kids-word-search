import { createEffect, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const EMPTY_GRID = {
  words: [],
  grid: [],
};

function App() {
  const [grid, setGrid] = createSignal(EMPTY_GRID);
  const [gridSize, setGridSize] = createSignal(12);

  const [wordListOptions, setWordListOptions] = createSignal<string[]>([]);
  const [selectedWordList, setSelectedWordList] = createSignal("EarlyPrimary");

  const [debugMode, setDebugMode] = createSignal(false);

  createEffect(async () => {
    setWordListOptions(await invoke("get_word_list_options", {}));
  });

  async function get_grid() {
    setGrid(
      await invoke("get_grid", { list: selectedWordList(), size: gridSize() })
    );
  }

  return (
    <div class="container">
      <form
        class="row controls"
        onSubmit={(e) => {
          e.preventDefault();
          get_grid();
        }}
      >
        <span>
          <label for="selected-list-input">Word List </label>
          <select
            id="selected-list-input"
            onChange={(e) => {
              setSelectedWordList(e.target.value);
            }}
          >
            {wordListOptions().map((opt) => (
              <option value={opt}>{opt}</option>
            ))}
          </select>
        </span>

        <span>
          <label for="grid-size-input">Grid Size </label>
          <input
            id="grid-size-input"
            type="number"
            min={5}
            max={20}
            value={gridSize()}
            onChange={(e) => {
              setGridSize(parseInt(e.currentTarget.value, 10));
              setGrid(EMPTY_GRID);
            }}
            placeholder="Enter a grid size"
          />
        </span>

        <span>
          <label for="use-debug-mode">Debug View</label>
          <input
            id="use-debug-mode"
            type="checkbox"
            checked={debugMode()}
            onChange={(e) => setDebugMode(e.target.checked)}
          />
        </span>

        <button type="submit" disabled={selectedWordList().length == 0}>
          Get Grid
        </button>
      </form>

      <div
        class="grid-container"
        style={{
          "grid-template-columns": "72px ".repeat(gridSize()),
          "grid-template-rows": "72px ".repeat(gridSize()),
        }}
      >
        {grid().grid.map((value: string) => {
          return (
            <div
              class="word-grid-item"
              style={{
               
                background:
                  debugMode() && value.toUpperCase() === value
                    ? "#AAAAAA"
                    : "#FFFFFF", 
              }}
            >
              <span>{value}</span>
            </div>
          );
        })}
      </div>

      <div>
        <h2>Words: {selectedWordList()}</h2>
        <div class="word-list">
          {grid().words.map((word) => (
            <div>{word}</div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;
