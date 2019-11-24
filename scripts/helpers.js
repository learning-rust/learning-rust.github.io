/* global hexo */

'use strict';

const { basename } = require('path');
const cheerio = require('cheerio');
const lunr = require('lunr');
const full_url_for = hexo.extend.helper.get('full_url_for').bind(hexo);

const localizedPath = ['docs'];

hexo.extend.helper.register('page_nav', function() {
  const type = this.page.canonical_path.split('/')[0];
  const sidebar = this.site.data.sidebar[type];
  const path = basename(this.path);
  const list = {};
  const prefix = 'sidebar.' + type + '.';

  for (let i in sidebar) {
    for (let j in sidebar[i]) {
      list[sidebar[i][j]] = j;
    }
  }

  const keys = Object.keys(list);
  const index = keys.indexOf(path);
  let result = '';

  if (index > 0) {
    result += `<a href="${keys[index - 1]}" class="article-footer-prev" title="${this.__(prefix + list[keys[index - 1]])}"><i class="fa fa-chevron-left"></i><span>${this.__('page.prev')}</span></a>`;
  }

  if (index < keys.length - 1) {
    result += `<a href="${keys[index + 1]}" class="article-footer-next" title="${this.__(prefix + list[keys[index + 1]])}"><span>${this.__('page.next')}</span><i class="fa fa-chevron-right"></i></a>`;
  }

  return result;
});

hexo.extend.helper.register('doc_sidebar', function(className) {
  const type = this.page.canonical_path.split('/')[0];
  const sidebar = this.site.data.sidebar[type];
  const path = basename(this.path);
  let result = '';
  const self = this;
  const prefix = 'sidebar.' + type + '.';

  if (typeof sidebar === 'undefined') {
    return '';
  }

  for (let [title, menu] of Object.entries(sidebar)) {
    result += '<strong class="' + className + '-title">' + self.__(prefix + title) + '</strong>';

    for (let [text, link] of Object.entries(menu)) {
      let itemClass = className + '-link';
      if (link === path) itemClass += ' current';

      result += '<a href="' + link + '" class="' + itemClass + '">' + self.__(prefix + text) + '</a>';
    }
  }

  return result;
});

hexo.extend.helper.register('header_menu', function(className) {
  const menu = this.site.data.menu;
  let result = '';
  const self = this;
  const lang = this.page.lang;
  const isEnglish = lang === 'en';

  for (let [title, path] of Object.entries(menu)) {
    if (!isEnglish && ~localizedPath.indexOf(title)) path = lang + path;

    result += `<a href="${self.url_for(path)}" class="${className}-link">${self.__('menu.' + title)}</a>`;
  }

  return result;
});

hexo.extend.helper.register('canonical_url', function(lang) {
  let path = this.page.path;
  if (lang && lang !== 'en') path = lang + '/' + path;

  return full_url_for(path);
});

hexo.extend.helper.register('url_for_lang', function(path) {
  const lang = this.page.lang;
  let url = this.url_for(path);

  if (lang !== 'en' && url[0] === '/') url = '/' + lang + url;

  return url;
});

hexo.extend.helper.register('raw_link', (path) => `https://github.com/learning-rust/site/edit/master/source/${path}`);

hexo.extend.helper.register('page_anchor', function(str) {
  const $ = cheerio.load(str, {decodeEntities: false});
  const headings = $('h1, h2, h3, h4, h5, h6');

  if (!headings.length) return str;

  headings.each(function() {
    const id = $(this).attr('id');

    $(this)
      .addClass('article-heading')
      .append(`<a class="article-anchor" href="#${id}" aria-hidden="true"></a>`);
  });

  return $.html();
});

hexo.extend.helper.register('lunr_index', function(data) {
  const index = lunr(function() {
    this.field('name', {boost: 10});
    this.field('tags', {boost: 50});
    this.field('description');
    this.ref('id');

    data.concat().sort((a, b) => {
      if (a.name > b.name) {
        return 1;
      } else if (b.name > a.name) {
        return -1;
      }
      return 0;
    }).forEach((item, i) => {
      const object = Object.assign({}, { id: i }, item);
      this.add(object);
    });
  });

  return JSON.stringify(index);
});

// Will be replace with full_url_for after hexo v4 release
hexo.extend.helper.register('canonical_path_for_nav', function() {
  const path = this.page.canonical_path;

  if (path.startsWith('docs/')) return path;
  return '';
});

hexo.extend.helper.register('lang_name', function(lang) {
  const data = this.site.data.languages[lang];
  return data.name || data;
});
