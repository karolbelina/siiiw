import { Board, Disc } from 'assg3';
import { View } from './view';

export class GameManager {
    constructor() {
        this.restart();
        this.view = new View(
            this.board.getColumns().length,
            this.board.getBound(),
            this.render.bind(this),
            this.onClick.bind(this)
        );
        this.player = Disc.Yellow;
    }

    restart() {
        this.board = new Board(7, 6);
        this.player = Disc.Yellow;
    }

    onClick() {
        const column = this.view.mouseColumnIndex;
        if(column !== undefined) {
            if(this.board.isValidLocation(column)) {
                this.board.push(column, this.player);
                this.player = this.player == Disc.Yellow ? Disc.Red : Disc.Yellow;
            }
        }
    }

    render() {
        this.view.render(this.board);
    }

    tick() {
        this.render();
    }

    run() {
        setInterval(this.tick.bind(this), 1000 / 60)
    }
}
