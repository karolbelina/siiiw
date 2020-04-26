const MARGIN = 1;

export class Button {
    constructor(view) {
        this.view = view;
        this.clicked = false;
    }

    render(index, hoverPredicate, position, opacity, toggle, color, iconName) {
        const discRadius = 0.4;

        let checkHover = () => {
            const x = (MARGIN - 1 + 0.5) * this.view.hudUnit - this.view.mouse.x;
            const y = (index + 0.5) * this.view.hudUnit - this.view.mouse.y;
            return Math.sqrt(x * x + y * y) < discRadius * this.view.hudUnit;
        };

        const hover = hoverPredicate && checkHover();

        if(this.view.mouse.pressed && !this.view.mouse.clicked && hover) {
            this.clicked = true;
            this.view.mouse.clicked = true;
        }

        if(hover || this.clicked || toggle) {
            this.view.context.beginPath();
            this.view.context.arc(
                (MARGIN - 1 + 0.5) * this.view.hudUnit,
                (index + 0.5) * this.view.hudUnit + (this.clicked ? 2 : 0),
                discRadius * this.view.hudUnit,
                0, 2 * Math.PI
            );
            this.view.context.fillStyle = color;
            this.view.context.fill();
            if(hover || this.clicked) {
                this.view.container.style.cursor = 'pointer';
            }
        }

        const buttonIconScale = 0.5

        this.drawIcon(
            (MARGIN - 1 + 0.5) * this.view.hudUnit,
            (position + 0.5) * this.view.hudUnit + (this.clicked ? 2 : 0),
            buttonIconScale * this.view.hudUnit,
            hover || this.clicked || toggle ? '#2c3e50' : color,
            opacity,
            Button.getIcon(iconName)
        );
    }

    drawIcon(x, y, scale, fillStyle, opacity, path) {
        const size = 24;
        this.view.context.save();
        this.view.context.scale(scale / size, scale / size);
        this.view.context.translate(x / (scale / size) - (size / 2), y / (scale / size) - (size / 2));
        this.view.context.fillStyle = fillStyle;
        this.view.context.globalAlpha = opacity;
        this.view.context.fill(path);
        this.view.context.restore();
    }

    static getIcon(name) {
        switch(name) {
            case 'play':
                return new Path2D('M8 5v14l11-7z');
            case 'restart':
                return new Path2D('M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z');
            case 'settings':
                return new Path2D('M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z');
            case 'stop':
                return new Path2D('M6 6h12v12H6z');
            case 'size':
                return new Path2D('M23 15h-2v2h2v-2zm0-4h-2v2h2v-2zm0 8h-2v2c1 0 2-1 2-2zM15 3h-2v2h2V3zm8 4h-2v2h2V7zm-2-4v2h2c0-1-1-2-2-2zM3 21h8v-6H1v4c0 1.1.9 2 2 2zM3 7H1v2h2V7zm12 12h-2v2h2v-2zm4-16h-2v2h2V3zm0 16h-2v2h2v-2zM3 3C2 3 1 4 1 5h2V3zm0 8H1v2h2v-2zm8-8H9v2h2V3zM7 3H5v2h2V3z');
            case 'human':
                return new Path2D('M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z');
            case 'computer':
                return new Path2D('M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z');
            default:
                throw "Invalid icon name";
        }
    }
}

const WINDOW_ANIMATION_DURATION = 5;

export class Window {
    constructor(view) {
        this.view = view;
        this.visible = false;
        this.frame = 0;
        this.humanAiSwitch = new Switch(this.view);
    }

    update() {
        if(this.visible) {
            if(this.frame < WINDOW_ANIMATION_DURATION) {
                this.frame += 1;
            }
        } else {
            if(this.frame > 0) {
                this.frame -= 1;
            }
        }

        this.humanAiSwitch.update();
    }

    render(index, backgroundColor) {

        if(this.frame > 0) {
            const scale = 1 - Math.pow(1 - this.frame / WINDOW_ANIMATION_DURATION, 3);

            const discRadius = 0.4;

            const radius = discRadius * this.view.hudUnit;
            const circlePadding = 1 - 2 * discRadius;
            this.view.context.font = `${this.view.hudUnit * 0.3}px Anonymous Pro`
            const calculatedWidth = this.view.context.measureText("Human").width + 0.8 * this.view.hudUnit + this.view.context.measureText("AI").width;
            const width = Math.max(scale * (calculatedWidth + 2 * discRadius * this.view.hudUnit), 2 * radius);
            const height = Math.max(scale * (this.view.hudUnit * (1 - circlePadding)), 2 * radius);
            const x = (MARGIN + circlePadding / 2) * this.view.hudUnit;
            const y = (index + 0.5) * this.view.hudUnit - height / 2;

            this.view.context.beginPath();
            this.view.context.moveTo(x + radius, y);
            this.view.context.arcTo(x + width, y, x + width, y + height, radius);
            this.view.context.arcTo(x + width, y + height, x, y + height, radius);
            this.view.context.arcTo(x, y + height, x, y, radius);
            this.view.context.arcTo(x, y, x + width, y, radius);
            this.view.context.closePath();
            this.view.context.fillStyle = backgroundColor;
            this.view.context.fill();

            // render content
            if(this.frame == WINDOW_ANIMATION_DURATION && this.visible) {
                var currentX = (MARGIN + 0.5) * this.view.hudUnit;
                this.view.context.fillStyle = '#2c3e50';
                this.view.context.font = `${this.view.hudUnit * 0.3}px Anonymous Pro`
                this.view.context.textBaseline = 'middle';
                this.view.context.fillText(
                    "Human",
                    currentX,
                    (index + 0.5) * this.view.hudUnit
                );

                currentX += this.view.context.measureText("Human").width + 0.1 * this.view.hudUnit;
                this.humanAiSwitch.render(
                    currentX,
                    (index + 0.5) * this.view.hudUnit,
                    0.6 * this.view.hudUnit,
                    this.view.hudUnit * (1 - circlePadding) * 0.4,
                );

                currentX += 0.7 * this.view.hudUnit;
                this.view.context.fillStyle = '#2c3e50';
                this.view.context.font = `${this.view.hudUnit * 0.3}px Anonymous Pro`
                this.view.context.textBaseline = 'middle';
                this.view.context.fillText(
                    "AI",
                    currentX,
                    (index + 0.5) * this.view.hudUnit
                );
            }
        }
    }
}

const SWITCH_ANIMATION_DURATION = 5;

class Switch {
    constructor(view) {
        this.view = view;
        this.clicked = false;
        this.toggled = false;
        this.frame = 0;
    }

    update() {
        if(this.toggled) {
            if(this.frame < SWITCH_ANIMATION_DURATION) {
                this.frame += 1;
            }
        } else {
            if(this.frame > 0) {
                this.frame -= 1;
            }
        }
    }

    render(x, y, width, height) {
        const radius = height / 2;

        this.view.context.beginPath();
        this.view.context.moveTo(x + radius, y - height / 2);
        this.view.context.arcTo(x + width, y - height / 2, x + width, y + height / 2, radius);
        this.view.context.arcTo(x + width, y + height / 2, x, y + height / 2, radius);
        this.view.context.arcTo(x, y + height / 2, x, y - height / 2, radius);
        this.view.context.arcTo(x, y - height / 2, x + width, y - height / 2, radius);
        this.view.context.closePath();
        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fill();

        const circleRadius = radius * 0.8;
        const position = this.toggled
            ? 1 - Math.pow(1 - this.frame / SWITCH_ANIMATION_DURATION, 3)
            : Math.pow(this.frame / SWITCH_ANIMATION_DURATION, 3);

        this.view.context.beginPath();
        this.view.context.arc(
            x + radius + position * (width - 2 * radius),
            y,
            circleRadius,
            0, 2 * Math.PI
        );
        this.view.context.fillStyle = '#ffffff';
        this.view.context.fill();

        const top = y - height / 2;
        const bottom = y + height / 2;
        const left = x;
        const right = x + width;
        const hover = this.view.mouse.x > left && this.view.mouse.x < right && this.view.mouse.y > top && this.view.mouse.y < bottom;

        if(this.view.mouse.pressed && !this.view.mouse.clicked && hover) {
            this.clicked = true;
            this.view.mouse.clicked = true;
        }

        if(hover || this.clicked) {
            this.view.container.style.cursor = 'pointer';
        }
    }
}
