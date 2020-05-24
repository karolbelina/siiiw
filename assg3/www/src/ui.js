const MARGIN = 1;
const discRadius = 0.4;

const drawRoundedRect = (ctx, x, y, width, height, radius) => {
    ctx.moveTo(x + radius, y);
    ctx.arcTo(x + width, y, x + width, y + height, radius);
    ctx.arcTo(x + width, y + height, x, y + height, radius);
    ctx.arcTo(x, y + height, x, y, radius);
    ctx.arcTo(x, y, x + width, y, radius);
}

export class Button {
    constructor(view) {
        this.view = view;
        this.clicked = false;
    }

    render(index, hoverPredicate, position, opacity, toggle, color, iconName) {
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
    constructor(view, aiSwitchToggled = false) {
        this.view = view;
        this.visible = false;
        this.openFrame = 0;
        this.aiExtendFrame = 0;
        this.humanAiSwitch = new Switch(this.view);
        this.algorithmDropdown = new Dropdown(this.view, ["Minimax", "Alpha-beta pruning"], 1);
        this.depthSlider = new Slider(this.view, x => Math.floor(x * 10), 0.45);
        this.evaluationFnDropdown = new Dropdown(this.view, ["Basic", "Line counter", "Advanced"], 2);
        const a = 0.8
        const p = 10
        const pointsFn = x => Math.floor(Math.pow(x + a, p) * 1000 / Math.pow(1 + a, p));
        this.rowCtrQuadsSlider = new Slider(this.view, pointsFn, 1);
        this.rowCtrTripsSlider = new Slider(this.view, pointsFn, 0.44);
        this.rowCtrDubsSlider = new Slider(this.view, pointsFn, 0.32);
        this.rowCtrSinglesSlider = new Slider(this.view, pointsFn, 0);
        this.advancedTripsSlider = new Slider(this.view, pointsFn, 0.44);
        this.advancedDubsSlider = new Slider(this.view, pointsFn, 0.32);
        this.advancedCentersSlider = new Slider(this.view, pointsFn, 0.2);
        if(aiSwitchToggled) {
            this.humanAiSwitch.toggled = true;
        }
    }

    mouseUp() {
        this.humanAiSwitch.mouseUp();
        this.algorithmDropdown.mouseUp();
        this.depthSlider.mouseUp();
        this.evaluationFnDropdown.mouseUp();
        this.rowCtrQuadsSlider.mouseUp();
        this.rowCtrTripsSlider.mouseUp();
        this.rowCtrDubsSlider.mouseUp();
        this.rowCtrSinglesSlider.mouseUp();
    }

    update() {
        if(this.visible) {
            if(this.openFrame < WINDOW_ANIMATION_DURATION) {
                this.openFrame += 1;
            }
        } else {
            if(this.openFrame > 0) {
                this.openFrame -= 1;
            }
        }

        if(this.humanAiSwitch.toggled) {
            if(this.aiExtendFrame < WINDOW_ANIMATION_DURATION) {
                this.aiExtendFrame += 1;
            }
        } else {
            if(this.aiExtendFrame > 0) {
                this.aiExtendFrame -= 1;
            }
        }

        this.humanAiSwitch.update();
    }

    render(index, backgroundColor) {
        if(this.openFrame > 0) {
            const calculateWidth = () => {
                if(this.humanAiSwitch.toggled) {
                    return this.algorithmDropdown.width() + 1.3 * this.view.hudUnit;
                }
                return this.humanAiSwitch.width();
            };

            const openScale = 1 - Math.pow(1 - this.openFrame / WINDOW_ANIMATION_DURATION, 3);
            const aiExtendScale = this.humanAiSwitch.toggled
                ? 1 - Math.pow(1 - this.aiExtendFrame / WINDOW_ANIMATION_DURATION, 3)
                : Math.pow(this.aiExtendFrame / WINDOW_ANIMATION_DURATION, 3);

            const padding = 2 * discRadius * this.view.hudUnit;

            const radius = discRadius * this.view.hudUnit;
            const circlePadding = 1 - 2 * discRadius;
            const lineHeight = this.view.hudUnit * (1 - circlePadding) / 2;
            
            this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`;
            this.view.context.textBaseline = 'middle';

            const width = Math.max(openScale * (calculateWidth() + padding), 2 * radius);
            const height = Math.max(openScale * padding + 7 * lineHeight * aiExtendScale, 2 * radius);
            const x = (MARGIN + circlePadding / 2) * this.view.hudUnit;
            const y = (index + 0.5) * this.view.hudUnit - height / 2;

            this.view.context.beginPath();
            drawRoundedRect(this.view.context, x, y, width, height, radius);
            this.view.context.closePath();
            this.view.context.strokeStyle = '#2c3e50';
            this.view.context.lineWidth = this.view.hudUnit * 0.075;
            this.view.context.stroke();
            this.view.context.fillStyle = backgroundColor;
            this.view.context.fill();

            // render content
            if(this.openFrame == WINDOW_ANIMATION_DURATION && this.visible) {
                this.humanAiSwitch.render(
                    (MARGIN + 0.5) * this.view.hudUnit,
                    y + lineHeight
                );

                if(this.aiExtendFrame == WINDOW_ANIMATION_DURATION && this.humanAiSwitch.toggled) {
                    const labelsX = (MARGIN + 0.35) * this.view.hudUnit;
                    const componentsX = (MARGIN + 2) * this.view.hudUnit;
                    this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                    this.view.context.fillStyle = '#2c3e50';
                    this.view.context.fillText("Algorithm", labelsX, y + 2 * lineHeight);
                    this.algorithmDropdown.render(
                        componentsX,
                        y + 2 * lineHeight,
                    );

                    this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                    this.view.context.fillStyle = '#2c3e50';
                    this.view.context.fillText("Depth", labelsX, y + 3 * lineHeight);
                    this.depthSlider.render(
                        componentsX,
                        y + 3 * lineHeight,
                        this.algorithmDropdown.width()
                    );

                    this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                    this.view.context.fillStyle = '#2c3e50';
                    this.view.context.fillText("Evaluation", labelsX, y + 4 * lineHeight);
                    this.evaluationFnDropdown.render(
                        componentsX,
                        y + 4 * lineHeight,
                    );

                    if(this.evaluationFnDropdown.getValue() == "Line counter") {
                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Quadruples", labelsX, y + 5 * lineHeight);
                        this.rowCtrQuadsSlider.render(
                            componentsX,
                            y + 5 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );

                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Triples", labelsX, y + 6 * lineHeight);
                        this.rowCtrTripsSlider.render(
                            componentsX,
                            y + 6 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );

                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Doubles", labelsX, y + 7 * lineHeight);
                        this.rowCtrDubsSlider.render(
                            componentsX,
                            y + 7 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );

                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Singles", labelsX, y + 8 * lineHeight);
                        this.rowCtrSinglesSlider.render(
                            componentsX,
                            y + 8 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );
                    } else if(this.evaluationFnDropdown.getValue() == "Advanced") {
                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Triples", labelsX, y + 5 * lineHeight);
                        this.advancedTripsSlider.render(
                            componentsX,
                            y + 5 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );

                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Doubles", labelsX, y + 6 * lineHeight);
                        this.advancedDubsSlider.render(
                            componentsX,
                            y + 6 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );

                        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
                        this.view.context.fillStyle = '#2c3e50';
                        this.view.context.fillText("Centers", labelsX, y + 7 * lineHeight);
                        this.advancedCentersSlider.render(
                            componentsX,
                            y + 7 * lineHeight,
                            this.evaluationFnDropdown.width()
                        );
                    }
                }
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

    mouseUp() {
        if(this.clicked) {
            this.clicked = false;
            this.toggled = !this.toggled;
        }
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

    render(x, y) {
        const height = this.view.hudUnit * 0.3;
        const actualX = x - height / 2;

        const spacing = 0.1 * this.view.hudUnit;

        this.view.context.font = `bold ${this.view.hudUnit * 0.25}px Anonymous Pro`
        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fillText("Human", actualX, y);

        const leftTextWidth = this.view.context.measureText("Human").width;

        const width = 0.6 * this.view.hudUnit;
        const radius = height / 2;
        const switchX = actualX + leftTextWidth + spacing;

        this.view.context.beginPath();
        drawRoundedRect(this.view.context, switchX, y - height / 2, width, height, radius);
        this.view.context.closePath();
        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fill();

        const circleRadius = radius * 0.8;
        const position = this.toggled
            ? 1 - Math.pow(1 - this.frame / SWITCH_ANIMATION_DURATION, 3)
            : Math.pow(this.frame / SWITCH_ANIMATION_DURATION, 3);

        this.view.context.beginPath();
        this.view.context.arc(
            switchX + radius + position * (width - 2 * radius),
            y,
            circleRadius,
            0, 2 * Math.PI
        );
        this.view.context.fillStyle = '#ffffff';
        this.view.context.fill();

        const top = y - height / 2;
        const bottom = y + height / 2;
        const left = switchX;
        const right = switchX + width;
        const hover = this.view.mouse.x > left && this.view.mouse.x < right && this.view.mouse.y > top && this.view.mouse.y < bottom;

        if(this.view.mouse.pressed && !this.view.mouse.clicked && hover) {
            this.clicked = true;
            this.view.mouse.clicked = true;
        }

        if(hover || this.clicked) {
            this.view.container.style.cursor = 'pointer';
        }

        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fillText("AI", actualX + leftTextWidth + spacing + width + spacing, y);
    }

    width() {
        const spacing = 0.1 * this.view.hudUnit;
        const width = 0.6 * this.view.hudUnit;
        const height = this.view.hudUnit * 0.3;
        const leftTextWidth = this.view.context.measureText("Human").width;
        const rightTextWidth = this.view.context.measureText("AI").width;
        
        return leftTextWidth + spacing + width + spacing + rightTextWidth - height;
    }
}

const DROPDOWN_ANIMATION_DURATION = 5;

class Dropdown {
    constructor(view, options, initialOption = 0) {
        this.view = view;
        this.clicked = false;
        this.options = options;
        this.selected = initialOption;
        this.frame = 0;
    }

    getValue() {
        return this.options[this.selected];
    }

    mouseUp() {
        if(this.clicked) {
            this.clicked = false;
            this.selected = (this.selected + 1) % this.options.length
        }
    }

    maxWidth() {
        this.view.context.font = `${this.view.hudUnit * 0.2}px Anonymous Pro`
        var maxWidth = 0;
        for(var i = 0; i < this.options.length; i++) {
            const width = this.view.context.measureText(this.options[i]).width;
            if(width > maxWidth) {
                maxWidth = width;
            }
        }
        return maxWidth;
    }

    render(x, y) {
        const width = this.maxWidth();
        const height = this.view.hudUnit * 0.3;
        const spacing = 0.1 * this.view.hudUnit;
        const radius = height / 2;
        y += this.clicked ? 2 : 0;

        this.view.context.beginPath();
        drawRoundedRect(this.view.context, x - height / 2, y - height / 2, width + 2 * spacing, height, radius);
        this.view.context.closePath();
        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fill();

        this.view.context.font = `${this.view.hudUnit * 0.2}px Anonymous Pro`
        const currentWidth = this.view.context.measureText(this.options[this.selected]).width;

        this.view.context.fillStyle = '#ffffff';
        this.view.context.fillText(
            this.options[this.selected],
            x + spacing + (width - currentWidth) / 2 - height / 2,
            y
        );

        const top = y - height / 2;
        const bottom = y + height / 2;
        const left = x - height / 2;
        const right = x - height / 2 + width + 2 * spacing;
        const hover = this.view.mouse.x > left && this.view.mouse.x < right && this.view.mouse.y > top && this.view.mouse.y < bottom;

        if(this.view.mouse.pressed && !this.view.mouse.clicked && hover) {
            this.clicked = true;
            this.view.mouse.clicked = true;
        }

        if(hover || this.clicked) {
            this.view.container.style.cursor = 'pointer';
        }
    }

    width() {
        const width = this.maxWidth();
        const spacing = 0.1 * this.view.hudUnit;

        return width + 2 * spacing;
    }
}

class Slider {
    constructor(view, fn, initialPosition) {
        this.view = view;
        this.clicked = false;
        this.position = initialPosition;
        this.fn = fn;
    }

    mouseUp() {
        if(this.clicked) {
            this.clicked = false;
        }
    }

    getValue() {
        return this.fn(this.position);
    }

    render(x, y, width) {
        const height = this.view.hudUnit * 0.3;
        const circleRadius = height / 3;
        const spacing = 0.1 * this.view.hudUnit;
        x += circleRadius - height / 2;

        this.view.context.beginPath();
        this.view.context.moveTo(x, y);
        this.view.context.lineTo(x + width - height, y);
        this.view.context.strokeStyle = '#2c3e50';
        this.view.context.lineWidth = this.view.hudUnit * 0.025;
        this.view.context.stroke();

        // update position
        if(this.clicked) {
            const position = (this.view.mouse.x - x) / (width - height);
            this.position = Math.max(0, Math.min(position, 1));
        }

        this.view.context.beginPath();
        this.view.context.arc(
            x + this.position * (width - height),
            y,
            circleRadius,
            0, 2 * Math.PI
        );
        this.view.context.strokeStyle = '#2c3e50';
        this.view.context.lineWidth = this.view.hudUnit * 0.05;
        this.view.context.stroke();
        this.view.context.fillStyle = '#ffffff';
        this.view.context.fill();

        const top = y - circleRadius;
        const bottom = y + circleRadius;
        const left = x - circleRadius;
        const right = x + width - height + circleRadius;
        const hover = this.view.mouse.x > left && this.view.mouse.x < right && this.view.mouse.y > top && this.view.mouse.y < bottom;

        if(this.view.mouse.pressed && !this.view.mouse.clicked && hover) {
            this.clicked = true;
            this.view.mouse.clicked = true;
        }

        if(hover || this.clicked) {
            this.view.container.style.cursor = 'pointer';
        }

        this.view.context.font = `bold ${this.view.hudUnit * 0.2}px Anonymous Pro`
        this.view.context.fillStyle = '#2c3e50';
        this.view.context.fillText(this.getValue(), x + width - height + circleRadius + spacing, y);
    }

    width() {

    }
}
