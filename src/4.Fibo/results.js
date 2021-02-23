import { testRunner } from '../../test-runner.js';
import { fibA, fibB, fibC, fibD } from './code.js';

/**
 * Рекурсивный фибоначи, нереально долго, уже 100ый элемент не стал ждать
 *
 * Test #0   [OK]  time: 0:00.000
 * Test #1   [OK]  time: 0:00.000
 * Test #2   [OK]  time: 0:00.000
 * Test #3   [OK]  time: 0:00.000
 * Test #4   [OK]  time: 0:00.000
 * Test #5   [OK]  time: 0:00.001
 * Test #6   [OK]  time: 0:00.000
 */
// testRunner(import.meta.url, (inp) => {
//   return fibA(+inp.trim());
// });

/**
 * Итеративно, до 1_000_000 еще терпимо
 *
 * Test #9   [OK]  time: 0:00.003
 * Test #10  [OK]  time: 0:00.097
 * Test #11  [OK]  time: 0:09.310
 * Test #12  [OK]  time: 36:05.904
 */
// testRunner(import.meta.url, (inp) => {
//   return fibB(+inp.trim());
// });

/**
 * По формуле золотого сечения, моментально, но не точно
 *
 * Test #0   [OK]  time: 0:00.001
 * Test #1   [OK]  time: 0:00.000
 * Test #2   [OK]  time: 0:00.000
 * Test #3   [OK]  time: 0:00.000
 * Test #4   [OK]  time: 0:00.000
 * Test #5   [OK]  time: 0:00.000
 * Test #6   [OK]  time: 0:00.000
 * Test #7  [FAIL] time: 0:00.000
 * Test #8  [FAIL] time: 0:00.000
 * Test #9   [OK]  time: 0:00.000
 * Test #10  [OK]  time: 0:00.000
 * Test #11  [OK]  time: 0:00.000
 * Test #12  [OK]  time: 0:00.000
 */
// testRunner(import.meta.url, (inp) => {
//   return fibC(+inp.trim());
// });

/**
 * Умножение матриц, результат точный, скорость терпимая
 * Test #9   [OK]  time: 0:00.000
 * Test #10  [OK]  time: 0:00.017
 * Test #11  [OK]  time: 0:01.887
 * Test #12  [OK]  time: 8:12.116
 */
// testRunner(import.meta.url, (inp) => {
//   return fibD(+inp.trim());
// });
