const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('tbody tr')];
const enCollator = new Intl.Collator('en');

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('td:nth-child(3)')];
const teamPenaltiesIndex =
  parseInt(focusSelect.querySelector('option:last-child').value);

const thead = document.querySelector('thead');
const modal = document.querySelector('div.smith section');

////////////////////////////////////////////////////////////////////////////////

function compareTeamRank(rowA, rowB) {
  let rankA = teamInfo[rowA.id].rank;
  let rankB = teamInfo[rowB.id].rank;
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
  let schoolA = teamInfo[rowA.id].school;
  let schoolB = teamInfo[rowB.id].school;
  return enCollator.compare(schoolA, schoolB);
}

function compareTeamState(rowA, rowB) {
  let stateA = teamInfo[rowA.id].state;
  let stateB = teamInfo[rowB.id].state;
  return enCollator.compare(stateA, stateB);
}

function sortTableBy(comparisonFunction) {
  rows.sort(comparisonFunction);
  for (let row of rows) {
    tbody.appendChild(row);
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

    if (eventIndex === 0 || eventIndex === teamPenaltiesIndex) {
      sortTableBy(compareTeamRank);
    } else {
      sortTableBy(compareRankInEvent(eventIndex));
    }
  } else {
    sortTableBy(optionToFunctionMap[option]);
  }
}

sortSelect.addEventListener('change', (e) => sortTable(e.target.value));

///////////////////////////////////////////////////////////////////////////////

function focusOnEvent(eventIndex) {
  if (eventIndex === 0) {
    focusHeader.removeAttribute('id');
    focusHeader.innerHTML = '';
    focusColumn.forEach((td, index) => {
      td.innerHTML = '';
      td.className = '';
    });
  } else {
    let col = eventIndex + 5;
    let eventHeader = document.querySelector(`th:nth-child(${col})`);

    focusHeader.id = 'focused';
    focusHeader.innerHTML = eventHeader.innerHTML;
    focusColumn.forEach((td) => {
      let tdEvent = td.parentElement.querySelector(`td:nth-child(${col})`);
      td.innerHTML = tdEvent.innerHTML;
      td.className = tdEvent.className;
    });
  }

  if (sortSelect.value === 'by Rank') {
    sortTable('by Rank');
  }
}

focusSelect.addEventListener('change', (e) => {
  focusOnEvent(parseInt(e.target.value));
});

///////////////////////////////////////////////////////////////////////////////

thead.addEventListener('click', (e) => {
  let col = e.target.cellIndex;
  if (col === 0) {
    sortSelect.value = 'by Number';
  } else if (col === 1) {
    sortSelect.value = 'by School';
  } else if (col === 2) {
    sortSelect.value = 'by Rank';
  } else if (col === 3 | col === 4) {
    focusSelect.value = 0;
    sortSelect.value = 'by Rank';
  } else if (col > 4) {
    focusSelect.value = (col - 4).toString();
  }
  focusSelect.dispatchEvent(new Event('change'));
  sortSelect.dispatchEvent(new Event('change'));
});

///////////////////////////////////////////////////////////////////////////////

function populateModal() {
  let teamNumber = location.hash.substring(1);
  if (teamNumber === NaN) { return; }

  modal.removeAttribute('id');
  document.getElementById(teamNumber).appendChild(modal);
  // ensure CSS transition
  window.setTimeout(() => modal.id = 'loaded', 10);
}

window.addEventListener('hashchange', () => {
  console.log(location.hash);
  populateModal();
});

window.addEventListener('beforeunload', () => {
  modal.removeAttribute('id');
});

populateModal();
