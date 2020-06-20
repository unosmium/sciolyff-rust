const sortSelect = document.querySelector('#sort');
const tbody = document.querySelector('tbody');
const rows = [...document.querySelectorAll('main tbody tr')];
const enCollator = new Intl.Collator('en');

const focusSelect = document.querySelector('#focus');
const focusHeader = document.querySelector('main th:nth-child(3)');
const focusColumn = [...document.querySelectorAll('main td:nth-child(3)')];
const focusOverflow = [...document.querySelectorAll('main tr :nth-child(5)')];
const tableLinks = [...document.querySelectorAll('main table a')];
const teamPenaltiesIndex =
  parseInt(focusSelect.querySelector('option:last-child').value);

const thead = document.querySelector('thead');
const close = document.getElementById('close');
const wrapper = document.getElementById('subway');
const nonModalFocusables =
  document.querySelectorAll('#subway, #subway select, #subway a');

let currentModalTeamNumber = NaN;
let currentModalEventIndex = NaN;

const modalBg = document.getElementById('smith');
const modal = document.querySelector('div#smith section');
const modalTeamNumber = modal.querySelector('h2 span');
const modalTeamName = modal.querySelector('p');
const modalOverall = modal.querySelector('td:nth-child(2)');
const modalColumn = [...modal.querySelectorAll('td:nth-child(2) div')].slice(1);
const modalLinks = [...modal.querySelectorAll('td:nth-child(3) a')];

const modalBody = modal.querySelector('#liver');
const modalNav = modal.querySelector('nav');
const modalArticle = modal.querySelector('article');
const modalBack = modalArticle.querySelector('button');
const modalH3 = modalArticle.querySelector('h3');
const modalP = modalArticle.querySelector('p');
const modalOverallInfo = modalArticle.querySelector('#overallInfo');
const modalPlacingInfo = modalArticle.querySelector('#placingInfo');
const mdDeetz = [...modalArticle.querySelectorAll('dd')];
const rawDeetz = document.getElementById('rawDetails');

const firstTableFocusable = document.querySelector('main table a');
const firstModalNavFocusable = document.querySelector('nav a');

let overallChart;
let eventChart;
const eventChartContainer = document.querySelector('#placingInfo .ct-chart');

let animationsDisabled = false;
let startX;

////////////////////////////////////////////////////////////////////////////////

function compareTeamRank(rowA, rowB) {
  let rankA = teamInfo[rowA.id].rank;
  let rankB = teamInfo[rowB.id].rank;
  return rankA - rankB;
}

function compareRankInEvent(eventIndex) {
  return (rowA, rowB) => {
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

sortSelect.addEventListener('change', e => {
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
    if (window.matchMedia('(max-width: 28em)').matches) {
      focusOverflow.forEach(tag => { tag.style.display = '' });
    }
    tableLinks.forEach(a => {
      a.setAttribute('href', a.getAttribute('href').split('-')[0]);
    });
  } else {
    let col = eventIndex + 5;
    let eventHeader = document.querySelector(`th:nth-child(${col})`);

    focusHeader.id = 'focused';
    focusHeader.innerHTML = eventHeader.innerHTML;
    focusColumn.forEach(td => {
      let tdEvent = td.parentElement.querySelector(`td:nth-child(${col})`);
      td.innerHTML = tdEvent.innerHTML;
      td.className = tdEvent.className;
    });
    if (window.matchMedia('(max-width: 28em)').matches) {
      focusOverflow.forEach(tag => { tag.style.display = 'none' });
    }
    tableLinks.forEach(a => {
      let baseHash = a.getAttribute('href').split('-')[0];
      a.setAttribute('href', `${baseHash}-${eventIndex}`);
    });
  }

  if (sortSelect.value === 'by Rank') {
    sortTable('by Rank');
  }
}

focusSelect.addEventListener('change', e => {
  let eventIndex = parseInt(e.target.value);
  focusOnEvent(eventIndex);
  pushQueryState(eventIndex, null);
});

///////////////////////////////////////////////////////////////////////////////

thead.addEventListener('click', e => {
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

tbody.addEventListener('click', e => {
  if (e.target.closest('td').cellIndex < 5) {
    if (e.target.tagName !== 'A') {
      e.target.closest('tr').querySelector('a').click();
    }
  }
});

function closeModal() {
  location.hash = '';
  history.replaceState(null, '', location.href.slice(0, -1));
}

window.addEventListener('click', e => {
  if (e.target === modalBg) {
    closeModal()
  }
});

close.addEventListener('click', e => closeModal());

///////////////////////////////////////////////////////////////////////////////

function populateModal(teamNumber) {
  let row = document.getElementById(`t${teamNumber}`);
  let rowOverall = row.querySelector('td:nth-child(4)');
  let info = teamInfo[`t${teamNumber}`];

  modalNav.style.visibility = 'visible';
  modalTeamNumber.innerHTML = teamNumber;
  modalTeamName.innerHTML = `${info.name} <small>${info.location}</small>`;
  modalOverall.innerHTML = rowOverall.innerHTML;

  modalColumn.forEach((td, i) => {
    let tdEvent = row.querySelector(`td:nth-child(${i + 6})`);
    td.innerHTML = tdEvent.innerHTML;
    td.className = tdEvent.className;
  });

  modalLinks.forEach((a, i) => { a.href = `#${teamNumber}-${i}` });
  modalBody.scrollLeft = 0;
  modalNav.scrollTop = 0;
  smith.className = 'visible';

  nonModalFocusables.forEach(tag => tag.setAttribute('tabindex', '-1'));
  wrapper.setAttribute('aria-hidden', 'true');
  close.focus();
}

function updateModalState() {
  let hashString = location.hash.substring(1).split('-');
  let teamNumber = parseInt(hashString[0]);
  let eventIndex = parseInt(hashString[1]);

  let oldModalTeamNumber = currentModalTeamNumber;
  currentModalTeamNumber = teamNumber;

  if (isNaN(teamNumber) || document.getElementById(`t${teamNumber}`) === null) {
    smith.className = '';
    modalNav.style.visibility = 'hidden';
    modalBack.style.display = 'none';
    wrapper.removeAttribute('aria-hidden');
    nonModalFocusables.forEach(tag => tag.removeAttribute('tabindex'));
    if (oldModalTeamNumber) {
      document.getElementById(`t${oldModalTeamNumber}`)
              .querySelector('a').focus();
    }
    return;

  } else if (oldModalTeamNumber === currentModalTeamNumber) {
    if (isNaN(eventIndex)) {
      animateHorizontalScroll(true);
      modalNav.querySelectorAll('a')[currentModalEventIndex].focus();
    }

  } else {
    populateModal(teamNumber);
  }

  if (!isNaN(eventIndex) &&
      eventIndex >= 0 &&
      eventIndex <= teamPenaltiesIndex) {
    let noAnimation = isNaN(oldModalTeamNumber);
    currentModalEventIndex = eventIndex;
    focusArticleOnEvent(eventIndex, noAnimation);
  } else if (!window.matchMedia('(max-width: 56em)').matches) {
    focusArticleOnEvent(0);
    history.replaceState(null, '', location.href + '-0');
  }
}

window.addEventListener("beforeunload", () => { smith.className = '' });
window.addEventListener('hashchange', () => updateModalState());

updateModalState();

///////////////////////////////////////////////////////////////////////////////

function animateHorizontalScroll(reverse, noAnimation) {
  let scrollLeftMax = modalBody.scrollWidth - modalBody.clientWidth;

  if (reverse) {
    modalNav.style.visibility = 'visible';
  } else {
    modalBack.style.display = 'block';
    modalBack.focus();
  }

  function donzo() {
    if (reverse) {
      modalBody.scrollLeft = 0;
      modalBack.style.display = 'none';
    } else {
      modalBody.scrollLeft = scrollLeftMax + 100;
      modalNav.style.visibility = 'hidden';
    }
  }

  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches ||
      noAnimation || animationsDisabled) {
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

window.addEventListener('resize', () => {
  let hashString = location.hash.substring(1).split('-');
  let teamNumber = parseInt(hashString[0]);
  let eventIndex = parseInt(hashString[1]);

  if (teamNumber && document.getElementById(`t${teamNumber}`) !== null) {
    if (!isNaN(eventIndex) &&
        eventIndex >= 0 &&
        eventIndex <= teamPenaltiesIndex) {
      let scrollLeftMax = modalBody.scrollWidth - modalBody.clientWidth;
      modalBody.scrollLeft = scrollLeftMax + 100;
    } else if (!window.matchMedia('(max-width: 56em)').matches) {
      focusArticleOnEvent(0);
      history.replaceState(null, '', location.href + '-0');
    }

    if (!window.matchMedia('(max-width: 56em)').matches) {
      modalBack.style.display = 'none';
      modalNav.style.visibility = 'visible';
    } else if (!isNaN(eventIndex) &&
               eventIndex >= 0 &&
               eventIndex <= teamPenaltiesIndex) {
      modalBack.style.display = 'block';
      modalNav.style.visibility = 'hidden';
    } else {
      modalBack.style.display = 'none';
      modalNav.style.visibility = 'visible';
    }
  }

  if (!window.matchMedia('(max-width: 28em)').matches) {
    focusOverflow.forEach(tag => { tag.style.display = '' });
  } else if (parseInt(focusSelect.value) !== 0) {
    focusOverflow.forEach(tag => { tag.style.display = 'none' });
  }
});

function getOrdinal(n) {
  let s = ["th", "st", "nd", "rd"],
  v = n%100;
  return n+(s[(v-20)%10]||s[v]||s[0]);
}

function populateOverall(teamNumber) {
  let team = teamInfo[`t${teamNumber}`];
  let tournamentName = document.querySelector('h1').textContent;

  modalOverallInfo.style.display = 'block';
  modalPlacingInfo.style.display = 'none';
  modalH3.innerHTML = 'Overall Rank';

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
  updateOverallChart(team);
}

function populatePenalties(teamNumber) {
  let team = teamInfo[`t${teamNumber}`];

  modalOverallInfo.style.display = 'none';
  modalPlacingInfo.style.display = 'none';
  modalH3.innerHTML = 'Team Penlties';

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

function populatePlacing(eventIndex, teamNumber) {
  let placing = placingInfo[`t${teamNumber}e${eventIndex}`];
  let _event = eventInfo[`e${eventIndex}`];

  modalOverallInfo.style.display = 'none';
  modalPlacingInfo.style.display = 'block';

  if (_event.trial) {
    modalH3.innerHTML = _event.name + ' (Trial)';
  } else if (_event.trialed) {
    modalH3.innerHTML = _event.name + ' (Trialed)';
  } else {
    modalH3.innerHTML = _event.name;
  }

  if (placing.disqualified) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} were <b>disqualified</b> from the event
    ${_event.name}, adding <b>${placing.points} points</b> toward their team's
    point total.`;

  } else if (placing.did_not_participate) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} <b>did not participate</b> in the event
    ${_event.name}, adding <b>${placing.points} points</b> toward their team's
    point total.`;

  } else if (placing.participation_only) {
    modalP.innerHTML = `
    Students from Team ${teamNumber} earned <b>participation-only</b> points in
    the event ${_event.name}, adding <b>${placing.points} points</b> toward their
    team's point total.`;

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
    ${_event.participation_count}</b> participating teams in the event
    ${_event.name}, earning <b>${placing.points} point${placing.points === 1 ?
    '' : 's'}</b> toward their team's point total.`;
  }

  if (_event.trial) {
    modalP.innerHTML = modalP.innerHTML + `
    (As a Trial event, ${_event.name} did not add points to any team's total.)`;

  } else if (_event.trialed) {
    modalP.innerHTML = modalP.innerHTML + `
    (As a Trialed event, ${_event.name} did not add points to any team's total,
    due to unforseen circumstances during the competition.)`;
  }

  updateEventChart(_event, placing);

  mdDeetz[0].innerHTML = placing.medal ? 'Yes':'No';
  mdDeetz[1].innerHTML = placing.exempt ? 'Yes':'No';
  mdDeetz[2].innerHTML = placing.dropped_as_part_of_worst_placings ? 'Yes':'No';
  mdDeetz[3].innerHTML = placing.points_limited_by_maximum_place ? 'Yes':'No';
  mdDeetz[4].innerHTML = placing.points_affected_by_exhibition ? 'Yes':'No';
  mdDeetz[5].innerHTML = placing.isolated_points;
}

function focusArticleOnEvent(eventIndex, noAnimation) {
  let teamNumber = parseInt(modalTeamNumber.innerHTML);
  if (!animationsDisabled) { modalArticle.scrollTop = 0; }

  if (eventIndex === 0) {
    populateOverall(teamNumber);
  } else if (eventIndex === teamPenaltiesIndex) {
    populatePenalties(teamNumber);
  } else {
    populatePlacing(eventIndex, teamNumber);
  }

  if (window.matchMedia('(max-width: 56em)').matches) {
    animateHorizontalScroll(false, noAnimation);
  }
}


modalNav.addEventListener('click', e => {
  let row = e.target.closest('tr');
  if (row) {
    location.hash = row.querySelector('a').getAttribute('href');
    e.preventDefault();
  }
});

modalBack.addEventListener('click', () => {
  location.hash = location.hash.split('-')[0];
});

///////////////////////////////////////////////////////////////////////////////

function updateBasedOnQueryString() {
  let search = new URLSearchParams(location.search);

  let oldFocusVal = focusSelect.value;
  if (search.has('focus')) {
    focusSelect.value = search.get('focus');
  } else {
    focusSelect.value = 0;
  }
  if (oldFocusVal !== focusSelect.value) {
    focusOnEvent(parseInt(focusSelect.value));
  }

  let oldSortVal = sortSelect.value;
  if (search.has('sort')) {
    sortSelect.value = search.get('sort');
  } else {
    sortSelect.value = 'by Rank';
  }
  if (oldSortVal !== sortSelect.value) {
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

firstTableFocusable.onfocus = () => { wrapper.scrollTop = 0 };
firstModalNavFocusable.onfocus = () => { modalNav.scrollTop = 0 };

////////////////////////////////////////////////////////////////////////////////

function updateOverallChart(team) {
  let data = {
    series: [
      [{ x: team.rank, y: team.points }],
      Object.entries(teamInfo).map(t => ({ x: t[1].rank, y: t[1].points }))
    ]
  };

  if (overallChart) {
    overallChart.update(data);
  } else {
    let options = {
      low: 0,
      showLine: false,
      axisX: {
        type: Chartist.AutoScaleAxis,
        onlyInteger: true
      },
      axisY: {
        onlyInteger: true
      }
    };
    overallChart = new Chartist.Line('#overallInfo .ct-chart', data, options);
  }
}

function updateEventChart(_event, placing) {
  let raws = _event.raws;
  if (raws.length === 0) {
    eventChartContainer.style.display = 'none';
    rawDeetz.innerHTML = 'Raw scores were not released for this event.';
    return;
  } else {
    eventChartContainer.style.display = 'block';
  }

  let highlight;
  if (placing.place) {
    let raw = raws[placing.place-1];

    if (raw.tiebreaker_rank === 1) {
      rawDeetz.innerHTML = `
      They earned a <b>score of ${raw.score}</b> in <b>Tier ${raw.tier}</b> and
      <b>won the tiebreaker</b> (if any).
      `;
    } else {
      rawDeetz.innerHTML = `
      They earned a <b>score of ${raw.score}</b> in <b>Tier ${raw.tier}</b> and
      ranked <b>${getOrdinal(raw.tiebreaker_rank)} in the tiebreaker</b>.
      `;
    }

    highlight = [{ x: raw.place, y: raw.score }];
  } else {
    rawDeetz.innerHTML = '';
    highlight = [];
  }

  let data = {
    series: [
      highlight,
      raws.map(r => ({ x: r.place, y: r.score }))
    ]
  };

  let options = {
    low: Math.min(0, ...raws.map(r => r.score)),
    showLine: false,
    axisX: {
      type: Chartist.AutoScaleAxis,
      onlyInteger: true
    }
  };

  if (eventChart) {
    eventChart.update(data, options);
  } else {
    eventChart = new Chartist.Line('#placingInfo .ct-chart', data, options);
  }
}

////////////////////////////////////////////////////////////////////////////////

window.addEventListener('touchstart', e => {
  startX = e.touches[0].clientX;
});

window.addEventListener('touchend', e => {
  if (Math.abs(startX - e.changedTouches[0].clientX) > 50) {
    modalBg.style.transition = 'none';
    animationsDisabled = true;
    window.setTimeout(() => {
      modalBg.style.transition = '';
      animationsDisabled = false;
    }, 150);
  }
});
