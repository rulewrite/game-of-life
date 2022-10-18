import { Cell, Universe } from 'game-of-life';
import { memory } from 'game-of-life/game_of_life_bg'; // WebAssembly 메모리 가져오기

const fps = new (class {
  constructor() {
    this.fps = document.getElementById('fps');
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // 마지막 프레임 렌더링 이후의 델타 시간을 초당 프레임 수(frames per second)로 변환
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * 1000;

    // 최근 100개 까지 저장
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    const { min, max, sum } = this.frames.reduce(
      ({ sum, min, max }, frame) => {
        return {
          sum: sum + frame,
          min: Math.min(frame, min),
          max: Math.max(frame, max),
        };
      },
      {
        sum: 0,
        min: Infinity,
        max: -Infinity,
      }
    );

    const mean = sum / this.frames.length;

    this.fps.textContent = `Frames per Second:\nlatest = ${Math.round(
      fps
    )}\navg of last 100 = ${Math.round(mean)}\nmin of last 100 = ${Math.round(
      min
    )}\nmax of last 100 = ${Math.round(max)}`.trim();
  }
})();

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

    [
      {
        color: ALIVE_COLOR,
        cell: Cell.Alive,
      },
      {
        color: DEAD_COLOR,
        cell: Cell.Dead,
      },
    ].forEach(({ color, cell }) => {
      ctx.fillStyle = color;

      for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
          const idx = getIndex(row, col);

          if (cells[idx] !== cell) {
            continue;
          }

          ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
          );
        }
      }
    });

    ctx.stroke();
  };

  canvas.addEventListener('click', (event) => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if (event.altKey) {
      universe.glider(row, col);
    } else if (event.shiftKey) {
      universe.pulsar(row, col);
    } else {
      universe.toggle_cell(row, col);
    }

    drawGrid();
    drawCells();
  });

  let animationId = null;
  let tickPerFrame = 1;

  const renderLoop = () => {
    fps.render();

    drawGrid();
    drawCells();

    for (let i = 0; i < tickPerFrame; i++) {
      universe.tick();
    }

    animationId = requestAnimationFrame(renderLoop);
  };

  const tickPerFrameInput = document.getElementById('tick-per-frame');
  const tickPerFrameValue = document.getElementById('tick-per-frame-value');
  tickPerFrameInput.value = tickPerFrame;
  tickPerFrameValue.textContent = tickPerFrame;
  tickPerFrameInput.addEventListener('change', (event) => {
    tickPerFrame = event.currentTarget.value;
    tickPerFrameValue.textContent = tickPerFrame;
  });

  const isPause = () => {
    return animationId === null;
  };

  const playPauseButton = document.getElementById('play-pause');

  const play = () => {
    playPauseButton.textContent = '⏸';
    renderLoop();
  };

  const pause = () => {
    playPauseButton.textContent = '▶';
    cancelAnimationFrame(animationId);
    animationId = null;
  };

  playPauseButton.addEventListener('click', () => {
    if (isPause()) {
      play();
      return;
    }

    pause();
  });

  play();

  document.getElementById('random').addEventListener('click', () => {
    universe.random();
  });

  document.getElementById('clear').addEventListener('click', () => {
    universe.clear();
  });
})();
