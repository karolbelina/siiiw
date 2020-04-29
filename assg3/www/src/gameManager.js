import { Board, Disc } from 'assg3';
import { View } from './view';

export class GameManager {
    constructor() {
        this.restart();
        this.playing = false;
        this.humanPlayer = false;
        const humanMove = () => {
            const move = this.view.mouseColumnIndex;
            if(move !== undefined && this.board.isValidLocation(move)) {
                this.board.push(move, this.player);
                this.view.calculateRipple(move);
                if(this.board.checkForWin(this.player) || this.board.checkForDraw()) {
                    this.stop();
                    return true;
                }
                this.player = this.player == Disc.Yellow ? Disc.Red : Disc.Yellow;
                return true;
            }
            return false;
        };
        const aiMove = (ai) => {
            const move = ai(this.board);
            this.board.push(move, this.player);
            this.view.calculateRipple(move);
            if(this.board.checkForWin(this.player) || this.board.checkForDraw()) {
                this.stop();
            }
            this.player = this.player == Disc.Yellow ? Disc.Red : Disc.Yellow;
        }

        const humanVsHumanMove = () => {
            return () => {
                humanMove();
                if(this.playing) {
                    this.view.dropPreview = this.player;
                }
            }
        };
        const humanVsAiMove = (ai) => {
            return async () => {
                if(this.humanPlayer) {
                    const validMove = humanMove();
                    if(validMove) {
                        this.humanPlayer = false;
                        this.view.dropPreview = undefined;
                        await this.timeout(500);
                        if(this.playing) {
                            aiMove(ai);
                            if(this.playing) {
                                this.humanPlayer = true;
                                this.view.dropPreview = this.player;
                            }
                        }
                    }
                }
            }
        }
        this.view = new View(
            this.board.columns.length,
            this.board.bound,
            this.render.bind(this),
            async (yellowAi, redAi) => {
                this.player = Disc.Yellow;
                this.play();
                if(!yellowAi && !redAi) {
                    this.onClick = humanVsHumanMove();
                    this.humanPlayer = true;
                    this.view.dropPreview = this.player;
                    window.addEventListener('mousedown', this.onClick);
                } else if(!yellowAi && redAi) {
                    this.onClick = humanVsAiMove(redAi);
                    this.humanPlayer = true;
                    this.view.dropPreview = this.player;
                    window.addEventListener('mousedown', this.onClick);
                } else if(yellowAi && !redAi) {
                    this.onClick = humanVsAiMove(yellowAi);
                    this.humanPlayer = false;
                    aiMove(yellowAi)
                    this.humanPlayer = true;
                    this.view.dropPreview = this.player;
                    window.addEventListener('mousedown', this.onClick);
                } else if(yellowAi && redAi) {
                    this.humanPlayer = false;
                    while(true) {
                        aiMove(yellowAi);
                        await this.timeout(0);
                        if(!this.playing) {
                            break;
                        }
                        aiMove(redAi);
                        await this.timeout(0);
                        if(!this.playing) {
                            break;
                        }
                    }
                }
            },
            this.stop.bind(this),
        );
    }

    play() {
        this.playing = true;
        this.restart();
    }

    restart() {
        this.board = new Board(7, 6);
    }

    stop() {
        this.playing = false;
        this.view.playing = false;
        this.view.dropPreview = undefined;
        window.removeEventListener('mousedown', this.onClick);
    }

    render() {
        this.view.render(this.board);
    }

    tick() {
        this.view.animate();
        this.render();
    }

    timeout(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async run() {
        setInterval(this.tick.bind(this), 1000 / 60);
    }
}
