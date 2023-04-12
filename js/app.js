import { h, Component } from "preact";

const PLAYER = "👾";

class GameGrid extends Component {
  state = { turns: 0 };

  onKeyPressed = (ev) => {
    if (ev.key === "w") {
      // try to move Nord, up
      window.Game.slide_view(window.Direction.N);
    } else if (ev.key === "s") {
      // try to move South, down
      window.Game.slide_view(window.Direction.S);
    } else if (ev.key === "a") {
      window.Game.slide_view(window.Direction.W);
    } else if (ev.key === "d") {
      window.Game.slide_view(window.Direction.E);
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
    const tiles = window.Game ? window.Game.render() : [[]];
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
