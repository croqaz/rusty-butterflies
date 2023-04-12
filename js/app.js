import { h, Component } from "preact";

const PLAYER = "👾";

class GameGrid extends Component {
  state = { turns: 0 };

  onKeyPressed = (ev) => {
    if (ev.key === "w") {
      // try to move Nord, up
      window.game.slide_view("n");
    } else if (ev.key === "s") {
      // try to move South, down
      window.game.slide_view("s");
    } else if (ev.key === "a") {
      window.game.slide_view("w");
    } else if (ev.key === "d") {
      window.game.slide_view("e");
    }
    this.setState({ turns: this.state.turns + 1 });
  };

  onMouseOver = (ev) => {
    ev.target.classList.add("selected");
  };

  onMouseLeave = (ev) => {
    ev.target.classList.remove("selected");
  };

  componentDidMount() {
    document.addEventListener("keydown", this.onKeyPressed);
  }

  componentWillUnmount() {
    this.removeEventListener("keydown", this.onKeyPressed);
  }

  render() {
    const tiles = window.game ? window.game.render() : [[]];
    const gridCol = (cell, c, r) => {
      return (
        <td
          key={"x" + (r + 1) + (c + 1)}
          onMouseOver={this.onMouseOver}
          onMouseLeave={this.onMouseLeave}>
          {h(cell.type, {}, cell.children)}
        </td>
      );
    };
    const gridRow = (row, r) => (
      <tr key={"y" + (r + 1)}>{row.map((cell, c) => gridCol(cell, c, r))}</tr>
    );
    return <table id="gameGrid">{tiles.map(gridRow)}</table>;
  }
}

class App extends Component {
  render() {
    return (
      <main id="app" tabIndex={0}>
        <GameGrid props={this.state} />
      </main>
    );
  }
}

export default App;
