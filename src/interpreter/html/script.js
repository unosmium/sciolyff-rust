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

let modalOpenedByUser = false;
const modalBg = document.getElementById('smith');
const modal = document.querySelector('div#smith section');
const modalTeamNumber = modal.querySelector('h2 span');
const modalTeamName = modal.querySelector('p');
const modalOverall = modal.querySelector('td:nth-child(2)');
const modalColumn = [...modal.querySelectorAll('td:nth-child(2)')].slice(1);

const modalBody = modal.querySelector('#liver');
const modalNav = modal.querySelector('nav');
const modalArticle = modal.querySelector('article');
const modalBack = modalArticle.querySelector('button');
const modalH3 = modalArticle.querySelector('h3');
const modalP = modalArticle.querySelector('p');
const modalOverallInfo = modalArticle.querySelector('#overallInfo');
const modalPlacingInfo = modalArticle.querySelector('#placingInfo');
const mdDeetz = [...modalArticle.querySelectorAll('dd')];

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

sortSelect.addEventListener('change', (e) => {
  sortTable(e.target.value);
  pushQueryState(null, e.target.value);
});

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
  let eventIndex = parseInt(e.target.value);
  focusOnEvent(eventIndex);
  pushQueryState(eventIndex, null);
});

///////////////////////////////////////////////////////////////////////////////

thead.addEventListener('click', (e) => {
  let col = e.target.closest('th').cellIndex;
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
  if (e.target.closest('td').cellIndex < 5) {
    if (e.target.tagName !== 'A') {
      e.target.closest('tr').querySelector('a').click();
    }
    modalOpenedByUser = true;
  }
});

function closeModal() {
  if (modalOpenedByUser) {
    history.back();
  } else {
    location.hash = '';
    history.replaceState(null, '', location.href.slice(0, -1));
  }
}

window.addEventListener('click', (e) => {
  if (e.target === modalBg) {
    closeModal()
  }
});

close.addEventListener('click', (e) => closeModal());

///////////////////////////////////////////////////////////////////////////////

function populateModal(teamNumber) {
  let row = document.getElementById(`t${teamNumber}`);
  let rowOverall = row.querySelector('td:nth-child(4)');
  let info = teamInfo[`t${teamNumber}`];

  modalTeamNumber.innerHTML = teamNumber;
  modalTeamName.innerHTML = `${info.name} <small>${info.location}</small>`;
  modalOverall.innerHTML = rowOverall.innerHTML;
  modalOverall.className = rowOverall.className;

  modalColumn.forEach((td, i) => {
    let tdEvent = row.querySelector(`td:nth-child(${i + 6})`);
    td.innerHTML = tdEvent.innerHTML;
    td.className = tdEvent.className;
  });
}

function updateModalState() {
  let hashString = location.hash.substring(1);

  if (hashString === '' || document.getElementById(`t${hashString}`) === null) {
    modalOpenedByUser = false;
    smith.className = '';
  } else {
    populateModal(parseInt(hashString));
    modalBody.scrollLeft = 0;
    modalNav.scrollTop = 0;
    smith.className = 'visible';
  }
}

window.addEventListener("beforeunload", () => smith.className = '');
window.addEventListener('hashchange', () => updateModalState());

updateModalState();

///////////////////////////////////////////////////////////////////////////////

function animateHorizontalScroll(reverse) {
  let scrollLeftMax = modalBody.scrollWidth - modalBody.clientWidth;

  function donzo() {
    if (reverse) {
      modalBody.scrollLeft = 0;
    } else {
      modalBody.scrollLeft = scrollLeftMax + 100;
    }
  }

  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
    donzo();
    return;
  }

  let msDuration = 200;
  let start;

  function zoop(timestep) {
    if (start === undefined) { start = timestep; }
    let elapsed = timestep - start;

    let t = (elapsed / msDuration);
    let y = t<0.5 ? 2*t*t : -1+(4-2*t)*t;

    if (reverse) {
      modalBody.scrollLeft = (1-y) * scrollLeftMax;
    } else {
      modalBody.scrollLeft = y * scrollLeftMax;
    }

    if (elapsed < msDuration) {
      window.requestAnimationFrame(zoop);
    } else {
      donzo();
    }
  }

  window.requestAnimationFrame(zoop);
}

function getOrdinal(n) {
  let s = ["th", "st", "nd", "rd"],
  v = n%100;
  return n+(s[(v-20)%10]||s[v]||s[0]);
}

function populateOverall(teamNumber) {
  let team = teamInfo[`t${teamNumber}`];
  let tournamentName = document.querySelector('h1').textContent;
  if (team.exhibition) {
    modalP.innerHTML = `
    At the ${tournamentName}, Team ${teamNumber} participated as in
    <b>${team.events_participated} events</b> and scored <b>${team.points}
    points</b>. As an <b>exhibition team</b>, they did not affect the final team
    rankings.
    `
  } else if (team.disqualified) {
    modalP.innerHTML = `
    At the ${tournamentName}, Team ${teamNumber} participated in
    <b>${team.events_participated} events</b> and scored <b>${team.points}
    points</b>, but were <b>disqualified</b> from the final team rankings.
    `
  } else {
    modalP.innerHTML = `
    At the ${tournamentName}, Team ${teamNumber} participated in
    <b>${team.events_participated} events</b> and scored <b>${team.points}
    points</b>, ranking them <b>${getOrdinal(team.rank)} out of
    ${nonexhibitionTeamCount}</b> competing teams.
    `
  }
}

function populatePenalties(teamNumber) {
  let team = teamInfo[`t${teamNumber}`];

  if (team.penalties === 0) {
    modalP.innerHTML = `
    Team ${teamNumber} did not recieve any team penalties at this competition.
    `;
  } else {
    modalP.innerHTML = `
    Team ${teamNumber}'s team penalties at this competition added
    <b>${team.penalties} points</b> to their total score.
    `;
  }
}

function populatePlacing(eventName, eventIndex, teamNumber) {
  let placing = placingInfo[`t${teamNumber}e${eventIndex}`];
  if (placing.disqualified) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} were <b>disqualified</b> from the event
    ${eventName}, adding <b>${placing.points} points</b> toward their team's
    point total.
    `;
  } else if (placing.did_not_participate) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} <b>did not participate</b> in the event
    ${eventName}, adding <b>${placing.points} points</b> toward their team's
    point total.
    `;
  } else if (placing.participation_only) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} earned <b>participation-only</b> points in
    the event ${eventName}, adding <b>${placing.points} points</b> toward their
    team's point total.
    `;
  } else {
    let placeText;
    if (placing.tie) {
      placeText = `tied for ${getOrdinal(placing.place)}`;
    } else if (placing.unknown) {
      placeText = 'unknown';
    } else {
      placeText = getOrdinal(placing.place);
    }
    modalP.innerHTML = `
    Students from Team ${teamNumber} placed <b>${placeText} out of
    ${eventParticipationCounts[eventIndex-1]}</b> participating teams in the
    event ${eventName}, earning <b>${placing.points} point${placing.points === 1
        ? '' : 's'}</b> toward their team's point total.  `;
  }

  mdDeetz[0].innerHTML = placing.medal ? 'Yes':'No';
  mdDeetz[1].innerHTML = placing.exempt ? 'Yes':'No';
  mdDeetz[2].innerHTML = placing.dropped_as_part_of_worst_placings ? 'Yes':'No';
  mdDeetz[3].innerHTML = placing.points_limited_by_maximum_place ? 'Yes':'No';
  mdDeetz[4].innerHTML = placing.points_affected_by_exhibition ? 'Yes':'No';
  mdDeetz[5].innerHTML = placing.isolated_points;
}

function populateArticle(eventName, eventIndex, teamNumber) {
  modalH3.innerHTML = eventName;

  if (eventIndex === 0) {
    modalOverallInfo.style.display = 'block';
    modalPlacingInfo.style.display = 'none';

    populateOverall(teamNumber);

  } else if (eventIndex === teamPenaltiesIndex) {
    modalOverallInfo.style.display = 'none';
    modalPlacingInfo.style.display = 'none';

    populatePenalties(teamNumber);

  } else {
    modalOverallInfo.style.display = 'none';
    modalPlacingInfo.style.display = 'block';

    populatePlacing(eventName, eventIndex, teamNumber);
  }
}


modalNav.addEventListener('click', (e) => {
  let row = e.target.closest('tr');

  if (row) {
    let eventName = row.querySelector('td').innerHTML;
    let eventIndex = [...modalNav.querySelectorAll('tr')].indexOf(row);
    let teamNumber = parseInt(modalTeamNumber.innerHTML);

    populateArticle(eventName, eventIndex, teamNumber);
    modalArticle.scrollTop = 0;
    animateHorizontalScroll(false);
  }
});

modalBack.addEventListener('click', () => animateHorizontalScroll(true));

///////////////////////////////////////////////////////////////////////////////

function updateBasedOnQueryString() {
  let search = new URLSearchParams(location.search);

  let oldFocusVal = focusSelect.value;
  if (search.has('focus')) {
    focusSelect.value = search.get('focus');
  } else {
    focusSelect.value = 0;
  }
  if(oldFocusVal !== focusSelect.value) {
    focusOnEvent(parseInt(focusSelect.value));
  }

  let oldSortVal = sortSelect.value;
  if (search.has('sort')) {
    sortSelect.value = search.get('sort');
  } else {
    sortSelect.value = 'by Rank';
  }
  if(oldSortVal !== sortSelect.value) {
    sortTable(sortSelect.value);
  }
}

function pushQueryState(eventIndex, sortOption) {
  let newSearch = new URLSearchParams(location.search);
  if (eventIndex === 0) {
    newSearch.delete('focus');
  } else if (eventIndex !== null) {
    newSearch.set('focus', eventIndex);
  }

  if (sortOption === 'by Rank') {
    newSearch.delete('sort');
  } else if (sortOption !== null) {
    newSearch.set('sort', sortOption);
  }

  let newURL = new URL(location);
  newURL.search = newSearch;
  history.pushState({}, '', newURL);
}

window.onpopstate = updateBasedOnQueryString;
updateBasedOnQueryString();
