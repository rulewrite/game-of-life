import { Cell, Universe } from 'wasm-game-of-life';
import { memory } from 'wasm-game-of-life/game_of_life_bg'; // WebAssembly 메모리 가져오기

(() => {
  const CELL_SIZE = 5; // px
  const GRID_COLOR = '#CCCCCC';
  const DEAD_COLOR = '#FFFFFF';
  const ALIVE_COLOR = '#000000';

  // universe 생성 및 너비와 높이 가져오기
  const universe = Universe.new();
  const width = universe.width();
  const height = universe.height();

  const canvas = document.getElementById('game-of-life-canvas');
  // 전체 셀을 위한 캔버스 영역과 각 셀 주위에 1px 테두리 추가
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext('2d');

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // 세로 선
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // 가로 선
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const getIndex = (row, column) => {
    return row * width + column;
  };

  const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  };

  const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
  };

  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
})();
