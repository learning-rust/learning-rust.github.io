{% set page.title = 'News' %}
<div id="content-wrap">
  <div class="wrapper">
    <div class="inner">
      {% for post in page.posts %}
        {% if loop.index == 1 %}
          {{ partial('partial/post', {post: post}) }}
        {% else %}
          <article class="article post archive-post" itemscope itemtype="http://schema.org/Article">
            <a class="archive-post-link" href="{{ url_for(post.path) }}">
              <strong class="archive-post-title" itemprop="name">{{ post.title }}</strong>
              <span class="archive-post-date">{{ time_tag(post.date) }}</span>
            </a>
          </article>
        {% endif %}
      {% endfor %}
    </div>
  </div>
</div>