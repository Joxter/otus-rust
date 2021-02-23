export function fibA(n) {
  if (n < 2) return n;
  return fibA(n - 1) + fibA(n - 2);
}

export function fibB(n) {
  if (n === 0) return '0';
  if (n === 1) return '1';

  let f0 = 0n;
  let f1 = 1n;
  let result;

  for (let i = 1; i < n; i++) {
    result = f1 + f0;

    f0 = f1;
    f1 = result;
  }

  return result;
}

export function fibC(n) {
  const f = (1 + Math.sqrt(5)) / 2;

  return Math.floor((f ** n) / Math.sqrt(5) + 0.5);
}

export function fibD(n) {
  if (n === 0) return '0';

  return powMatrix([
    [1n, 1n],
    [1n, 0n],
  ], n - 2)[0][0];
}


function mulMatrix(a, b) {
  return [
    [a[0][0] * b[0][0] + a[0][1] * b[1][0], a[0][0] * b[0][1] + a[0][1] * b[1][1]],
    [a[1][0] * b[0][0] + a[1][1] * b[1][0], a[1][0] * b[0][1] + a[1][1] * b[1][1]],
  ];
}


export function powMatrix(m, n) {
  let result = [
    [1n, 1n],
    [1n, 0n],
  ];

  for (let i = 1; n > 0; i += i) {
    if (n % 2 === 1) {
      result = mulMatrix(result, pow2Matrix(m, i));
    }
    n = Math.floor(n / 2);
  }

  return result;
}


function pow2Matrix(m, n) {
  if (n === 0) return [
    [1n, 1n],
    [1n, 0n],
  ];

  for (let i = 1; i < n; i += i) {
    m = mulMatrix(m, m);
  }

  return m;
}
