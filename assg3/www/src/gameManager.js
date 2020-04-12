import { Board, Disc, connectFourMinimax } from 'assg3';
import { View } from './view';

export class GameManager {
    constructor() {
        this.restart();
        this.view = new View(
            this.board.getColumns().length,
            this.board.getBound(),
            this.render.bind(this)
        );
        this.player = Disc.Yellow;
    }

    restart() {
        this.board = new Board(7, 6);
        this.player = Disc.Yellow;
    }

    nextPlayer() {
        if(this.player == Disc.Yellow) {
            this.player = Disc.Red;
        } else {
            this.player = Disc.Yellow;
        }
    }

    humanMove() {
        return new Promise((resolve, reject) => {
            const fn = () => {
                const column = this.view.mouseColumnIndex;
                if(column !== undefined) {
                    window.removeEventListener('mousedown', fn, false);
                    resolve(column);
                }
            }
            window.addEventListener('mousedown', fn);
        })
    }

    aiMove() {
        return new Promise(async (resolve, reject) => {
            const move = connectFourMinimax(this.board, this.player, 5).column;
            resolve(move);
        });
    }

    render() {
        this.view.render(this.board);
    }

    tick() {
        this.render();
    }

    async move(decisionFunction) {
        do {
            var move = await decisionFunction();
        } while (!this.board.isValidLocation(move));
        this.board.push(move, this.player);
        this.view.calculateRipple(move);
        this.nextPlayer();
    }

    async gameLoop() {
        while(true) {
            await this.move(this.humanMove.bind(this));
            await this.move(this.aiMove.bind(this));
        }
    }

    async run() {
        setInterval(this.tick.bind(this), 1000 / 60);
        this.gameLoop();
    }
}
