// debounce https://stackoverflow.com/questions/23181243/throttling-a-mousemove-event-to-fire-no-more-than-5-times-a-second
// ignore fast events, good for capturing double click
// @param (callback): function to be run when done
// @param (delay): integer in milliseconds
// @param (id): string value of a unique event id
// @doc (event.timeStamp): http://api.jquery.com/event.timeStamp/
// @bug (event.currentTime): https://bugzilla.mozilla.org/show_bug.cgi?id=238041
var ignoredEvent = (function () {
  var last = {},
    diff,
    time;

  return function (callback, delay, id) {
    time = new Date().getTime();
    id = id || 'ignored event';
    diff = last[id] ? time - last[id] : time;

    if (diff > delay) {
      last[id] = time;
      callback();
    }
  };
})();

var overLapping = function (a, b) {
  var aRect = a.getBoundingClientRect();
  var bRect = b.getBoundingClientRect();

  return (
    !(
      aRect.right < bRect.left ||
      aRect.left > bRect.right ||
      aRect.bottom < bRect.top ||
      aRect.top > bRect.bottom
    ) &&
    /// determine overlapping by area
    (Math.abs(aRect.left - bRect.left) + Math.abs(aRect.right - bRect.right)) /
      Math.max(aRect.width, bRect.width) <
      0.5 &&
    (Math.abs(aRect.bottom - bRect.bottom) + Math.abs(aRect.top - bRect.top)) /
      Math.max(aRect.height, bRect.height) <
      0.5
  );
};

var searchIntersections = function (root) {
  let parent = undefined,
    current = root;
  while (current) {
    if (current.classList.contains('typst-group')) {
      parent = current;
      break;
    }
    current = current.parentElement;
  }
  if (!current) {
    console.log('no group found');
    return;
  }
  const group = parent;
  const children = group.children;
  const childCount = children.length;

  const res = [];

  for (let i = 0; i < childCount; i++) {
    const child = children[i];
    if (!overLapping(child, root)) {
      continue;
    }
    res.push(child);
  }

  return res;
};

var getRelatedElements = function (event) {
  let relatedElements = event.target.relatedElements;
  if (relatedElements === undefined || relatedElements === null) {
    relatedElements = event.target.relatedElements = searchIntersections(event.target);
  }
  return relatedElements;
};

var linkmove = function (event) {
  ignoredEvent(
    function () {
      const elements = getRelatedElements(event);
      if (elements === undefined || elements === null) {
        return;
      }
      for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        if (elem.classList.contains('hover')) {
          continue;
        }
        elem.classList.add('hover');
      }
    },
    200,
    'mouse-move',
  );
};

var linkleave = function (event) {
  const elements = getRelatedElements(event);
  if (elements === undefined || elements === null) {
    return;
  }
  for (var i = 0; i < elements.length; i++) {
    var elem = elements[i];
    if (!elem.classList.contains('hover')) {
      continue;
    }
    elem.classList.remove('hover');
  }
};

function findAncestor(el, cls) {
  while (el && !el.classList.contains(cls)) el = el.parentElement;
  return el;
}

window.typstProcessSvg = function (docRoot) {
  var elements = docRoot.getElementsByClassName('pseudo-link');

  for (var i = 0; i < elements.length; i++) {
    var elem = elements[i];
    elem.addEventListener('mousemove', linkmove);
    elem.addEventListener('mouseleave', linkleave);
  }

  if (true) {
    setTimeout(() => {
      window.layoutText(docRoot);
    }, 0);
  }

  docRoot.addEventListener('click', event => {
    let elem = event.target;
    const origin = elem.closest(`a`);
    
    // override target _blank
    if (origin && !origin.getAttribute('onclick')) {
      if (origin.getAttribute('target') === '_blank') {
        // remove the target attribute
        origin.removeAttribute('target');
      }
      return;
    }

    while (elem) {
      const span = elem.getAttribute('data-span');
      if (span) {
        console.log('source-span of this svg element', span);

        const docRoot = document.body || document.firstElementChild;
        const basePos = docRoot.getBoundingClientRect();

        const vw = window.innerWidth || 0;
        const left = event.clientX - basePos.left + 0.015 * vw;
        const top = event.clientY - basePos.top + 0.015 * vw;

        triggerRipple(
          docRoot,
          left,
          top,
          'typst-debug-react-ripple',
          'typst-debug-react-ripple-effect .4s linear',
        );
        return;
      }
      elem = elem.parentElement;
    }
  });

  if (window.location.hash) {
    console.log('hash', window.location.hash);

    // parse location.hash = `loc-${page}x${x.toFixed(2)}x${y.toFixed(2)}`;
    const hash = window.location.hash;
    const hashParts = hash.split('-');
    if (hashParts.length === 2 && hashParts[0] === '#loc') {
      const locParts = hashParts[1].split('x');
      if (locParts.length === 3) {
        const page = Number.parseInt(locParts[0]);
        const x = Number.parseFloat(locParts[1]);
        const y = Number.parseFloat(locParts[2]);
        window.handleTypstLocation(docRoot, page, x, y);
      }
    }
  }
};

window.layoutText = function (svg) {
  const divs = svg.querySelectorAll('.tsel');
  const ctx = document.createElementNS('http://www.w3.org/1999/xhtml', 'canvas').getContext('2d');

  const layoutBegin = performance.now();

  for (let d of divs) {
    if (d.getAttribute('data-typst-layout-checked')) {
      continue;
    }

    if (d.style.fontSize) {
      const foreignObj = d.parentElement;
      const innerText = d.innerText;
      const targetWidth = Number.parseFloat(foreignObj.getAttribute('width'));
      const currentX = Number.parseFloat(foreignObj.getAttribute('x')) || 0;
      ctx.font = `${d.style.fontSize} sans-serif`;
      const selfWidth = ctx.measureText(innerText).width;

      const scale = targetWidth / selfWidth;

      d.style.transform = `scaleX(${scale})`;
      foreignObj.setAttribute('width', selfWidth);
      foreignObj.setAttribute('x', currentX - (selfWidth - targetWidth) * 0.5);

      d.setAttribute('data-typst-layout-checked', '1');
    }
  }

  // console.log(`layoutText used time ${performance.now() - layoutBegin} ms`);
};

window.handleTypstLocation = function (elem, page, x, y) {
  const docRoot = findAncestor(elem, 'typst-doc');
  const children = docRoot.children;
  let nthPage = 0;
  for (let i = 0; i < children.length; i++) {
    if (children[i].tagName === 'g') {
      nthPage++;
    }
    if (nthPage == page) {
      const pageElem = children[i];
      const dataWidth = pageElem.getAttribute('data-page-width');
      const dataHeight = pageElem.getAttribute('data-page-height');
      const rect = pageElem.getBoundingClientRect();
      // const xOffsetInner = Math.max(0, x / dataWidth - 0.05) * rect.width;
      const xOffsetInner = (x / dataWidth) * rect.width;
      const yOffsetInner = Math.max(0, y / dataHeight - 0.1) * rect.height;
      const xOffsetInnerFix = (x / dataWidth) * rect.width - xOffsetInner;
      const yOffsetInnerFix = (y / dataHeight) * rect.height - yOffsetInner;

      const docRoot = document.body || document.firstElementChild;
      const basePos = docRoot.getBoundingClientRect();

      const xOffset = rect.left - basePos.left + xOffsetInner;
      const yOffset = rect.top - basePos.top + yOffsetInner;
      const left = xOffset + xOffsetInnerFix;
      const top = yOffset + yOffsetInnerFix;

      const vw = window.innerWidth || 0;
      window.scrollTo(xOffset, yOffset - 0.1 * vw);

      triggerRipple(
        docRoot,
        left + 25, // centralize the ripple
        top - 25,
        'typst-jump-ripple',
        'typst-jump-ripple-effect .4s linear',
      );

      // todo: multiple documents
      location.hash = `loc-${page}x${x.toFixed(2)}x${y.toFixed(2)}`;
      return;
    }
  }
};

function triggerRipple(docRoot, left, top, className, animation) {
  const ripple = document.createElement('div');

  ripple.className = className;
  ripple.style.left = left.toString() + 'px';
  ripple.style.top = top.toString() + 'px';

  docRoot.appendChild(ripple);

  ripple.style.animation = animation;
  ripple.onanimationend = () => {
    docRoot.removeChild(ripple);
  };
}
