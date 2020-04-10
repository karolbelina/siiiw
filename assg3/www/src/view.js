export class View {
    constructor(boardWidth, boardHeight, onViewChange = () => {}, onClick = () => {}) {
        this.boardWidth = boardWidth;
        this.boardHeight = boardHeight;
        this.container = document.getElementById('container');
        this.onViewChange = onViewChange;

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

    render(board) {
        this.context.clearRect(0, 0, this.context.canvas.width, this.context.canvas.height)

        board.getColumns().forEach((column, x) => {
            const pointX = (x + 0.5) * this.unitOnScreen;

            this.context.beginPath();
            this.context.moveTo(pointX, 0.5 * this.unitOnScreen);
            this.context.lineTo(pointX, (this.boardHeight - 0.5) * this.unitOnScreen);
            this.context.strokeStyle = "#ffffff";
            this.context.stroke();

            column.forEach((disc, y) => {
                this.context.beginPath()
                this.context.arc(
                    (x + 0.5) * this.unitOnScreen,
                    (this.boardHeight - y - 0.5) * this.unitOnScreen,
                    this.unitOnScreen / 2.5,
                    0, 2 * Math.PI
                )
                this.context.fillStyle = disc == 0 ? '#ebdb34' : '#e74c3c'
                this.context.fill()
            });
        });
    }
}
