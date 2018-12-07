import test from 'ava';

function point(pointStr) {
  let split = pointStr.split(',');
  return { x: parseInt(split[0]), y: parseInt(split[1]) };
}

function nextX(point) {
  return {
    x: point.x + 1,
    y: point.y
  };
}

function nextY(point) {
  return {
    x: point.x,
    y: point.y + 1
  };
}

function extendX(point, length = 1) {
  let points = [point];
  for (var i = 1; i < length; i++) {
    points.push(nextX(points[points.length - 1]));
  }
  return points;
}

function extendY(point, length = 1) {
  let points = [point];
  for (var i = 1; i < length; i++) {
    points.push(nextY(points[points.length - 1]));
  }
  return points;
}

function box(ident) {
  var [p, extensions] = ident.split(': ');
  var init = point(p);
  var [xLength, yLength] = extensions.split('x');
  var xs = extendX(init, xLength);
  var ys = extendY(init, yLength);
  return xs.concat(ys);
}

test('point 1,3 returns x = 1 and y = 3', t => {
  t.deepEqual(point('1,3'), { x: 1, y: 3 });
});

test('point 4,7 returns x = 4 and y = 7', t => {
  t.deepEqual(point('4,7'), { x: 4, y: 7 });
});

test('nextX given point 1,3 returns 2,3', t => {
  let result = nextX(point('1,3'));
  t.deepEqual(result, { x: 2, y: 3 });
});

test('nextY given point 1,3 returns 1,4', t => {
  let result = nextY(point('1,3'));
  t.deepEqual(result, { x: 1, y: 4 });
});

test('extendX given point 1,3 returns only 1,3', t => {
  let result = extendX(point('1,3'));
  t.deepEqual(result, [{ x: 1, y: 3 }]);
});

test('extendX given point 1,3 and 2 returns 1,3 and 2,3', t => {
  let result = extendX(point('1,3'), 2);
  t.deepEqual(result, [point('1,3'), point('2,3')]);
});

test('extendX given point 2,3 and 3 returns 2,3 3,3 4,3', t => {
  let result = extendX(point('2,3'), 3);
  t.deepEqual(result, [point('2,3'), point('3,3'), point('4,3')]);
});

test('extendY given point 1,3 returns only 1,3', t => {
  let result = extendY(point('1,3'));
  t.deepEqual(result, [{ x: 1, y: 3 }]);
});

test('extendY given point 1,3 and 2 returns 1,3 and 1,4', t => {
  let result = extendY(point('1,3'), 2);
  t.deepEqual(result, [point('1,3'), point('1,4')]);
});

test('extendY given point 2,3 and 3 returns 2,3 2,4 2,5', t => {
  let result = extendY(point('2,3'), 3);
  t.deepEqual(result, [point('2,3'), point('2,4'), point('2,5')]);
});

test('box given 1,3: 2x2 returns 1,3 2,3 1,4 2,4', t => {
  let result = box('1,3: 2x2');
  console.log(result);
  t.deepEqual(result, [point('1,3'), point('2,3'), point('1,4'), point('2,4')])
});