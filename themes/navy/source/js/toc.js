(function() {
  'use strict';

  var header = document.getElementById('header');
  var headerHeight = header.clientHeight;
  var toc = document.getElementById('article-toc');
  var tocTop = document.getElementById('article-toc-top');

  if (!toc) return;

  function updateSidebarPosition() {
    var scrollTop = document.scrollingElement.scrollTop;

    if (scrollTop > headerHeight) {
      toc.classList.add('fixed');
    } else {
      toc.classList.remove('fixed');
    }
  }

  window.addEventListener('scroll', function() {
    window.requestAnimationFrame(updateSidebarPosition);
  });

  updateSidebarPosition();

  tocTop.addEventListener('click', function(e) {
    e.preventDefault();
    document.scrollingElement.scrollTop = 0;
  });
}());
