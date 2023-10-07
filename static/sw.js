const cacheName = 'learning-rust-{{ now.Format "2006-01-02" }}';
const staticAssets = [
    './',
    './index.html',
    './manifest.json',
    './docs/**/*',
    './font/*',
    './img/icon/favicon.ico',
    './img/icon/icon-16.png',
    './img/icon/icon-32.png',
    './img/icon/icon-180.png',
    './img/icon/icon-192.png',
    './img/icon/icon-512.png',
    './img/icon/icon-vector.svg',
    './img/icon/maskable-icon-192.png',
    './img/icon/maskable-icon-512.png',
    './js/base.min.js',
    './js/component/docsearch.min.js',
    './scss/base.css',
    './scss/component/docsearch.css',
    './scss/home.css',
];

self.addEventListener('install', async e => {
    const cache = await caches.open(cacheName);
    await cache.addAll(staticAssets);
    return self.skipWaiting();
});

self.addEventListener('activate', e => {
    self.clients.claim();
});

self.addEventListener('fetch', async e => {
    const req = e.request;
    const url = new URL(req.url);

    if (url.origin === location.origin) {
        e.respondWith(cacheFirst(req));
    } else {
        e.respondWith(networkFirst(req));
    }
});

async function cacheFirst(req) {
    const cache = await caches.open(cacheName);
    const cached = await cache.match(req);
    return cached || fetch(req);
}

async function networkFirst(req) {
    const cache = await caches.open(cacheName);
    try {
        const fresh = await fetch(req);
        cache.put(req, fresh.clone());
        return fresh;
    } catch (e) {
        const cached = await cache.match(req);
        return cached;
    }
}