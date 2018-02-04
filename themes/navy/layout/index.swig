<div id="banner-outer">
  <header id="banner" class="wrapper">
    <div class="inner">
      <h2 id="banner-title">{{ page.subtitle }}</h2>
      <div id="banner-share">{{ partial('partial/share') }}</div>
    </div>
  </header>
</div>
<div id="content-wrap">
  <div class="wrapper">
    <div class="inner">
      {{ page.content }}
      <div id="intro-get-started-wrap">
        <a href="docs/a1.why_rust.html" id="intro-get-started-link">{{ __('index.get_started') }}</a>
      </div>
    </div>
  </div>
</div>