const { performance } = require('perf_hooks');
const t0 = performance.now();

const wishlist = 'tMlsioaplnKlflgiruKanliaebeLlkslikkpnerikTasatamkDpsdakeraBeIdaegptnuaKtmteorpu' +
    'TaTtbtsesOHXxonibmksekaaoaKtrssegnveinRedlkkkroeekVtkekymmlooLnanoKtlstoepHrpeutdynfSneloietbol';

const cutInHalfAndSwitch = inputStr => 
    inputStr.slice(inputStr.length/2, inputStr.length) + inputStr.slice(0, inputStr.length/2);

const swapEveryOther = inputStr => {
    let swapped = '';
    for (let i = 0; i<inputStr.length; i+=2) {
        swapped += inputStr[i+1] + inputStr[i];
    }
    return swapped;
};

const swapLastThreeFirstThreeAndSoOn = inputStr => {
    let front = '';
    let back = '';
    for (let i = 0; i<inputStr.length/2; i += 3) {
        back = inputStr.slice(i, i+3) + back;
    }
    for (let i = inputStr.length; i>=(inputStr.length/2)+3; i -= 3) {
        front += inputStr.slice(i-3, i);
    }
    return front+back;
};

console.log(
    cutInHalfAndSwitch(
        swapEveryOther(
            swapLastThreeFirstThreeAndSoOn(
                wishlist))));

console.log(`Execution time: ${performance.now() - t0}`);