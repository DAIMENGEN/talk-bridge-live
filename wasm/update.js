import {exec} from "child_process";

function checkDependencyInstalled(dependency) {
    return new Promise((resolve) => {
        exec(`yarn list --pattern ${dependency}`, (err, stdout) => {
            resolve(!err && stdout.includes(dependency));
        });
    });
}

function removeDependencies(dependencies, devDependencies = []) {
    let removeCommand = "yarn remove ";
    removeCommand += dependencies.join(" ");

    if (devDependencies.length > 0) {
        removeCommand += " " + devDependencies.join(" ");
    }

    return new Promise((resolve, reject) => {
        exec(removeCommand, (err, stdout, stderr) => {
            if (err) {
                reject(`Error removing dependencies: ${stderr}`);
            } else {
                resolve(stdout);
            }
        });
    });
}

function installDependencies(dependencies, devDependencies = []) {
    let installCommand = "yarn add ";
    installCommand += dependencies.join(" ");

    if (devDependencies.length > 0) {
        installCommand += " --dev " + devDependencies.join(" ");
    }

    return new Promise((resolve, reject) => {
        exec(installCommand, (err, stdout, stderr) => {
            if (err) {
                reject(`Error installing dependencies: ${stderr}`);
            } else {
                resolve(stdout);
            }
        });
    });
}

async function updateDependencies() {
    try {
        const isInstalled = await checkDependencyInstalled("src-wasm");
        if (isInstalled) {
            await removeDependencies(["src-wasm"], []);
            console.log("Old src-wasm dependencies removed.");
        } else {
            console.log("Old src-wasm does not exist, skipping removal.");
        }
        await installDependencies(["src-wasm@file:./src-wasm/pkg"], []);
        console.log("New src-wasm dependencies installed.");
    } catch (error) {
        console.error(error);
    }
}

updateDependencies().then();
