
function frequencyChanges(changes) {
  return changes.map(change => {
    let x = parseInt(change.substr(1));
    return change.charAt(0) === '-' ? -x : x;
  }).reduce((a, b) => a + b, 0);
}

module.exports = { frequencyChanges }