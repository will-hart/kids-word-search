import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const EMPTY_GRID = {
  words: [],
  grid: []
}

function App() {
  const [grid, setGrid] = createSignal(EMPTY_GRID);
  const [gridSize, setGridSize] = createSignal(12);

  async function get_grid() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGrid(await invoke("get_grid", { size: gridSize() }));
  }

  return (
    <div class="container">
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_grid();
        }}
      >
        <input
          id="grid-size-input"
          type="numeric"
          value={gridSize()}
          onChange={(e) => {
            setGridSize(parseInt(e.currentTarget.value, 10))
            setGrid(EMPTY_GRID)
          }}
          placeholder="Enter a grid size"
        />
        <button type="submit">Get Grid</button>
      </form>

      <div style={{
        display: "grid",
        margin: "30px auto 0",
        "grid-template-columns": "72px ".repeat(gridSize()),
        "grid-template-rows": "72px ".repeat(gridSize()),
        "place-items": "center",
        "gap": 0,
        border: "1px solid black",
        background: "#CCCCCC",
      }}>{grid().grid.map((value) => {
          return <div style ={{
            border: "1px solid black",
            "place-self": "stretch",
            margin: 0,
            display: "grid",
            "place-content": "center",
            background: "#FFFFFF",
            "font-size": "1.3em",
            "font-weight": "bold",
          }}><span>{value}</span></div>
      })}</div>

      <h2>Words</h2>
      <ul>{grid().words.map((word) => <li>{word}</li>)}</ul>
    </div>
  );
}

export default App;
