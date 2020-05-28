const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('tbody tr')];
const enCollator = new Intl.Collator('en');

function compareTeamRank(rowA, rowB) {
  let rankA = parseInt(rowA.querySelector('td:nth-child(4)').textContent);
  let rankB = parseInt(rowB.querySelector('td:nth-child(4)').textContent);
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

///////////////////////////////////////////////////////////////////////////////

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('td:nth-child(3)')];

focusSelect.addEventListener('change', (e) => {
  let col = parseInt(e.target.value);

  if (col === 0) {
    focusHeader.style.cssText = '';
    focusHeader.style.width = '';
    focusHeader.innerHTML = '';
    focusColumn.forEach((td, index) => td.innerHTML = '');
    return;
  }

  focusHeader.style.cssText = 'width:4em;direction:rtl;padding-right:0.5em;';

  let eventHeader = document.querySelector(`th:nth-child(${col + 5})`);
  let eventColumn = [...document.querySelectorAll(`td:nth-child(${col + 5})`)];

  focusHeader.innerHTML = eventHeader.innerHTML;
  focusColumn.forEach((td, index) => {
    td.innerHTML = eventColumn[index].innerHTML;
  });
});
