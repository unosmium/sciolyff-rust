const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('main tbody tr')];
const enCollator = new Intl.Collator('en');

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('td:nth-child(3)')];
const teamPenaltiesIndex =
  parseInt(focusSelect.querySelector('option:last-child').value);

const thead = document.querySelector('thead');

const modal = document.querySelector('div.smith section');
const modalTeamNumber = modal.querySelector('h1');
const modalTeamName = modal.querySelector('p');
const modalOverall = modal.querySelector('td:last-child');
const modalColumn = [...modal.querySelectorAll('td:last-child')].slice(1);

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

tbody.addEventListener('click', (e) => {
  if (e.target.tagName !== 'A') {
    e.target.closest('tr').querySelector('a').click();
  }
});

window.addEventListener('click', (e) => {
  if (e.target.className === 'smith') {
    location.hash = '';
    history.replaceState(null, '', location.href.slice(0, -1));
  }
});

///////////////////////////////////////////////////////////////////////////////

function populateModal() {
  let teamNumber = parseInt(location.hash.substring(1));
  let smith = document.getElementById(teamNumber);
  if (isNaN(teamNumber) || smith === null) { return; }

  let row = document.getElementById(`t${teamNumber}`);
  let rowOverall = row.querySelector('td:nth-child(4)');
  let info = teamInfo[`t${teamNumber}`];

  modalTeamNumber.innerHTML = `Team ${teamNumber}`;
  modalTeamName.innerHTML = `${info.name} <small>${info.location}</small>`;
  modalOverall.innerHTML = rowOverall.innerHTML;
  modalOverall.className = rowOverall.className;

  modalColumn.forEach((td, i) => {
    let tdEvent = row.querySelector(`td:nth-child(${i + 6})`);
    td.innerHTML = tdEvent.innerHTML;
    td.className = tdEvent.className;
  });

  smith.appendChild(modal);
}

window.addEventListener('hashchange', () => populateModal());

populateModal();
