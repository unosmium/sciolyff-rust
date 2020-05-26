const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('tbody tr')];
const enCollator = new Intl.Collator('en');

function compareTeamNumber(rowA, rowB) {
  let numA = parseInt(rowA.querySelector('td:nth-child(1)').textContent);
  let numB = parseInt(rowB.querySelector('td:nth-child(1)').textContent);
  return numA - numB;
}

function compareTeamName(rowA, rowB) {
  let nameA = rowA.querySelector('td:nth-child(2)').textContent;
  let nameB = rowB.querySelector('td:nth-child(2)').textContent;
  return enCollator.compare(nameA, nameB);
}

function compareTeamRank(rowA, rowB) {
  let rankA = parseInt(rowA.querySelector('td:nth-child(3)').textContent);
  let rankB = parseInt(rowB.querySelector('td:nth-child(3)').textContent);
  return rankA - rankB;
}

function sortTableBy(comparisonFunction) {
  rows.sort(comparisonFunction);
  for (let row of rows) {
    tbody.insertAdjacentElement('beforeend', row);
  }
}
