const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('tbody tr')];
const enCollator = new Intl.Collator('en');

function compareTeamRank(rowA, rowB) {
  let rankA = parseInt(rowA.querySelector('td:nth-child(3)').textContent);
  let rankB = parseInt(rowB.querySelector('td:nth-child(3)').textContent);
  return rankA - rankB;
}

function compareTeamNumber(rowA, rowB) {
  let numA = parseInt(rowA.querySelector('td:nth-child(1)').textContent);
  let numB = parseInt(rowB.querySelector('td:nth-child(1)').textContent);
  return numA - numB;
}

function compareTeamSchool(rowA, rowB) {
  let schoolA = teamSchoolsAndStates[rowA.id][0];
  let schoolB = teamSchoolsAndStates[rowB.id][0];
  return enCollator.compare(schoolA, schoolB);
}

function compareTeamState(rowA, rowB) {
  let stateA = teamSchoolsAndStates[rowA.id][1];
  let stateB = teamSchoolsAndStates[rowB.id][1];
  return enCollator.compare(stateA, stateB);
}

function sortTableBy(comparisonFunction) {
  rows.sort(comparisonFunction);
  for (let row of rows) {
    tbody.insertAdjacentElement('beforeend', row);
  }
}

const optionToFunctionMap = {
  'by Rank': compareTeamRank,
  'by Number': compareTeamNumber,
  'by School': compareTeamSchool,
  'by State': compareTeamState,
}

function sortTable(option) {
  sortTableBy(optionToFunctionMap[option]);
}

sortSelect.addEventListener('change', (e) => sortTable(e.target.value));
