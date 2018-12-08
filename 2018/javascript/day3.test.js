import test from 'ava';
const DATA = require('./day3Data');

class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
  toString() {
    return `${this.x},${this.y}`
  }
}

function point(pointStr) {
  let [x, y] = pointStr.split(',');
  return createPoint(parseInt(x), parseInt(y));
}

function createPoint(x, y) {
  return new Point(x, y);
}

function nextX(point) {
  return createPoint(point.x + 1, point.y);
}

function nextY(point) {
  return createPoint(point.x, point.y + 1);
}

function extendX(point, length = 1) {
  let points = [];
  for (var i = 1; i < length; i++) {
    let previousPoint = points.length == 0 ? point : points[points.length - 1];
    points.push(nextX(previousPoint));
  }
  return points;
}

function extendY(point, length = 1) {
  let points = [];
  for (var i = 1; i < length; i++) {
    let previousPoint = points.length == 0 ? point : points[points.length - 1];
    points.push(nextY(previousPoint));
  }
  return points;
}

function box(ident) {
  var [p, extensions] = ident.split(': ');
  var init = point(p);
  var [xLength, yLength] = extensions.split('x');
  var xs = extendX(init, xLength);
  xs.unshift(init);
  var ys = xs.map(x => extendY(x, yLength)).reduce((acc, curr) => acc.concat(curr), []);
  var box = xs.concat(ys);
  return box;
}

// ended up being unused because it was way to slow to compare each and every box
function intersection(boxA, boxB) {
  return boxA.filter(pointA => {
    if (boxB.includes(pointA)) {
      return true;
    }
    return boxB.filter(pointB => pointB.toString() === pointA.toString()).length > 0;
  });
}

function claim(str) {
  let split = str.split(' @ ');
  return {
    id: split[0],
    box: box(split[1])
  };
}

function buildCanvas(claims) {
  let canvas = new Map();
  for (var c of claims) {
    for (var point of c.box) {
      let key = point.toString();
      if (!canvas.has(key)) {
        canvas.set(key, []);
      }
      canvas.get(key).push(c.id);
    }
  }
  return canvas;
}

function overlap(claimStrs) {
  let claims = claimStrs.map(claim);
  let canvas = buildCanvas(claims);

  var inches = 0;
  for (var entry of canvas) {
    if (entry[1].length > 1) {
      inches++;
    }
  }
  return inches;
}

test('point 1,3 returns x = 1 and y = 3', t => {
  t.deepEqual(point('1,3'), new Point(1, 3));
});

test('point 4,7 returns x = 4 and y = 7', t => {
  t.deepEqual(point('4,7'), new Point(4, 7));
});

test('nextX given point 1,3 returns 2,3', t => {
  let result = nextX(point('1,3'));
  t.deepEqual(result, new Point(2, 3));
});

test('nextY given point 1,3 returns 1,4', t => {
  let result = nextY(point('1,3'));
  t.deepEqual(result, new Point(1, 4));
});

test('extendX given point 1,3 returns empty', t => {
  let result = extendX(point('1,3'));
  t.deepEqual(result, []);
});

test('extendX given point 1,3 and 2 returns 2,3', t => {
  let result = extendX(point('1,3'), 2);
  t.deepEqual(result, [point('2,3')]);
});

test('extendX given point 2,3 and 3 returns 3,3 4,3', t => {
  let result = extendX(point('2,3'), 3);
  t.deepEqual(result, [point('3,3'), point('4,3')]);
});

test('extendY given point 1,3 returns empty', t => {
  let result = extendY(point('1,3'));
  t.deepEqual(result, []);
});

test('extendY given point 1,3 and 2 returns 1,4', t => {
  let result = extendY(point('1,3'), 2);
  t.deepEqual(result, [point('1,4')]);
});

test('extendY given point 2,3 and 3 returns 2,4 2,5', t => {
  let result = extendY(point('2,3'), 3);
  t.deepEqual(result, [point('2,4'), point('2,5')]);
});

test('box given 1,3: 2x2 returns 1,3 2,3 1,4 2,4', t => {
  let result = box('1,3: 2x2');
  t.deepEqual(result, [point('1,3'), point('2,3'), point('1,4'), point('2,4')])
});

test('box given 3,1: 4x4', t => {
  let result = box('3,1: 4x4').sort();
  let expected = [
    point('3,1'), point('3,2'), point('3,3'), point('3,4'),
    point('4,1'), point('4,2'), point('4,3'), point('4,4'),
    point('5,1'), point('5,2'), point('5,3'), point('5,4'),
    point('6,1'), point('6,2'), point('6,3'), point('6,4'),
  ].sort();
  t.deepEqual(result, expected);
});

test('intersection', t => {
  let result = intersection(box('1,3: 4x4'), box('3,1: 4x4'))
  t.deepEqual(result,
    [
      point('3,3'), point('4,3'),
      point('3,4'), point('4,4')
    ]
  );
});

test('overlap', t => {
  let result = overlap(['#1 @ 1,3: 4x4', '#2 @ 3,1: 4x4', '#3 @ 5,5: 2x2']);
  t.is(result, 4);
});

test('part1', t => {
  let result = overlap(DATA);
  t.is(result, 111935);
});

test('part 2', t => {
  let claims = DATA.map(claim);
  let result = buildCanvas(claims);
  let ids = claims.map(c => c.id);
  let badIds = new Set();
  for (var entry of result) {
    if (entry[1].length > 1) {
      entry[1].forEach(id => badIds.add(id))
    }
  }
  let goodIds = ids.filter(id => !badIds.has(id));
  t.deepEqual(goodIds, ['#650']);
});