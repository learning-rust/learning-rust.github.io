<!-- Scripts -->
<!-- build:js build/js/main.js -->
{{ js('js/lang_select') }}
{{ js('js/toc') }}
{{ js('js/mobile_nav') }}
<!-- endbuild -->
<!-- Algolia -->
{% if config.algolia[page.lang] %}
<script type="text/javascript" src="https://cdn.jsdelivr.net/docsearch.js/1/docsearch.min.js"></script>
<script src="//cdn.bootcss.com/highlight.js/9.12.0/highlight.min.js"></script>
<script src="//cdn.bootcss.com/highlight.js/9.12.0/languages/rust.min.js"></script>
<script type="text/javascript">
document.getElementById('search-input-wrap').classList.add('on');
docsearch({
  apiKey: '{{ config.algolia[page.lang].api_key }}',
  indexName: '{{ config.algolia[page.lang].index_name }}',
  inputSelector: '#search-input'
});

hljs.initHighlightingOnLoad();
</script>
{% endif %}
