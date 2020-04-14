export class View {
    constructor(boardWidth, boardHeight, onViewChange = () => {}, onClick = () => {}) {
        this.boardWidth = boardWidth;
        this.boardHeight = boardHeight;
        this.container = document.getElementById('container');
        this.onViewChange = onViewChange;
        this.startTime = performance.now();
        this.ripples = [];

        this.setUp();
  
        window.addEventListener('resize', () => {
            const [child] = this.container.children;
            if(child) {
                this.container.removeChild(child)
            }
            this.setUp();
            this.onViewChange();
        });

        window.addEventListener('mousedown', () => {
            onClick();
        });

        window.addEventListener('mousemove', (event) => {
            const { left } = this.context.canvas.getBoundingClientRect();
            const position = Math.floor((event.x - left) / this.unitOnScreen);
            this.mouseColumnIndex = position >= 0 && position < this.boardWidth ? position : undefined;
        });
    }
  
    setUp() {
        const { width, height } = this.container.getBoundingClientRect()

        const canvas = document.createElement('canvas');
        this.container.appendChild(canvas);
        this.context = canvas.getContext('2d');

        this.unitOnScreen = Math.min(width / this.boardWidth, height / this.boardHeight);

        canvas.setAttribute('width', this.unitOnScreen * this.boardWidth);
        canvas.setAttribute('height', this.unitOnScreen * this.boardHeight);
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

    render(board) {
        this.context.clearRect(0, 0, this.context.canvas.width, this.context.canvas.height);

        const discRadius = 0.4;

        const time = performance.now() - this.startTime;
        const rippleOffsets = this.getRippleOffsets(time);

        board.columns.forEach((column, x) => {
            const pointX = (x + 0.5) * this.unitOnScreen;

            this.context.beginPath();
            this.context.moveTo(pointX, 0.5 * this.unitOnScreen + rippleOffsets[x]);
            this.context.lineTo(pointX, (this.boardHeight - 0.5) * this.unitOnScreen + rippleOffsets[x]);
            this.context.strokeStyle = "#ffffff";
            this.context.stroke();

            column.forEach((disc, y) => {
                this.context.beginPath();
                this.context.arc(
                    (x + 0.5) * this.unitOnScreen,
                    (this.boardHeight - y - 0.5) * this.unitOnScreen + rippleOffsets[x],
                    discRadius * this.unitOnScreen,
                    0, 2 * Math.PI
                );
                this.context.fillStyle = disc == 0 ? '#ebdb34' : '#e74c3c';
                this.context.fill();
            });
        });

        if(this.dropPreview !== undefined && this.mouseColumnIndex !== undefined) {
            this.context.beginPath();
            this.context.arc(
                (this.mouseColumnIndex + 0.5) * this.unitOnScreen,
                (this.boardHeight - (board.columns[this.mouseColumnIndex].length) - 0.5) * this.unitOnScreen,
                this.unitOnScreen / 2.5,
                0, 2 * Math.PI
            );
            this.context.strokeStyle = this.dropPreview == 0 ? '#ebdb34' : '#e74c3c';
            this.context.lineWidth = 2;
            const circumference = Math.PI * this.unitOnScreen / 2.5;
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
                (x + 0.5) * this.unitOnScreen,
                (this.boardHeight - 1 - y + 0.5) * this.unitOnScreen + rippleOffsets[x]
            );
            for(var i = 1; i < 4; i++) {
                x += direction[0];
                y += direction[1];
                this.context.lineTo(
                    (x + 0.5) * this.unitOnScreen,
                    (this.boardHeight - 1 - y + 0.5) * this.unitOnScreen + rippleOffsets[x]
                );
            }
            this.context.lineWidth = 2 * discRadius * this.unitOnScreen;
            this.context.lineCap = 'round';
            this.context.strokeStyle = board.columns[x][y] == 0 ? '#ebdb34' : '#e74c3c';
            this.context.stroke();
            this.context.lineWidth = 1;
        }
    }
}
