<head prefix="og: http://ogp.me/ns#">
  <meta charset="utf-8">
  <meta http-equiv="x-ua-compatible" content="ie=edge">
  <title>{% if page.title %}{{ page.title }} | {% endif %}{{ config.title }}</title>
  <meta name="description" content="Rust Programming Language Tutorials">
  <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
  <!-- Canonical links -->
  <link rel="canonical" href="{{ url }}">
  <!-- Alternative links -->
  {% if page.layout == 'page' or page.layout == 'index' %}
    {% for lang in site.data.languages %}
      <link rel="alternative" hreflang="{{ loop.key }}" href="{{ canonical_url(loop.key) }}">
    {% endfor %}
  {% endif %}
  <!-- Icon -->
  <link rel="apple-touch-icon" href="{{ url_for('icon.png') }}">
  <!-- CSS -->
  <!-- build:css build/css/navy.css -->
  {{ css('css/navy') }}
  <!-- endbuild -->
  <link rel="stylesheet" href="//cdn.bootcss.com/highlight.js/9.12.0/styles/tomorrow.min.css">
  <link href="https://fonts.googleapis.com/css?family=Lato:300,400,700" rel="stylesheet" type="text/css">
  <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.3.0/css/font-awesome.min.css">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/docsearch.js@2/dist/cdn/docsearch.min.css">
  <!-- Google Analytics -->
  {{ partial('google_analytics') }}
</head>
