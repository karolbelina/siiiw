import { Board, Disc, connectFourMinimax } from 'assg3';
import { View } from './view';

export class GameManager {
    constructor() {
        this.restart();
        this.view = new View(
            this.board.columns.length,
            this.board.bound,
            this.render.bind(this)
        );
        this.yellowPlayer = this.setupHuman(Disc.Yellow);
        this.redPlayer = this.setupHuman(Disc.Red);
    }

    restart() {
        this.board = new Board(7, 6);
    }

    setupHuman(color) {
        return () => {
            return new Promise(resolve => {
                const fn = () => {
                    const column = this.view.mouseColumnIndex;
                    if(column !== undefined) {
                        window.removeEventListener('mousedown', fn, false);
                        this.view.dropPreview = undefined;
                        resolve(column);
                    }
                }
                this.view.dropPreview = color;
                window.addEventListener('mousedown', fn);
            });
        };
    }

    setupAi(color) {
        return () => {
            return new Promise(async resolve => {
                const move = connectFourMinimax(this.board, color, 4).column;
                await this.timeout(500);
                resolve(move);
            });
        };
    }

    render() {
        this.view.render(this.board);
    }

    tick() {
        this.render();
    }

    async move(decisionFunction, color) {
        do {
            var move = await decisionFunction();
        } while (!this.board.isValidLocation(move));
        this.board.push(move, color);
        this.view.calculateRipple(move);
    }

    timeout(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async gameLoop() {
        while(true) {
            await this.move(this.yellowPlayer, Disc.Yellow);
            if(this.board.checkForWin(Disc.Yellow)) {
                console.log("Yellow wins")
                break;
            }
            if(this.board.checkForDraw()) {
                console.log("Draw")
                break;
            }
            await this.move(this.redPlayer, Disc.Red);
            if(this.board.checkForWin(Disc.Red)) {
                console.log("Red wins")
                break;
            }
            if(this.board.checkForDraw()) {
                console.log("Draw")
                break;
            }
        }
    }

    async run() {
        setInterval(this.tick.bind(this), 1000 / 60);
        this.gameLoop();
    }
}
