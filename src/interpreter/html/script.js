const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('main tbody tr')];
const enCollator = new Intl.Collator('en');

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('main th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('main td:nth-child(3)')];
const teamPenaltiesIndex =
  parseInt(focusSelect.querySelector('option:last-child').value);

const thead = document.querySelector('thead');
const close = document.getElementById('close');

const modal = document.querySelector('div.smith section');
const modalTeamNumber = modal.querySelector('h1');
const modalTeamName = modal.querySelector('p');
const modalOverall = modal.querySelector('td:nth-child(2)');
const modalColumn = [...modal.querySelectorAll('td:nth-child(2)')].slice(1);

const modalBody = modal.querySelector('#liver');
const modalNav = modal.querySelector('nav');
const modalArticle = modal.querySelector('article');

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
  if (e.target.tagName !== 'A' && e.target.closest('td').cellIndex <= 2) {
    e.target.closest('tr').querySelector('a').click();
  }
});

function closeModal() {
  location.hash = '';
  history.replaceState(null, '', location.href.slice(0, -1));
}

window.addEventListener('click', (e) => {
  if (e.target.className === 'smith') {
    closeModal()
  }
});

close.addEventListener('click', (e) => closeModal());

///////////////////////////////////////////////////////////////////////////////

function populateModal() {
  let hashString = location.hash.substring(1);
  if (hashString === '') { return; }

  let smith = document.getElementById(hashString);
  if (smith === null || smith.className !== 'smith') { return; }

  let teamNumber = parseInt(hashString);
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
  modalBody.scrollLeft = 0;
  modalBody.scrollTop = 0;
}

window.addEventListener('hashchange', () => populateModal());

populateModal();

///////////////////////////////////////////////////////////////////////////////


function animateHorizontalScroll() {
  let scrollLeftMax = modalBody.scrollWidth - modalBody.clientWidth;
  let msDuration = 150;
  let start;

  function zoop(timestep) {
    if (start === undefined) { start = timestep; }
    let elapsed = timestep - start;

    let t = (elapsed / msDuration);
    let y = t<0.5 ? 2*t*t : -1+(4-2*t)*t;

    modalBody.scrollLeft = y * scrollLeftMax;

    if (elapsed < msDuration) {
      window.requestAnimationFrame(zoop);
    } else {
      modalBody.scrollLeft = scrollLeftMax + 100;
    }
  }

  window.requestAnimationFrame(zoop);
}

modalNav.addEventListener('click', (e) => {
  modalArticle.scrollTop = 0;
  modalArticle.innerHTML = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas ut odio lectus. Cras eu risus at diam feugiat mattis at eget mi. Mauris id dignissim libero. Integer interdum nisi et fermentum sollicitudin. Vestibulum sed urna pharetra metus fermentum feugiat. Fusce vitae tincidunt augue. Duis ultrices viverra mi, et gravida tellus hendrerit ac. Donec rhoncus ornare porttitor. Mauris viverra posuere tellus, vitae rutrum nunc sagittis nec. Sed vehicula posuere eros vel semper. Fusce turpis arcu, fermentum a neque at, bibendum lobortis diam. Suspendisse potenti. Fusce iaculis viverra turpis sed iaculis. Aliquam erat volutpat. Vivamus rhoncus, tellus at rhoncus faucibus, mauris velit tincidunt lorem, sit amet vehicula risus felis vitae urna. Duis vulputate risus at lacus scelerisque, vitae porttitor purus sodales. Curabitur faucibus hendrerit consequat. Vestibulum non massa ante. In suscipit faucibus sapien eget rutrum. Vestibulum vel nulla urna. Ut nec turpis purus. Cras congue diam eu urna malesuada mattis. Cras vitae commodo velit. Maecenas ac imperdiet dui. Proin malesuada facilisis nibh, ac efficitur nulla feugiat a. Aenean euismod vitae lorem eu ullamcorper. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam posuere vestibulum neque, id condimentum ex gravida sed. Donec tellus urna, pretium vel iaculis in, suscipit sit amet velit. Aenean sed sagittis augue, placerat blandit odio. Nunc tincidunt vitae lacus ut ornare. Quisque viverra ex eget dui volutpat sodales. Curabitur varius tempus facilisis. Nunc vulputate arcu felis, at feugiat nulla convallis in. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nam blandit ac diam eget lacinia. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. In dignissim dignissim euismod. Integer sem neque, tempus venenatis laoreet sed, iaculis aliquam orci. Suspendisse potenti. Quisque tortor nisi, pellentesque ut commodo a, pulvinar vitae nisi. Donec tristique tempus est, a imperdiet libero consectetur ac. Vivamus ac bibendum enim, sit amet volutpat lacus. Quisque id egestas ex. Curabitur sit amet ultricies felis, at finibus justo. In blandit commodo erat nec ornare. Vivamus in ante in dui posuere venenatis venenatis et felis. Sed et orci at lacus eleifend ornare. Fusce quis nibh et nunc auctor dapibus vitae vel elit.'
  animateHorizontalScroll();
});
