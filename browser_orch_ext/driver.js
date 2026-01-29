import { chromium } from 'playwright';
import { spawn } from 'child_process';
import { find_chrome_executable } from "find-chrome-executable";

const STATE_JS = `(
    () => {
        const getState = () => {
            const a = Array.from(document.querySelectorAll("*[data-r]"));
            return {
                elements: a.map((x) => {
                    const r = x.getAttribute("data-r");
                    const bounding = x.getBoundingClientRect();
                    const isButton = x.tagName === "BUTTON" || x.type === "button" || x.type === "submit";
                    return {
                        attributes: Array.from(x.attributes).map((x) => [x.name, x.value]),
                        // r value of the element
                        r: r,
                        // Name of the element (for buttons)
                        name: isButton ? x.innerText : x.getAttribute("aria-label") ?? x.getAttribute("alt") ?? x.placeholder ?? "",
                        // Various metadata about the element that is useful for the model
                        metadata: {
                            tagName: x.tagName,
                            type: x.type,
                            hasValue: x.value != null && x.value !== "",
                            isChecked: x.checked ?? false,
                            isDisabled: x.disabled ?? false,
                            isRquired: x.required ?? false,
                            isReadOnly: x.readOnly ?? false,
                        },
                    };
                }),
                viewport: {
                    x: window.scrollX,
                    y: window.scrollY,
                    width: window.innerWidth,
                    height: window.innerHeight,
                    scale: window.devicePixelRatio,
                },
            };
        }
        return getState();
    }
)()`

export class Driver {
    constructor(executable_path, driver_type) {
        this.executable_path = executable_path;
        this.driver_type = driver_type;
        this.process = null;
        this.browser = null;
        this.page = null;
    }

    async start() {
        if (this.driver_type === 'playwright') {
            this.browser = await chromium.launch({ headless: true, executablePath: this.executable_path });
            this.page = await this.browser.newPage();
        } else {
            this.process = spawn(this.executable_path, [
                '--remote-debugging-port=9222',
                '--user-data-dir=/tmp/chromium',
            ]);
        }
    }

    async stop() {
        if (this.driver_type === 'playwright') {
            await this.browser.close();
        } else {
            this.process.kill();
        }
    }

    async handle_action(action) {
        // Only playwright driver type supports page actions
        if (this.driver_type !== 'playwright' || !this.page) {
            return { type: "Error", error: "Action not supported for this driver type" };
        }

        switch (action.action) {
            case 'Navigate':
                await this.page.goto(action.url);
                return { type: "Complete" };
            case 'State':
                const state = await this.page.evaluate(STATE_JS);
                return { type: "State", ...state };
            case 'Hover':
                await this.page.hover(`*[data-r="${action.i}"]`);
                return { type: "Complete" };
            case 'Click':
                await this.page.click(`*[data-r="${action.i}"]`);
                return { type: "Complete" };
            case 'Type':
                await this.page.type(`*[data-r="${action.i}"]`, action.text);
                return { type: "Complete" };
            case 'Scroll':
                await this.page.evaluate(({ x, y }) => {
                    window.scrollBy(x, y);
                }, { x: action.x, y: action.y });
                return { type: "Complete" };
            case 'Select':
                await this.page.selectOption(`*[data-r="${action.i}"]`, action.value);
                return { type: "Complete" };
            default:
                return { type: "Error", error: "ElementNotFound" };
        }
    }
}

export function find_executable() {
    return find_chrome_executable();
}
