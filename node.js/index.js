const fs = require('node:fs');
const spawn = require('child_process').spawn;
const os = require('os');

let badAppleSong;
let isRunning = true;

process.on('SIGINT', cleanExit);
process.on('SIGTERM', cleanExit);
process.on('exit', function() {
    if (badAppleSong) {
        badAppleSong.kill();
    }
});

function cleanExit() {
    isRunning = false;
    if (badAppleSong) {
        badAppleSong.kill();
    }
    process.exit();
}

console.clear();
badApple();

function badApple() {
    const prefix = "../common/res/out";
    const sufix = ".jpg.txt";
    const maxFrame = 6572;

    let curFrame = 1;

    /**
    * Esse é o código que toca a música durante a animação.
    * Ele checa se o usuário está usando windows, porquê usa Powershell e objetos do Windows pra tocar a música.
    * 
    * @var __dirname é o caminho para a pasta onde está esse arquivo, 
    * assim o caminho estará sempre certo, mesmo que a pasta seja mudada de lugar, já que o objeto requer o caminho absoluto.
    */
    if (os.platform() === "win32") {
        badAppleSong = spawn("powershell", [`($mediaPlayer = New-Object System.Media.SoundPlayer('${__dirname}\\..\\common\\res\\badapple.wav')).PlaySync()`]);
    }

    function playAnimation() {
        console.time('Animation Execution Time');
        if (!isRunning) return;

        /**
        * On previous versions, I used to clear the screen every time a new frame was played.
        * Setting the cursor position to the top left instead overwrites every frame with the new one
        * and is a major performance improvement, as clearing the screen is an expensive operation.
        * 
        * For comparison, the animation, when running in unlimited fps finished at 50.3 seconds
        * using the screen-clearing version, and 0.8 seconds when setting the cursor position instead.
        */
        //process.stdout.write('\x1Bc');
        setCursorPosition(0,0);

        try {
            const txt = fs.readFileSync(`${prefix}${format(curFrame)}${sufix}`, 'utf8');
            console.log(txt);
        } catch (e) {
            console.error("Error reading file:", e);
        }

        curFrame++;
        if (curFrame < maxFrame && isRunning) {
            setTimeout(playAnimation, 33); // Schedule next frame
            //playAnimation(); //For performance testing purposes
        } else {
            console.timeEnd('Animation Execution Time');
            process.exit(); // Exit when animation completes or if isRunning is false
        }
    }

    playAnimation();

    /**
     * Função que pega um número e converte ele para uma string com 4 dígitos.
     * @param {Number} n O número que será formatado
     * @returns {String} O número em 4 dígitos
     */
    function format(n) {
        var s = "000" + n;
        return s.substr(s.length - 4);
    }
}

function setCursorPosition(x, y) {
    process.stdout.write(`\x1B[${y};${x}H`);
}
