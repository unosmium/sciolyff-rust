const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('tbody tr')];
const enCollator = new Intl.Collator('en');

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('td:nth-child(3)')];

////////////////////////////////////////////////////////////////////////////////

function compareTeamRank(rowA, rowB) {
  let rankA = teamInfo[rowA.id][2];
  let rankB = teamInfo[rowB.id][2];
  return rankA - rankB;
}

function compareRankInEvent(eventIndex) {
  return function(rowA, rowB) {
    let rankA = placingInfo[`${rowA.id}e${eventIndex}`].order;
    let rankB = placingInfo[`${rowB.id}e${eventIndex}`].order;
    return rankA - rankB;
  };
}

function compareTeamNumber(rowA, rowB) {
  let numA = parseInt(rowA.querySelector('td:nth-child(1)').textContent);
  let numB = parseInt(rowB.querySelector('td:nth-child(1)').textContent);
  return numA - numB;
}

function compareTeamSchool(rowA, rowB) {
  let schoolA = teamInfo[rowA.id][0];
  let schoolB = teamInfo[rowB.id][0];
  return enCollator.compare(schoolA, schoolB);
}

function compareTeamState(rowA, rowB) {
  let stateA = teamInfo[rowA.id][1];
  let stateB = teamInfo[rowB.id][1];
  return enCollator.compare(stateA, stateB);
}

function sortTableBy(comparisonFunction) {
  rows.sort(comparisonFunction);
  for (let row of rows) {
    tbody.insertAdjacentElement('beforeend', row);
  }
}

const optionToFunctionMap = {
  'by Number': compareTeamNumber,
  'by School': compareTeamSchool,
  'by State': compareTeamState,
}

function sortTable(option) {
  if (option === 'by Rank') {
    let eventIndex = parseInt(focusSelect.value);

    if (eventIndex === 0) {
      sortTableBy(compareTeamRank);
    } else {
      sortTableBy(compareRankInEvent(eventIndex));
    }
  }
  sortTableBy(optionToFunctionMap[option]);
}

sortSelect.addEventListener('change', (e) => sortTable(e.target.value));

///////////////////////////////////////////////////////////////////////////////

function focusOnEvent(eventIndex) {
  if (eventIndex === 0) {
    focusHeader.style.cssText = '';
    focusHeader.style.width = '';
    focusHeader.innerHTML = '';
    focusColumn.forEach((td, index) => td.innerHTML = '');

    if (sortSelect.value === 'by Rank') {
      sortTable('by Rank');
    }
    return;
  }

  focusHeader.style.cssText =
    'width:4em; text-indent:-16em; text-align:right; padding-right:0.5em;';

  let col = eventIndex + 5;
  let eventHeader = document.querySelector(`th:nth-child(${col})`);

  focusHeader.innerHTML = eventHeader.innerHTML;
  focusColumn.forEach((td) => {
    tdEvent = td.parentElement.querySelector(`td:nth-child(${col})`);
    td.innerHTML = tdEvent.innerHTML;
  });

  if (sortSelect.value === 'by Rank') {
    sortTable('by Rank');
  }
}

focusSelect.addEventListener('change', (e) => {
  focusOnEvent(parseInt(e.target.value));
});
