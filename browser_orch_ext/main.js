import {ฝtool_code
print(default_api.Driver,ฝtool_code
print(default_api.find_executable)ฝtool_code
print(default_api.from './driver';

const DRIVER_TYPE = process.env.DRIVER_TYPE || 'playwright';

async function main() {
    const driver = new Driver(find_executable(), DRIVER_TYPE);
    await driver.start();

    process.on("message", async (message) => {
        const response = await driver.handle_action(message);
        process.send(response);
    });

    process.on("exit", () => {
        driver.stop();
    });
}

main();
