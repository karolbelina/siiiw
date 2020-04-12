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

    render(board) {
        this.context.clearRect(0, 0, this.context.canvas.width, this.context.canvas.height)

        var timeOffset = performance.now() - this.startTime;
        board.getColumns().forEach((column, x) => {
            const pointX = (x + 0.5) * this.unitOnScreen;

            var rippleOffset = 0;
            this.ripples.forEach((ripple) => {
                rippleOffset += ripple[x](timeOffset);
            });

            this.context.beginPath();
            this.context.moveTo(pointX, 0.5 * this.unitOnScreen + rippleOffset);
            this.context.lineTo(pointX, (this.boardHeight - 0.5) * this.unitOnScreen + rippleOffset);
            this.context.strokeStyle = "#ffffff";
            this.context.stroke();

            column.forEach((disc, y) => {
                this.context.beginPath()
                this.context.arc(
                    (x + 0.5) * this.unitOnScreen,
                    (this.boardHeight - y - 0.5) * this.unitOnScreen + rippleOffset,
                    this.unitOnScreen / 2.5,
                    0, 2 * Math.PI
                )
                this.context.fillStyle = disc == 0 ? '#ebdb34' : '#e74c3c'
                this.context.fill()
            });
        });

        if(this.mouseColumnIndex !== undefined) {
            this.context.beginPath()
            this.context.arc(
                (this.mouseColumnIndex + 0.5) * this.unitOnScreen,
                (this.boardHeight - (board.getColumns()[this.mouseColumnIndex].length) - 0.5) * this.unitOnScreen,
                this.unitOnScreen / 2.5,
                0, 2 * Math.PI
            )
            this.context.strokeStyle = '#e74c3c';
            this.context.lineWidth = 2;
            const circumference = Math.PI * this.unitOnScreen / 2.5;
            this.context.setLineDash([circumference / 8, circumference / 8]);
            this.context.lineDashOffset = (performance.now() - this.startTime) * 0.01;
            this.context.stroke()
            this.context.setLineDash([]);
            this.context.lineWidth = 1;
        }
    }
}
