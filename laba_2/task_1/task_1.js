const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

function ask(question) {
    return new Promise(resolve => rl.question(question, resolve));
}

function longestOnes(s, K) {
    let left = 0;
    let zeroCount = 0;
    let maxLen = 0;

    for (let right = 0; right < s.length; right++) {
        if (s[right] === '0') zeroCount++;

        while (zeroCount > K) {
            if (s[left] === '0') zeroCount--;
            left++;
        }
        maxLen = Math.max(maxLen, right - left + 1);
    }
    return maxLen;
}
async function getBinaryString() {
    while (true) {
        const input = await ask("Введите строку из 0 и 1: ");
        const s = input.trim();
      
        if (!s) {
            console.log("Пустая строка. Попробуйте ещё раз.");
            continue;
        }
        if (/^[01]+$/.test(s)) return s;
        
        console.log("Ошибка! Только символы '0' и '1'. Повтор:");
    }
}
async function getK() {
    while (true) {
        const input = await ask("Введите K (сколько нулей можно заменить): ");
        const k = Number(input.trim());
        
        if (Number.isInteger(k) && k >= 0) return k;
        
        console.log("Ошибка! Введите целое неотрицательное число.");
    }
}
async function main() {
    console.log("=== Longest Ones ===\n");
    
    const s = await getBinaryString();
    const K = await getK();
    const result = longestOnes(s, K);
    
    console.log(`\n Ответ: ${result}`);
    rl.close();
}

main();