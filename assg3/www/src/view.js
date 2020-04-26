import { Button, Window } from './ui';

const MARGIN = 1;
const ICON_COUNT = 4;
const BUTTON_ANIMATION_DURATION = 5;

export class View {
    constructor(boardWidth, boardHeight, onViewChange, onPlay, onStop) {
        this.boardWidth = boardWidth;
        this.boardHeight = boardHeight;
        this.container = document.getElementById('container');

        // callbacks
        this.onViewChange = onViewChange;
        this.onPlay = onPlay;
        this.onStop = onStop;

        this.startTime = performance.now();
        this.ripples = [];
        this.mouse = {
            x: 0,
            y: 0,
            pressed: false,
            clicked: false,
        };

        this.playing = false;
        this.settingsVisible = false;
        this.settingsButtonAnimationFrame = BUTTON_ANIMATION_DURATION;
        this.settingsButtonsAnimationFrame = 0;
        
        this.setUp();

        this.playStopButton = new Button(this);
        this.settingsButton = new Button(this);
        this.yellowPlayerSettingButton = new Button(this);
        this.redPlayerSettingButton = new Button(this);
        this.yellowPlayerWindow = new Window(this);
        this.redPlayerWindow = new Window(this);
  
        window.addEventListener('resize', () => {
            const [child] = this.container.children;
            if(child) {
                this.container.removeChild(child)
            }
            this.setUp();
            this.onViewChange();
        });

        window.addEventListener('mousemove', (event) => {
            const { left, top } = this.context.canvas.getBoundingClientRect();
            this.mouse.x = event.x - left;
            this.mouse.y = event.y - top;
            const position = Math.floor(this.mouse.x / this.boardUnit) - MARGIN;
            this.mouseColumnIndex = position >= 0 && position < this.boardWidth ? position : undefined;
        });

        window.addEventListener('mousedown', () => {
            this.mouse.pressed = true;
        });

        window.addEventListener('mouseup', () => {
            if(this.playStopButton.clicked) {
                this.playStopButton.clicked = false;
                if(!this.playing) {
                    this.playing = true;
                    this.settingsVisible = false;
                    this.yellowPlayerWindow.visible = false;
                    this.redPlayerWindow.visible = false;
                    this.onPlay();
                } else {
                    this.playing = false;
                    this.onStop();
                }
            } else if(this.settingsButton.clicked) {
                this.settingsButton.clicked = false;
                if(!this.settingsVisible) {
                    this.settingsVisible = true;
                } else {
                    this.settingsVisible = false;
                    this.yellowPlayerWindow.visible = false;
                    this.redPlayerWindow.visible = false;
                }
            } else if(this.yellowPlayerSettingButton.clicked) {
                this.yellowPlayerSettingButton.clicked = false;
                if(!this.yellowPlayerWindow.visible) {
                    this.yellowPlayerWindow.visible = true;
                    this.redPlayerWindow.visible = false;
                } else {
                    this.yellowPlayerWindow.visible = false;
                }
            } else if(this.redPlayerSettingButton.clicked) {
                this.redPlayerSettingButton.clicked = false;
                if(!this.redPlayerWindow.visible) {
                    this.redPlayerWindow.visible = true;
                    this.yellowPlayerWindow.visible = false;
                } else {
                    this.redPlayerWindow.visible = false;
                }
            }

            if(this.yellowPlayerWindow.humanAiSwitch.clicked) {
                this.yellowPlayerWindow.humanAiSwitch.clicked = false;
                this.yellowPlayerWindow.humanAiSwitch.toggled = !this.yellowPlayerWindow.humanAiSwitch.toggled;
            } else if(this.redPlayerWindow.humanAiSwitch.clicked) {
                this.redPlayerWindow.humanAiSwitch.clicked = false;
                this.redPlayerWindow.humanAiSwitch.toggled = !this.redPlayerWindow.humanAiSwitch.toggled;
            }

            this.mouse.pressed = false;
            this.mouse.clicked = false;
        });
    }
  
    setUp() {
        const { width, height } = this.container.getBoundingClientRect()

        const canvas = document.createElement('canvas');
        this.container.appendChild(canvas);
        this.context = canvas.getContext('2d');

        this.boardUnit = Math.min(width / (this.boardWidth + 2 * MARGIN), height / this.boardHeight);
        this.hudUnit = this.boardHeight / Math.max(this.boardHeight, ICON_COUNT) * this.boardUnit;

        canvas.setAttribute('width', this.boardUnit * (this.boardWidth + 2 * MARGIN));
        canvas.setAttribute('height', this.boardUnit * this.boardHeight);
    }

    calculateRipple(column) {
        var ripple = []
        const t0 = performance.now() - this.startTime;
        const A = 10;
        const w = 0.02;
        const b = 0.1;
        for(var x = 0; x < this.boardWidth; x++) {
            const distance = Math.abs(x - column);
            const dt = distance * 40;
            const offset = t0 + dt;
            const damp = (dt + 10) * 0.001;
            
            const rippleFunction = (t) => {
                const tt = t - offset;
                return t - t0 < dt ? 0 : A * Math.exp(-b * damp * tt) * Math.cos(w * tt - Math.PI/2);
            };
            ripple.push(rippleFunction);
        }
        this.ripples.push(ripple);
    }

    getRippleOffsets(time) {
        var rippleOffsets = [];
        for(var x = 0; x < this.boardWidth; x++) {
            var rippleOffset = 0;
            this.ripples.forEach((ripple) => {
                rippleOffset += ripple[x](time);
            });
            rippleOffsets.push(rippleOffset);
        }
        return rippleOffsets;
    }

    animate() {
        if(this.playing) {
            if(this.settingsButtonAnimationFrame > 0) {
                this.settingsButtonAnimationFrame -= 1;
            }
        } else {
            if(this.settingsButtonAnimationFrame < BUTTON_ANIMATION_DURATION) {
                this.settingsButtonAnimationFrame += 1;
            }
        }

        if(this.settingsVisible) {
            if(this.settingsButtonsAnimationFrame < BUTTON_ANIMATION_DURATION) {
                this.settingsButtonsAnimationFrame += 1;
            }
        } else {
            if(this.settingsButtonsAnimationFrame > 0) {
                this.settingsButtonsAnimationFrame -= 1;
            }
        }

        this.yellowPlayerWindow.update();
        this.redPlayerWindow.update();
    }

    render(board) {
        this.context.clearRect(0, 0, this.context.canvas.width, this.context.canvas.height);

        const discRadius = 0.4;

        const time = performance.now() - this.startTime;
        const rippleOffsets = this.getRippleOffsets(time);

        board.columns.forEach((column, x) => {
            const pointX = (x + 0.5 + MARGIN) * this.boardUnit;

            this.context.beginPath();
            this.context.moveTo(pointX, 0.5 * this.boardUnit + rippleOffsets[x]);
            this.context.lineTo(pointX, (this.boardHeight - 0.5) * this.boardUnit + rippleOffsets[x]);
            this.context.strokeStyle = "#ffffff";
            this.context.lineWidth = 1;
            this.context.stroke();

            column.forEach((disc, y) => {
                this.context.beginPath();
                this.context.arc(
                    (x + 0.5 + MARGIN) * this.boardUnit,
                    (this.boardHeight - y - 0.5) * this.boardUnit + rippleOffsets[x],
                    discRadius * this.boardUnit,
                    0, 2 * Math.PI
                );
                this.context.fillStyle = disc == 0 ? '#ebdb34' : '#e74c3c';
                this.context.fill();
            });
        });

        if(this.dropPreview !== undefined && this.mouseColumnIndex !== undefined) {
            this.context.beginPath();
            this.context.arc(
                (this.mouseColumnIndex + 0.5 + MARGIN) * this.boardUnit,
                (this.boardHeight - (board.columns[this.mouseColumnIndex].length) - 0.5) * this.boardUnit,
                this.boardUnit / 2.5,
                0, 2 * Math.PI
            );
            this.context.strokeStyle = this.dropPreview == 0 ? '#ebdb34' : '#e74c3c';
            this.context.lineWidth = 2;
            const circumference = Math.PI * this.boardUnit / 2.5;
            this.context.setLineDash([circumference / 8, circumference / 8]);
            this.context.lineDashOffset = time * 0.01;
            this.context.stroke()
            this.context.setLineDash([]);
            this.context.lineWidth = 1;
        }

        if(board.fourInARow) {
            const [position, direction] = board.fourInARow;
            this.context.beginPath();
            var x = position[0];
            var y = position[1];
            this.context.moveTo(
                (x + 0.5 + MARGIN) * this.boardUnit,
                (this.boardHeight - 1 - y + 0.5) * this.boardUnit + rippleOffsets[x]
            );
            for(var i = 1; i < 4; i++) {
                x += direction[0];
                y += direction[1];
                this.context.lineTo(
                    (x + 0.5 + MARGIN) * this.boardUnit,
                    (this.boardHeight - 1 - y + 0.5) * this.boardUnit + rippleOffsets[x]
                );
            }
            this.context.lineWidth = 2 * discRadius * this.boardUnit;
            this.context.lineCap = 'round';
            this.context.strokeStyle = board.columns[x][y] == 0 ? '#ebdb34' : '#e74c3c';
            this.context.stroke();
            this.context.lineWidth = 1;
        }

        this.container.style.cursor = 'default';

        this.playStopButton.render(
            0,
            true,
            0,
            1,
            this.playing,
            '#ffffff',
            !this.playing ? 'play' : 'stop'
        );

        this.settingsButton.render(
            1,
            this.settingsButtonAnimationFrame == BUTTON_ANIMATION_DURATION,
            1 - Math.pow(1 - this.settingsButtonAnimationFrame / BUTTON_ANIMATION_DURATION, 3),
            this.settingsButtonAnimationFrame / BUTTON_ANIMATION_DURATION,
            this.settingsVisible,
            '#ffffff',
            'settings',
        );

        this.yellowPlayerSettingButton.render(
            2,
            this.settingsButtonsAnimationFrame == BUTTON_ANIMATION_DURATION,
            1 + (1 - Math.pow(1 - this.settingsButtonsAnimationFrame / BUTTON_ANIMATION_DURATION, 3)),
            this.settingsButtonsAnimationFrame / BUTTON_ANIMATION_DURATION,
            this.yellowPlayerWindow.visible,
            '#ebdb34',
            this.yellowPlayerWindow.humanAiSwitch.toggled ? 'computer' : 'human',
        );

        this.redPlayerSettingButton.render(
            3,
            this.settingsButtonsAnimationFrame == BUTTON_ANIMATION_DURATION,
            1 + (1 - Math.pow(1 - this.settingsButtonsAnimationFrame / BUTTON_ANIMATION_DURATION, 3)) * 2,
            this.settingsButtonsAnimationFrame / BUTTON_ANIMATION_DURATION,
            this.redPlayerWindow.visible,
            '#e74c3c',
            this.redPlayerWindow.humanAiSwitch.toggled ? 'computer' : 'human',
        );

        this.yellowPlayerWindow.render(
            2,
            '#ebdb34'
        );

        this.redPlayerWindow.render(
            3,
            '#e74c3c'
        );
    }
}
